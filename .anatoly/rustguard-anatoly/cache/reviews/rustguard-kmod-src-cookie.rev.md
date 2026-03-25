# Review: `rustguard-kmod/src/cookie.rs`

**Verdict:** NEEDS_REFACTOR

## Symbols

| Symbol | Kind | Exported | Correction | Over-eng. | Utility | Duplication | Tests | Confidence |
|--------|------|----------|------------|-----------|---------|-------------|-------|------------|
| COOKIE_SECRET_LIFETIME_NS | constant | no | OK | LEAN | USED | UNIQUE | GOOD | 60% |
| COOKIE_LEN | constant | no | OK | LEAN | USED | UNIQUE | GOOD | 60% |
| LABEL_COOKIE | constant | no | OK | LEAN | USED | UNIQUE | GOOD | 60% |
| LABEL_MAC1 | constant | no | OK | LEAN | USED | UNIQUE | GOOD | 60% |
| hash | function | no | NEEDS_FIX | ACCEPTABLE | USED | DUPLICATE | NONE | 75% |
| mac | function | no | OK | LEAN | USED | DUPLICATE | NONE | 75% |
| random_bytes | function | no | OK | LEAN | USED | DUPLICATE | NONE | 75% |
| CookieChecker | class | yes | OK | LEAN | USED | UNIQUE | NONE | 80% |
| CookieState | class | yes | OK | LEAN | USED | UNIQUE | NONE | 80% |
| constant_time_eq | function | no | OK | LEAN | USED | DUPLICATE | NONE | 80% |

### Details

#### `COOKIE_SECRET_LIFETIME_NS` (L14–L14)

- **Utility [USED]**: Constant used in is_fresh (L107) and maybe_rotate_secret (L91) for timing comparisons on secret rotation.
- **Duplication [UNIQUE]**: No similar constants found in RAG results. Protocol-specific timeout constant (120 seconds in nanoseconds).
- **Correction [OK]**: 120 seconds expressed in nanoseconds matches the WireGuard RFC specification for cookie lifetime.
- **Overengineering [LEAN]**: Simple named constant encoding the 120-second lifetime in nanoseconds. Inline arithmetic (120 * 1_000_000_000) is clear and self-documenting — no abstraction needed.
- **Tests [GOOD]**: Pure compile-time constant with no runtime behavior. Per rule 6, constants require no dedicated tests; value is indirectly exercised through any secret-rotation logic.
- **UNDOCUMENTED [UNDOCUMENTED]**: Private constant with no /// doc comment. Name is moderately self-descriptive (lifetime in nanoseconds = 120 s). Tolerated under leniency rules for private items. (deliberated: confirmed — UNDOCUMENTED is technically correct — no individual doc comment exists. However, the module-level doc block (L3-L12) explains the cookie mechanism, and the constant name encodes its semantics (cookie secret, lifetime, nanoseconds). Low priority under private-item leniency.)

#### `COOKIE_LEN` (L15–L15)

- **Utility [USED]**: Constant used in make_cookie array initialization (L89), encrypt call (L113), and compute_mac2 (L177-178).
- **Duplication [UNIQUE]**: No similar constants found. Cookie size constant specific to the mechanism (16 bytes).
- **Correction [OK]**: 16-byte cookie size is correct per WireGuard specification.
- **Overengineering [LEAN]**: Minimal constant for the 16-byte cookie length, used throughout the module. Appropriate named constant with no unnecessary complexity.
- **Tests [GOOD]**: Pure compile-time constant with no runtime behavior. Per rule 6, no dedicated tests needed; its value is implicitly validated by any cookie roundtrip.
- **UNDOCUMENTED [UNDOCUMENTED]**: Private constant with no /// doc comment. Name is self-descriptive. Tolerated under leniency rules for private items. (deliberated: confirmed — UNDOCUMENTED is technically correct — no doc comment. Name is fully self-descriptive (cookie length = 16). Module-level docs provide protocol context. Tolerated under private-item leniency.)

