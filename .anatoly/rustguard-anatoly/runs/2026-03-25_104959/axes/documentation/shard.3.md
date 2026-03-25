# Documentation — Shard 3

## Findings

| File | Verdict | Documentation | Conf. | Details |
|------|---------|---------------|-------|---------|
| `rustguard-kmod/src/cookie.rs` | NEEDS_REFACTOR | 3 | 80% | [details](../reviews/rustguard-kmod-src-cookie.rev.md) |
| `rustguard-enroll/src/client.rs` | NEEDS_REFACTOR | 2 | 88% | [details](../reviews/rustguard-enroll-src-client.rev.md) |
| `rustguard-crypto/src/blake2s.rs` | NEEDS_REFACTOR | 4 | 92% | [details](../reviews/rustguard-crypto-src-blake2s.rev.md) |
| `rustguard-enroll/src/packet.rs` | NEEDS_REFACTOR | 2 | 92% | [details](../reviews/rustguard-enroll-src-packet.rev.md) |
| `rustguard-enroll/src/state.rs` | NEEDS_REFACTOR | 4 | 92% | [details](../reviews/rustguard-enroll-src-state.rev.md) |
| `rustguard-crypto/src/x25519.rs` | NEEDS_REFACTOR | 4 | 87% | [details](../reviews/rustguard-crypto-src-x25519.rev.md) |
| `rustguard-tun/examples/tun_echo.rs` | NEEDS_REFACTOR | 1 | 85% | [details](../reviews/rustguard-tun-examples-tun_echo.rev.md) |
| `rustguard-tun/src/uring.rs` | NEEDS_REFACTOR | 3 | 90% | [details](../reviews/rustguard-tun-src-uring.rev.md) |
| `rustguard-crypto/src/tai64n.rs` | NEEDS_REFACTOR | 1 | 92% | [details](../reviews/rustguard-crypto-src-tai64n.rev.md) |
| `rustguard-kmod/src/allowedips.rs` | NEEDS_REFACTOR | 1 | 88% | [details](../reviews/rustguard-kmod-src-allowedips.rev.md) |

## Symbol Details

### `rustguard-kmod/src/cookie.rs`

| Symbol | Lines | Documentation | Conf. | Detail |
|--------|-------|---------------|-------|--------|
| `CookieChecker` | L66–L71 | PARTIAL | 80% | [USED] Server-side public API struct for cookie generation and MAC2 validatio... |
| `CookieState` | L74–L77 | PARTIAL | 80% | [USED] Client-side public API struct for storing decrypted cookies. Known fal... |
| `constant_time_eq` | L202–L205 | PARTIAL | 80% | [USED] Constant-time equality check called in verify_mac1 (L122) and verify_m... |

### `rustguard-enroll/src/client.rs`

