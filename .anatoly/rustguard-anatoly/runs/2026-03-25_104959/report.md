<p align="center">
  <img src="https://raw.githubusercontent.com/r-via/anatoly/main/assets/imgs/logo.jpg" width="400" alt="Anatoly" />
</p>

# Anatoly Audit Report

## Executive Summary

- **Files reviewed:** 41
- **Global verdict:** CRITICAL
- **Clean files:** 4
- **Files with findings:** 37

| Category | High | Medium | Low | Total |
|----------|------|--------|-----|-------|
| Correction errors | 47 | 8 | 0 | 55 |
| Utility | 1 | 1 | 0 | 2 |
| Duplicates | 10 | 27 | 0 | 37 |
| Test coverage gaps | 51 | 18 | 161 | 230 |
| Best practices | 20 | 3 | 0 | 23 |
| Documentation gaps | 28 | 4 | 117 | 149 |

## Axes

| Axis | Files | Shards | Link |
|------|-------|--------|------|
| Correction | 27 | 3 | [axes/correction/index.md](./axes/correction/index.md) |
| Utility | 3 | 1 | [axes/utility/index.md](./axes/utility/index.md) |
| Duplication | 7 | 1 | [axes/duplication/index.md](./axes/duplication/index.md) |
| Tests | 37 | 4 | [axes/tests/index.md](./axes/tests/index.md) |
| Documentation | 32 | 4 | [axes/documentation/index.md](./axes/documentation/index.md) |
| Best Practices | 15 | 2 | [axes/best-practices/index.md](./axes/best-practices/index.md) |

## Run Statistics

| Metric | Value |
|--------|-------|
| Run ID | `2026-03-25_104959` |
| Duration | 79.4 min |
| API cost | $48.76 |

**Phase durations:**

| Phase | Duration |
|-------|----------|
| scan | 598ms |
| estimate | 212ms |
| triage | 3ms |
| rag-index | 801.3s |
| review | 3476.5s |
| internal-docs | 172.5s |
| report | 69ms |

**Per-axis breakdown:**

| Axis | Calls | Duration | Cost | Tokens (in/out) |
|------|-------|----------|------|-----------------|
| utility | 38 | 38.9m | $2.05 | 349 / 319883 |
| duplication | 38 | 26.3m | $1.69 | 350 / 232156 |
| correction | 38 | 129.1m | $14.58 | 87 / 530237 |
| overengineering | 38 | 24.7m | $3.72 | 107 / 92575 |
| tests | 38 | 26.7m | $4.62 | 83 / 104317 |
| best_practices | 38 | 76.7m | $8.22 | 117 / 268783 |
| documentation | 38 | 40.4m | $6.61 | 83 / 173248 |

## Axis Summary

**Utility** — 378 symbols evaluated

| Verdict | Count | % |
|---------|-------|---|
| USED | 375 | 99% |
| DEAD | 2 | 1% |
| LOW_VALUE | 1 | 0% |

**Duplication** — 378 symbols evaluated

| Verdict | Count | % |
|---------|-------|---|
| UNIQUE | 341 | 90% |
| DUPLICATE | 37 | 10% |

**Correction** — 378 symbols evaluated

| Verdict | Count | % |
|---------|-------|---|
| OK | 323 | 85% |
| NEEDS_FIX | 50 | 13% |
| ERROR | 5 | 1% |

**Overengineering** — 378 symbols evaluated

| Verdict | Count | % |
|---------|-------|---|
| LEAN | 352 | 93% |
| ACCEPTABLE | 26 | 7% |

**Tests** — 374 symbols evaluated

| Verdict | Count | % |
|---------|-------|---|
| GOOD | 144 | 39% |
| WEAK | 70 | 19% |
| NONE | 160 | 43% |

**Documentation** — 374 symbols evaluated

| Verdict | Count | % |
|---------|-------|---|
| DOCUMENTED | 111 | 30% |
| PARTIAL | 123 | 33% |
| UNDOCUMENTED | 140 | 37% |

**Best Practices** — 38 files evaluated

| Metric | Value |
|--------|-------|
| Average score | 7.3/10 |
| Min / Max | 1.5 / 9.5 |

## Deliberation

| Metric | Value |
|--------|-------|
| Files deliberated | 37 |
| Symbols reclassified | 111 |
| Actions removed | 82 |
| Verdict changes | 2 |

**Verdict changes:**