#### `LABEL_COOKIE` (L16–L16)

- **Utility [USED]**: Label used in hash() calls for cookie key derivation in create_reply (L131) and process_reply (L158).
- **Duplication [UNIQUE]**: No RAG results for this constant. Protocol label constants are not typically consolidated across modules.
- **Correction [OK]**: b"cookie--" is the correct 8-byte label per the WireGuard protocol specification.
- **Overengineering [LEAN]**: Direct byte-string label constant matching the WireGuard spec. No abstraction warranted for a static protocol label.
- **Tests [GOOD]**: Pure compile-time byte-string constant with no runtime behavior. Per rule 6, no dedicated test required.
- **UNDOCUMENTED [UNDOCUMENTED]**: Private constant with no /// doc comment. Value b"cookie--" is a WireGuard protocol domain label. Tolerated under leniency rules for private items. (deliberated: confirmed — UNDOCUMENTED is technically correct. The value b"cookie--" is a WireGuard domain-separation label visible in the literal. Private constant with clear naming; tolerated under leniency.)

#### `LABEL_MAC1` (L17–L17)

- **Utility [USED]**: Label used in hash() call for MAC1 key derivation in verify_mac1 (L119).
- **Duplication [UNIQUE]**: Similar protocol label exists in noise.rs. However, these WireGuard protocol labels are independently used in different modules.
- **Correction [OK]**: b"mac1----" is the correct 8-byte label per the WireGuard protocol specification.
- **Overengineering [LEAN]**: Direct byte-string label constant matching the WireGuard spec. Same reasoning as LABEL_COOKIE — minimal and correct.
- **Tests [GOOD]**: Pure compile-time byte-string constant with no runtime behavior. Per rule 6, no dedicated test required.
- **UNDOCUMENTED [UNDOCUMENTED]**: Private constant with no /// doc comment. Value b"mac1----" is a WireGuard protocol domain label. Tolerated under leniency rules for private items. (deliberated: confirmed — UNDOCUMENTED is technically correct. Same reasoning as LABEL_COOKIE — WireGuard protocol label b"mac1----" with self-descriptive name.)

#### `hash` (L41–L51)

- **Utility [USED]**: Helper function called in verify_mac1 (L119), create_reply (L131), and process_reply (L158) for key derivation.
- **Duplication [DUPLICATE]**: RAG score 0.966 with near-identical implementation in noise.rs. Both hash variable-length chunk arrays via wg_blake2s_hash. Only difference is array capacity (4 vs 8).
- **Correction [NEEDS_FIX]**: ptrs and lens arrays are fixed at size 4 and only populated for up to 4 chunks via .take(4), but chunks.len() is passed as num_chunks to the C function unconditionally. If chunks.len() > 4, the C function will read beyond the 4-element stack arrays, causing undefined behaviour (out-of-bounds memory read). The argument should be chunks.len().min(4) as u32. Currently only called with 2 chunks, so the bug is latent but the interface is incorrect.
- **Overengineering [ACCEPTABLE]**: Handles up to 4 chunks via fixed-size arrays to interface with the C FFI. The cap at 4 chunks is a reasonable constraint given the call sites. Slightly awkward (silent truncation beyond 4 chunks) but justified by the no-alloc kernel-module context where dynamic dispatch is unavailable.
- **Tests [NONE]**: No test file exists for rustguard-kmod/src/cookie.rs. There is no rustguard-kmod/tests/ directory in the project structure. This private helper has zero test coverage — no determinism checks, no multi-chunk behavior, no known-vector validation.
- **UNDOCUMENTED [UNDOCUMENTED]**: Private function with no /// doc comment. Wraps wg_blake2s_hash for multi-chunk hashing; chunk-count limit of 4 and 32-byte output are implicit. The known false positive for hash in rustguard-crypto does not apply here — that instance had a doc comment at its file's line 8, which this instance lacks. Tolerated under private-item leniency. (deliberated: confirmed — NEEDS_FIX confirmed: if chunks.len() > 4, the C function reads beyond the 4-element ptrs/lens arrays — genuine UB. Currently latent (only called with 2 chunks) but the interface is unsafe-by-contract without documenting the limit. Fix to chunks.len().min(4) is trivial and correct. DUPLICATE confirmed: 96% identical to noise.rs::hash within the same crate; extracting to a shared crypto_helpers module is feasible and warranted. NONE tests is factual — no test infrastructure exists for rustguard-kmod. ACCEPTABLE overengineering agreed — fixed-size array is a pragmatic no-alloc kernel constraint. UNDOCUMENTED agreed — private helper with no doc comment. Raising confidence to 75 as all findings are well-supported.)

