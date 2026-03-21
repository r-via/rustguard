# RustGuard

A clean-room WireGuard implementation in Rust. No libwg. Cross-platform. One small exception to "pure Rust": a 73-line BPF C program (`xdp_wg.c`) for the AF_XDP packet filter, pre-compiled to a 1.4KB `.o` and embedded via `include_bytes!`.

**7,400 lines of Rust · 79 tests · 17 commits · 6 crates · 637KB static binary**

Built as an experiment to understand WireGuard internals by implementing the full protocol from scratch, then pushing userspace performance as far as it can go on commodity hardware.

## Architecture

```
rustguard-crypto/     X25519, ChaCha20-Poly1305, HMAC-BLAKE2s, HKDF, TAI64N, XChaCha20
rustguard-core/       Noise_IK handshake, transport sessions, replay window, timers, cookies
rustguard-tun/        macOS utun, Linux TUN, multi-queue TUN, AF_XDP, io_uring, BPF loader
rustguard-daemon/     Standard wg.conf tunnel mode (rustguard up)
rustguard-enroll/     Zero-config enrollment, IP pool, Zigbee-style pairing, persistence
rustguard-cli/        CLI: up, serve, join, open, close, status, genkey, pubkey
```

## The Experiment — Commit by Commit

### Phase 1: Protocol Implementation

**Commit 1 — Working tunnel in pure Rust**

Started with the WireGuard whitepaper and built up from cryptographic primitives. X25519 key exchange, ChaCha20-Poly1305 AEAD, BLAKE2s hashing with HKDF key derivation, TAI64N timestamps. Implemented the Noise_IKpsk2 handshake (initiation → response → transport), wire format serialization, and a macOS utun TUN device via raw kernel control sockets. Wired it all into a tunnel loop: TUN reads plaintext → encrypt → UDP send, UDP recv → decrypt → TUN write.

*First test: two peers on localhost, ICMP ping through utun interfaces. 1.2ms RTT. It worked.*

**Commit 2 — Replay protection, timers, Linux TUN**

Added the 2048-bit sliding window anti-replay bitmap. Built the timer state machine (rekey after 120s or 2^60 messages, keepalive, handshake retry with jitter, session expiry). Ported TUN to Linux (`/dev/net/tun` with `IFF_TUN | IFF_NO_PI`). Added signal handling for clean shutdown with route cleanup. Replaced the nanosecond clock hack for sender indices with a proper CSPRNG.

*Tested on two Debian 12 VMs on a bridged network. 0% packet loss, <1ms RTT.*

**Commit 3 — Cookie mechanism, PSK support, XChaCha20**

Implemented WireGuard's DoS protection: `CookieChecker` (server side, rotating secrets) and `CookieState` (client side). XChaCha20-Poly1305 for cookie encryption. MAC1 always present, MAC2 required under load. Added pre-shared key support with backward-compatible wrappers. Cookie Reply message (type 3) with full wire format.

**Commit 4 — IPv6 dual-stack**

Extended the tunnel to handle both IPv4 and IPv6 packets based on IP version byte inspection. Dual-stack TUN configuration on both macOS (`ifconfig inet6`) and Linux (`ip -6 addr add`). AllowedIPs parser handles both v4 and v6 CIDR ranges. Route table updated for both families.

*Dual-stack ping (v4 + v6) between Debian VMs, 0% packet loss.*

### Phase 2: Security Audit

**Commit 5 — Critical and high fixes (OWASP-style audit)**

Went through the codebase with an adversarial mindset:

- **HMAC-BLAKE2s was wrong.** Was using keyed BLAKE2s directly instead of RFC 2104 HMAC (ipad/opad double-hash). KDF output was incorrect and non-interoperable. Fixed.
- **MAC1 checked too late.** DH operations happened before MAC1 verification — an attacker could burn CPU with unauthenticated packets. Moved MAC1 check first.
- **Timestamp replay not enforced.** Stale handshake initiations were accepted, enabling session hijack via captured packets. Fixed.
- **Replay window poisoning.** Window updated before AEAD verification — garbage counters could advance the window. Split into `check()` / `update()`.
- **FD leaks** on TUN creation error paths (both platforms).
- **Linux ifreq padding** was 12 bytes instead of 20 — reading stack garbage for MTU.
- **Peer lookup operator precedence bug** for handshake responses.
- `constant_time_eq` used `black_box` to prevent LLVM from optimizing out the comparison.

**Commit 6 — Medium fixes**

