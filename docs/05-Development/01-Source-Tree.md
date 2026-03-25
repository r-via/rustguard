# Source Tree

> Annotated map of the RustGuard workspace: seven crates, their responsibilities, and how they fit together.

## Overview

RustGuard is a Cargo workspace of seven crates totalling 8,500+ lines of Rust. Two crates (`rustguard-crypto`, `rustguard-core`) compile in dual `std`/`no_std` mode and are shared by both the userspace daemon and the kernel module. The remaining crates are userspace-only. The kernel module (`rustguard-kmod`) is built out-of-tree and requires a C shim alongside the Rust sources.

Dependencies flow in one direction:

```
rustguard-crypto
    └── rustguard-core
            ├── rustguard-tun
            ├── rustguard-enroll
            │       └── rustguard-daemon
            └── rustguard-cli
```

`rustguard-kmod` depends on `rustguard-crypto` and `rustguard-core` via their `no_std` feature paths; it does not link against any userspace crate.

## Top-Level Workspace

```
rustguard/
├── Cargo.toml              # workspace manifest — all seven crates listed as members
├── Cargo.lock
├── rustguard-crypto/
├── rustguard-core/
├── rustguard-tun/
├── rustguard-daemon/
├── rustguard-enroll/
├── rustguard-cli/
└── rustguard-kmod/
```

## Crate Reference

### `rustguard-crypto`

Cryptographic primitives. Dual `std`/`no_std` — suitable for both userspace and the kernel module.

| Primitive | Details |
|---|---|
| X25519 | Elliptic-curve Diffie–Hellman key exchange |
| ChaCha20-Poly1305 | AEAD cipher for transport packets |
| XChaCha20-Poly1305 | Extended-nonce AEAD, used for cookie encryption |
| HMAC-BLAKE2s | RFC 2104 double-hash (ipad/opad), used by the KDF |
| HKDF | Key derivation built on HMAC-BLAKE2s |
| TAI64N | 96-bit external timestamp format for handshake anti-replay |

All equality checks on secret data use `subtle::ConstantTimeEq`. Secret material carries `ZeroizeOnDrop`.

No `std`-only APIs appear in this crate. Every file begins with:

```rust
#![cfg_attr(not(feature = "std"), no_std)]
```

### `rustguard-core`

Noise_IKpsk2 handshake state machine and transport session management. Dual `std`/`no_std`.

| Component | Responsibility |
|---|---|
| Handshake initiator / responder | Full Noise_IK wire format: Initiation (type 1), Response (type 2) |
| `CookieChecker` | Server-side DoS protection — rotating secrets, MAC2 verification |
| `CookieState` | Client-side cookie storage and MAC2 production |
| Replay window | 2048-bit sliding window bitmap; `check()` / `update()` are separate to prevent window poisoning before AEAD verification |
| Timer state machine | Rekey after 120 s or 2⁶⁰ messages; keepalive; handshake retry with jitter; session expiry |
| Session store | Per-peer transport session indexed by sender index (CSPRNG-assigned) |

MAC1 verification always precedes DH operations — unauthenticated packets are rejected before any expensive computation. See [Code Conventions](03-Code-Conventions.md) for the full rationale.

### `rustguard-tun`

Platform I/O backends. Userspace only.

| Backend | Platform | Notes |
|---|---|---|
| `utun` | macOS | Raw kernel control sockets |
| Linux TUN | Linux | `/dev/net/tun`, `IFF_TUN \| IFF_NO_PI`, `O_CLOEXEC` |
| Multi-queue TUN | Linux | Parallel queue TUN for SMP scaling |
| AF_XDP | Linux | Kernel-bypass via XDP socket |
| io_uring | Linux | Async I/O with `io_uring` submission queues |
| BPF loader | Linux | Loads eBPF programs for AF_XDP path |

### `rustguard-daemon`