> **Duplicate of** `rustguard-kmod/src/noise.rs:hash` — 96% identical logic—both hash arbitrary chunks to [u8; 32] using wg_blake2s_hash, differ only in max chunk array size

#### `mac` (L53–L57)

- **Utility [USED]**: BLAKE2s MAC function called in make_cookie (L89), verify_mac1 (L120), verify_mac2 (L126), compute_mac2 (L181).
- **Duplication [DUPLICATE]**: RAG score 0.983 with identical function in noise.rs. Both perform BLAKE2s-256 MAC with same signature and wg_blake2s_256_mac call.
- **Correction [OK]**: Correctly delegates to wg_blake2s_256_mac with proper pointer and length arguments. Output buffer is correctly sized at 32 bytes.
- **Overengineering [LEAN]**: Thin, direct wrapper over the C BLAKE2s-MAC FFI. Does exactly one thing with no unnecessary indirection.
- **Tests [NONE]**: No test file exists for rustguard-kmod/src/cookie.rs. The private keyed-MAC wrapper is entirely untested — no key-sensitivity checks, no known-vector validation, no empty-data edge case.
- **UNDOCUMENTED [UNDOCUMENTED]**: Private function with no /// doc comment. Wraps wg_blake2s_256_mac; key/data semantics and 32-byte output are undescribed. The known false positive for mac in rustguard-crypto does not apply here — that instance had a multi-line /// block, which is absent in this file. Tolerated under private-item leniency. (deliberated: confirmed — DUPLICATE confirmed: 100% identical to noise.rs::mac within the same crate. Both are trivial FFI wrappers and consolidation into a shared module is straightforward. NONE tests is factual. UNDOCUMENTED tolerated under private-item leniency. Raising confidence as the duplication is unambiguous.)

> **Duplicate of** `rustguard-kmod/src/noise.rs:mac` — 100% identical—same signature and wg_blake2s_256_mac FFI call

#### `random_bytes` (L59–L63)

- **Utility [USED]**: RNG function called in new (L82), maybe_rotate_secret (L87), and create_reply (L132) for nonce/secret generation.
- **Duplication [DUPLICATE]**: RAG score 0.971 with identical generic function in noise.rs. Both generate random [u8; N] arrays via wg_get_random_bytes.
- **Correction [OK]**: Correctly fills a stack buffer of N bytes via wg_get_random_bytes and returns it. N as u32 cast is safe for any reasonable const generic value.
- **Overengineering [LEAN]**: Const-generic wrapper over the kernel RNG FFI. The generic N avoids code duplication for different buffer sizes and is idiomatic Rust for no-std/no-alloc contexts.
- **Tests [NONE]**: No test file found for this module. The generic const-sized random buffer wrapper is completely untested; no output-non-zero or uniqueness smoke tests exist.
- **UNDOCUMENTED [UNDOCUMENTED]**: Private generic function with no /// doc comment. Thin wrapper around wg_get_random_bytes using const-generic N. Tolerated under private-item leniency. (deliberated: confirmed — DUPLICATE confirmed: 99% identical to noise.rs::random_bytes, same const-generic signature and FFI call. Same-crate deduplication is practical. NONE tests and UNDOCUMENTED are factual and tolerated per kernel module and private-item leniency respectively.)

