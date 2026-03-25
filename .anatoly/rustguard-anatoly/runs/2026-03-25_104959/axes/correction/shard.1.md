# Correction — Shard 1

## Findings

| File | Verdict | Correction | Conf. | Details |
|------|---------|------------|-------|---------|
| `rustguard-tun/src/linux_mq.rs` | CRITICAL | 1 | 88% | [details](../reviews/rustguard-tun-src-linux_mq.rev.md) |
| `rustguard-tun/src/xdp.rs` | CRITICAL | 3 | 92% | [details](../reviews/rustguard-tun-src-xdp.rev.md) |
| `rustguard-enroll/src/server.rs` | CRITICAL | 2 | 92% | [details](../reviews/rustguard-enroll-src-server.rev.md) |
| `rustguard-core/src/replay.rs` | CRITICAL | 1 | 90% | [details](../reviews/rustguard-core-src-replay.rev.md) |
| `rustguard-kmod/src/replay.rs` | CRITICAL | 1 | 88% | [details](../reviews/rustguard-kmod-src-replay.rev.md) |
| `rustguard-kmod/src/noise.rs` | NEEDS_REFACTOR | 2 | 92% | [details](../reviews/rustguard-kmod-src-noise.rev.md) |
| `rustguard-core/src/handshake.rs` | NEEDS_REFACTOR | 1 | 90% | [details](../reviews/rustguard-core-src-handshake.rev.md) |
| `rustguard-kmod/src/lib.rs` | NEEDS_REFACTOR | 5 | 95% | [details](../reviews/rustguard-kmod-src-lib.rev.md) |
| `rustguard-daemon/src/tunnel.rs` | NEEDS_REFACTOR | 2 | 88% | [details](../reviews/rustguard-daemon-src-tunnel.rev.md) |
| `rustguard-core/src/cookie.rs` | NEEDS_REFACTOR | 4 | 90% | [details](../reviews/rustguard-core-src-cookie.rev.md) |

## Symbol Details

### `rustguard-tun/src/linux_mq.rs`

| Symbol | Lines | Correction | Conf. | Detail |
|--------|-------|------------|-------|--------|
| `MultiQueueTun` | L77–L81 | ERROR | 88% | [DEAD] Exported public struct with 0 runtime importers and 0 type-only import... |

### `rustguard-tun/src/xdp.rs`

