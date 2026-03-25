# Documentation — Shard 1

## Findings

| File | Verdict | Documentation | Conf. | Details |
|------|---------|---------------|-------|---------|
| `rustguard-tun/src/linux_mq.rs` | CRITICAL | 1 | 88% | [details](../reviews/rustguard-tun-src-linux_mq.rev.md) |
| `rustguard-tun/src/xdp.rs` | CRITICAL | 4 | 92% | [details](../reviews/rustguard-tun-src-xdp.rev.md) |
| `rustguard-enroll/src/server.rs` | CRITICAL | 3 | 92% | [details](../reviews/rustguard-enroll-src-server.rev.md) |
| `rustguard-core/src/replay.rs` | CRITICAL | 1 | 90% | [details](../reviews/rustguard-core-src-replay.rev.md) |
| `rustguard-kmod/src/replay.rs` | CRITICAL | 1 | 88% | [details](../reviews/rustguard-kmod-src-replay.rev.md) |
| `rustguard-kmod/src/noise.rs` | NEEDS_REFACTOR | 20 | 92% | [details](../reviews/rustguard-kmod-src-noise.rev.md) |
| `rustguard-core/src/handshake.rs` | NEEDS_REFACTOR | 14 | 90% | [details](../reviews/rustguard-core-src-handshake.rev.md) |
| `rustguard-kmod/src/lib.rs` | NEEDS_REFACTOR | 9 | 95% | [details](../reviews/rustguard-kmod-src-lib.rev.md) |
| `rustguard-daemon/src/tunnel.rs` | NEEDS_REFACTOR | 6 | 88% | [details](../reviews/rustguard-daemon-src-tunnel.rev.md) |
| `rustguard-core/src/cookie.rs` | NEEDS_REFACTOR | 5 | 90% | [details](../reviews/rustguard-core-src-cookie.rev.md) |

## Symbol Details

### `rustguard-tun/src/linux_mq.rs`

| Symbol | Lines | Documentation | Conf. | Detail |
|--------|-------|---------------|-------|--------|
| `MultiQueueTun` | L77–L81 | PARTIAL | 88% | [DEAD] Exported public struct with 0 runtime importers and 0 type-only import... |

### `rustguard-tun/src/xdp.rs`