> **Duplicate of** `rustguard-kmod/src/noise.rs:random_bytes` — 99% identical—same generic signature and wg_get_random_bytes FFI call

#### `CookieChecker` (L66–L71)

- **Utility [USED]**: Server-side public API struct for cookie generation and MAC2 validation. Known false positive: workspace crate public types consumed across modules.
- **Duplication [UNIQUE]**: Struct definition for server-side cookie validation state. No similar struct found. Contains public key, secret, and protocol state.
- **Correction [OK]**: Struct fields correctly capture public key, rotating secret, secret generation timestamp, and load flag. No correctness issues in the struct definition itself.
- **Overengineering [LEAN]**: Holds exactly the state required by the WireGuard cookie spec (public key, rolling secret, timestamp, load flag). No extraneous fields or premature generalization.
- **Tests [NONE]**: No test file exists under rustguard-kmod. The server-side pub(crate) struct and all its methods (new, verify_mac1, verify_mac2, create_reply, maybe_rotate_secret, make_cookie) have zero test coverage. Critical security logic (MAC verification, secret rotation timing, DoS-protection reply creation) is entirely untested.
- **PARTIAL [PARTIAL]**: pub(crate) struct with a /// summary at L65 ('Server-side: generates cookies and validates MAC2.'). No per-field /// doc comments visible on any of the four fields. Methods carry per-method docs but lack # Examples sections. Constructor new() has no doc comment. Previously reclassified to PARTIAL by deliberation. (deliberated: confirmed — NONE tests is factual and concerning — this is security-critical MAC verification and cookie generation logic with zero coverage. No test infrastructure exists for rustguard-kmod. PARTIAL documentation is correct: struct has a summary doc comment at L65 but fields and constructor lack documentation. For a security module, documenting the constructor's expected inputs (32-byte Curve25519 public key) would be valuable.)

#### `CookieState` (L74–L77)

- **Utility [USED]**: Client-side public API struct for storing decrypted cookies. Known false positive: workspace crate public types consumed across modules.
- **Duplication [UNIQUE]**: Struct definition for client-side cookie storage. No similar struct found. Distinct purpose from CookieChecker.
- **Correction [OK]**: Struct fields correctly model optional stored cookie and receipt timestamp. Using Option<[u8; COOKIE_LEN]> is correct and avoids implicit zero-value ambiguity.
- **Overengineering [LEAN]**: Minimal client-side cookie state: an optional cookie value and a receipt timestamp. Perfectly sized for its single responsibility.
- **Tests [NONE]**: No test file exists under rustguard-kmod. The client-side pub(crate) struct and all its methods (new, process_reply, compute_mac2, is_fresh) have zero test coverage. Key behaviors — decrypt-and-store roundtrip, freshness expiry, zero-return when no cookie — are completely untested.
- **PARTIAL [PARTIAL]**: pub(crate) struct with a /// summary at L73 ('Client-side: stores a received cookie for MAC2.'). Neither field (cookie, received) carries a /// doc comment. Methods have per-method docs but lack # Examples sections. Previously reclassified to PARTIAL by deliberation. (deliberated: confirmed — NONE tests is factual — client-side cookie decrypt-and-store, freshness expiry, and zero-return-when-no-cookie behaviors are untested. PARTIAL documentation is correct: struct-level summary exists at L73 but fields and constructor are undocumented.)

#### `constant_time_eq` (L202–L205)