- `rustguard-core/tests/integration.rs`: CLEAN → NEEDS_REFACTOR
- `rustguard-tun/src/lib.rs`: CLEAN → NEEDS_REFACTOR

**Reclassified files:**

- `rustguard-core/src/cookie.rs`: 9 symbol(s) — Major reclassifications: (1) Three exported symbols (COOKIE_LEN, CookieChecker, CookieState) reclassified DEAD → USED — these are the primary public API of a library crate and pre-computed analysis likely missed cross-crate workspace usage. (2) Two std-gated functions (elapsed_since L38-40, fill_random L261-263) had NEEDS_FIX findings that describe no_std behavior incorrectly applied to the std variants — reclassified to OK. (3) no_std elapsed_since reclassified NEEDS_FIX → OK as intentional documented design (kernel module uses its own checker). (4) encode_addr reclassified NEEDS_FIX → OK since `alloc::vec::Vec` compiles (proven by passing tests). (5) random_nonce DUPLICATE → UNIQUE since deduplicating a 4-line function across separate crates would add unnecessary coupling. Remaining NEEDS_FIX: CookieState's no_std process_reply (silently stores cookie but never marks fresh), fill_random no_std stub (zero-fills with no warning), and constant_time_eq_16's data-dependent branch. These are legitimate concerns warranting NEEDS_REFACTOR verdict, with the fill_random stub being the most serious.
- `rustguard-core/src/handshake.rs`: 2 symbol(s) — Overall assessment: This is a well-structured WireGuard Noise_IKpsk2 handshake implementation with correct cryptographic operations and good separation of concerns (std/no_std layers). Two key reclassifications: (1) mix_key and compute_mac1 DUPLICATE → UNIQUE — the parallel implementations in rustguard-core (userspace, x25519-dalek types) and rustguard-kmod (kernel module, raw bytes/C FFI) are architecturally necessary, not accidental duplication. Cross-crate deduplication would force inappropriate dependencies between fundamentally different compilation targets. Actions 4 and 8 removed accordingly. (2) process_response NEEDS_FIX confirmed — missing MAC1 verification on Response messages is a real DoS vector, as sender_index is plaintext-observable. The responder side correctly checks MAC1 first, making the asymmetry a clear oversight. Test coverage is consistently WEAK across all symbols — the tests cover happy paths and basic failure modes but lack known-answer vectors, PSK testing, replay-protection testing, and isolated primitive verification, which are important gaps for a cryptographic library. Documentation is consistently PARTIAL/UNDOCUMENTED — acceptable for private helpers but insufficient for the public API surface of a security-critical crate. Verdict remains NEEDS_REFACTOR driven by the process_response MAC1 gap and systematic test coverage weakness.
- `rustguard-core/src/replay.rs`: 1 symbol(s) — The critical finding is the confirmed ERROR in shift_window's sub-word bit-shift logic (L112-116). The reverse-iteration right-shift moves seen-counter marks toward lower ages instead of higher ages on each window advance, silently discarding replay-protection state for any shift whose low 6 bits are nonzero. This is a real security vulnerability in an anti-replay primitive. The DEAD utility finding for ReplayWindow was a false positive — it's a pub type in a library crate and constitutes the module's intended public API; action 2 (remove dead code) is removed. Tests exist but are WEAK rather than NONE: they exercise core behavior but miss the shift_window bug because no test re-checks a previously-accepted counter after a window shift. Documentation is PARTIAL, not UNDOCUMENTED, since fields and methods have doc comments but the struct definition does not. Verdict remains CRITICAL due to the confirmed shift_window direction inversion.
- `rustguard-core/src/session.rs`: 1 symbol(s) — The primary reclassification is DEAD→USED: `TransportSession` is a public struct in a library crate (`rustguard-core`) clearly designed for consumption by other crates in the workspace. Flagging it as dead code is a false positive from intra-crate-only import analysis. Action 1 (remove dead code) is removed accordingly. Documentation was incorrectly marked UNDOCUMENTED despite the detail text confirming DOCUMENTED status and thorough `///` comments on the struct, fields, and most methods. Action 2 (add JSDoc — wrong terminology for Rust) is removed since the symbol is documented; the minor gap of two undocumented public methods is captured in best_practices suggestions. Tests were marked NONE but the file contains 6 substantive tests; reclassified to WEAK due to missing coverage of `encrypt_to`, `decrypt_in_place`, and nonce exhaustion edge cases. This is the remaining legitimate finding. Verdict stays NEEDS_REFACTOR based on the WEAK test coverage of public API surface.
- `rustguard-core/src/timers.rs`: 3 symbol(s) — Overall assessment: The file implements WireGuard timer constants and session lifecycle correctly for std builds, with two genuine bugs: (1) needs_keepalive has a logical contradiction when last_sent is None, silently suppressing keepalives; (2) the REKEY_TIMEOUT doc comment is semantically wrong. The no_std elapsed_since returning Duration::ZERO is an intentional design pattern but the absence of _at() query method variants leaves the no_std time path incomplete. Removed actions 4 and 5 because cfg-gated one-liner stubs in separate modules are not meaningful duplication. Removed action 7 because KEEPALIVE_TIMEOUT is a protocol constant exported for downstream consumers, not dead code. Removed action 8 because KEEPALIVE_TIMEOUT already has a doc comment — the UNDOCUMENTED finding contradicted the evaluator's own detail text. The std elapsed_since (L21-23) was incorrectly assessed as having no_std problems — it is the correct std variant. Verdict remains NEEDS_REFACTOR due to the confirmed needs_keepalive bug and incomplete no_std story.
- `rustguard-core/tests/integration.rs`: 1 symbol(s) — Overall this is an excellent integration test suite scoring 9.5/10 on best practices. The only actionable findings are: (1) out_of_order_delivery has WEAK test coverage missing replay window boundary cases, and (2) wire_format_roundtrip is ACCEPTABLE rather than LEAN due to its combined scope. The do_handshake PARTIAL documentation finding is reclassified to DOCUMENTED — for a private test helper, the existing doc comment plus the self-documenting return type signature are fully adequate under test-module leniency. Action 1 (complete docs for do_handshake) is removed as the documentation is sufficient for its context and also uses incorrect terminology ('JSDoc' in a Rust file). Verdict remains CLEAN — no corrections needed, no dead code, no duplications, and findings are minor quality observations only.
- `rustguard-crypto/src/aead.rs`: 8 symbol(s) — Major reclassifications: (1) ALL DEAD→USED: Every exported symbol was flagged DEAD due to zero in-crate importers, but this is `rustguard-crypto`, a library crate. All public functions and constants constitute the crate's public API consumed by downstream crates (e.g., `rustguard` main crate). This is a textbook false positive for library exports. All 8 'Remove dead code' actions (3,5,7,9,11,13,15,17) removed. (2) UNDOCUMENTED→PARTIAL for 7 symbols: All public functions have `///` doc comments in the source — the UNDOCUMENTED classification was factually wrong. Only AEAD_TAG_LEN (L7) truly lacks a doc comment. Documentation actions 6,8,10,12,14,16,18 removed as they say 'Add JSDoc documentation' for symbols that already have Rust doc comments (action 4 retained for AEAD_TAG_LEN which genuinely has none). (3) Tests NONE→corrected for seal (WEAK, 5 tests use it), open (GOOD, 5 tests with comprehensive failure modes), AEAD_TAG_LEN/MAX_PACKET_SIZE (GOOD, compile-time constants). xseal, xopen, seal_to, open_to remain NONE — genuine test gaps on security-critical paths. Two confirmed code issues remain: MAX_PACKET_SIZE doc/code mismatch (action 1) and open_to missing ct_len>buf.len() guard (action 2). Verdict remains NEEDS_REFACTOR due to the open_to bounds check issue and untested x-variant and in-place functions.
- `rustguard-crypto/src/blake2s.rs`: 4 symbol(s) — The three DEAD utility findings (actions 1, 3, 5) are false positives. This is a library crate (rustguard-crypto) whose exported functions constitute its public API — absence of in-workspace importers is expected and does not indicate dead code. Similarly, all three UNDOCUMENTED assessments for exported functions contradicted both the detail text and the source code, which clearly shows /// doc comments on every public function; reclassified to PARTIAL. The NONE test ratings also contradicted the detail text which described existing tests for each function; reclassified to WEAK. The remaining legitimate concerns are: (1) weak test coverage across all functions — no cryptographic reference vectors for BLAKE2s hash, MAC, HMAC, or HKDF, which is a real gap for security-critical code; (2) panic via .expect() in the public mac() function; (3) non-idiomatic indexed loops in hmac_blake2s. These justify NEEDS_REFACTOR but not CRITICAL.
- `rustguard-crypto/src/tai64n.rs`: 2 symbol(s) — Overall: The file is a well-implemented, minimal TAI64N timestamp type for WireGuard. Two major reclassifications: (1) Tai64n DEAD → USED — this is a library crate and the pub struct is its intended export; cross-crate imports were outside analysis scope. Action 1 (remove dead code) is therefore invalid and removed. (2) Both symbols had documentation misclassified as UNDOCUMENTED despite having `///` doc comments visible in the source. Tai64n has 4 lines of struct-level docs (PARTIAL due to missing method docs on as_bytes/from_bytes), and TAI64_EPOCH_OFFSET has a clear explanatory doc comment (DOCUMENTED). Action 2 is removed because it incorrectly claims the struct is undocumented (it has struct-level docs) and uses JavaScript terminology ('JSDoc') for a Rust file. The legitimate remaining findings are: WEAK test coverage (roundtrip-only tests that don't validate actual byte encodings against known values) and PARTIAL documentation (two public methods missing docs). These warrant NEEDS_REFACTOR but not CRITICAL.
- `rustguard-daemon/src/peer.rs`: 1 symbol(s) — The critical reclassification is Peer's utility from DEAD to USED. A WireGuard daemon's core Peer struct is definitively not dead code; the analysis lacked cross-module visibility within the crate. Removing action 1 (delete dead code) as it would be destructive. Documentation was inconsistently labeled UNDOCUMENTED when the detail and source both show partial coverage (struct-level doc + 2/7 field docs). Action 2 (improve documentation) remains valid. Tests NONE is confirmed — no test coverage exists. The best_practices WARNs (missing Debug derive, redundant double-access in has_active_session, Result return for from_config, missing doc comments on public items) are all legitimate improvement suggestions. Verdict remains NEEDS_REFACTOR due to absent tests and incomplete documentation, but no CRITICAL issues exist.
- `rustguard-daemon/src/tunnel.rs`: 1 symbol(s) — Overall NEEDS_REFACTOR is the correct verdict. Two confirmed correctness bugs justify this: (1) the MSG_RESPONSE peer-matching heuristic fails in multi-peer configurations by selecting the wrong peer, and (2) the sigprocmask placement in ctrlc_handler allows signals to be delivered to worker threads, potentially preventing clean shutdown. The nonce-exhaustion bug in the timer thread (action 3) is a lower-severity but real issue. Action 6 removed because `run` is reclassified from DEAD to USED — it is the sole public entry point of the tunnel daemon module and is clearly meant to be called from main.rs. The 'JSDoc' terminology in documentation actions (4,5,7,9,10,11) is incorrect for Rust (should be 'rustdoc'), but the underlying recommendation to add documentation is valid so they are retained. Cross-crate duplication of rand_index and base64_key (actions 8,12) is real but the effort to deduplicate trivial 4-line functions across Rust crates is disproportionate — would require a shared utility crate.
- `rustguard-enroll/src/client.rs`: 2 symbol(s) — Overall: Two DEAD classifications on JoinConfig and run are clear false positives — both are public API symbols in a library crate where cross-workspace imports weren't captured by the analysis tool. This is a well-documented systematic limitation. Removing actions 4 and 6 (dead code removal suggestions) accordingly. The NEEDS_FIX findings on run (no shutdown mechanism, unreachable Ok(())) and add_route (missing cfg fallback) are valid and retained. Duplication of rand_index and base64_key within the same crate is genuine and easily fixable. The crate has zero test coverage which is the most significant gap. Documentation is missing on public API items. Verdict remains NEEDS_REFACTOR due to the correction issues and lack of tests, but is not CRITICAL since no ERROR-level findings exist.
- `rustguard-enroll/src/control.rs`: 1 symbol(s) — Overall assessment: The file is a well-structured, idiomatic Rust control socket module for a library crate. The key reclassification is `is_open` from DEAD to USED — it follows the same library-crate export pattern as `send_command`, `start_listener`, and `cleanup` (zero in-scope importers, but clearly part of the intended public API surface). Action 4 (remove dead code `is_open`) is consequently removed. The NEEDS_FIX on `open_window` is preserved — the u64→i64 cast and unchecked addition are genuine defensive-coding gaps, though low practical risk. The pervasive NONE tests finding across all symbols is accurate and the most significant gap. Documentation is consistently PARTIAL — one-liners exist for most public functions but lack parameter descriptions, `# Errors` sections, and examples. The best-practices findings (unwrap in production, Relaxed ordering, silent write error discards) are all valid concerns. Verdict remains NEEDS_REFACTOR due to the combination of zero test coverage, the open_window casting issue, and documentation gaps.
- `rustguard-enroll/src/fast_udp.rs`: 7 symbol(s) — Overall: The file is a well-designed platform-abstraction module for batched UDP I/O. The dominant issue was false-positive DEAD classifications across all pub symbols. The static analysis tool failed to account for (1) active intra-file usage of constants and types, and (2) the standard Rust pattern of cfg-gated platform-specific pub functions that are consumed via `mod` imports within the crate. All 7 'remove dead code' actions are invalid and removed. Documentation was systematically misclassified as UNDOCUMENTED when the detail text and source code show PARTIAL docs exist (except macOS send_packet which is truly undocumented). The legitimate concerns remain: missing SAFETY comments on 6 unsafe blocks (best practices FAIL), missing Debug/Default derives on RecvBatch (WARN), and zero test coverage for the module. Verdict stays NEEDS_REFACTOR based on the unsafe-without-SAFETY-comments finding and absent tests.
- `rustguard-enroll/src/packet.rs`: 2 symbol(s) — Major reclassification: both exported symbols (ParsedUdp and parse_eth_udp) were marked DEAD, but this is a library crate where public types and functions are consumed by downstream crates. The original evaluator's detail text itself acknowledged this Known False Positive pattern in both cases, yet still classified them DEAD. Reclassified both to USED with high confidence. Consequently, actions 2 and 4 ('Remove dead code') are invalid and removed. Documentation was marked UNDOCUMENTED for both exported symbols, but both have existing /// doc comments visible in the source — reclassified to PARTIAL. Action 5 removed since parse_eth_udp already has documentation. Tests for ParsedUdp and parse_eth_udp reclassified from NONE to WEAK since the test parse_ipv4_udp_frame does exercise these through the IPv4 path, as the evaluator's own detail text confirms. The IHL bounds check bug in parse_ipv4_udp (action 1) is a legitimate NEEDS_FIX — the logic is directly verifiable and the fix is straightforward. Action 3 retained despite misleading 'JSDoc' terminology since the intent (add field-level doc comments to ParsedUdp) is valid. Overall verdict remains NEEDS_REFACTOR due to the IHL validation bug and weak test coverage, but the codebase is otherwise well-structured with correct parsing logic.
- `rustguard-enroll/src/protocol.rs`: 3 symbol(s) — Overall assessment: The code is correct, lean, well-structured, and free of security issues. All corrections are OK, all symbols are USED and UNIQUE, and all are appropriately engineered. The two areas for improvement are (1) test coverage gaps — especially the asymmetry where the response path lacks the failure-mode tests that the request path has — and (2) documentation gaps on public exports (constants lacking `///`, public functions with minimal docs). Three private symbols (two magic constants, random_nonce) were reclassified from UNDOCUMENTED to DOCUMENTED under private-item leniency, as they have self-descriptive names and the evaluators themselves noted the leniency applies. No actions removed — all 8 target public symbols where the documentation findings genuinely apply. Verdict remains NEEDS_REFACTOR due to the WEAK test findings on 6 public functions and missing docs on 2 public constants.
- `rustguard-enroll/src/state.rs`: 4 symbol(s) — All four DEAD findings are false positives. This is a Rust library crate (rustguard-enroll) whose pub items constitute the public API for consumption by other workspace crates. The utility evaluator only checked for in-crate importers, which is the wrong scope for a library's public API. All UNDOCUMENTED findings are incorrect — every public item has /// doc comments as confirmed by the best_practices rule 9 ('All public items carry /// doc comments'). They are PARTIAL (missing field docs, #Errors sections, examples) but definitely not UNDOCUMENTED. Test findings for save and load were incorrectly NONE when both functions are explicitly called in inline tests; WEAK is appropriate given limited edge-case coverage. All 8 actions are removed: actions 1/3/5/7 target non-existent dead code, and actions 2/4/6/8 reference 'JSDoc' (wrong language — this is Rust) for items that already have doc comments. The code is well-written with a 9/10 best practices score, proper error handling, atomic writes, and no unsafe code. Verdict upgraded to CLEAN.
- `rustguard-kmod/src/lib.rs`: 3 symbol(s) — Overall assessment: NEEDS_REFACTOR is the correct verdict. Five confirmed NEEDS_FIX correction findings exist: (1) RustGuard Drop unconditionally calls wg_genl_exit without tracking whether init succeeded; (2) do_xmit has a 2KB stack buffer risk and unconditional stats counting; (3) handle_initiation sends responses to pre-configured endpoint instead of observed source, breaking NAT/roaming; (4) handle_response irreversibly consumes pending_handshake before validation; (5) handle_transport has same 2KB stack buffer risk. All are directly verifiable in source code. Three LOW_VALUE utility findings on FFI stub functions (rustguard_dev_uninit, rustguard_genl_get, rustguard_genl_set) are reclassified to USED — these are required callback stubs for C ABI compatibility and cannot be removed. Corresponding removal actions 13, 15, 17 are invalidated. Tests are NONE across the entire crate (no test file exists for rustguard-kmod), which is concerning for security-critical kernel code but somewhat expected given the kernel module testing constraints. Documentation findings (PARTIAL/UNDOCUMENTED) are confirmed but most are on private items where leniency applies.
- `rustguard-kmod/src/noise.rs`: 4 symbol(s) — Overall: NEEDS_REFACTOR is the correct verdict. Two genuine NEEDS_FIX findings remain: (1) hash() passes uncapped chunks.len() to the C function while only populating 8 array slots — a latent OOB read if called with >8 chunks, and (2) process_response omits zeroizing the PSK-derived AEAD key, leaving sensitive material on the stack. Both are low-medium severity since all current callers are safe. Four DEAD classifications on the main pub(crate) API functions (create_initiation, process_response, process_initiation, MSG_TRANSPORT) are false positives — these are the module's primary entry points necessarily imported by other crate modules. The 0-importer count reflects incomplete cross-module analysis, a known limitation for kernel module crates. Several documentation fields in the merged JSON contradicted their own detail text (UNDOCUMENTED at field level vs PARTIAL/DOCUMENTED in the detail string), corrected in deliberation. The duplication findings for constant_time_eq, hash, mac, and random_bytes with cookie.rs are genuine — extracting shared crypto helpers to a common module would be a valid refactor. The module-wide lack of tests (NONE across all functions) is the most significant systemic issue but is expected for a kernel module with C FFI dependencies. Removed 5 actions: 3 dead-code-removal actions for API functions that are not dead (22, 27, 29, 31), and 1 documentation action for an already-documented constant (23).
- `rustguard-kmod/src/replay.rs`: 3 symbol(s) — The critical carry-propagation bug in shift_window is confirmed through manual analysis and a concrete counterexample — the loop direction is reversed, causing anti-replay bypass. This keeps the verdict at CRITICAL. However, the DEAD utility finding on ReplayWindow is a false positive: pub(crate) structs in kernel module crates are internal API types, and this is the canonical anti-replay implementation for the WireGuard kmod. Action 2 (remove dead code) is removed accordingly. Documentation findings on private constants were over-penalized at low confidence — both are adequately self-documenting for private items. ReplayWindow documentation is PARTIAL (2/3 public methods documented, struct-level doc missing), not UNDOCUMENTED as originally classified.
- `rustguard-kmod/src/timers.rs`: 2 symbol(s) — Overall NEEDS_REFACTOR due to two confirmed medium-severity logic bugs in SessionTimers (is_dead inactivity vs session-age mismatch, needs_keepalive unreachable path for fresh sessions) and one low-severity off-by-one in REKEY_AFTER_MESSAGES. Reclassified SessionTimers from DEAD to USED — it is the module's primary pub(crate) API struct for a WireGuard kernel module, and the DEAD finding stems from incomplete cross-file analysis. Fixed two documentation false positives: both KEEPALIVE_TIMEOUT_NS and SessionTimers have clear `///` doc comments in the source but were incorrectly merged as UNDOCUMENTED. Removed action 5 (remove SessionTimers as dead) since it's reclassified to USED, and action 6 (add documentation to SessionTimers) since it's already documented. The one genuinely dead symbol is KEEPALIVE_TIMEOUT_NS (private, zero references); action 4 for its removal remains valid. The missing SAFETY comment on the unsafe FFI block and absence of tests for the impl block are legitimate hygiene concerns.
- `rustguard-tun/examples/tun_echo.rs`: 2 symbol(s) — Overall: NEEDS_REFACTOR driven by the valid NEEDS_FIX on main's hardcoded IP header offset of 20 bytes. Action 1 (compute IHL dynamically) is the only substantive improvement needed. Actions 3-6 removed: icmp_checksum and ip_checksum are not over-engineered or low-value—they are intentional semantic wrappers that improve readability in this pedagogical example file. Named wrappers in a teaching context communicate intent at call sites and are appropriate for a smoke-test example. Action 2 (documentation for main) is kept as a low-priority hygiene item, though the file-level comments already cover the purpose well. The NONE tests findings across all symbols are factual but expected—this is an example binary requiring root privileges and a TUN device, making automated testing impractical.
- `rustguard-tun/src/bpf_loader.rs`: 1 symbol(s) — Overall: NEEDS_REFACTOR is the correct verdict. Three confirmed NEEDS_FIX issues exist: (1) fd leak in XdpProgram::load_and_attach on error paths, (2) insufficient NLMSG_ERROR bounds check in attach_xdp_netlink (n>=16 should be n>=20), and (3) 4 bytes of uninitialized trailing padding in BpfAttrProgLoad. The XdpProgram DEAD finding is reclassified to USED — it is the sole public API of this module in a library crate, matching the known false-positive pattern for cross-crate exports invisible to single-crate analysis. Action 4 (remove dead code XdpProgram) is therefore removed. Zero test coverage across all functions is the biggest systemic issue; parse_and_patch_elf is especially notable as a pure function that could be easily unit-tested. The 10 private constants mirroring Linux kernel ABI values are UNDOCUMENTED but tolerated under private-item leniency — their names are the documentation. Best practices findings about unwrap() in parse_and_patch_elf and missing SAFETY comments on unsafe blocks are valid quality concerns captured in the best_practices section.
- `rustguard-tun/src/linux_mq.rs`: 1 symbol(s) — Overall assessment: Downgraded from CRITICAL to NEEDS_REFACTOR. The original CRITICAL verdict was driven by two findings that required correction: (1) Action 1 (unsafe-in-closure at L107) is a false positive — Rust 2021 and earlier editions propagate unsafe context into closures, so libc::close(fd0) inside the map_err closure is valid without an inner unsafe block. This action is removed. (2) Action 2 (fd leak at L116) is a genuine bug — when open_tun_queue fails mid-loop, previously accumulated fds leak. This alone warrants ERROR on MultiQueueTun's correction axis but does not rise to CRITICAL since it's a resource leak on an error path, not UB. Action 21 (remove dead code for MultiQueueTun) is removed — the struct is the primary public API of this library module, meant for external consumers. Being unexported internally does not make a public library type dead. The significant code duplication between linux_mq.rs and linux.rs (12 constants, 3 structs, 3 helpers, 1 large function) is the dominant refactoring concern. Extracting shared items into a common internal module would substantially reduce maintenance burden. The duplication actions (3-20, 23) remain valid as a consolidated refactoring opportunity.
- `rustguard-tun/src/linux.rs`: 19 symbol(s) — Overall assessment: NEEDS_REFACTOR is the correct verdict. The file has one real ABI issue (IfreqAddr missing 8-byte padding, action 1), meaningful duplication with linux_mq.rs (4 functions could be extracted to a shared module, actions 2-4, 6), and a public function (create) lacking Rustdoc (action 5). All actions are valid. The 11 private constants and 3 private structs were incorrectly flagged as UNDOCUMENTED — these are standard Linux kernel ABI names that are self-documenting by convention, and the evaluator's own detail acknowledged 'tolerated under private-item leniency.' Reclassifying these to DOCUMENTED significantly improves the docs_coverage picture. The NONE test findings are factually correct but expected for a privileged Linux FFI module. The IfreqAddr NEEDS_FIX is a genuine concern — while functionally harmless for the specific ioctls used, the kernel reads 8 bytes past the struct allocation via copy_from_user, which is an ABI contract violation that should be fixed with minimal effort. Actions 5, 7, 8 incorrectly reference 'JSDoc' when the file is Rust (should be 'Rustdoc' / '/// doc comments'), but the intent is correct so they are retained.
- `rustguard-tun/src/macos.rs`: 23 symbol(s) — Overall assessment: The file is a well-structured macOS utun platform implementation with real but non-critical bugs. Three confirmed NEEDS_FIX issues remain: (1) IfreqMtu struct is 20 bytes but kernel reads 32 — real UB from reading uninitialized stack memory; (2) configure_address closes socket before capturing errno, allowing close() to clobber the real error; (3) set_mtu has the same errno bug plus inherits the struct padding issue. The create() num+1 overflow is low-severity but technically correct. Major reclassifications: all three public functions (create, read, write) were incorrectly flagged as DEAD — they are the public API of a conditionally-compiled platform module and are clearly USED. close_and_error duplication with linux.rs is intentional cross-platform module isolation, not refactorable duplication. Most UNDOCUMENTED findings on private items were reclassified to DOCUMENTED under private-item leniency — these are well-known Darwin/BSD identifiers whose names serve as documentation. read and write had inconsistent UNDOCUMENTED classification despite having /// doc comments, corrected to PARTIAL. Verdict remains NEEDS_REFACTOR due to the errno-clobbering bugs and struct padding UB.
- `rustguard-tun/src/uring.rs`: 1 symbol(s) — Overall NEEDS_REFACTOR is confirmed. The file has two substantive correctness concerns: (1) slot_ptr's aliasing UB from casting *const to *mut through a shared reference — a real stacked-borrows violation that should be fixed, and (2) pending_reads arithmetic underflow risk that could cause catastrophic fill_reads over-submission in release builds. Both are medium-severity issues with small-effort fixes. The missing bounds validation in submit_write is a secondary concern. The four unsafe blocks without SAFETY comments is a best-practices violation for a library crate. Tests are absent but the platform-specific nature of io_uring makes this somewhat expected — unit tests for BufferPool alloc/free logic would still be valuable. Actions 6 and 7 incorrectly say 'JSDoc' instead of 'rustdoc' but their intent is valid, so they are retained. Action 4's claim about 'silent memory corruption' in release is factually incorrect for safe Rust array indexing (it always panics), but the action's recommendation to add a bounds check is still good practice for a public API, so it is retained at its current low severity.

---

## Methodology

Each file is evaluated through 7 independent axis evaluators running in parallel.
Every symbol (function, class, variable, type) is analysed individually and receives a rating per axis along with a confidence score (0–100).
Findings with confidence < 30 are discarded; those with confidence < 60 are excluded from verdict computation.

| Axis | Model | Ratings | Description |
|------|-------|---------|-------------|
| Utility | haiku | USED / DEAD / LOW_VALUE | Is this symbol actually used in the codebase? |
| Duplication | haiku | UNIQUE / DUPLICATE | Is this symbol a copy of logic that exists elsewhere? |
| Correction | sonnet | OK / NEEDS_FIX / ERROR | Does this symbol contain bugs or correctness issues? |
| Overengineering | haiku | LEAN / OVER / ACCEPTABLE | Is the implementation unnecessarily complex? |
| Tests | haiku | GOOD / WEAK / NONE | Does this symbol have adequate test coverage? |
| Best Practices | sonnet | Score 0–10 (17 rules) | Does the file follow TypeScript best practices? |
| Documentation | haiku | DOCUMENTED / PARTIAL / UNDOCUMENTED | Are exported symbols properly documented with JSDoc? |

See each axis folder for detailed rating criteria and methodology.

### Severity Classification

- **High**: ERROR corrections, or NEEDS_FIX / DEAD / DUPLICATE with confidence >= 80%.
- **Medium**: NEEDS_FIX / DEAD / DUPLICATE with confidence < 80%, or OVER (any confidence).
- **Low**: LOW_VALUE utility or remaining minor findings.

### Verdict Rules

- **CLEAN**: No actionable findings with confidence >= 60%.
- **NEEDS_REFACTOR**: At least one confirmed finding (DEAD, DUPLICATE, OVER, or NEEDS_FIX) with confidence >= 60%.
- **CRITICAL**: At least one ERROR correction found.

### Inter-axis Coherence

After individual evaluation, coherence rules reconcile contradictions:

- If utility = DEAD, tests is forced to NONE (no point testing dead code).
- If utility = DEAD, documentation is forced to UNDOCUMENTED (no point documenting dead code).
- If correction = ERROR, overengineering is forced to ACCEPTABLE (complexity is secondary to correctness).

*Generated: 2026-03-25T14:30:01.970Z*
