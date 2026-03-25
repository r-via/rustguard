# Overview

> What RustGuard is, why it was built, and what it provides.

## Overview

RustGuard is a clean-room WireGuard implementation written entirely in Rust — no `libwg`, no foreign-function bindings to the reference implementation. It ships as both a cross-platform userspace daemon and an optional out-of-tree Linux kernel module targeting kernel 6.10 or later.

The project implements the full WireGuard protocol stack from cryptographic primitives up to the tunnel loop, covering the Noise_IKpsk2 handshake, ChaCha20-Poly1305 transport encryption, DoS-mitigation cookies, replay protection, and session timers. On top of the protocol core it adds a zero-config enrollment mode that eliminates manual key exchange and IP assignment.

RustGuard is structured as a Cargo workspace of seven crates, each with a clearly bounded responsibility. The workspace totals over 8,500 lines of Rust and includes 80 tests.

## Crate Structure

| Crate | Responsibility |
|---|---|
| `rustguard-crypto` | X25519, ChaCha20-Poly1305, XChaCha20-Poly1305, HMAC-BLAKE2s, HKDF, TAI64N timestamps. Compiles under both `std` and `no_std`. |
| `rustguard-core` | Noise_IKpsk2 handshake, transport sessions, 2048-bit sliding replay window, timer state machine. Dual `std`/`no_std`. |
| `rustguard-tun` | TUN device drivers: macOS `utun` (kernel control socket), Linux `/dev/net/tun`, multi-queue TUN, AF_XDP, `io_uring`, BPF loader. |
| `rustguard-daemon` | Standard `wg.conf` tunnel mode — reads a WireGuard config file and drives the tunnel loop. |
| `rustguard-enroll` | Zero-config enrollment protocol: token-derived key exchange, CIDR IP pool allocator, pairing window, runtime control socket, JSON state persistence. |
| `rustguard-cli` | CLI binary (`rustguard`) providing the `up`, `serve`, `join`, `open`, `close`, `status`, `genkey`, and `pubkey` subcommands. |
| `rustguard-kmod` | Out-of-tree Linux kernel module (Rust + C shim). Targets Linux 6.10 and later. Eliminates the TUN bottleneck by operating in-kernel. |

## Key Capabilities

### Full WireGuard Protocol

RustGuard implements the WireGuard specification in its entirety:

- **Cryptography** — X25519 Diffie-Hellman, ChaCha20-Poly1305 AEAD, HMAC-BLAKE2s (RFC 2104), HKDF key derivation, TAI64N timestamps.
- **Handshake** — Noise_IKpsk2 (initiation → response → transport), including the optional pre-shared key layer for post-quantum resistance.
- **DoS protection** — `CookieChecker` (server-side rotating secret) and `CookieState` (client-side). MAC1 is always verified before any DH computation; MAC2 is required under load.
- **Replay protection** — 2048-bit sliding window with a `check()` / `update()` split: the window is not advanced until AEAD verification succeeds.
- **Session management** — Rekey after 120 seconds or 2^60 messages, keepalive, handshake retry with jitter, session expiry.
- **Dual-stack** — IPv4 and IPv6 AllowedIPs, dual-stack TUN configuration on both Linux and macOS.

### Zero-Config Enrollment

The `rustguard-enroll` crate provides a pairing protocol that removes the manual steps of the standard WireGuard setup flow:

- The server publishes an IP pool (`--pool 10.150.0.0/24`) and a shared token.
- Clients call `rustguard join <host:port> --token <secret>` to derive an XChaCha20 key from the token and complete an authenticated key exchange.
- The server assigns the next available address from the pool; the client's tunnel comes up without any config file.
- A **Zigbee-style pairing window** (`rustguard open <seconds>` / `rustguard close`) limits when new peers can join, modelling physical-presence authorization.
- Server state — peer public keys and assigned IPs — is persisted to `~/.rustguard/state.json` and survives restarts.

### Kernel Module

`rustguard-kmod` is an out-of-tree Linux kernel module that bypasses the TUN device entirely, eliminating the kernel↔userspace copy on the data path. It targets Linux 6.10+ with `CONFIG_RUST=y`.

## Operating Modes

RustGuard exposes two mutually exclusive operating modes, both accessed through the same `rustguard` binary:

| Mode | Entry point | When to use |
|---|---|---|
| Standard tunnel | `rustguard up wg0.conf` | Migrating from upstream WireGuard, scripted deployments, maximum interoperability. |
| Zero-config enrollment | `rustguard serve` + `rustguard join` | Homelabs, ad-hoc meshes, any environment where manual key distribution is undesirable. |

## Platform Support

| Platform | Userspace daemon | Kernel module |
|---|---|---|
| Linux (kernel ≥ 6.10) | ✓ | ✓ |
| Linux (kernel < 6.10) | ✓ | ✗ |
| macOS | ✓ | ✗ |

## Examples

### Generate a key pair

```bash
rustguard genkey | tee private.key | rustguard pubkey
```

### Bring up a standard tunnel

```bash
rustguard up wg0.conf
```

### Start an enrollment server and open a pairing window

```bash
rustguard serve --pool 10.150.0.0/24 --token mysecret &
rustguard open 60
```

### Join from a client

```bash
rustguard join 203.0.113.1:51820 --token mysecret
```

## See Also

- [Installation](02-Installation.md) — prerequisites, build steps, and first-run verification.
- [Configuration](03-Configuration.md) — `wg.conf` format reference and all enrollment flags.
- [Quick Start](04-Quick-Start.md) — end-to-end tutorial bringing up a tunnel in under five minutes.
- [System Overview](../02-Architecture/01-System-Overview.md) — high-level component diagram and crate responsibilities.
- [Core Concepts](../02-Architecture/02-Core-Concepts.md) — glossary of WireGuard and RustGuard domain terms.