- **Utility [USED]**: Constant-time equality check called in verify_mac1 (L122) and verify_mac2 (L128) for constant-time MAC verification.
- **Duplication [DUPLICATE]**: RAG score 0.982 with identical function in noise.rs. Both perform constant-time comparison using wg_crypto_memneq with identical logic.
- **Correction [OK]**: Kernel wg_crypto_memneq returns 0 when the two buffers are equal and non-zero otherwise (consistent with Linux crypto_memneq semantics). The == 0 check is correct. Length guard prevents differing-length slices from being considered equal.
- **Overengineering [LEAN]**: Security-critical wrapper over kernel crypto_memneq, correctly commented and justified. Cannot be replaced by a simple == due to compiler optimization risks. The length check before the unsafe call is correct and necessary.
- **Tests [NONE]**: No test file found for rustguard-kmod. This security-critical constant-time comparison function (used in MAC verification) has no tests: no equal-slices pass, no unequal-slices fail, no length-mismatch short-circuit, and no validation that the kernel memneq wrapper correctly returns 0 on equality.
- **PARTIAL [PARTIAL]**: Private function with a /// doc comment explaining the security rationale (kernel crypto_memneq, cannot be optimized away). Missing parameter descriptions and explicit return semantics, but as a private 4-line helper with a meaningful security comment this is above UNDOCUMENTED. No # Examples section. (deliberated: confirmed — DUPLICATE confirmed: 100% identical to noise.rs::constant_time_eq. Security-critical function that should ideally live in one place to ensure consistent behavior. NONE tests is factual and notable — this is a security primitive used in MAC verification with no coverage. PARTIAL documentation is correct: has a meaningful doc comment explaining the security rationale (kernel crypto_memneq, cannot be optimized away) but lacks parameter/return descriptions. Raising confidence slightly as all findings are well-evidenced.)

> **Duplicate of** `rustguard-kmod/src/noise.rs:constant_time_eq` — 100% identical—length check followed by wg_crypto_memneq for constant-time equality

## Best Practices — 7/10

### Rules

| # | Rule | Status | Severity | Detail |
|---|------|--------|----------|--------|
| 2 | No unsafe blocks without clear justification comment | WARN | CRITICAL | Nine unsafe blocks call extern C FFI functions. Only `constant_time_eq` has an inline justification comment (the M7 note). All others — `hash`, `mac`, `random_bytes`, `maybe_rotate_secret`, `new`, `create_reply`, `process_reply`, `is_fresh` — lack any SAFETY comment explaining which invariants are upheld (pointer validity, alignment, lifetime of borrows). The extern declarations make FFI obvious, but individual call-sites must still document their safety contracts. [L55-L57,L62-L64,L69-L71,L82-L84,L88-L90,L131-L138,L161-L168,L184-L186,L201-L203] |
| 3 | Proper error handling with Result/Option | FAIL | HIGH | In `create_reply` the return value of `wg_xchacha20poly1305_encrypt` (an `i32` indicating success/failure) is silently discarded via a trailing semicolon. A failed encryption leaves `encrypted_cookie` zeroed yet the reply buffer is still returned and used as if encryption succeeded. The decrypt counterpart in `process_reply` correctly checks its return value. [L125-L138] |
| 4 | Derive common traits on public types | WARN | MEDIUM | `CookieChecker` and `CookieState` are both `pub(crate)` exported types but carry no `#[derive]` attributes. At minimum `Debug` should be derived for both to aid in logging and test assertions. `Clone` and `PartialEq` may be intentionally omitted for security reasons (cookie values should not be freely copyable), but `Debug` has no such objection. [L72-L77,L79-L82] |
| 9 | Documentation comments on public items | WARN | MEDIUM | Both structs have `///` doc comments, and most `pub(crate)` methods are documented. However, `CookieChecker::new` and `CookieState::new` are undocumented constructors. For a security-sensitive module these are important entry points and should explain their expected inputs (e.g., that `our_public` is a 32-byte Curve25519 public key). [L84-L91,L149-L151] |

### Suggestions

