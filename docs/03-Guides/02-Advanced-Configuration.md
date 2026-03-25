# Advanced Configuration

> Tuning, overrides, and advanced options for RustGuard — covering high-performance TUN backends, kernel module mode, pre-shared keys, IPv6 dual-stack, enrollment server tuning, and DoS protection behavior.

## Overview

This page documents configuration options and operational modes beyond the defaults covered in [Common Workflows](01-Common-Workflows.md). It assumes a working RustGuard installation. Topics are organized by concern: transport backend selection, cryptographic hardening, enrollment tuning, and protocol-level behavior.

For basic tunnel setup and zero-config enrollment, see [Common Workflows](01-Common-Workflows.md). For configuration file field reference, see [Configuration](../01-Getting-Started/03-Configuration.md).

---

## Pre-Shared Keys (PSK)

RustGuard supports optional pre-shared keys (PSK) as specified in the WireGuard protocol (Noise_IKpsk2). A PSK adds a symmetric secret to the handshake, providing post-quantum resistance against an adversary who records traffic today and breaks elliptic curve cryptography in the future.

PSK support is implemented with backward-compatible wrappers in `rustguard-crypto` — tunnels without a PSK remain interoperable with standard WireGuard peers.

To enable PSK, add a `PresharedKey` field to the relevant `[Peer]` section in the configuration file:

```ini
[Interface]
PrivateKey = <base64-private-key>
Address = 10.0.0.1/24
ListenPort = 51820

[Peer]
PublicKey = <base64-peer-public-key>
AllowedIPs = 10.0.0.2/32
Endpoint = 203.0.113.42:51820
PresharedKey = <base64-32-byte-psk>
```

The PSK must be identical on both peers. A mismatched PSK causes the Noise handshake to produce an incorrect chaining key; the handshake silently fails and no transport session is established.

Generate a PSK with any CSPRNG source:

```bash
# Generate a 32-byte base64-encoded PSK
dd if=/dev/urandom bs=32 count=1 2>/dev/null | base64
```

---

## IPv6 Dual-Stack

RustGuard handles both IPv4 and IPv6 packets in a single tunnel by inspecting the IP version byte of each decapsulated packet. Both address families are supported in `AllowedIPs` using standard CIDR notation.

Configure a dual-stack interface by assigning both address families to the tunnel and listing both CIDR ranges in `AllowedIPs`:

```ini
[Interface]
PrivateKey = <base64-private-key>
Address = 10.0.0.1/24, fd00::1/64
ListenPort = 51820

[Peer]
PublicKey = <base64-peer-public-key>
AllowedIPs = 10.0.0.2/32, fd00::2/128
Endpoint = 203.0.113.42:51820
```

On Linux, RustGuard configures IPv6 addresses via `ip -6 addr add` and installs routes for both families. On macOS, `ifconfig inet6` is used for the utun interface. Both platforms are handled by `rustguard-tun`.

---

## High-Performance TUN Backends

`rustguard-tun` provides several TUN backends beyond the default single-queue Linux TUN device. These are designed to push userspace throughput toward line rate on supported hardware.

### Multi-Queue TUN

Multi-queue TUN allows parallel packet processing across CPU cores by opening multiple TUN file descriptors with `IFF_MULTI_QUEUE`. This reduces per-core bottlenecks on high-traffic tunnels and is available on Linux.

### AF_XDP

AF_XDP (eXpress Data Path) bypasses the kernel network stack entirely, delivering packets directly from NIC driver memory to userspace. This backend is implemented in `rustguard-tun` and targets Linux systems with XDP-capable NICs. AF_XDP requires:

- Linux kernel with `CONFIG_XDP_SOCKETS` enabled
- An XDP-capable NIC driver (the reference test rig used Intel i225 2.5G NICs)
- `CAP_NET_ADMIN` or equivalent privilege

### io_uring

The io_uring backend uses Linux's `io_uring` subsystem for asynchronous I/O submission and completion, reducing syscall overhead for high-packet-rate workloads. It is implemented in `rustguard-tun` and targets Linux kernels with io_uring support.

### BPF Loader

`rustguard-tun` includes a BPF loader for attaching eBPF programs to the tunnel data path. This enables in-kernel packet filtering and steering without additional userspace round-trips.

---

## Kernel Module Mode

`rustguard-kmod` is an out-of-tree Linux kernel module (Rust + C shim) that eliminates the TUN device bottleneck by running the WireGuard data path inside the kernel. It targets Linux 6.10+.

### Prerequisites

- Linux kernel 6.10 or later
- Rust toolchain with `no_std` support (the `rustguard-crypto` and `rustguard-core` crates are dual `std`/`no_std`)
- Kernel headers for the running kernel version
- `rustguard-kmod` built from the `rustguard-kmod/` crate

### Architecture

The kernel module is implemented in two layers:

