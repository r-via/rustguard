# Public API

> Complete reference for all exported functions and methods across the RustGuard crate family.

## Overview

RustGuard exposes a layered public API across its seven crates. Each crate is independently versioned and can be used in isolation. Crates marked **`no_std` compatible** export their full surface in both hosted and bare-metal environments.

For type definitions (structs, enums, traits), see [Types and Interfaces](./03-Types-and-Interfaces.md). For configuration schemas consumed by the daemon and enrollment commands, see [Configuration Schema](./02-Configuration-Schema.md).

| Crate | Purpose | `no_std` |
|-------|---------|----------|
| `rustguard-crypto` | Cryptographic primitives | ✓ |
| `rustguard-core` | Handshake, sessions, replay protection | ✓ |
| `rustguard-tun` | TUN/network device management | — |
| `rustguard-daemon` | Tunnel lifecycle (`wg.conf` mode) | — |
| `rustguard-enroll` | Zero-config enrollment server/client | — |

---

## rustguard-crypto

**Crate:** `rustguard-crypto` · **`no_std` compatible**

### Key Generation

#### `X25519KeyPair::generate`

Generates a new X25519 key pair using the platform CSPRNG (`getrandom`). The private key scalar is clamped per RFC 7748 before the corresponding public key is derived.

```rust
pub fn generate() -> X25519KeyPair
```