Standard `wg.conf`-compatible tunnel daemon invoked by `rustguard up`. Reads a WireGuard `.conf` file, configures the TUN interface, and runs the tunnel loop:

```
TUN read → encrypt (ChaCha20-Poly1305) → UDP send
UDP recv → decrypt → TUN write
```

Handles `SIGINT` / `SIGTERM` for clean shutdown including route table cleanup on both macOS and Linux.

### `rustguard-enroll`

Zero-config peer enrollment — no manual key exchange, no config files. Designed for homelab and IoT contexts.

| Component | Responsibility |
|---|---|
| Enrollment server | Listens for join requests; derives a per-session XChaCha20 key from the shared token |
| IP pool allocator | CIDR-based; server is assigned `.1`, clients receive sequential addresses |
| Pairing window | UNIX domain control socket; enrollment is closed by default, opened for a bounded duration via `rustguard open <seconds>` |
| Persistence | Peer state (public keys, assigned IPs) written to `~/.rustguard/state.json`; survives restarts |

The enrollment protocol is token-derived: the token never leaves the local network in plaintext. `rustguard up wg0.conf` continues to function independently of this crate.

### `rustguard-cli`

Command-line interface. Parses arguments and dispatches to the appropriate crate.

| Command | Description |
|---|---|
| `up <config>` | Start a standard `wg.conf`-based tunnel via `rustguard-daemon` |
| `serve` | Start an enrollment server (`rustguard-enroll`) |
| `join <addr>` | Enroll as a peer against a running server |
| `open <seconds>` | Open the enrollment window for the specified duration |
| `close` | Close the enrollment window immediately |
| `status` | Print enrollment window state and connected peer count |
| `genkey` | Generate a new X25519 private key (base64) |
| `pubkey` | Derive the public key from a private key read on stdin |

### `rustguard-kmod`

Out-of-tree Linux kernel module targeting kernel 6.10+. Not part of the standard Cargo workspace build — it is built with the kernel build system.

| File / Component | Description |
|---|---|
| Rust sources | Core protocol logic imported from `rustguard-crypto` / `rustguard-core` via `no_std` |
| C shim | Thin C layer compiled by the kernel build system; bridges Rust to kernel API surfaces not yet covered by the Rust-in-kernel bindings |
| `rust-toolchain.toml` | Pins the nightly compiler required for Rust-in-kernel compilation |
| `Makefile` | Drives `make KERNELDIR=...` out-of-tree build |

The kernel module eliminates the TUN copy overhead present in the userspace daemon by processing packets entirely in-kernel.

## Examples

### Navigating to a specific crate

```bash
# Inspect the crypto crate source
ls rustguard-crypto/src/

# Inspect the enrollment crate
ls rustguard-enroll/src/
```

### Building and testing a single crate during development

```bash
# Iterate on the core protocol crate
cargo test -p rustguard-core

# Check the enroll crate compiles without warnings
cargo clippy -p rustguard-enroll -- -D warnings
```

### Verifying the no_std boundary has not been broken

```bash
cargo build -p rustguard-crypto --target thumbv7em-none-eabihf
cargo build -p rustguard-core   --target thumbv7em-none-eabihf
```

These commands confirm that no `std`-only dependency has been introduced into the dual-mode crates. They are part of the pre-release checklist.

### Building the kernel module

```bash
cd rustguard-kmod
make KERNELDIR=/lib/modules/$(uname -r)/build
```

Requires Linux 6.10+ kernel headers and a nightly Rust toolchain configured in `rust-toolchain.toml`.

## See Also

- [System Overview](../02-Architecture/01-System-Overview.md) — high-level component diagram
- [Build and Test](02-Build-and-Test.md) — full build commands, test runner, benchmark infrastructure
- [Code Conventions](03-Code-Conventions.md) — `no_std` patterns, security-critical rules, crate dependency constraints
- [Release Process](04-Release-Process.md) — workspace versioning and pre-release checklist