Replaced hand-rolled constant-time comparisons with `subtle::ConstantTimeEq`. Swapped `/dev/urandom` for the `getrandom` crate. Added `ZeroizeOnDrop` on handshake state (chaining key, hash, PSK). `encrypt()` returns `Option` instead of panicking on nonce exhaustion. `O_CLOEXEC` on all file descriptors.

### Phase 3: Zero-Config Enrollment

**Commit 7 — serve + join**

The standard WireGuard setup flow (generate keys on both sides, exchange public keys, assign IPs, write config files) is painful for homelabs. Built a new `rustguard-enroll` crate with a custom enrollment protocol: token-derived XChaCha20 key encrypts the key exchange, CIDR-based IP pool allocator (server gets .1, clients get sequential IPs).

```
# Server
rustguard serve --pool 10.150.0.0/24 --token mysecret

# Client (any machine)
rustguard join 1.2.3.4:51820 --token mysecret
```

No config files. No manual key exchange. No IP planning. Backward compatible — `rustguard up wg0.conf` still works.

**Commit 8 — Zigbee-style pairing window**

Enrollment always-open felt wrong. Added a UNIX domain control socket for runtime management. Server starts with enrollment closed. Physical presence model:

```
rustguard open 60      # open enrollment for 60 seconds
rustguard close        # close immediately
rustguard status       # show window state + peer count
```

Window auto-closes on expiry via atomic timestamp. Existing peers unaffected.

**Commit 9 — Persistence + benchmarks**

Server state (peer keys, assigned IPs) persisted to `~/.rustguard/state.json`. Survives restarts. Added benchmark infrastructure.

### Phase 4: Performance

The interesting part. Test rig: two Debian 12 VMs on an n305 hypervisor, Intel i225 2.5G NICs passed through (`enp7s0`, `192.168.99.1/2`). All benchmarks with `iperf3`, single TCP stream unless noted.

**Baseline (no VPN):** 2,356 Mbps
**Kernel WireGuard:** 1,096 Mbps (46.5% of wire speed)

**Commit 10 — Lock optimization**

Replaced global Mutex with `RwLock` for peer table (concurrent reads for packet routing) and per-peer `Mutex` for session state. `Arc<EnrolledPeer>` for lock-free references after lookup. In-place AEAD (`seal_to` / `open_to`) to avoid heap allocation per packet.

*Result: 175 Mbps.* TUN syscall overhead dominates, not locking.

**Commit 11 — recvmmsg batched UDP**

`recvmmsg` with `MSG_WAITFORONE` — receive up to 32 datagrams per syscall on the inbound path. macOS fallback to single `recvfrom`.

*Result: 173 Mbps.* Confirmed: TUN read/write is the bottleneck, not UDP.

**Commit 12 — AF_XDP zero-copy sockets**