**Returns:** A new [`X25519KeyPair`](./03-Types-and-Interfaces.md#x25519keypair).

```rust
use rustguard_crypto::X25519KeyPair;

let keypair = X25519KeyPair::generate();
println!("public: {:?}", keypair.public);
// public: [0xf3, 0x2a, ..., 0x7c]
```

#### `X25519KeyPair::from_private`

Derives the public key from an existing 32-byte private key scalar. The private scalar is clamped before the public key point is computed.

```rust
pub fn from_private(private: [u8; 32]) -> X25519KeyPair
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `private` | `[u8; 32]` | Raw 32-byte private key scalar |

```rust
use rustguard_crypto::X25519KeyPair;

let private_bytes: [u8; 32] = load_from_secure_storage();
let keypair = X25519KeyPair::from_private(private_bytes);
println!("derived public: {:?}", keypair.public);
```

### AEAD Encryption

#### `ChaCha20Poly1305Key::encrypt`

Encrypts `plaintext` under the key using ChaCha20-Poly1305. The `nonce` is a 64-bit packet counter; callers must ensure it is never reused under the same key. Returns `None` when nonce exhaustion is detected, signalling that the session must be rekeyed before transmitting further packets.

```rust
pub fn encrypt(&self, plaintext: &[u8], nonce: u64, aad: &[u8]) -> Option<Vec<u8>>
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `plaintext` | `&[u8]` | Data to encrypt |
| `nonce` | `u64` | Monotonically increasing packet counter |
| `aad` | `&[u8]` | Additional authenticated data (authenticated but not encrypted) |

**Returns:** `Some(ciphertext_with_tag)` on success; `None` on nonce exhaustion.

```rust
use rustguard_crypto::ChaCha20Poly1305Key;

let key = ChaCha20Poly1305Key([0x42u8; 32]);
let plaintext = b"hello tunnel";

match key.encrypt(plaintext, 0, b"") {
    Some(ciphertext) => println!("encrypted {} bytes", ciphertext.len()),
    None => eprintln!("nonce exhausted — initiate rekey"),
}
```

#### `ChaCha20Poly1305Key::decrypt`

Decrypts and authenticates `ciphertext`. Returns `None` if the Poly1305 tag does not verify (authentication failure) or if the ciphertext is too short to contain a tag.

```rust
pub fn decrypt(&self, ciphertext: &[u8], nonce: u64, aad: &[u8]) -> Option<Vec<u8>>
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `ciphertext` | `&[u8]` | Ciphertext including the appended 16-byte Poly1305 tag |
| `nonce` | `u64` | Counter value that was used during encryption |
| `aad` | `&[u8]` | Must match the AAD used during `encrypt` |

**Returns:** `Some(plaintext)` on success; `None` on authentication failure.

### Timestamps

#### `TAI64N::now`

Returns the current TAI64N timestamp from the system clock. Embedded in handshake initiations and validated against the peer's most recently accepted timestamp to prevent handshake replay attacks.

```rust
pub fn now() -> TAI64N
```

```rust
use rustguard_crypto::TAI64N;

let ts = TAI64N::now();
println!("seconds={} nanoseconds={}", ts.seconds, ts.nanoseconds);
```

#### `TAI64N::is_after`

Returns `true` if `self` is strictly after `other`. Used to reject stale handshake initiations.

```rust
pub fn is_after(&self, other: &TAI64N) -> bool
```

### Constant-Time Equality

#### `constant_time_eq`

Compares two byte slices in constant time using `subtle::ConstantTimeEq`. The `black_box` barrier prevents LLVM from optimizing away the comparison. Returns `true` if and only if the slices have equal length and identical contents.

```rust
pub fn constant_time_eq(a: &[u8], b: &[u8]) -> bool
```

```rust
use rustguard_crypto::constant_time_eq;

let mac_received = compute_mac1(&packet);
let mac_expected = [0xabu8; 32];

if !constant_time_eq(&mac_received, &mac_expected) {
    return Err(AuthError::MacMismatch);
}
```

---

## rustguard-core

**Crate:** `rustguard-core` · **`no_std` compatible**

### Replay Window

The 2048-bit sliding window anti-replay bitmap protects transport sessions from packet replay. The `check` / `update` split ensures that the AEAD tag is verified before the window advances; advancing the window on an unauthenticated counter was a security bug fixed in Commit 5.

#### `ReplayWindow::new`

Creates a new zeroed replay window.

```rust
pub fn new() -> ReplayWindow
```

#### `ReplayWindow::check`

Tests whether `counter` is acceptable. Returns `false` if the counter falls behind the window or has already been received.

```rust
pub fn check(&self, counter: u64) -> bool
```

#### `ReplayWindow::update`

Advances the window to include `counter`. Must only be called after `check` returned `true` **and** the AEAD tag has been verified successfully.

```rust
pub fn update(&mut self, counter: u64)
```

```rust
use rustguard_core::ReplayWindow;

let mut window = ReplayWindow::new();
let counter: u64 = received_packet.counter;

if window.check(counter) {
    // Authenticate BEFORE advancing the window
    if let Some(plaintext) = session_key.decrypt(&ciphertext, counter, b"") {
        window.update(counter);
        tun.write(&plaintext)?;
    }
}
```

### Handshake

#### `Handshake::initiate`

Constructs a Noise_IKpsk2 handshake initiation message. Embeds the current `TAI64N` timestamp, encrypts the local static key under the responder's public key, and computes MAC1. If a valid cookie from a prior `CookieReply` is held, MAC2 is also appended.

```rust
pub fn initiate(
    local: &X25519KeyPair,
    remote_public: &X25519PublicKey,
    psk: Option<&[u8; 32]>,
) -> Result<(HandshakeState, Vec<u8>), HandshakeError>
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `local` | `&X25519KeyPair` | Local static key pair |
| `remote_public` | `&X25519PublicKey` | Responder's known public key |
| `psk` | `Option<&[u8; 32]>` | Optional 32-byte pre-shared key for Noise_IKpsk2 |

#### `Handshake::respond`

Processes a received initiation message and produces a response. MAC1 is verified before any DH operations are performed. Stale TAI64N timestamps are rejected to prevent handshake replay.

```rust
pub fn respond(
    local: &X25519KeyPair,
    initiation: &[u8],
    psk: Option<&[u8; 32]>,
) -> Result<(HandshakeState, Vec<u8>), HandshakeError>
```

#### `HandshakeState::finalize`

Derives the two symmetric transport session keys from a completed handshake exchange.

```rust
pub fn finalize(self) -> Result<TransportSession, HandshakeError>
```

```rust
use rustguard_core::Handshake;
use rustguard_crypto::{X25519KeyPair, X25519PublicKey};

let client_kp = X25519KeyPair::generate();
let server_kp = X25519KeyPair::generate();
let server_pub = X25519PublicKey(server_kp.public);

// Client builds initiation
let (client_state, init_bytes) = Handshake::initiate(&client_kp, &server_pub, None)?;

// Server processes initiation, produces response
let (server_state, resp_bytes) = Handshake::respond(&server_kp, &init_bytes, None)?;

// Both sides derive transport session keys
let client_session = client_state.finalize()?;
let server_session = server_state.finalize()?;
```

### Cookie Protection

#### `CookieChecker::verify_mac1`

Server-side: verifies the MAC1 field of an incoming handshake message against the server's static public key. MAC1 verification must occur before any DH operations on that message.

```rust
pub fn verify_mac1(&self, message: &[u8]) -> bool
```

#### `CookieChecker::issue_cookie_reply`

Issues a `CookieReply` (type 3) message when the server is under load. The cookie is encrypted with `XChaCha20Poly1305` under a key derived from a rotating server secret and the requester's source IP address.

```rust
pub fn issue_cookie_reply(&self, message: &[u8], sender_addr: &[u8]) -> Vec<u8>
```

#### `CookieState::consume_cookie_reply`

Client-side: decrypts and stores a received `CookieReply`. The stored cookie is subsequently included as MAC2 in the next handshake initiation.

```rust
pub fn consume_cookie_reply(&mut self, reply: &[u8]) -> Result<(), CookieError>
```

---

## rustguard-tun

**Crate:** `rustguard-tun`

### TUN Device

#### `TunDevice::open`

Opens a TUN interface. On Linux, opens `/dev/net/tun` with `IFF_TUN | IFF_NO_PI`. On macOS, opens a `utun` device via kernel control socket. All file descriptors are opened with `O_CLOEXEC`.

```rust
pub fn open(name: &str) -> Result<TunDevice, TunError>
```

#### `TunDevice::read`

Reads one IP packet from the TUN interface into `buf`. Returns the number of bytes written into `buf`.

```rust
pub fn read(&self, buf: &mut [u8]) -> Result<usize, TunError>
```

#### `TunDevice::write`

Writes one IP packet to the TUN interface from `buf`.

```rust
pub fn write(&self, buf: &[u8]) -> Result<usize, TunError>
```

```rust
use rustguard_tun::TunDevice;

let tun = TunDevice::open("wg0")?;
let mut buf = vec![0u8; 65535];

loop {
    let n = tun.read(&mut buf)?;
    let plaintext = &buf[..n];
    // encrypt plaintext and send over UDP socket
}
```

---

## rustguard-enroll

**Crate:** `rustguard-enroll`

### Enrollment Server

#### `EnrollServer::new`

Creates a new enrollment server for the given IP pool and token. Enrollment begins **closed**; call `open_window` before accepting new peers.

```rust
pub fn new(pool: Ipv4Net, token: &str) -> Result<EnrollServer, EnrollError>
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `pool` | `Ipv4Net` | CIDR block for client IP allocation; the server claims the `.1` address |
| `token` | `&str` | Shared secret used to derive the XChaCha20 enrollment key |

#### `EnrollServer::open_window`

Opens the enrollment window for `duration_secs` seconds. The window closes automatically via an atomic expiry timestamp. Existing peers are unaffected.

```rust
pub fn open_window(&self, duration_secs: u64)
```

#### `EnrollServer::close_window`

Closes the enrollment window immediately.

```rust
pub fn close_window(&self)
```

#### `EnrollServer::status`

Returns the current enrollment status, including whether the window is open, remaining seconds, and the number of enrolled peers.

```rust
pub fn status(&self) -> EnrollStatus
```

### Enrollment Client

#### `enroll_join`

Contacts the enrollment server at `endpoint` and completes the enrollment protocol. Returns a `TunnelConfig` containing the assigned IP address, server public key, and WireGuard endpoint, ready for use with `Tunnel::from_config`.

```rust
pub fn enroll_join(endpoint: SocketAddr, token: &str) -> Result<TunnelConfig, EnrollError>
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `endpoint` | `SocketAddr` | Server address and port |
| `token` | `&str` | Shared secret matching the server's enrollment token |

```rust
use rustguard_enroll::{EnrollServer, enroll_join};
use std::net::SocketAddr;

// Server side — open a 60-second enrollment window
let server = EnrollServer::new("10.150.0.0/24".parse()?, "mysecret")?;
server.open_window(60);

// Client side (separate host)
let endpoint: SocketAddr = "192.0.2.1:51820".parse()?;
let config = enroll_join(endpoint, "mysecret")?;
println!("assigned IP: {}", config.assigned_ip);
// assigned IP: 10.150.0.2
```

---

## rustguard-daemon

**Crate:** `rustguard-daemon`

### Tunnel Lifecycle

#### `Tunnel::from_config`

Parses a `wg.conf`-format config file, creates the TUN interface, and installs the routes for all configured `AllowedIPs` ranges. Both IPv4 and IPv6 CIDR entries are supported.

```rust
pub fn from_config(path: &Path) -> Result<Tunnel, DaemonError>
```

#### `Tunnel::run`

Enters the tunnel event loop: reads plaintext from TUN, encrypts and forwards over UDP; receives UDP datagrams, decrypts, and writes plaintext back to TUN. Blocks until a SIGINT or SIGTERM signal is received, after which routes are removed and the TUN interface is closed.

```rust
pub fn run(&self) -> Result<(), DaemonError>
```

```rust
use rustguard_daemon::Tunnel;
use std::path::Path;

let tunnel = Tunnel::from_config(Path::new("/etc/rustguard/wg0.conf"))?;
tunnel.run()?; // blocks until shutdown signal; cleans up routes on exit
```

---

## See Also

- [Types and Interfaces](./03-Types-and-Interfaces.md) — Struct and enum definitions for all types referenced in this API
- [Configuration Schema](./02-Configuration-Schema.md) — `wg.conf`, enrollment flags, and `state.json` reference
- [Common Workflows](../03-Guides/01-Common-Workflows.md) — Step-by-step usage guides
- [System Overview](../02-Architecture/01-System-Overview.md) — How the crates interact at a high level
- [Data Flow](../02-Architecture/03-Data-Flow.md) — Packet lifecycle through the encrypt/decrypt pipeline