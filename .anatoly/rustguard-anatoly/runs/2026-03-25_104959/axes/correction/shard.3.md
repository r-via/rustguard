# Correction — Shard 3

## Findings

| File | Verdict | Correction | Conf. | Details |
|------|---------|------------|-------|---------|
| `rustguard-enroll/src/packet.rs` | NEEDS_REFACTOR | 1 | 92% | [details](../reviews/rustguard-enroll-src-packet.rev.md) |
| `rustguard-kmod/src/timers.rs` | NEEDS_REFACTOR | 2 | 90% | [details](../reviews/rustguard-kmod-src-timers.rev.md) |
| `rustguard-crypto/src/x25519.rs` | NEEDS_REFACTOR | 2 | 87% | [details](../reviews/rustguard-crypto-src-x25519.rev.md) |
| `rustguard-tun/examples/tun_echo.rs` | NEEDS_REFACTOR | 1 | 85% | [details](../reviews/rustguard-tun-examples-tun_echo.rev.md) |
| `rustguard-tun/src/uring.rs` | NEEDS_REFACTOR | 2 | 90% | [details](../reviews/rustguard-tun-src-uring.rev.md) |
| `rustguard-kmod/src/allowedips.rs` | NEEDS_REFACTOR | 2 | 88% | [details](../reviews/rustguard-kmod-src-allowedips.rev.md) |
| `rustguard-enroll/src/pool.rs` | NEEDS_REFACTOR | 1 | 90% | [details](../reviews/rustguard-enroll-src-pool.rev.md) |

## Symbol Details

### `rustguard-enroll/src/packet.rs`

| Symbol | Lines | Correction | Conf. | Detail |
|--------|-------|------------|-------|--------|
| `parse_ipv4_udp` | L30–L52 | NEEDS_FIX | 85% | [USED] Non-exported helper function called directly by parse_eth_udp at L24. ... |

### `rustguard-kmod/src/timers.rs`

| Symbol | Lines | Correction | Conf. | Detail |
|--------|-------|------------|-------|--------|
| `REKEY_AFTER_MESSAGES` | L25–L25 | NEEDS_FIX | 85% | [USED] Constant is used in the needs_rekey method at line 88 comparing send_c... |
| `SessionTimers` | L38–L51 | NEEDS_FIX | 88% | [DEAD] Exported struct with 0 runtime importers per pre-computed analysis. Ru... |

### `rustguard-crypto/src/x25519.rs`

| Symbol | Lines | Correction | Conf. | Detail |
|--------|-------|------------|-------|--------|
| `StaticSecret` | L7–L7 | NEEDS_FIX | 82% | [USED] Pub struct in library crate; core X25519 type. Follows pattern of know... |
| `EphemeralSecret` | L11–L11 | NEEDS_FIX | 82% | [USED] Pub struct in library crate for ephemeral keys. Identical pattern to k... |

### `rustguard-tun/examples/tun_echo.rs`

| Symbol | Lines | Correction | Conf. | Detail |
|--------|-------|------------|-------|--------|
| `main` | L12–L79 | NEEDS_FIX | 78% | [USED] Entry point of the binary, executed by Rust runtime. Essential to the ... |

### `rustguard-tun/src/uring.rs`