| Symbol | Lines | Correction | Conf. | Detail |
|--------|-------|------------|-------|--------|
| `XdpUmemReg` | L41–L48 | NEEDS_FIX | 70% | [USED] Non-exported struct constructed and used for UMEM registration | [UNIQ... |
| `XdpSocket` | L148–L160 | ERROR | 92% | [USED] Exported struct; main pub API for AF_XDP zero-copy UDP I/O | [UNIQUE] ... |
| `make_ring` | L439–L447 | NEEDS_FIX | 85% | [USED] Non-exported function called four times to construct Ring objects | [U... |

### `rustguard-enroll/src/server.rs`

| Symbol | Lines | Correction | Conf. | Detail |
|--------|-------|------------|-------|--------|
| `ServerState` | L39–L47 | NEEDS_FIX | 85% | [USED] Non-exported struct instantiated at L181–188 as Arc<ServerState>. Clon... |
| `run` | L64–L532 | ERROR | 92% | [USED] Exported public function named 'run', the apparent entry point for enr... |

### `rustguard-core/src/replay.rs`

| Symbol | Lines | Correction | Conf. | Detail |
|--------|-------|------------|-------|--------|
| `ReplayWindow` | L13–L19 | ERROR | 90% | [DEAD] Exported struct with 0 runtime importers and 0 type-only importers per... |

### `rustguard-kmod/src/replay.rs`

| Symbol | Lines | Correction | Conf. | Detail |
|--------|-------|------------|-------|--------|
| `ReplayWindow` | L11–L14 | ERROR | 88% | [DEAD] Exported struct (pub(crate)) with zero runtime and type-only importers... |

### `rustguard-kmod/src/noise.rs`

| Symbol | Lines | Correction | Conf. | Detail |
|--------|-------|------------|-------|--------|
| `hash` | L72–L92 | NEEDS_FIX | 85% | [USED] Non-exported BLAKE2s hash function called in initial_chain_key (line 1... |
| `process_response` | L350–L425 | NEEDS_FIX | 85% | [DEAD] Exported pub(crate) function with 0 known importers and no local calls... |

### `rustguard-core/src/handshake.rs`

| Symbol | Lines | Correction | Conf. | Detail |
|--------|-------|------------|-------|--------|
| `process_response` | L173–L221 | NEEDS_FIX | 76% | [USED] Called in full_handshake_and_transport test (line 392) and failure sce... |

### `rustguard-kmod/src/lib.rs`

| Symbol | Lines | Correction | Conf. | Detail |
|--------|-------|------------|-------|--------|
| `RustGuard` | L143–L143 | NEEDS_FIX | 88% | [USED] Struct implements kernel::Module and referenced in module! macro (line... |
| `do_xmit` | L345–L416 | NEEDS_FIX | 85% | [USED] Function called in rustguard_xmit (line 345); implements packet encryp... |
| `handle_initiation` | L474–L532 | NEEDS_FIX | 86% | [USED] Function called in do_rx (line 452); implements responder-side handsha... |
| `handle_response` | L536–L561 | NEEDS_FIX | 87% | [USED] Function called in do_rx (line 455); implements initiator-side handsha... |
| `handle_transport` | L566–L634 | NEEDS_FIX | 86% | [USED] Function called in do_rx (line 458); implements packet decryption, rep... |

### `rustguard-daemon/src/tunnel.rs`

| Symbol | Lines | Correction | Conf. | Detail |
|--------|-------|------------|-------|--------|
| `run` | L48–L489 | NEEDS_FIX | 88% | [DEAD] Exported public function with 0 runtime or type-only importers per pre... |
| `ctrlc_handler` | L504–L525 | NEEDS_FIX | 88% | [USED] Sets up signal handler for clean shutdown; called at L146 during initi... |

### `rustguard-core/src/cookie.rs`

| Symbol | Lines | Correction | Conf. | Detail |
|--------|-------|------------|-------|--------|
| `CookieState` | L60–L65 | NEEDS_FIX | 85% | [DEAD] Exported struct with 0 external importers per pre-computed analysis. R... |
| `fill_random` | L261–L263 | NEEDS_FIX | 80% | [USED] Non-exported no_std stub version; alternative implementation called wh... |
| `fill_random` | L266–L271 | NEEDS_FIX | 80% | [USED] Non-exported no_std stub version; alternative implementation called wh... |
| `constant_time_eq_16` | L273–L279 | NEEDS_FIX | 70% | [USED] Non-exported function used in verify_mac1 (line 140) and verify_mac2_f... |

## Quick Wins

- [ ] <!-- ACT-a4f709-2 --> **[correction · high · small]** `rustguard-core/src/cookie.rs`: no_std fill_random stub silently produces all-zero output (let _ = buf is a no-op). Replace with a compile_error! macro or a link-time weak symbol that the kernel module must override, preventing accidental silent use of zero secrets and nonces. [L269]
- [ ] <!-- ACT-f55ce3-1 --> **[correction · high · small]** `rustguard-core/src/replay.rs`: Fix shift_window bit-shift direction (lines 112-116): replace the reversed right-shift loop with a forward left-shift loop. Change `self.bitmap.iter_mut().rev()` → `self.bitmap.iter_mut()`, change `*word >> bit_shift` → `*word << bit_shift`, and change the carry extraction from `*word << (64 - bit_shift)` → `*word >> (64 - bit_shift)`. This ensures that existing seen-counter marks migrate toward higher ages (higher bit indices) on each window advance, matching the invariant stated in the bitmap comment and preventing replay of any counter whose top-delta has non-zero low 6 bits. [L112]
- [ ] <!-- ACT-a732f2-1 --> **[correction · high · small]** `rustguard-enroll/src/server.rs`: Add an upper-bound check before the decrypt_buf copy in the MSG_TRANSPORT handler: if ct.len() > decrypt_buf.len() { continue; } — or size decrypt_buf to match the maximum possible inbound payload (at least 4096 - TRANSPORT_HEADER_SIZE to cover the configured AF_XDP frame size). Without this check any peer or attacker can crash the inbound thread by sending a single oversized UDP datagram. [L415]
- [ ] <!-- ACT-c30836-1 --> **[correction · high · small]** `rustguard-kmod/src/replay.rs`: Fix carry propagation direction in shift_window: replace `for i in (0..BITMAP_LEN).rev()` with `for i in 0..BITMAP_LEN`. Carry must flow from word[i] into word[i+1] (lower-index/newer → higher-index/older). The final carry after word[BITMAP_LEN-1] is correctly discarded as those bits fall outside the window. The current reversed iteration propagates carry backwards (older→newer) and then drops the only carry that matters. [L91]
- [ ] <!-- ACT-8fc69f-1 --> **[correction · high · small]** `rustguard-tun/src/xdp.rs`: tx_send copies data.len() bytes into a UMEM frame (self.umem.add(frame_addr)) without checking that data.len() <= self.frame_size. If the caller passes a slice larger than one frame, adjacent frames are silently overwritten, corrupting other live packets and causing undefined memory behaviour. [L348]
- [ ] <!-- ACT-a4f709-1 --> **[correction · medium · small]** `rustguard-core/src/cookie.rs`: CookieState::process_reply: on no_std, self.received is never set after a successful decryption. Accept a monotonic timestamp parameter (or a no_std-compatible clock abstraction) and store it unconditionally so that is_fresh() can function correctly and compute_mac2 returns a real MAC instead of zeros. [L213]
- [ ] <!-- ACT-5920d4-1 --> **[correction · medium · small]** `rustguard-core/src/handshake.rs`: Add MAC1 verification in process_response before any DH operations: compute expected_mac1 = compute_mac1(&our_static.public_key(), &msg.to_bytes()[..60]) and return None if it does not constant-time-equal msg.mac1. Without this check, an attacker who observed the plaintext sender_index in the Initiation can craft fake Response messages that survive the receiver_index guard and force two DH exponentiations per fake packet. [L183]
- [ ] <!-- ACT-bd20cf-1 --> **[correction · medium · small]** `rustguard-daemon/src/tunnel.rs`: Store the originating peer index alongside the sender_index in pending_handshakes (change the tuple type to (u32, Instant, InitiatorHandshake, usize) where the last element is the peer index). In the MSG_RESPONSE handler, after removing the entry, use the stored peer index directly instead of the fragile heuristic that finds 'any peer without an active session', which silently assigns the session to the wrong peer when multiple peers are configured. [L265]
- [ ] <!-- ACT-bd20cf-2 --> **[correction · medium · small]** `rustguard-daemon/src/tunnel.rs`: Block SIGINT and SIGTERM in the main thread before calling ctrlc_handler (and before spawning any worker threads) using sigprocmask/pthread_sigmask so that all spawned threads inherit the blocked mask. This ensures that the only thread eligible to receive those signals via sigwait is the dedicated signal-handler thread, preventing the shutdown closure from being silently dropped when the signal is delivered to a worker thread. [L504]
- [ ] <!-- ACT-a732f2-3 --> **[correction · medium · small]** `rustguard-enroll/src/server.rs`: ServerState.pending_handshakes is allocated but never populated or queried. Either implement WireGuard cookie challenge-response using this field (required for correct protocol behaviour under load), or remove it to eliminate dead state that implies unimplemented functionality. [L44]
- [ ] <!-- ACT-bb41a6-1 --> **[correction · medium · small]** `rustguard-kmod/src/lib.rs`: Add a `genl_registered: bool` field to the RustGuard struct (converting it from a unit struct) and set it to true only when wg_genl_init() returns 0; guard the wg_genl_exit() call in Drop behind this flag to avoid unregistering a genetlink family that was never successfully registered. [L305]
- [ ] <!-- ACT-bb41a6-2 --> **[correction · medium · small]** `rustguard-kmod/src/lib.rs`: Replace the 2048-byte on-stack `buf` array in do_xmit with a heap allocation (e.g., kmalloc / KBox) to prevent kernel stack overflow in the TX path. [L366]
- [ ] <!-- ACT-bb41a6-4 --> **[correction · medium · small]** `rustguard-kmod/src/lib.rs`: Extend handle_initiation to accept the source IP and port of the incoming packet (passed from do_rx via the SKB or an extra parameter) and send the response to that address rather than the pre-configured peer.endpoint_ip/port, so NAT and roaming scenarios work correctly. [L506]
- [ ] <!-- ACT-bb41a6-5 --> **[correction · medium · small]** `rustguard-kmod/src/lib.rs`: Restructure handle_response to restore pending_handshake if process_response fails: either pass a reference instead of moving the state, or re-assign peer.pending_handshake = Some(pending) on None, so that a single malformed or attacker-injected response cannot permanently block the handshake. [L543]
- [ ] <!-- ACT-bb41a6-6 --> **[correction · medium · small]** `rustguard-kmod/src/lib.rs`: Replace the 2048-byte on-stack `plaintext_buf` array in handle_transport with a heap allocation to avoid kernel stack overflow in the deep RX call chain. [L600]
- [ ] <!-- ACT-58e5ff-1 --> **[correction · medium · small]** `rustguard-kmod/src/noise.rs`: Fix hash() to pass min(chunks.len(), 8) as num_chunks to wg_blake2s_hash, or add a debug_assert!(chunks.len() <= 8). Currently chunks.len() is passed unconditionally while only 8 array slots are populated, causing out-of-bounds reads in the C shim if the function is ever called with more than 8 chunks. [L89]
- [ ] <!-- ACT-22946f-2 --> **[correction · medium · small]** `rustguard-tun/src/linux_mq.rs`: Before propagating the Err from `open_tun_queue(None)?` inside the additional-queues loop, close all file descriptors already accumulated in `fds`. Currently, if open_tun_queue fails, the function returns early and fds is dropped as a plain Vec<i32> with no close logic, leaking fd0 and any previously opened queue fds. Match the cleanup pattern used in the ioctl-failure branch (lines 127-129). [L116]
- [ ] <!-- ACT-8fc69f-2 --> **[correction · medium · small]** `rustguard-tun/src/xdp.rs`: tx_send does not check TX ring fullness before writing a descriptor. It should verify prod.wrapping_sub(self.tx_ring.consumer()) < self.ring_size; omitting this allows the producer to lap the consumer and overwrite unprocessed TX entries when the ring is saturated. [L356]
- [ ] <!-- ACT-8fc69f-3 --> **[correction · medium · small]** `rustguard-tun/src/xdp.rs`: XdpSocket::drop munmaps only UMEM. The four ring mmap regions produced by mmap_ring (rx_map, tx_map, fill_map, comp_map) are consumed by make_ring but their base pointers and sizes are discarded. Neither Ring nor XdpSocket stores them, so every XdpSocket drop permanently leaks those four mappings. [L396]
- [ ] <!-- ACT-8fc69f-4 --> **[correction · medium · small]** `rustguard-tun/src/xdp.rs`: create_inner ignores the return values of all four setsockopt ring-size calls (XDP_UMEM_FILL_RING, XDP_UMEM_COMPLETION_RING, XDP_RX_RING, XDP_TX_RING). If any fail, the socket proceeds with an unconfigured or default ring size, silently producing a broken XdpSocket. [L217]
- [ ] <!-- ACT-8fc69f-5 --> **[correction · medium · small]** `rustguard-tun/src/xdp.rs`: In create_inner, if any mmap_ring call after the first fails (via the ? operator), previously mmapped ring regions are leaked: a tx_map failure leaves rx_map unmapped; a fill_map failure leaves rx_map and tx_map unmapped; a comp_map failure leaves all three. Additionally, if bind fails (L276), all four ring mmaps are leaked because only UMEM is freed in the error path. [L241]
- [ ] <!-- ACT-a4f709-4 --> **[correction · low · small]** `rustguard-core/src/cookie.rs`: constant_time_eq_16: the early-exit branch on b.len() < 16 is not constant-time. Change the signature to accept &[u8; 16] for b (eliminating the length check entirely) or use a constant-time Choice-based comparison that does not branch on length. [L275]
- [ ] <!-- ACT-bd20cf-3 --> **[correction · low · small]** `rustguard-daemon/src/tunnel.rs`: In the timer thread's keepalive path, when session.encrypt(&[]) returns None (nonce counter exhausted), set peer.session = None before executing 'continue'. Without this, the exhausted session remains Some, peer.has_active_session() continues to return true, and the rekey_requests collection loop never fires for that peer, making rekeying impossible via the timer path when there is no concurrent outbound packet traffic to clear the session. [L340]
- [ ] <!-- ACT-a732f2-2 --> **[correction · low · small]** `rustguard-enroll/src/server.rs`: Guard the pool-size println shift expression: validate that pool_prefix is in [1, 31] before computing (1u32 << (32 - config.pool_prefix)) - 3, or use a saturating/checked shift, to prevent panic when pool_prefix is 0. [L112]
- [ ] <!-- ACT-bb41a6-3 --> **[correction · low · small]** `rustguard-kmod/src/lib.rs`: Check the i32 return value of wg_socket_send and only call wg_tx_stats when it returns 0, so TX statistics remain accurate. [L413]
- [ ] <!-- ACT-58e5ff-2 --> **[correction · low · small]** `rustguard-kmod/src/noise.rs`: In process_response, declare the PSK HKDF output key as 'mut' (e.g. 'let (new_ck, mut t, mut key) = hkdf(&ck, &state.psk)') and call zeroize(&mut key) after decrypt_and_hash, mirroring the zeroize(&mut key3) call in process_initiation. Without this, the PSK-derived AEAD key persists on the stack after the function returns. [L398]
- [ ] <!-- ACT-8fc69f-6 --> **[correction · low · small]** `rustguard-tun/src/xdp.rs`: XdpUmemReg includes tx_metadata_len (kernel 6.6+ only). On pre-6.6 kernels the setsockopt(XDP_UMEM_REG) call receives an optlen larger than the kernel expects and returns EINVAL, making XdpSocket::create unconditionally fail on those kernels. [L41]
