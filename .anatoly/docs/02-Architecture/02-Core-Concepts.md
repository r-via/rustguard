# Core Concepts

> Glossary and explanation of the key domain concepts, protocol terms, and security primitives used across RustGuard.

## Overview

RustGuard implements the WireGuard protocol from scratch across seven crates. This page defines the terminology used throughout the documentation and explains the cryptographic, protocol, and operational concepts that appear in architecture diagrams, API references, and guides. For structural context, see [System Overview](01-System-Overview.md). For how these concepts interact at runtime, see [Data Flow](03-Data-Flow.md).

---

## Cryptographic Primitives

RustGuard's cryptographic layer lives entirely in `rustguard-crypto`, written as a `no_std`-compatible crate shared by both the userspace daemon and the kernel module.

### X25519
Elliptic-curve Diffie–Hellman over Curve25519. Used during the Noise_IKpsk2 handshake to derive shared secrets from static and ephemeral key pairs. Both initiator and responder generate an ephemeral X25519 keypair per handshake; the static keypairs are long-lived peer identities.

### ChaCha20-Poly1305
Authenticated encryption with associated data (AEAD). All transport packets are encrypted with ChaCha20-Poly1305. The nonce is a 64-bit counter derived from the session send counter. The 16-byte Poly1305 authentication tag is appended to every ciphertext. XChaCha20-Poly1305 (extended 192-bit nonce) is used separately for cookie encryption.

### HMAC-BLAKE2s
RFC 2104 HMAC construction over BLAKE2s-256. Used as the pseudorandom function (PRF) throughout the HKDF key derivation chain and for computing MAC1. The initial implementation incorrectly used keyed BLAKE2s directly; this was corrected in the Phase 2 security audit (see [Design Decisions — ADR-004](04-Design-Decisions.md)).

### HKDF
HMAC-based Key Derivation Function. Derives session keys, chaining keys, and other key material from the handshake's running hash and DH outputs. All calls use HMAC-BLAKE2s as the underlying PRF.

### TAI64N
A 96-bit external timestamp format (64-bit seconds since TAI epoch + 32-bit nanoseconds). Embedded in handshake initiation messages. The responder rejects any initiation whose TAI64N timestamp is not strictly greater than the most recent accepted timestamp from that initiator, preventing replay of captured initiation packets.

---

## Handshake Protocol

### Noise_IKpsk2
The WireGuard handshake pattern. `I` denotes that the initiator's static key is sent during the handshake (encrypted); `K` denotes that the responder's static key is known to the initiator in advance (pre-configured). `psk2` places the pre-shared key (PSK) mix at the second message position.

A PSK of all-zeroes is functionally equivalent to no PSK — the `psk2` modifier is backward-compatible. Peers that do not configure a PSK implicitly use the zero PSK.

### Initiator / Responder
The two roles in a Noise_IKpsk2 handshake. The **initiator** opens a new session by sending a type-1 Initiation message. The **responder** replies with a type-2 Response message. Either peer can act as initiator; roles are per-handshake, not per-peer.

### MAC1 / MAC2
Two message authentication codes present on handshake messages.

- **MAC1** is always present. It is an HMAC-BLAKE2s over the message using a key derived from the responder's public key. The responder verifies MAC1 as the *first* operation — before any DH computation — ensuring unauthenticated packets are rejected cheaply.
- **MAC2** is required when the responder is under load. It is a cookie-keyed MAC that proves the sender's IP address is reachable.

### Cookie Mechanism
RustGuard's DoS protection layer, comprising two components in `rustguard-core`:

- **`CookieChecker`** (server side): holds a rotating random secret. When under load, it sends a type-3 Cookie Reply message encrypted with XChaCha20-Poly1305. The cookie is derived from the client's IP address and the rotating secret.
- **`CookieState`** (client side): stores the most recently received cookie and includes it as MAC2 in subsequent handshake messages.

### Sender Index / Receiver Index
Four-byte identifiers assigned by each peer to tag its outbound sessions. The sender index is chosen using a CSPRNG and is carried in every wire message. On receipt, the receiver looks up the session by the receiver index it previously assigned.

---

## Transport Sessions

### Session
A pair of symmetric keys — `T_send` and `T_recv` — derived at the end of a successful Noise_IKpsk2 handshake via HKDF. Each key is used with an independent 64-bit counter. Sessions are managed by `rustguard-core`.

### Send Counter / Nonce
A 64-bit integer incremented atomically for every encrypted transport packet. The nonce passed to ChaCha20-Poly1305 is constructed from this counter. A session is torn down and renegotiated before the counter reaches 2⁶⁰ to prevent nonce reuse.

### AllowedIPs
A per-peer set of CIDR prefixes. On the outbound path, `rustguard-core` selects which peer session to use based on the packet's destination IP matched against AllowedIPs. On the inbound path, the decrypted packet's source IP is verified against the sending peer's AllowedIPs; mismatches are dropped.

---

## Replay Protection

### Replay Window
A 2048-bit sliding bitmap maintained per session in `rustguard-core`. Each bit corresponds to one receive counter position relative to the highest counter seen. Duplicate or out-of-order packets within the window are detected and dropped.

The replay window exposes two distinct operations — by design (see [Design Decisions — ADR-009](04-Design-Decisions.md)):

