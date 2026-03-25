# Types and Interfaces

> Public types, structs, enums, and traits exported across RustGuard's seven crates.

## Overview

RustGuard organises its public type surface across four library crates. Each crate is independently versioned and supports dual `std`/`no_std` compilation unless stated otherwise.

| Crate | Responsibility |
|---|---|
| `rustguard-crypto` | Cryptographic primitives: keys, AEAD ciphers, KDF, timestamps |
| `rustguard-core` | Protocol state: handshake, sessions, replay window, timers, cookies |
| `rustguard-tun` | TUN/AF_XDP/io_uring device abstractions |
| `rustguard-enroll` | Zero-config enrollment: IP pool, token, peer registry |

Configuration-related types (what goes in `wg.conf` or `state.json`) are covered in [Configuration Schema](02-Configuration-Schema.md). CLI command signatures are covered in [Public API](01-Public-API.md).

---

## Cryptographic Types (`rustguard-crypto`)

### `PrivateKey` and `PublicKey`

X25519 Diffie-Hellman key types. Both are 32-byte newtypes. `PrivateKey` implements `ZeroizeOnDrop` to scrub key material from memory on drop.

```rust
pub struct PrivateKey([u8; 32]);   // X25519 scalar — ZeroizeOnDrop
pub struct PublicKey([u8; 32]);    // X25519 point
```

`PrivateKey` is generated via the system CSPRNG (backed by the `getrandom` crate). `PublicKey` is derived from a `PrivateKey` via the X25519 base-point multiplication.

```rust
let private_key = PrivateKey::generate();
let public_key  = PublicKey::from(&private_key);

// Base64 round-trip (used in wg.conf)
let encoded  = private_key.to_base64();
let restored = PrivateKey::from_base64(&encoded)?;
```

### `PresharedKey`

Optional 32-byte symmetric key for the PSK layer in `Noise_IKpsk2`. Implements `ZeroizeOnDrop`.

```rust
pub struct PresharedKey([u8; 32]);  // ZeroizeOnDrop
```

When absent, the handshake falls back to a zero-filled PSK, preserving protocol compatibility.

### `Tai64N`

TAI64N external timestamp used in handshake initiation messages to prevent replay attacks. Encodes seconds since the TAI epoch plus nanoseconds in a fixed 12-byte big-endian format.

```rust
pub struct Tai64N {
    pub seconds:     u64,
    pub nanoseconds: u32,
}

impl Tai64N {
    pub fn now() -> Self;
    pub fn to_bytes(&self) -> [u8; 12];
    pub fn from_bytes(bytes: &[u8; 12]) -> Self;
    pub fn is_after(&self, other: &Tai64N) -> bool;
}
```

`is_after` is used during handshake validation to enforce the timestamp monotonicity requirement and reject stale initiations.

### `ChaCha20Poly1305` and `XChaCha20Poly1305`

AEAD cipher wrappers. `ChaCha20Poly1305` is the primary transport cipher. `XChaCha20Poly1305` (extended 192-bit nonce) is used for cookie encryption.

```rust
impl ChaCha20Poly1305 {
    /// Returns None on nonce exhaustion (2^64 - 1 messages).
    pub fn encrypt(
        key:        &[u8; 32],
        nonce:      u64,
        plaintext:  &[u8],
        aad:        &[u8],
    ) -> Option<Vec<u8>>;

    pub fn decrypt(
        key:        &[u8; 32],
        nonce:      u64,
        ciphertext: &[u8],
        aad:        &[u8],
    ) -> Option<Vec<u8>>;
}
```

`encrypt` returns `Option<Vec<u8>>` rather than panicking on nonce exhaustion. Callers must treat `None` as a signal to rekey immediately.

### `HmacBlake2s`

RFC 2104 HMAC constructed over BLAKE2s-256. Used in the WireGuard KDF chain. The implementation uses the canonical ipad/opad double-hash construction; keyed BLAKE2s alone is **not** equivalent.

```rust
pub fn hmac_blake2s(key: &[u8], data: &[u8]) -> [u8; 32];
pub fn hkdf_blake2s(
    chaining_key: &[u8; 32],
    input:        &[u8],
    n_outputs:    usize,
) -> Vec<[u8; 32]>;
```