- **Rust core** — `rustguard-crypto` and `rustguard-core` compile as `no_std` and are linked into the module. Cryptographic primitives, the Noise_IK handshake, replay window, and session management all run in-kernel.
- **C shim** — A thin C layer handles kernel API surface (netdev registration, socket operations, module init/exit) that is not yet expressible directly in Rust kernel bindings.

This architecture keeps security-critical cryptographic code in Rust while minimizing unsafe C surface.

---

## Enrollment Server Tuning

### Pool Sizing

The IP pool is specified as a CIDR range on the `serve` command. The server always takes the `.1` address; clients receive sequential addresses from the remainder of the pool.

Choose a pool large enough for the expected number of peers. A `/24` supports up to 253 clients; a `/16` supports up to 65,533:

```bash
# Small homelab (up to 253 peers)
rustguard serve --pool 10.150.0.0/24 --token mysecret

# Larger fleet (up to 65,533 peers)
rustguard serve --pool 10.150.0.0/16 --token mysecret
```

Pool size cannot be changed without restarting the server. On restart, the previous state file is replaced and all existing IP assignments are lost — previously enrolled peers must re-enroll.

### State Persistence

The server persists enrolled peer public keys and their assigned IP addresses to `~/.rustguard/state.json`. This file is read on startup, so peers that enrolled in a previous session do not need to re-enroll after a server restart (provided the pool CIDR and token are unchanged).

```bash
# Inspect current peer assignments
cat ~/.rustguard/state.json
```

The state file is a plain JSON document. It should not be edited manually; inconsistencies between the file and a running server are not reconciled at runtime.

### Enrollment Window

The enrollment window is **closed by default** when the server starts. Open it explicitly before expecting clients to join:

```bash
# Open for 60 seconds, then auto-close
rustguard open 60

# Close immediately
rustguard close

# Query current window state and peer count
rustguard status
```

The window auto-closes via an atomic timestamp comparison — no background thread is required. Existing connected peers are not affected by window state changes.

---

## DoS Protection — Cookie Behavior

RustGuard implements the WireGuard cookie mechanism to limit the CPU cost of unauthenticated handshake initiations under load.

Two message authentication codes are present on every handshake initiation:

- **MAC1** — Always computed and checked before any Diffie-Hellman operations. An invalid MAC1 causes the packet to be dropped immediately. This is enforced by `rustguard-core`; the MAC1 check is performed before any DH is attempted (a regression fixed in the security audit, Commit 5).
- **MAC2** — Required only when the server is under load. The server rotates a secret periodically (managed by `CookieChecker`) and sends a Cookie Reply (message type 3, encrypted with XChaCha20-Poly1305) to clients that do not present a valid MAC2. Clients store the received cookie in `CookieState` and include MAC2 in subsequent retries.

This mechanism is transparent to operators; no configuration is required. It activates automatically when the server detects elevated load.

---

## Timer Behavior

`rustguard-core` implements the WireGuard timer state machine with the following fixed intervals:

| Event | Interval |
|---|---|
| Rekey (time-based) | 120 seconds |
| Rekey (message-based) | 2⁶⁰ messages |
| Handshake retry | Exponential backoff with jitter |
| Session expiry | Per WireGuard specification |

These values match the WireGuard specification and are not currently user-configurable. Keepalive behavior follows the `PersistentKeepalive` field in the configuration file when present.

---

## Examples

### Dual-Stack Tunnel with PSK and Custom Pool

The following example shows a server-side configuration combining dual-stack addressing and a pre-shared key, alongside the zero-config enrollment command:

```ini
# server-wg0.conf
[Interface]
PrivateKey = <base64-server-private-key>
Address = 10.100.0.1/24, fd10::1/64
ListenPort = 51820

[Peer]
PublicKey = <base64-client-public-key>
AllowedIPs = 10.100.0.2/32, fd10::2/128
PresharedKey = <base64-32-byte-psk>
```

```bash
# Bring up with standard config
rustguard up server-wg0.conf
```

### Zero-Config Enrollment with Large Pool

```bash
# On the server — start with a /16 pool
rustguard serve --pool 10.200.0.0/16 --token strongsecret

# Open enrollment for 120 seconds
rustguard open 120

# On each client
rustguard join 198.51.100.1:51820 --token strongsecret

# Check how many peers enrolled
rustguard status

# Close enrollment immediately when done
rustguard close
```

---

## See Also

- [Common Workflows](01-Common-Workflows.md) — Standard tunnel setup and basic enrollment walkthrough
- [Configuration](../01-Getting-Started/03-Configuration.md) — Full `wg.conf` field reference
- [Troubleshooting](03-Troubleshooting.md) — Diagnosing enrollment failures, pool exhaustion, and TUN errors
- [System Overview](../02-Architecture/01-System-Overview.md) — Crate responsibilities and component boundaries
- [Data Flow](../02-Architecture/03-Data-Flow.md) — How packets move through userspace and kernel module paths
- [Design Decisions](../02-Architecture/04-Design-Decisions.md) — Architecture Decision Records covering backend selection and the kernel module approach