| Symbol | Lines | Documentation | Conf. | Detail |
|--------|-------|---------------|-------|--------|
| `JoinConfig` | L23–L26 | UNDOCUMENTED | 88% | [DEAD] Exported struct with 0 imports per pre-computed analysis (Rule 2 → DEA... |
| `run` | L28–L192 | UNDOCUMENTED | 85% | [DEAD] Exported function with 0 imports per pre-computed analysis (Rule 2 → D... |

### `rustguard-crypto/src/blake2s.rs`

| Symbol | Lines | Documentation | Conf. | Detail |
|--------|-------|---------------|-------|--------|
| `hash` | L9–L15 | PARTIAL | 92% | [DEAD] Exported function. Pre-computed analysis shows 0 runtime importers and... |
| `mac` | L21–L29 | PARTIAL | 91% | [DEAD] Exported function. Pre-computed analysis shows 0 runtime importers and... |
| `hkdf` | L37–L57 | PARTIAL | 92% | [DEAD] Exported function. Pre-computed analysis shows 0 runtime importers and... |
| `hmac_blake2s` | L64–L88 | PARTIAL | 85% | [USED] Non-exported function. Called 4 times in hkdf (lines 48, 51, 56, 61) a... |

### `rustguard-enroll/src/packet.rs`

| Symbol | Lines | Documentation | Conf. | Detail |
|--------|-------|---------------|-------|--------|
| `ParsedUdp` | L9–L12 | PARTIAL | 92% | [DEAD] Exported struct with 0 file importers per pre-computed analysis. Thoug... |
| `parse_eth_udp` | L16–L28 | PARTIAL | 92% | [DEAD] Exported function with 0 file importers per pre-computed analysis. Mai... |

### `rustguard-enroll/src/state.rs`

| Symbol | Lines | Documentation | Conf. | Detail |
|--------|-------|---------------|-------|--------|
| `PersistedPeer` | L13–L16 | PARTIAL | 92% | [DEAD] Exported struct with 0 runtime importers and 0 type-only importers; ru... |
| `default_state_path` | L19–L21 | PARTIAL | 90% | [DEAD] Exported function with 0 runtime importers and 0 type-only importers; ... |
| `save` | L25–L45 | PARTIAL | 92% | [DEAD] Exported function with 0 runtime importers and 0 type-only importers; ... |
| `load` | L48–L84 | PARTIAL | 92% | [DEAD] Exported function with 0 runtime importers and 0 type-only importers; ... |

### `rustguard-crypto/src/x25519.rs`

| Symbol | Lines | Documentation | Conf. | Detail |
|--------|-------|---------------|-------|--------|
| `StaticSecret` | L7–L7 | PARTIAL | 82% | [USED] Pub struct in library crate; core X25519 type. Follows pattern of know... |
| `EphemeralSecret` | L11–L11 | PARTIAL | 82% | [USED] Pub struct in library crate for ephemeral keys. Identical pattern to k... |
| `PublicKey` | L16–L16 | PARTIAL | 87% | [USED] Pub struct in library crate with security-critical constant-time equal... |
| `SharedSecret` | L28–L28 | PARTIAL | 87% | [USED] Pub struct in library crate with zeroization on drop. Follows known fa... |

### `rustguard-tun/examples/tun_echo.rs`

| Symbol | Lines | Documentation | Conf. | Detail |
|--------|-------|---------------|-------|--------|
| `main` | L12–L79 | PARTIAL | 78% | [USED] Entry point of the binary, executed by Rust runtime. Essential to the ... |

### `rustguard-tun/src/uring.rs`

| Symbol | Lines | Documentation | Conf. | Detail |
|--------|-------|---------------|-------|--------|
| `BufferPool` | L26–L31 | PARTIAL | 80% | [USED] Public struct exported as part of module API. Used as field type in Ur... |
| `UringTun` | L96–L102 | PARTIAL | 82% | [USED] Main public API struct of the module. Provides four public methods (ne... |
| `READ_FLAG` | L105–L105 | PARTIAL | 88% | [USED] Internal bit flag constant used to encode operation type in io_uring u... |

### `rustguard-crypto/src/tai64n.rs`

| Symbol | Lines | Documentation | Conf. | Detail |
|--------|-------|---------------|-------|--------|
| `Tai64n` | L9–L9 | PARTIAL | 92% | [DEAD] Pre-computed analysis shows 0 runtime importers and 0 type-only import... |

### `rustguard-kmod/src/allowedips.rs`

| Symbol | Lines | Documentation | Conf. | Detail |
|--------|-------|---------------|-------|--------|
| `AllowedIps` | L28–L31 | PARTIAL | 85% | [USED] Exported struct matching known false-positive pattern (similar to Peer... |

## Hygiene

- [ ] <!-- ACT-bc7d6e-2 --> **[documentation · medium · trivial]** `rustguard-crypto/src/blake2s.rs`: Add JSDoc documentation for exported symbol: `hash` (`hash`) [L9-L15]
- [ ] <!-- ACT-bc7d6e-4 --> **[documentation · medium · trivial]** `rustguard-crypto/src/blake2s.rs`: Add JSDoc documentation for exported symbol: `mac` (`mac`) [L21-L29]
- [ ] <!-- ACT-bc7d6e-6 --> **[documentation · medium · trivial]** `rustguard-crypto/src/blake2s.rs`: Add JSDoc documentation for exported symbol: `hkdf` (`hkdf`) [L37-L57]
- [ ] <!-- ACT-ca1d92-5 --> **[documentation · medium · trivial]** `rustguard-enroll/src/client.rs`: Add JSDoc documentation for exported symbol: `JoinConfig` (`JoinConfig`) [L23-L26]
- [ ] <!-- ACT-ca1d92-7 --> **[documentation · medium · trivial]** `rustguard-enroll/src/client.rs`: Add JSDoc documentation for exported symbol: `run` (`run`) [L28-L192]
- [ ] <!-- ACT-4cf3f3-3 --> **[documentation · medium · trivial]** `rustguard-enroll/src/packet.rs`: Add JSDoc documentation for exported symbol: `ParsedUdp` (`ParsedUdp`) [L9-L12]
- [ ] <!-- ACT-bc7d6e-7 --> **[documentation · low · trivial]** `rustguard-crypto/src/blake2s.rs`: Complete JSDoc documentation for: `hmac_blake2s` (`hmac_blake2s`) [L64-L88]
- [ ] <!-- ACT-7726d0-3 --> **[documentation · low · trivial]** `rustguard-crypto/src/x25519.rs`: Complete JSDoc documentation for: `StaticSecret` (`StaticSecret`) [L7-L7]
- [ ] <!-- ACT-7726d0-4 --> **[documentation · low · trivial]** `rustguard-crypto/src/x25519.rs`: Complete JSDoc documentation for: `EphemeralSecret` (`EphemeralSecret`) [L11-L11]
- [ ] <!-- ACT-7726d0-5 --> **[documentation · low · trivial]** `rustguard-crypto/src/x25519.rs`: Complete JSDoc documentation for: `PublicKey` (`PublicKey`) [L16-L16]
- [ ] <!-- ACT-7726d0-6 --> **[documentation · low · trivial]** `rustguard-crypto/src/x25519.rs`: Complete JSDoc documentation for: `SharedSecret` (`SharedSecret`) [L28-L28]
- [ ] <!-- ACT-a01ecd-6 --> **[documentation · low · trivial]** `rustguard-kmod/src/allowedips.rs`: Complete JSDoc documentation for: `AllowedIps` (`AllowedIps`) [L28-L31]
- [ ] <!-- ACT-9105c7-5 --> **[documentation · low · trivial]** `rustguard-kmod/src/cookie.rs`: Complete JSDoc documentation for: `CookieChecker` (`CookieChecker`) [L66-L71]
- [ ] <!-- ACT-9105c7-6 --> **[documentation · low · trivial]** `rustguard-kmod/src/cookie.rs`: Complete JSDoc documentation for: `CookieState` (`CookieState`) [L74-L77]
- [ ] <!-- ACT-9105c7-8 --> **[documentation · low · trivial]** `rustguard-kmod/src/cookie.rs`: Complete JSDoc documentation for: `constant_time_eq` (`constant_time_eq`) [L202-L205]
- [ ] <!-- ACT-fe7cbd-2 --> **[documentation · low · trivial]** `rustguard-tun/examples/tun_echo.rs`: Complete JSDoc documentation for: `main` (`main`) [L12-L79]
- [ ] <!-- ACT-0723df-6 --> **[documentation · low · trivial]** `rustguard-tun/src/uring.rs`: Complete JSDoc documentation for: `BufferPool` (`BufferPool`) [L26-L31]
- [ ] <!-- ACT-0723df-7 --> **[documentation · low · trivial]** `rustguard-tun/src/uring.rs`: Complete JSDoc documentation for: `UringTun` (`UringTun`) [L96-L102]