| Symbol | Lines | Correction | Conf. | Detail |
|--------|-------|------------|-------|--------|
| `BufferPool` | L26–L31 | NEEDS_FIX | 80% | [USED] Public struct exported as part of module API. Used as field type in Ur... |
| `UringTun` | L96–L102 | NEEDS_FIX | 82% | [USED] Main public API struct of the module. Provides four public methods (ne... |

### `rustguard-kmod/src/allowedips.rs`

| Symbol | Lines | Correction | Conf. | Detail |
|--------|-------|------------|-------|--------|
| `TrieNode` | L16–L25 | NEEDS_FIX | 88% | [USED] Core internal data structure heavily used locally. TrieNode::new() ins... |
| `AllowedIps` | L28–L31 | NEEDS_FIX | 85% | [USED] Exported struct matching known false-positive pattern (similar to Peer... |

### `rustguard-enroll/src/pool.rs`

| Symbol | Lines | Correction | Conf. | Detail |
|--------|-------|------------|-------|--------|
| `IpPool` | L9–L20 | NEEDS_FIX | 90% | [USED] Exported pub struct in library crate rustguard-enroll. Zero in-crate i... |

## Quick Wins

- [x] <!-- ACT-7726d0-1 --> **[correction · medium · small]** `rustguard-crypto/src/x25519.rs`: StaticSecret::diffie_hellman does not check for an all-zero DH result. Add a constant-time zero check (e.g. shared.to_bytes().ct_eq(&[0u8;32])) and either return an error or panic/abort, as required by RFC 7748 and the WireGuard specification. Consider changing the return type to Result<SharedSecret, ZeroSharedSecret> to propagate the failure. [L47]
- [x] <!-- ACT-7726d0-2 --> **[correction · medium · small]** `rustguard-crypto/src/x25519.rs`: EphemeralSecret::diffie_hellman has the same missing all-zero DH output check. Apply the identical fix: constant-time comparison against [0u8;32] and surface a failure when the result is zero. [L76]
- [x] <!-- ACT-4cf3f3-1 --> **[correction · medium · small]** `rustguard-enroll/src/packet.rs`: After computing ihl at line 35, add 'if ihl < 20 { return None; }' to reject malformed IPv4 packets whose IHL field encodes a header smaller than the mandatory 20 bytes. Without this guard, packets with IHL values 0–4 are incorrectly parsed: the source port is read from within the IP header itself and the payload slice starts at the wrong offset, producing a structurally plausible but semantically wrong ParsedUdp. [L35]
- [x] <!-- ACT-a01ecd-2 --> **[correction · medium · small]** `rustguard-kmod/src/allowedips.rs`: Add cidr range guards in insert_v4 (cidr > 32 → return early or return Err) and insert_v6 (cidr > 128). Without this, an out-of-range cidr silently builds phantom nodes beyond the address width; lookups can then match those phantom nodes for any address sharing all bits with the inserted prefix, corrupting the longest-prefix result. [L54]
- [x] <!-- ACT-050448-2 --> **[correction · medium · small]** `rustguard-kmod/src/timers.rs`: Fix is_dead() to measure inactivity, not session age: replace `self.session_established` with `self.last_received.max(self.last_sent)` (or equivalent) as the baseline for the DEAD_SESSION_TIMEOUT_NS comparison, so an actively-used session is not prematurely zeroed. [L111]
- [x] <!-- ACT-050448-3 --> **[correction · medium · small]** `rustguard-kmod/src/timers.rs`: Fix needs_keepalive() fallback when last_sent == 0: replace `now.saturating_sub(self.last_received)` with `u64::MAX` (or `now_ns()`) so that since_last_send is treated as maximally large when no packet has ever been sent, allowing the first keepalive to fire correctly. [L126]
- [x] <!-- ACT-fe7cbd-1 --> **[correction · medium · small]** `rustguard-tun/examples/tun_echo.rs`: Replace hardcoded IP header offset 20 with `let ihl = (reply[0] & 0x0f) as usize * 4;` and use `ihl` in place of 20 for the ICMP-type check (`reply[ihl] == 8`), the ICMP checksum slice (`&reply[ihl..n]`), the IP checksum slice (`&reply[..ihl]`), and tighten the length guard to `n >= ihl + 8`. [L35]
- [x] <!-- ACT-0723df-1 --> **[correction · medium · small]** `rustguard-tun/src/uring.rs`: Replace `self.pending_reads -= 1` with `self.pending_reads = self.pending_reads.saturating_sub(1)` (or add a debug_assert!(self.pending_reads > 0) guard) in both submit_and_wait() and poll() to prevent panic-on-underflow in debug builds and silent counter corruption in release builds. [L194]
- [x] <!-- ACT-0723df-2 --> **[correction · medium · small]** `rustguard-tun/src/uring.rs`: Same saturating_sub fix required in poll() at the second occurrence of pending_reads -= 1. [L226]
- [x] <!-- ACT-0723df-3 --> **[correction · medium · small]** `rustguard-tun/src/uring.rs`: Add bounds validation at the top of submit_write(): return an Err if buf_idx >= NUM_BUFS or if !self.bufs.in_flight[buf_idx]. This prevents out-of-bounds pointer arithmetic in slot_ptr() and prevents aliasing a buffer still owned by a pending read SQE. [L161]
- [x] <!-- ACT-92d12d-1 --> **[correction · low · small]** `rustguard-enroll/src/pool.rs`: Mirror the prefix_len == 0 guard from `new` inside `contains`: `let mask = if self.prefix_len == 0 { 0 } else { u32::MAX << (32 - self.prefix_len) };` to prevent a shift-overflow panic in debug mode and incorrect results in release mode. [L74]
- [x] <!-- ACT-a01ecd-1 --> **[correction · low · small]** `rustguard-kmod/src/allowedips.rs`: Remove the `bit` field from TrieNode or implement proper compressed-trie logic: set node.bit to the actual bit position tested at each node during insertion and read it during lookup instead of using `depth`. As-is, the field is always 0 and the struct's own documentation is false. [L18]
- [x] <!-- ACT-a01ecd-3 --> **[correction · low · small]** `rustguard-kmod/src/allowedips.rs`: Add peer_idx < MAX_PEERS validation in insert_v4 and insert_v6. The constant MAX_PEERS is defined in this module and callers index into peer arrays bounded by it; accepting and returning unchecked peer_idx values can produce out-of-bounds accesses at the call site. [L54]
- [x] <!-- ACT-a01ecd-5 --> **[correction · low · small]** `rustguard-kmod/src/allowedips.rs`: Remove the dead assignments on lines 116-117 inside the cidr==0 branch of insert (`node.cidr = 0; node.peer_idx = peer_idx;`). Both are unconditionally overwritten on lines 119-120. The dead writes suggest an incomplete refactor and can mislead future readers about the intended sentinel value. [L116]
- [x] <!-- ACT-050448-1 --> **[correction · low · small]** `rustguard-kmod/src/timers.rs`: Change REKEY_AFTER_MESSAGES from `(1u64 << 60) - 1` to `1u64 << 60` to match the canonical WireGuard kernel constant and eliminate the off-by-one. [L25]
- [x] <!-- ACT-0723df-4 --> **[correction · low · small]** `rustguard-tun/src/uring.rs`: Add a bounds check in BufferPool::free() — assert!(idx < NUM_BUFS) or return early — to prevent a silent out-of-bounds panic or (in unsafe contexts) memory corruption when called with an invalid index. [L70]
- [x] <!-- ACT-0723df-5 --> **[correction · low · small]** `rustguard-tun/src/uring.rs`: Consider replacing the &self receiver of slot_ptr with &mut self, or wrapping bufs.data in UnsafeCell<Vec<u8>>, to make the *mut u8 derivation sound under Rust's aliasing rules. At minimum, document the safety invariant: callers must not hold any Rust reference to a slot while that slot's in_flight flag is true. [L54]