Built a full AF_XDP stack: UMEM allocation (mmap'd shared memory), RX/TX/Fill/Completion ring buffers with atomics, `XDP_USE_NEED_WAKEUP` for minimal syscalls, frame pool for TX buffers. All unsafe contained behind a safe Rust API.

**Commit 13 — BPF loader + XDP filter**

Wrote a minimal BPF ELF loader (no libbpf dependency) that parses section headers, extracts programs, and patches map relocations. The XDP program filters WireGuard packets (UDP port 51820, message types 1-4) and redirects them to the AF_XDP socket. Non-WireGuard traffic passes through the kernel stack normally.

**Commit 14 — AF_XDP wired into the server**

`rustguard serve --pool 10.150.0.0/24 --token s --xdp enp7s0` — the full pipeline. BPF program loaded, XDP filter attached, AF_XDP socket receiving WireGuard packets zero-copy.

**Commit 15 — BPF ELF relocation fix**

The BPF loader's relocation patching was broken on real hardware (worked in VMs by luck). Fixed the ELF section parsing to correctly patch map FD references in BPF instructions.

*AF_XDP result: ~175 Mbps.* Same as before — TUN is still the bottleneck. UDP path is now zero-copy but it doesn't matter when the slow side is TUN read/write syscalls.

**Commit 16 — Multi-queue TUN**

`IFF_MULTI_QUEUE` on Linux opens N TUN file descriptors for the same interface. N outbound worker threads, each with its own TUN fd and UDP socket. Inbound still single-threaded (one UDP recv loop dispatching to peers).

```
rustguard serve --pool 10.150.0.0/24 --token s --queues 2
```

*Result: **472 Mbps** with 2 queues.* Nearly 3× improvement. This is the real win — parallelizing TUN I/O across cores.

**Commit 17 — io_uring TUN engine**

Replaced blocking `read()`/`write()` on TUN fds with io_uring. Pre-registered buffer pool (256 slots × 2KB). Pre-fill SQ with `ReadFixed` SQEs. On completion: process packet, push `WriteFixed` SQE. One `io_uring_enter()` per batch instead of per packet.

*Result: 225 Mbps.* Slower than multi-queue. At 30K packets/sec, the io_uring submission overhead exceeds the savings from batching. io_uring wins at higher packet rates (small packets, many flows) but loses to raw multi-queue for bulk throughput.

### Performance Summary

| Configuration | Throughput | % of Wire Speed |
|---|---|---|
| Bare metal (no VPN) | 2,356 Mbps | 100% |
| Kernel WireGuard | 1,096 Mbps | 46.5% |
| **RustGuard (2 queues)** | **472 Mbps** | **20.0%** |
| RustGuard (io_uring) | 225 Mbps | 9.6% |
| RustGuard (single queue) | 175 Mbps | 7.4% |
| RustGuard (AF_XDP) | ~175 Mbps | 7.4% |

The kernel WireGuard implementation runs inside the network stack with zero-copy access to sk_buffs. A userspace implementation must cross the kernel boundary twice per packet (TUN read + TUN write). That's the fundamental tax. Multi-queue TUN parallelizes that tax across cores, which is the single biggest win available to userspace.

## Usage

### Standard mode (wg.conf compatible)

```bash
rustguard up /etc/wireguard/wg0.conf
```

### Zero-config mode

```bash
# Server
rustguard serve --pool 10.150.0.0/24 --token <shared-secret> --port 51820

# Client
rustguard join 203.0.113.1:51820 --token <shared-secret>

# Manage enrollment
rustguard open 60    # accept new peers for 60 seconds
rustguard close      # stop accepting
rustguard status     # show peers and window state
```

### Key management

```bash
rustguard genkey                        # generate private key
rustguard genkey | rustguard pubkey     # derive public key
```

### Performance flags (Linux only)

```bash
rustguard serve --pool 10.150.0.0/24 --token s --queues 2      # multi-queue TUN
rustguard serve --pool 10.150.0.0/24 --token s --xdp enp7s0    # AF_XDP zero-copy
rustguard serve --pool 10.150.0.0/24 --token s --uring          # io_uring batched I/O
```

## Building

```bash
# Native
cargo build --release

# Cross-compile for Linux (from macOS)
cargo build --release --target x86_64-unknown-linux-musl
```

The release profile strips symbols, enables LTO, and optimizes for size (`opt-level = "z"`). Produces a ~637KB static binary.

## Test Infrastructure

- Two Debian 12 VMs on an Intel N305 hypervisor
- Intel i225 2.5G NICs passed through to VMs (PCIe passthrough)
- Direct connection: `192.168.99.1` ↔ `192.168.99.2`
- Benchmarks via `iperf3`, single TCP stream, 30-second runs
- Cross-compiled from M1 Mac via `musl-cross`

## What I Learned

1. **TUN is the bottleneck, not crypto.** ChaCha20-Poly1305 runs at >10 Gbps on modern CPUs. The kernel boundary crossing for TUN read/write is what kills userspace VPN throughput.

2. **Multi-queue TUN is the biggest win.** Parallelizing TUN I/O across cores gave a 2.7× improvement. Everything else (AF_XDP, io_uring, recvmmsg) was noise in comparison.

3. **AF_XDP solves the wrong problem.** Zero-copy UDP is fast, but the bottleneck is on the TUN side. AF_XDP would matter if both sides were AF_XDP (no TUN at all), but then you're building a kernel module with extra steps.

4. **io_uring has overhead at low packet rates.** At bulk-transfer packet rates (~30K pps), the submission queue management overhead exceeds the savings from batching. io_uring wins at higher packet rates with small packets.

5. **The WireGuard protocol is beautifully simple.** Noise_IK, 1-RTT handshake, 64-bit nonce counter, rotating keys. Jason Donenfeld's 4,000 lines of kernel code is an achievement in discipline. Building it from scratch is the best way to appreciate that.

6. **Security audits find real bugs.** The HMAC-BLAKE2s implementation was wrong (keyed BLAKE2s ≠ HMAC). MAC1 was checked too late. The replay window was updatable before AEAD verification. None of these showed up in functional tests.

## License

MIT OR Apache-2.0