- Check the return value of wg_xchacha20poly1305_encrypt in create_reply and propagate failure instead of silently returning a zeroed reply.
  ```typescript
  // Before
  unsafe {
      wg_xchacha20poly1305_encrypt(
          key.as_ptr(), nonce.as_ptr(),
          cookie.as_ptr(), COOKIE_LEN as u32,
          mac1.as_ptr(), 16,
          encrypted_cookie.as_mut_ptr(),
      );
  }
  // After
  // SAFETY: all pointers derive from valid Rust references with lifetimes
  // that encompass this call; buffer sizes match the API contract.
  let ret = unsafe {
      wg_xchacha20poly1305_encrypt(
          key.as_ptr(), nonce.as_ptr(),
          cookie.as_ptr(), COOKIE_LEN as u32,
          mac1.as_ptr(), 16,
          encrypted_cookie.as_mut_ptr(),
      )
  };
  if ret != 0 { return None; }  // change return type to Option<[u8; 64]>
  ```
- Add SAFETY comments to each unsafe block documenting pointer validity invariants.
  ```typescript
  // Before
  unsafe { wg_blake2s_hash(ptrs.as_ptr(), lens.as_ptr(), chunks.len() as u32, out.as_mut_ptr()) };
  // After
  // SAFETY: ptrs[i] point into `chunks` slice data which is alive for this call;
  // lens[i] match the corresponding slice lengths; out is a valid 32-byte stack buffer.
  unsafe { wg_blake2s_hash(ptrs.as_ptr(), lens.as_ptr(), chunks.len() as u32, out.as_mut_ptr()) };
  ```
- Derive Debug on public-crate types to aid in diagnostics and testing.
  ```typescript
  // Before
  pub(crate) struct CookieChecker {
      our_public: [u8; 32],
      secret: [u8; 32],
      secret_generated: u64,
      pub(crate) under_load: bool,
  }
  // After
  #[derive(Debug)]
  pub(crate) struct CookieChecker {
      our_public: [u8; 32],
      secret: [u8; 32],
      secret_generated: u64,
      pub(crate) under_load: bool,
  }
  ```
- Document the new() constructors to explain the expected inputs and initialization semantics.
  ```typescript
  // Before
  pub(crate) fn new(our_public: [u8; 32]) -> Self {
  // After
  /// Creates a new `CookieChecker` bound to the given 32-byte Curve25519 public key.
  /// Immediately generates a fresh random cookie secret valid for `COOKIE_SECRET_LIFETIME_NS`.
  pub(crate) fn new(our_public: [u8; 32]) -> Self {
  ```

## Actions

### Quick Wins

- **[correction · medium · small]** In hash(), change chunks.len() as u32 to chunks.len().min(4) as u32 when passing num_chunks to wg_blake2s_hash. The ptrs/lens arrays have a fixed capacity of 4; passing a larger value causes the C function to read beyond the stack-allocated arrays, which is undefined behaviour. [L49]

### Refactors

- **[duplication · medium · small]** Deduplicate: `hash` duplicates `hash` in `rustguard-kmod/src/noise.rs` (`hash`) [L41-L51]
- **[duplication · medium · small]** Deduplicate: `mac` duplicates `mac` in `rustguard-kmod/src/noise.rs` (`mac`) [L53-L57]
- **[duplication · medium · small]** Deduplicate: `random_bytes` duplicates `random_bytes` in `rustguard-kmod/src/noise.rs` (`random_bytes`) [L59-L63]
- **[duplication · medium · small]** Deduplicate: `constant_time_eq` duplicates `constant_time_eq` in `rustguard-kmod/src/noise.rs` (`constant_time_eq`) [L202-L205]

### Hygiene

- **[documentation · low · trivial]** Complete JSDoc documentation for: `CookieChecker` (`CookieChecker`) [L66-L71]
- **[documentation · low · trivial]** Complete JSDoc documentation for: `CookieState` (`CookieState`) [L74-L77]
- **[documentation · low · trivial]** Complete JSDoc documentation for: `constant_time_eq` (`constant_time_eq`) [L202-L205]