---

## Protocol Types (`rustguard-core`)

### `HandshakeState`

Accumulates ephemeral key material, chaining key, transcript hash, and optional PSK across the `Noise_IK` initiation/response exchange. All sensitive fields implement `ZeroizeOnDrop`.

```rust
pub struct HandshakeState {
    // chaining_key, hash, psk — ZeroizeOnDrop
}

impl HandshakeState {
    pub fn new_initiator(
        local_static:  &PrivateKey,
        remote_static: &PublicKey,
        psk:           Option<&PresharedKey>,
    ) -> Self;

    pub fn new_responder(local_static: &PrivateKey) -> Self;

    /// Processes an inbound initiation message.
    /// MAC1 is verified *before* any DH operations are performed.
    pub fn consume_initiation(
        &mut self,
        msg: &InitiationMessage,
    ) -> Result<HandshakeResponse, HandshakeError>;

    pub fn consume_response(
        &mut self,
        msg: &ResponseMessage,
    ) -> Result<SessionKeys, HandshakeError>;
}
```

> **Security note:** MAC1 is verified before any DH operations. This prevents an attacker from burning CPU with unauthenticated packets (see Commit 5 in the README).

### `SessionKeys`

Output of a completed handshake. Contains the send and receive symmetric keys for the transport session.

```rust
pub struct SessionKeys {
    pub initiator_to_responder: [u8; 32],
    pub responder_to_initiator: [u8; 32],
    pub sender_index:           u32,
    pub receiver_index:         u32,
}
```

### `ReplayWindow`

2048-bit sliding window for anti-replay protection. The `check`/`update` split ensures the window is only advanced on successful AEAD verification, preventing replay window poisoning by garbage counters.

```rust
pub struct ReplayWindow { /* 2048-bit bitmap */ }

impl ReplayWindow {
    pub fn new() -> Self;

    /// Returns true if the counter is within the valid window and not replayed.
    /// Does NOT advance the window.
    pub fn check(&self, counter: u64) -> bool;

    /// Advances the window to include `counter`.
    /// Must only be called after AEAD decryption succeeds.
    pub fn update(&mut self, counter: u64);
}
```

Typical usage:

```rust
if replay_window.check(counter) {
    if let Some(plaintext) = ChaCha20Poly1305::decrypt(&key, counter, ciphertext, aad) {
        replay_window.update(counter);
        // deliver plaintext
    }
}
```

### `CookieChecker`

Server-side DoS protection component. Maintains a rotating secret and validates MAC2 fields on inbound handshake messages when the server is under load.

```rust
pub struct CookieChecker { /* rotating secret, last rotation time */ }

impl CookieChecker {
    pub fn new() -> Self;

    /// Rotates the internal secret. Called periodically by the timer state machine.
    pub fn rotate_secret(&mut self);

    /// Validates MAC1 and (optionally) MAC2 on an inbound message.
    pub fn validate(&self, msg: &[u8], peer_public_key: &PublicKey) -> CookieValidation;

    /// Generates a Cookie Reply message (type 3) for a sender that failed MAC2.
    pub fn make_cookie_reply(&self, initiator_addr: &SocketAddr, mac1: &[u8; 16]) -> CookieReply;
}

pub enum CookieValidation {
    Valid,
    RequiresCookie,
    Invalid,
}
```

### `CookieState`

Client-side cookie storage. Caches the most recent cookie received from a server and attaches it as MAC2 on subsequent initiations when challenged.

```rust
pub struct CookieState { /* last cookie, expiry */ }

impl CookieState {
    pub fn new() -> Self;

    /// Stores a cookie from a received Cookie Reply message.
    pub fn store(&mut self, cookie: &[u8; 16]);

    /// Returns the current valid cookie, or None if expired/absent.
    pub fn get(&self) -> Option<&[u8; 16]>;
}
```

### `TimerState`

Manages the WireGuard timer state machine: rekey intervals, keepalive scheduling, handshake retry with jitter, and session expiry.