| Symbol | Lines | Documentation | Conf. | Detail |
|--------|-------|---------------|-------|--------|
| `XdpDesc` | L62–L66 | UNDOCUMENTED | 90% | [USED] Exported struct; pub type in library crate, matches false-positive pat... |
| `XdpConfig` | L126–L132 | PARTIAL | 88% | [USED] Exported struct; pub type in library crate, parameter to create() | [U... |
| `XdpSocket` | L148–L160 | PARTIAL | 92% | [USED] Exported struct; main pub API for AF_XDP zero-copy UDP I/O | [UNIQUE] ... |
| `if_nametoindex` | L449–L458 | UNDOCUMENTED | 88% | [USED] Exported function in library crate, called in create_inner setup | [UN... |

### `rustguard-enroll/src/server.rs`

| Symbol | Lines | Documentation | Conf. | Detail |
|--------|-------|---------------|-------|--------|
| `ServeConfig` | L49–L62 | PARTIAL | 72% | [USED] Exported public struct serving as sole parameter to pub fn run(). Pre-... |
| `run` | L64–L532 | UNDOCUMENTED | 92% | [USED] Exported public function named 'run', the apparent entry point for enr... |
| `setup_xdp` | L548–L577 | PARTIAL | 75% | [USED] Non-exported function (cfg gated to target_os = linux) called at L113 ... |

### `rustguard-core/src/replay.rs`

| Symbol | Lines | Documentation | Conf. | Detail |
|--------|-------|---------------|-------|--------|
| `ReplayWindow` | L13–L19 | PARTIAL | 90% | [DEAD] Exported struct with 0 runtime importers and 0 type-only importers per... |

### `rustguard-kmod/src/replay.rs`

| Symbol | Lines | Documentation | Conf. | Detail |
|--------|-------|---------------|-------|--------|
| `ReplayWindow` | L11–L14 | PARTIAL | 88% | [DEAD] Exported struct (pub(crate)) with zero runtime and type-only importers... |

### `rustguard-kmod/src/noise.rs`

| Symbol | Lines | Documentation | Conf. | Detail |
|--------|-------|---------------|-------|--------|
| `zeroize` | L46–L50 | PARTIAL | 85% | [USED] Exported pub(crate) function used extensively within the file: called ... |
| `constant_time_eq` | L53–L58 | PARTIAL | 85% | [USED] Non-exported helper function called in process_response (line 377) to ... |
| `TAI64_EPOCH_OFFSET` | L67–L67 | PARTIAL | 85% | [USED] Non-exported constant used in tai64n_now (line 234) for TAI64N timesta... |
| `hash` | L72–L92 | PARTIAL | 85% | [USED] Non-exported BLAKE2s hash function called in initial_chain_key (line 1... |
| `mac` | L95–L105 | PARTIAL | 85% | [USED] Non-exported keyed BLAKE2s MAC function called in compute_mac1 (line 2... |
| `hkdf` | L108–L119 | PARTIAL | 80% | [USED] Non-exported key derivation function called in mix_key (line 194), pro... |
| `seal` | L122–L136 | PARTIAL | 80% | [USED] Non-exported AEAD encryption function called in encrypt_and_hash (line... |
| `open` | L139–L153 | PARTIAL | 80% | [USED] Non-exported AEAD decryption function called in decrypt_and_hash (line... |
| `dh` | L156–L160 | PARTIAL | 82% | [USED] Non-exported Curve25519 DH function called in create_initiation (lines... |
| `generate_keypair` | L163–L171 | PARTIAL | 82% | [USED] Non-exported ephemeral keypair generation called in create_initiation ... |
| `encrypt_and_hash` | L199–L203 | PARTIAL | 82% | [USED] Non-exported helper that encrypts and mixes ciphertext into hash, call... |
| `decrypt_and_hash` | L206–L210 | PARTIAL | 82% | [USED] Non-exported helper that decrypts and mixes ciphertext into hash, call... |
| `compute_mac1` | L213–L219 | PARTIAL | 85% | [USED] Non-exported function called in create_initiation (line 328), process_... |
| `tai64n_now` | L224–L238 | PARTIAL | 85% | [USED] Non-exported function called in create_initiation (line 319) to genera... |
| `INITIATION_SIZE` | L249–L249 | UNDOCUMENTED | 90% | [USED] Exported pub(crate) constant used in create_initiation (line 289) for ... |
| `RESPONSE_SIZE` | L250–L250 | UNDOCUMENTED | 90% | [USED] Exported pub(crate) constant used in process_response (line 350) for p... |
| `InitiatorState` | L271–L278 | PARTIAL | 85% | [USED] Exported pub(crate) struct returned from create_initiation (line 289) ... |
| `create_initiation` | L284–L347 | PARTIAL | 88% | [DEAD] Exported pub(crate) function with 0 known importers and no local calls... |
| `process_response` | L350–L425 | PARTIAL | 85% | [DEAD] Exported pub(crate) function with 0 known importers and no local calls... |
| `process_initiation` | L434–L539 | PARTIAL | 88% | [DEAD] Exported pub(crate) function with 0 known importers and no local calls... |

### `rustguard-core/src/handshake.rs`

| Symbol | Lines | Documentation | Conf. | Detail |
|--------|-------|---------------|-------|--------|
| `initial_chain_key` | L23–L25 | PARTIAL | 85% | [USED] Called in create_initiation_with (line 119) and process_initiation_wit... |
| `mix_hash` | L32–L34 | PARTIAL | 85% | [USED] Called in encrypt_and_hash (line 46), decrypt_and_hash (line 54), and ... |
| `mix_key` | L38–L41 | PARTIAL | 90% | [USED] Called multiple times in create_initiation_with (lines 132, 145) and p... |
| `encrypt_and_hash` | L44–L48 | PARTIAL | 72% | [USED] Called in create_initiation_with (lines 139, 148) and process_initiati... |
| `decrypt_and_hash` | L51–L55 | PARTIAL | 72% | [USED] Called in process_response (line 222) and process_initiation_with (lin... |
| `compute_mac1` | L59–L65 | PARTIAL | 88% | [USED] Called in create_initiation_with (line 162), process_initiation_with (... |
| `InitiatorHandshake` | L72–L81 | PARTIAL | 80% | [USED] Returned by create_initiation_with (line 163) and accepted by process_... |
| `create_initiation` | L85–L91 | PARTIAL | 88% | [USED] Called in full_handshake_and_transport test (line 389) and other tests... |
| `create_initiation_psk` | L95–L104 | PARTIAL | 85% | [USED] Called by create_initiation (line 91) and used as entry point for PSK-... |
| `create_initiation_with` | L108–L170 | PARTIAL | 78% | [USED] Called by create_initiation_psk (line 103); core function implementing... |
| `process_response` | L173–L221 | PARTIAL | 76% | [USED] Called in full_handshake_and_transport test (line 392) and failure sce... |
| `process_initiation` | L227–L233 | PARTIAL | 88% | [USED] Called in tests (lines 390, 408, 428, 442); convenience wrapper for re... |
| `process_initiation_psk` | L237–L246 | PARTIAL | 85% | [USED] Called by process_initiation (line 232); PSK-based responder handshake... |
| `process_initiation_with` | L249–L357 | PARTIAL | 77% | [USED] Called by process_initiation_psk (line 245); core function implementin... |

### `rustguard-kmod/src/lib.rs`

| Symbol | Lines | Documentation | Conf. | Detail |
|--------|-------|---------------|-------|--------|
| `AEAD_TAG_SIZE` | L85–L85 | PARTIAL | 85% | [USED] Exported constant used locally in do_xmit (line 397) and handle_transp... |
| `rustguard_xmit` | L341–L343 | PARTIAL | 75% | [USED] FFI export (#[no_mangle] pub extern "C"), TX callback registered with ... |
| `rustguard_rx` | L422–L424 | PARTIAL | 75% | [USED] FFI export (#[no_mangle] pub extern "C"), RX callback registered with ... |
| `handle_initiation` | L474–L532 | PARTIAL | 86% | [USED] Function called in do_rx (line 452); implements responder-side handsha... |
| `handle_response` | L536–L561 | PARTIAL | 87% | [USED] Function called in do_rx (line 455); implements initiator-side handsha... |
| `handle_transport` | L566–L634 | PARTIAL | 86% | [USED] Function called in do_rx (line 458); implements packet decryption, rep... |
| `rustguard_dev_uninit` | L638–L638 | PARTIAL | 90% | [LOW_VALUE] FFI export (#[no_mangle] pub extern "C") marked as device teardow... |
| `rustguard_genl_get` | L642–L646 | PARTIAL | 90% | [LOW_VALUE] FFI export (#[no_mangle] pub extern "C") genetlink GET callback m... |
| `rustguard_genl_set` | L650–L656 | PARTIAL | 90% | [LOW_VALUE] FFI export (#[no_mangle] pub extern "C") genetlink SET callback m... |

### `rustguard-daemon/src/tunnel.rs`

| Symbol | Lines | Documentation | Conf. | Detail |
|--------|-------|---------------|-------|--------|
| `TunnelState` | L30–L35 | PARTIAL | 78% | [USED] Contains tunnel state (peers, handshakes, secrets) shared across threa... |
| `RouteEntry` | L41–L45 | PARTIAL | 78% | [USED] Struct used at L113-128 to track added routes and clean them up at L48... |
| `run` | L48–L489 | PARTIAL | 88% | [DEAD] Exported public function with 0 runtime or type-only importers per pre... |
| `rand_index` | L492–L496 | PARTIAL | 75% | [USED] Generates random u32 sender indices for handshakes; called at L173 and... |
| `fill_random` | L499–L501 | PARTIAL | 75% | [USED] Fills buffer with OS random bytes; called by rand_index() at L496 | [U... |
| `ctrlc_handler` | L504–L525 | PARTIAL | 88% | [USED] Sets up signal handler for clean shutdown; called at L146 during initi... |

### `rustguard-core/src/cookie.rs`

| Symbol | Lines | Documentation | Conf. | Detail |
|--------|-------|---------------|-------|--------|
| `COOKIE_SECRET_LIFETIME` | L21–L21 | PARTIAL | 82% | [USED] Non-exported constant used locally in maybe_rotate_secret (line 90) an... |
| `COOKIE_LEN` | L24–L24 | PARTIAL | 90% | [DEAD] Exported const with 0 external importers per pre-computed analysis. Ru... |
| `CookieChecker` | L48–L57 | PARTIAL | 90% | [DEAD] Exported struct with 0 external importers per pre-computed analysis. R... |
| `CookieState` | L60–L65 | PARTIAL | 85% | [DEAD] Exported struct with 0 external importers per pre-computed analysis. R... |
| `encode_addr` | L227–L243 | PARTIAL | 86% | [USED] Non-exported std-only function used locally on lines 130 (create_reply... |

## Hygiene

- [ ] <!-- ACT-a4f709-8 --> **[documentation · medium · trivial]** `rustguard-core/src/cookie.rs`: Add JSDoc documentation for exported symbol: `COOKIE_LEN` (`COOKIE_LEN`) [L24-L24]
- [ ] <!-- ACT-a4f709-10 --> **[documentation · medium · trivial]** `rustguard-core/src/cookie.rs`: Add JSDoc documentation for exported symbol: `CookieChecker` (`CookieChecker`) [L48-L57]
- [ ] <!-- ACT-a4f709-12 --> **[documentation · medium · trivial]** `rustguard-core/src/cookie.rs`: Add JSDoc documentation for exported symbol: `CookieState` (`CookieState`) [L60-L65]
- [ ] <!-- ACT-f55ce3-3 --> **[documentation · medium · trivial]** `rustguard-core/src/replay.rs`: Add JSDoc documentation for exported symbol: `ReplayWindow` (`ReplayWindow`) [L13-L19]
- [ ] <!-- ACT-bd20cf-7 --> **[documentation · medium · trivial]** `rustguard-daemon/src/tunnel.rs`: Add JSDoc documentation for exported symbol: `run` (`run`) [L48-L489]
- [ ] <!-- ACT-a732f2-5 --> **[documentation · medium · trivial]** `rustguard-enroll/src/server.rs`: Add JSDoc documentation for exported symbol: `run` (`run`) [L64-L532]
- [ ] <!-- ACT-58e5ff-24 --> **[documentation · medium · trivial]** `rustguard-kmod/src/noise.rs`: Add JSDoc documentation for exported symbol: `INITIATION_SIZE` (`INITIATION_SIZE`) [L249-L249]
- [ ] <!-- ACT-58e5ff-25 --> **[documentation · medium · trivial]** `rustguard-kmod/src/noise.rs`: Add JSDoc documentation for exported symbol: `RESPONSE_SIZE` (`RESPONSE_SIZE`) [L250-L250]
- [ ] <!-- ACT-58e5ff-28 --> **[documentation · medium · trivial]** `rustguard-kmod/src/noise.rs`: Add JSDoc documentation for exported symbol: `create_initiation` (`create_initiation`) [L284-L347]
- [ ] <!-- ACT-58e5ff-30 --> **[documentation · medium · trivial]** `rustguard-kmod/src/noise.rs`: Add JSDoc documentation for exported symbol: `process_response` (`process_response`) [L350-L425]
- [ ] <!-- ACT-58e5ff-32 --> **[documentation · medium · trivial]** `rustguard-kmod/src/noise.rs`: Add JSDoc documentation for exported symbol: `process_initiation` (`process_initiation`) [L434-L539]
- [ ] <!-- ACT-c30836-3 --> **[documentation · medium · trivial]** `rustguard-kmod/src/replay.rs`: Add JSDoc documentation for exported symbol: `ReplayWindow` (`ReplayWindow`) [L11-L14]
- [ ] <!-- ACT-22946f-22 --> **[documentation · medium · trivial]** `rustguard-tun/src/linux_mq.rs`: Add JSDoc documentation for exported symbol: `MultiQueueTun` (`MultiQueueTun`) [L77-L81]
- [ ] <!-- ACT-8fc69f-7 --> **[documentation · medium · trivial]** `rustguard-tun/src/xdp.rs`: Add JSDoc documentation for exported symbol: `XdpDesc` (`XdpDesc`) [L62-L66]
- [ ] <!-- ACT-8fc69f-10 --> **[documentation · medium · trivial]** `rustguard-tun/src/xdp.rs`: Add JSDoc documentation for exported symbol: `if_nametoindex` (`if_nametoindex`) [L449-L458]
- [ ] <!-- ACT-a4f709-6 --> **[documentation · low · trivial]** `rustguard-core/src/cookie.rs`: Complete JSDoc documentation for: `COOKIE_SECRET_LIFETIME` (`COOKIE_SECRET_LIFETIME`) [L21-L21]
- [ ] <!-- ACT-a4f709-13 --> **[documentation · low · trivial]** `rustguard-core/src/cookie.rs`: Complete JSDoc documentation for: `encode_addr` (`encode_addr`) [L227-L243]
- [ ] <!-- ACT-5920d4-2 --> **[documentation · low · trivial]** `rustguard-core/src/handshake.rs`: Complete JSDoc documentation for: `initial_chain_key` (`initial_chain_key`) [L23-L25]
- [ ] <!-- ACT-5920d4-3 --> **[documentation · low · trivial]** `rustguard-core/src/handshake.rs`: Complete JSDoc documentation for: `mix_hash` (`mix_hash`) [L32-L34]
- [ ] <!-- ACT-5920d4-5 --> **[documentation · low · trivial]** `rustguard-core/src/handshake.rs`: Complete JSDoc documentation for: `mix_key` (`mix_key`) [L38-L41]
- [ ] <!-- ACT-5920d4-6 --> **[documentation · low · trivial]** `rustguard-core/src/handshake.rs`: Complete JSDoc documentation for: `encrypt_and_hash` (`encrypt_and_hash`) [L44-L48]
- [ ] <!-- ACT-5920d4-7 --> **[documentation · low · trivial]** `rustguard-core/src/handshake.rs`: Complete JSDoc documentation for: `decrypt_and_hash` (`decrypt_and_hash`) [L51-L55]
- [ ] <!-- ACT-5920d4-9 --> **[documentation · low · trivial]** `rustguard-core/src/handshake.rs`: Complete JSDoc documentation for: `compute_mac1` (`compute_mac1`) [L59-L65]
- [ ] <!-- ACT-5920d4-10 --> **[documentation · low · trivial]** `rustguard-core/src/handshake.rs`: Complete JSDoc documentation for: `InitiatorHandshake` (`InitiatorHandshake`) [L72-L81]
- [ ] <!-- ACT-5920d4-11 --> **[documentation · low · trivial]** `rustguard-core/src/handshake.rs`: Complete JSDoc documentation for: `create_initiation` (`create_initiation`) [L85-L91]
- [ ] <!-- ACT-5920d4-12 --> **[documentation · low · trivial]** `rustguard-core/src/handshake.rs`: Complete JSDoc documentation for: `create_initiation_psk` (`create_initiation_psk`) [L95-L104]
- [ ] <!-- ACT-5920d4-13 --> **[documentation · low · trivial]** `rustguard-core/src/handshake.rs`: Complete JSDoc documentation for: `create_initiation_with` (`create_initiation_with`) [L108-L170]
- [ ] <!-- ACT-5920d4-14 --> **[documentation · low · trivial]** `rustguard-core/src/handshake.rs`: Complete JSDoc documentation for: `process_response` (`process_response`) [L173-L221]
- [ ] <!-- ACT-5920d4-15 --> **[documentation · low · trivial]** `rustguard-core/src/handshake.rs`: Complete JSDoc documentation for: `process_initiation` (`process_initiation`) [L227-L233]
- [ ] <!-- ACT-5920d4-16 --> **[documentation · low · trivial]** `rustguard-core/src/handshake.rs`: Complete JSDoc documentation for: `process_initiation_psk` (`process_initiation_psk`) [L237-L246]
- [ ] <!-- ACT-5920d4-17 --> **[documentation · low · trivial]** `rustguard-core/src/handshake.rs`: Complete JSDoc documentation for: `process_initiation_with` (`process_initiation_with`) [L249-L357]
- [ ] <!-- ACT-bd20cf-4 --> **[documentation · low · trivial]** `rustguard-daemon/src/tunnel.rs`: Complete JSDoc documentation for: `TunnelState` (`TunnelState`) [L30-L35]
- [ ] <!-- ACT-bd20cf-5 --> **[documentation · low · trivial]** `rustguard-daemon/src/tunnel.rs`: Complete JSDoc documentation for: `RouteEntry` (`RouteEntry`) [L41-L45]
- [ ] <!-- ACT-bd20cf-9 --> **[documentation · low · trivial]** `rustguard-daemon/src/tunnel.rs`: Complete JSDoc documentation for: `rand_index` (`rand_index`) [L492-L496]
- [ ] <!-- ACT-bd20cf-10 --> **[documentation · low · trivial]** `rustguard-daemon/src/tunnel.rs`: Complete JSDoc documentation for: `fill_random` (`fill_random`) [L499-L501]
- [ ] <!-- ACT-bd20cf-11 --> **[documentation · low · trivial]** `rustguard-daemon/src/tunnel.rs`: Complete JSDoc documentation for: `ctrlc_handler` (`ctrlc_handler`) [L504-L525]
- [ ] <!-- ACT-a732f2-4 --> **[documentation · low · trivial]** `rustguard-enroll/src/server.rs`: Complete JSDoc documentation for: `ServeConfig` (`ServeConfig`) [L49-L62]
- [ ] <!-- ACT-a732f2-8 --> **[documentation · low · trivial]** `rustguard-enroll/src/server.rs`: Complete JSDoc documentation for: `setup_xdp` (`setup_xdp`) [L548-L577]
- [ ] <!-- ACT-bb41a6-7 --> **[documentation · low · trivial]** `rustguard-kmod/src/lib.rs`: Complete JSDoc documentation for: `AEAD_TAG_SIZE` (`AEAD_TAG_SIZE`) [L85-L85]
- [ ] <!-- ACT-bb41a6-8 --> **[documentation · low · trivial]** `rustguard-kmod/src/lib.rs`: Complete JSDoc documentation for: `rustguard_xmit` (`rustguard_xmit`) [L341-L343]
- [ ] <!-- ACT-bb41a6-9 --> **[documentation · low · trivial]** `rustguard-kmod/src/lib.rs`: Complete JSDoc documentation for: `rustguard_rx` (`rustguard_rx`) [L422-L424]
- [ ] <!-- ACT-bb41a6-10 --> **[documentation · low · trivial]** `rustguard-kmod/src/lib.rs`: Complete JSDoc documentation for: `handle_initiation` (`handle_initiation`) [L474-L532]
- [ ] <!-- ACT-bb41a6-11 --> **[documentation · low · trivial]** `rustguard-kmod/src/lib.rs`: Complete JSDoc documentation for: `handle_response` (`handle_response`) [L536-L561]
- [ ] <!-- ACT-bb41a6-12 --> **[documentation · low · trivial]** `rustguard-kmod/src/lib.rs`: Complete JSDoc documentation for: `handle_transport` (`handle_transport`) [L566-L634]
- [ ] <!-- ACT-bb41a6-14 --> **[documentation · low · trivial]** `rustguard-kmod/src/lib.rs`: Complete JSDoc documentation for: `rustguard_dev_uninit` (`rustguard_dev_uninit`) [L638-L638]
- [ ] <!-- ACT-bb41a6-16 --> **[documentation · low · trivial]** `rustguard-kmod/src/lib.rs`: Complete JSDoc documentation for: `rustguard_genl_get` (`rustguard_genl_get`) [L642-L646]
- [ ] <!-- ACT-bb41a6-18 --> **[documentation · low · trivial]** `rustguard-kmod/src/lib.rs`: Complete JSDoc documentation for: `rustguard_genl_set` (`rustguard_genl_set`) [L650-L656]
- [ ] <!-- ACT-58e5ff-3 --> **[documentation · low · trivial]** `rustguard-kmod/src/noise.rs`: Complete JSDoc documentation for: `zeroize` (`zeroize`) [L46-L50]
- [ ] <!-- ACT-58e5ff-5 --> **[documentation · low · trivial]** `rustguard-kmod/src/noise.rs`: Complete JSDoc documentation for: `constant_time_eq` (`constant_time_eq`) [L53-L58]
- [ ] <!-- ACT-58e5ff-6 --> **[documentation · low · trivial]** `rustguard-kmod/src/noise.rs`: Complete JSDoc documentation for: `TAI64_EPOCH_OFFSET` (`TAI64_EPOCH_OFFSET`) [L67-L67]
- [ ] <!-- ACT-58e5ff-8 --> **[documentation · low · trivial]** `rustguard-kmod/src/noise.rs`: Complete JSDoc documentation for: `hash` (`hash`) [L72-L92]
- [ ] <!-- ACT-58e5ff-10 --> **[documentation · low · trivial]** `rustguard-kmod/src/noise.rs`: Complete JSDoc documentation for: `mac` (`mac`) [L95-L105]
- [ ] <!-- ACT-58e5ff-11 --> **[documentation · low · trivial]** `rustguard-kmod/src/noise.rs`: Complete JSDoc documentation for: `hkdf` (`hkdf`) [L108-L119]
- [ ] <!-- ACT-58e5ff-12 --> **[documentation · low · trivial]** `rustguard-kmod/src/noise.rs`: Complete JSDoc documentation for: `seal` (`seal`) [L122-L136]
- [ ] <!-- ACT-58e5ff-13 --> **[documentation · low · trivial]** `rustguard-kmod/src/noise.rs`: Complete JSDoc documentation for: `open` (`open`) [L139-L153]
- [ ] <!-- ACT-58e5ff-14 --> **[documentation · low · trivial]** `rustguard-kmod/src/noise.rs`: Complete JSDoc documentation for: `dh` (`dh`) [L156-L160]
- [ ] <!-- ACT-58e5ff-15 --> **[documentation · low · trivial]** `rustguard-kmod/src/noise.rs`: Complete JSDoc documentation for: `generate_keypair` (`generate_keypair`) [L163-L171]
- [ ] <!-- ACT-58e5ff-18 --> **[documentation · low · trivial]** `rustguard-kmod/src/noise.rs`: Complete JSDoc documentation for: `encrypt_and_hash` (`encrypt_and_hash`) [L199-L203]
- [ ] <!-- ACT-58e5ff-19 --> **[documentation · low · trivial]** `rustguard-kmod/src/noise.rs`: Complete JSDoc documentation for: `decrypt_and_hash` (`decrypt_and_hash`) [L206-L210]
- [ ] <!-- ACT-58e5ff-20 --> **[documentation · low · trivial]** `rustguard-kmod/src/noise.rs`: Complete JSDoc documentation for: `compute_mac1` (`compute_mac1`) [L213-L219]
- [ ] <!-- ACT-58e5ff-21 --> **[documentation · low · trivial]** `rustguard-kmod/src/noise.rs`: Complete JSDoc documentation for: `tai64n_now` (`tai64n_now`) [L224-L238]
- [ ] <!-- ACT-58e5ff-26 --> **[documentation · low · trivial]** `rustguard-kmod/src/noise.rs`: Complete JSDoc documentation for: `InitiatorState` (`InitiatorState`) [L271-L278]
- [ ] <!-- ACT-8fc69f-8 --> **[documentation · low · trivial]** `rustguard-tun/src/xdp.rs`: Complete JSDoc documentation for: `XdpConfig` (`XdpConfig`) [L126-L132]
- [ ] <!-- ACT-8fc69f-9 --> **[documentation · low · trivial]** `rustguard-tun/src/xdp.rs`: Complete JSDoc documentation for: `XdpSocket` (`XdpSocket`) [L148-L160]