| Operation | Timing | Purpose |
|---|---|---|
| `check()` | Before AEAD decryption | Reject already-seen counters without touching window state |
| `update()` | After successful AEAD decryption | Advance the bitmap |

Calling `update()` only after verified decryption prevents an attacker from poisoning the window with garbage packets carrying arbitrary counter values.

---

## TUN Backends

### TUN Device
A virtual network interface whose packets are readable and writable as file descriptors in userspace. RustGuard reads plaintext IP packets from the TUN device (outbound path) and writes decrypted packets to it (inbound path). Managed by `rustguard-tun`.

### utun (macOS)
The macOS kernel control socket interface for creating virtual network interfaces. RustGuard creates and manages a `utunN` interface directly via raw `PF_SYSTEM` / `SYSPROTO_CONTROL` socket calls without any helper library.

### Multi-Queue TUN
A Linux TUN configuration that opens multiple file descriptors to the same interface, allowing parallel packet processing across CPU cores. Enabled via the `IFF_MULTI_QUEUE` flag.

### AF_XDP
A Linux socket family providing zero-copy packet I/O between userspace and the NIC driver, bypassing the kernel network stack. `rustguard-tun` includes an AF_XDP backend that eliminates the copy overhead of the standard TUN path.

### io_uring
A Linux asynchronous I/O interface. `rustguard-tun` provides an io_uring backend for TUN reads and writes, reducing syscall overhead on the hot path.

---

## Enrollment Protocol

### Enrollment Token
A shared secret passed to both `rustguard serve` (server) and `rustguard join` (client). The token is used to derive an XChaCha20-Poly1305 key that encrypts the key exchange during enrollment. No config files or manual key exchange are required.

### IP Pool
A CIDR block configured on the server (e.g., `10.150.0.0/24`). `rustguard-enroll` allocates IPs sequentially: the server takes `.1`; joining clients receive the next available address. Pool state is persisted to `~/.rustguard/state.json`.

### Pairing Window
A time-bounded enrollment window inspired by Zigbee physical presence pairing. The server starts with enrollment closed. Enrollment is explicitly opened for a configurable duration (e.g., 60 seconds) via `rustguard open <seconds>` and closes automatically on expiry. The window state is managed via an atomic timestamp. Existing peer sessions are unaffected by window state changes.

### Control Socket
A UNIX domain socket created by `rustguard-enroll` at runtime. Accepts `open`, `close`, and `status` commands while the tunnel is live, enabling runtime management without restarting the daemon.

---

## Timer State Machine

`rustguard-core` implements the WireGuard timer state machine governing session lifecycle. All timers include jitter to prevent synchronized retries across peers.

| Event | Action |
|---|---|
| 120 seconds elapsed | Initiate rekey |
| 2⁶⁰ messages sent | Initiate rekey |
| No traffic for keepalive interval | Send keepalive packet |
| No handshake response | Retry with exponential backoff + jitter |
| Session expiry | Tear down session, require new handshake |

---

## `no_std` Compatibility

`rustguard-crypto` and `rustguard-core` are written without dependencies on the Rust standard library. This allows them to be compiled directly into `rustguard-kmod` (the Linux kernel module), which cannot link against `std`. The `std` feature flag on each crate re-enables convenience trait implementations (e.g., `std::error::Error`) for userspace builds. All other crates (`rustguard-tun`, `rustguard-daemon`, `rustguard-enroll`, `rustguard-cli`) are `std`-only.

---

## Examples

The following example shows a complete zero-config enrollment flow, touching the enrollment token, pairing window, and IP pool concepts described above.

```bash
# Server: start enrollment server with a /24 IP pool
rustguard serve --pool 10.150.0.0/24 --token mysecret

# Server: open the pairing window for 60 seconds (physical-presence model)
rustguard open 60

# Client A: join through the open window
rustguard join 198.51.100.1:51820 --token mysecret
# Client A is assigned 10.150.0.2/24; server holds 10.150.0.1/24

# Client B: join while window is still open
rustguard join 198.51.100.1:51820 --token mysecret
# Client B is assigned 10.150.0.3/24

# Server: inspect current window state and peer count
rustguard status

# Server: close the window immediately (no further enrollments accepted)
rustguard close
```

The following example shows a standard `wg.conf`-compatible tunnel, using AllowedIPs and pre-shared key concepts.

```ini
[Interface]
PrivateKey = <base64-encoded X25519 private key>
Address = 10.0.0.1/24
ListenPort = 51820

[Peer]
PublicKey = <base64-encoded X25519 public key>
PresharedKey = <base64-encoded 32-byte PSK>
AllowedIPs = 10.0.0.2/32, 2001:db8::/64
Endpoint = 203.0.113.5:51820
```

```bash
# Bring up the tunnel using the config file above
rustguard up wg0.conf
```

---

## See Also

- [System Overview](01-System-Overview.md) — Crate map and component diagram showing where each concept is implemented
- [Data Flow](03-Data-Flow.md) — Runtime packet flow showing how the replay window, AllowedIPs, and session keys interact
- [Design Decisions](04-Design-Decisions.md) — ADRs explaining *why* key security decisions (MAC1 ordering, replay window split, HMAC-BLAKE2s) were made
- [Public API](../04-API-Reference/01-Public-API.md) — Exported function signatures for `rustguard-core` and `rustguard-crypto`
- [Common Workflows](../03-Guides/01-Common-Workflows.md) — Step-by-step guides for tunnel setup, enrollment, and key generation