```rust
pub struct TimerState { /* per-session timer fields */ }

impl TimerState {
    /// Returns true if a new handshake should be initiated
    /// (120s elapsed or 2^60 messages sent on the current session).
    pub fn needs_rekey(&self) -> bool;

    pub fn on_packet_sent(&mut self);
    pub fn on_packet_received(&mut self);
    pub fn on_handshake_complete(&mut self);
    pub fn next_keepalive_deadline(&self) -> Option<Instant>;
}
```

---

## TUN Device Types (`rustguard-tun`)

### `TunDevice`

Platform-abstracted TUN interface. The concrete variant is selected at compile time or runtime based on platform and feature flags.

```rust
pub enum TunDevice {
    Utun(UtunDevice),       // macOS — kernel control socket
    LinuxTun(LinuxTunDevice), // Linux /dev/net/tun, IFF_TUN | IFF_NO_PI
    MultiQueue(MultiQueueTunDevice),
    AfXdp(AfXdpSocket),
    IoUring(IoUringTun),
}

pub trait Tun {
    fn read(&self, buf: &mut [u8])  -> io::Result<usize>;
    fn write(&self, buf: &[u8])     -> io::Result<usize>;
    fn mtu(&self)                   -> io::Result<u32>;
    fn name(&self)                  -> &str;
}
```

All file descriptors are opened with `O_CLOEXEC`. File descriptors are closed and routes are cleaned up on `Drop`.

---

## Enrollment Types (`rustguard-enroll`)

### `IpPool`

CIDR-based address allocator. The server claims `.1`; subsequent `allocate()` calls return sequential addresses.

```rust
pub struct IpPool { /* base network, allocation bitmap */ }

impl IpPool {
    /// Constructs a pool from a CIDR range (e.g., "10.150.0.0/24").
    pub fn new(cidr: &str) -> Result<Self, IpPoolError>;

    /// Allocates the next available host address.
    pub fn allocate(&mut self) -> Option<Ipv4Addr>;

    /// Returns an address to the pool.
    pub fn release(&mut self, addr: Ipv4Addr);

    /// Returns the server address (.1 of the network).
    pub fn server_addr(&self) -> Ipv4Addr;
}
```

### `EnrollmentToken`

Derives the 32-byte XChaCha20-Poly1305 key used to encrypt the enrollment key exchange from a shared secret string.

```rust
pub struct EnrollmentToken([u8; 32]);  // derived key — ZeroizeOnDrop

impl EnrollmentToken {
    pub fn from_secret(secret: &str) -> Self;
}
```

---

## Examples

### Complete Enrollment Type Workflow

```rust
use rustguard_enroll::{EnrollmentToken, IpPool};
use rustguard_crypto::{PrivateKey, PublicKey};

// Server side: initialise pool and derive token key
let mut pool  = IpPool::new("10.150.0.0/24").expect("valid CIDR");
let token     = EnrollmentToken::from_secret("mysecret");
let server_ip = pool.server_addr();          // 10.150.0.1

// Allocate IP for an enrolling client
let client_ip = pool.allocate().expect("pool not exhausted"); // 10.150.0.2

// Client side: generate keypair
let client_priv = PrivateKey::generate();
let client_pub  = PublicKey::from(&client_priv);

println!("client public key: {}", client_pub.to_base64());
println!("assigned IP:       {client_ip}");
```

### Replay Window with AEAD Decryption

```rust
use rustguard_core::ReplayWindow;
use rustguard_crypto::ChaCha20Poly1305;

let mut window = ReplayWindow::new();
let key        = [0u8; 32]; // transport receive key
let counter    = 42u64;

if window.check(counter) {
    match ChaCha20Poly1305::decrypt(&key, counter, ciphertext, aad) {
        Some(plaintext) => {
            window.update(counter);
            deliver(plaintext);
        }
        None => { /* AEAD authentication failed — drop packet */ }
    }
}
```

---

## See Also

- [Public API](01-Public-API.md) — CLI commands and high-level library entry points
- [Configuration Schema](02-Configuration-Schema.md) — `wg.conf`, enrollment flags, and `state.json` schema
- [Core Concepts](../02-Architecture/02-Core-Concepts.md) — Noise_IK, PSK, replay window, and cookie mechanism explained
- [Data Flow](../02-Architecture/03-Data-Flow.md) — How these types interact during packet encryption and handshake