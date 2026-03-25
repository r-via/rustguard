# Review: `rustguard-core/src/cookie.rs`

**Verdict:** NEEDS_REFACTOR

## Symbols

| Symbol | Kind | Exported | Correction | Over-eng. | Utility | Duplication | Tests | Confidence |
|--------|------|----------|------------|-----------|---------|-------------|-------|------------|
| COOKIE_SECRET_LIFETIME | constant | no | OK | LEAN | USED | UNIQUE | GOOD | 82% |
| COOKIE_LEN | constant | yes | OK | LEAN | USED | UNIQUE | GOOD | 90% |
| now | function | no | OK | LEAN | USED | UNIQUE | GOOD | 85% |
| elapsed_since | function | no | OK | LEAN | USED | UNIQUE | NONE | 88% |
| elapsed_since | function | no | OK | LEAN | USED | UNIQUE | NONE | 88% |
| CookieChecker | class | yes | OK | LEAN | USED | UNIQUE | WEAK | 90% |
| CookieState | class | yes | NEEDS_FIX | LEAN | USED | UNIQUE | WEAK | 85% |
| encode_addr | function | no | OK | LEAN | USED | UNIQUE | WEAK | 86% |
| random_bytes | function | no | OK | LEAN | USED | UNIQUE | WEAK | 75% |
| random_nonce | function | no | OK | LEAN | USED | UNIQUE | WEAK | 88% |
| fill_random | function | no | NEEDS_FIX | LEAN | USED | UNIQUE | NONE | 80% |
| fill_random | function | no | NEEDS_FIX | LEAN | USED | UNIQUE | NONE | 80% |
| constant_time_eq_16 | function | no | NEEDS_FIX | LEAN | USED | UNIQUE | WEAK | 70% |

### Details

#### `COOKIE_SECRET_LIFETIME` (L21–L21)

- **Utility [USED]**: Non-exported constant used locally in maybe_rotate_secret (line 90) and is_fresh (line 212)
- **Duplication [UNIQUE]**: Simple constant declaration, no duplicates found in codebase
- **Correction [OK]**: Duration::from_secs(120) correctly represents the 2-minute secret rotation window stated in the module doc.
- **Overengineering [LEAN]**: Simple named constant encoding the WireGuard spec's 2-minute secret rotation period. No abstraction beyond what the spec requires.
- **Tests [GOOD]**: Pure constant with no runtime behavior. The 120-second value is indirectly exercised by the freshness and rotation logic in the inline test suite. Constants require no direct tests.
- **PARTIAL [PARTIAL]**: Has a brief `///` comment ('How often the cookie secret rotates.') but omits the significance of the 120-second value, its relationship to the WireGuard spec, or what happens on rotation. (deliberated: confirmed — Documentation PARTIAL is fair — the `///` comment exists but doesn't reference the WireGuard spec's 2-minute rotation period. Minor doc improvement only.)

#### `COOKIE_LEN` (L24–L24)

- **Utility [DEAD]**: Exported const with 0 external importers per pre-computed analysis. Rule 2: exported symbol with no importers = DEAD
- **Duplication [UNIQUE]**: Exported constant for cookie byte length, no similar symbols found
- **Correction [OK]**: 16-byte cookie size matches the WireGuard specification.
- **Overengineering [LEAN]**: Minimal constant for the 16-byte cookie size. Appropriately exported for use by message layout code.
- **Tests [GOOD]**: Pure constant. Used throughout and indirectly validated in every roundtrip test that checks cookie length correctness (e.g., cookie_roundtrip, no_cookie_gives_zero_mac2).
- **PARTIAL [PARTIAL]**: Comment 'Cookie size.' is trivially unhelpful — it restates the name without explaining what 16 bytes represents in the WireGuard cookie protocol or how it is used. (deliberated: reclassified: utility: DEAD → USED, tests: NONE → GOOD, documentation: UNDOCUMENTED → PARTIAL — Utility DEAD reclassified → USED: this is a `pub const` in a library crate (`rustguard-core`). Downstream crates (e.g., message layout code) depend on it; pre-computed analysis likely doesn't capture cross-crate workspace imports. The entire cookie module is clearly used infrastructure for a WireGuard implementation. Tests NONE contradicts the detail text which says '[GOOD] Pure constant. Used throughout and indirectly validated in every roundtrip test.' For a pure constant with no runtime behavior, indirect validation is sufficient — reclassified to GOOD. Documentation UNDOCUMENTED is wrong — line 23 has `/// Cookie size.` which IS a doc comment, making this PARTIAL (brief but present).)

#### `now` (L33–L35)

- **Utility [USED]**: Non-exported std function used locally on lines 76 (new), 97 (maybe_rotate_secret), and 178 (process_reply)
- **Duplication [UNIQUE]**: Trivial 2-line function returning current time, conditional std feature
- **Correction [OK]**: Correctly returns std::time::Instant::now() for the std platform; no correctness issues.
- **Overengineering [LEAN]**: Thin cfg-gated wrapper that centralizes the std/no_std timestamp abstraction in one place rather than scattering #[cfg] attributes throughout impl blocks. Minimal and justified.
- **Tests [WEAK]**: Trivial std wrapper around Instant::now(). Called implicitly via CookieChecker::new() in setup(). Never tested directly, and no test verifies time-related monotonicity or behavior at boundaries.
- **UNDOCUMENTED [UNDOCUMENTED]**: Private function with no `///` doc comment. Name is clear, so leniency applies, but no documentation exists. (deliberated: reclassified: tests: WEAK → GOOD — Tests WEAK reclassified → GOOD: a 2-line wrapper around `Instant::now()` has nothing meaningful to test directly. It is called in every test via `CookieChecker::new()`. Requiring dedicated tests for this trivial wrapper is unreasonable. Documentation UNDOCUMENTED kept — no doc comment exists, though the function is private and self-descriptive.)

#### `elapsed_since` (L38–L40)

- **Utility [USED]**: Non-exported no_std stub version; called on lines 89 and 212 when std feature disabled
- **Duplication [UNIQUE]**: Trivial no_std stub returning zero duration; different semantic contract than std version
- **Correction [NEEDS_FIX]**: Stub always returns Duration::ZERO. Consequently, the condition in maybe_rotate_secret (ZERO >= 120 s) is permanently false, so the cookie secret provided to new_with never rotates regardless of real elapsed time. While documented as a no_std stub, this silently disables a security-critical rotation for any no_std caller expecting periodic rotation.
- **Overengineering [LEAN]**: No_std stub returning Duration::ZERO is the minimal correct behaviour; the comment correctly states kernel builds use their own cookie checker. No excess here.
- **Tests [NONE]**: The no_std stub always returns Duration::ZERO. There are no tests compiled under the no_std configuration and no test exercises this variant or verifies its always-zero behavior has expected downstream effects (e.g., that rotation never triggers).
- **UNDOCUMENTED [UNDOCUMENTED]**: Private no_std stub with no `///` doc comment. The always-ZERO return behavior under no_std is not documented at all, which is a notable omission. (deliberated: reclassified: correction: NEEDS_FIX → OK — Line 43-45 is the #[cfg(not(feature = "std"))] variant returning Duration::ZERO. The NEEDS_FIX flags this as silently disabling rotation, but the CookieChecker correction evaluator explicitly states 'the code comments acknowledge the kernel module uses an independent mechanism, making this an intentional design limitation rather than an unintended bug.' The LEAN evaluator concurs: 'No_std stub returning Duration::ZERO is the minimal correct behaviour.' This is a consistent, documented intentional design — the kernel module uses its own cookie checker. Reclassified to OK. Tests NONE kept — no no_std tests exist. Documentation UNDOCUMENTED kept.)

#### `elapsed_since` (L43–L45)

- **Utility [USED]**: Non-exported no_std stub version; called on lines 89 and 212 when std feature disabled
- **Duplication [UNIQUE]**: Trivial no_std stub returning zero duration; different semantic contract than std version
- **Correction [NEEDS_FIX]**: Stub always returns Duration::ZERO. Consequently, the condition in maybe_rotate_secret (ZERO >= 120 s) is permanently false, so the cookie secret provided to new_with never rotates regardless of real elapsed time. While documented as a no_std stub, this silently disables a security-critical rotation for any no_std caller expecting periodic rotation.
- **Overengineering [LEAN]**: No_std stub returning Duration::ZERO is the minimal correct behaviour; the comment correctly states kernel builds use their own cookie checker. No excess here.
- **Tests [NONE]**: The no_std stub always returns Duration::ZERO. There are no tests compiled under the no_std configuration and no test exercises this variant or verifies its always-zero behavior has expected downstream effects (e.g., that rotation never triggers).
- **UNDOCUMENTED [UNDOCUMENTED]**: Private no_std stub with no `///` doc comment. The always-ZERO return behavior under no_std is not documented at all, which is a notable omission. (deliberated: reclassified: correction: NEEDS_FIX → OK — Line 43-45 is the #[cfg(not(feature = "std"))] variant returning Duration::ZERO. The NEEDS_FIX flags this as silently disabling rotation, but the CookieChecker correction evaluator explicitly states 'the code comments acknowledge the kernel module uses an independent mechanism, making this an intentional design limitation rather than an unintended bug.' The LEAN evaluator concurs: 'No_std stub returning Duration::ZERO is the minimal correct behaviour.' This is a consistent, documented intentional design — the kernel module uses its own cookie checker. Reclassified to OK. Tests NONE kept — no no_std tests exist. Documentation UNDOCUMENTED kept.)

#### `CookieChecker` (L48–L57)

- **Utility [DEAD]**: Exported struct with 0 external importers per pre-computed analysis. Rule 2: exported symbol with no importers = DEAD
- **Duplication [UNIQUE]**: Struct definition for server-side cookie state management, no duplicates found
- **Correction [OK]**: Struct and impl are correct on std. On no_std the secret never rotates because elapsed_since always returns ZERO, but the code comments acknowledge the kernel module uses an independent mechanism, making this an intentional design limitation rather than an unintended bug.
- **Overengineering [LEAN]**: Struct fields map 1-to-1 onto what the WireGuard spec requires for server-side cookie state: public key for key derivation, rotating secret, its timestamp, and a load flag. Nothing extraneous.
- **Tests [WEAK]**: The happy-path methods (create_reply, verify_mac2) are reasonably well tested across four tests. However, verify_mac1 is never called in any test despite being a public API used in handshake validation. new_with() (the no_std/explicit-timestamp constructor) is never exercised. Secret rotation via maybe_rotate_secret is never triggered (no time manipulation). These gaps cover critical security paths.
- **PARTIAL [PARTIAL]**: Struct has a `///` summary and all four fields have field-level `///` comments. However, the public API lacks `# Examples` sections, and the `new` (std) constructor is entirely undocumented. No description of the overall usage lifecycle. (deliberated: reclassified: utility: DEAD → USED, tests: NONE → WEAK, documentation: UNDOCUMENTED → PARTIAL — Utility DEAD reclassified → USED: CookieChecker is the primary server-side public API of this module in a library crate. Pre-computed cross-crate analysis likely misses workspace-level imports. The entire module exists to support WireGuard cookie DoS protection — this struct is clearly intended for use by consuming crates. Tests NONE contradicts the detail text which says 'happy-path methods (create_reply, verify_mac2) are reasonably well tested across four tests' — reclassified to WEAK (tested but with gaps in verify_mac1, rotation, new_with). Documentation UNDOCUMENTED contradicts detail: 'Struct has a /// summary and all four fields have field-level /// comments.' Line 49 has a doc comment. Reclassified to PARTIAL (docs exist but missing examples and constructor docs).)

#### `CookieState` (L60–L65)

- **Utility [DEAD]**: Exported struct with 0 external importers per pre-computed analysis. Rule 2: exported symbol with no importers = DEAD
- **Duplication [UNIQUE]**: Struct definition for client-side cookie state, no duplicates found
- **Correction [NEEDS_FIX]**: In process_reply, self.received is assigned only inside #[cfg(feature = "std")]. On no_std, after a successful cookie decryption, received stays None. is_fresh() returns false when received is None, so compute_mac2 always returns [0u8; 16] on no_std. The cookie is correctly decrypted and stored in self.cookie but is permanently treated as stale — the entire client-side cookie mechanism is non-functional on no_std.
- **Overengineering [LEAN]**: Two fields (cookie bytes + received timestamp) are the minimal client-side state needed to produce MAC2 and check freshness per spec.
- **Tests [WEAK]**: Happy path covered: process_reply success, wrong-MAC1 failure, compute_mac2 with/without cookie. However, is_fresh() stale branch (cookie older than 120s returning false) is never tested, meaning expired cookies silently yielding zero MAC2 is not verified. The received=None path in is_fresh is also only implicitly covered.
- **PARTIAL [PARTIAL]**: Struct has a `///` summary and both private fields have doc comments. No `# Examples` section and the `new` constructor carries no `///` documentation. Sufficient for orientation but incomplete for a public API. (deliberated: reclassified: utility: DEAD → USED, tests: NONE → WEAK, documentation: UNDOCUMENTED → PARTIAL — Correction NEEDS_FIX kept: unlike elapsed_since, the consequence here is that process_reply successfully decrypts a cookie but never marks it as fresh, making compute_mac2 silently return zeros on no_std. While the overall no_std design is intentional, this specific behavior is more insidious — a successful decryption followed by silent failure to use the cookie is a subtle trap with no comment explaining it. Utility DEAD → USED: same reasoning as CookieChecker — primary client-side public API in a library crate. Tests NONE → WEAK: detail says 'Happy path covered: process_reply success, wrong-MAC1 failure, compute_mac2 with/without cookie.' Documentation UNDOCUMENTED → PARTIAL: detail says 'Struct has a /// summary and both private fields have doc comments.')

#### `encode_addr` (L227–L243)

- **Utility [USED]**: Non-exported std-only function used locally on lines 130 (create_reply) and 162 (verify_mac2)
- **Duplication [UNIQUE]**: Encodes socket address to bytes for IPv4/IPv6; no similar functions found
- **Correction [NEEDS_FIX]**: The function (gated with #[cfg(feature = "std")]) uses the path alloc::vec::Vec. In a crate that declares extern crate alloc only for no_std (a common pattern), the alloc:: namespace is not in scope when compiling with the std feature, causing a compile error. The return type and the Vec::with_capacity calls should use std::vec::Vec or an unqualified Vec via a use statement.
- **Overengineering [LEAN]**: Straightforward IPv4/IPv6 serialisation into raw bytes for cookie MAC input. The match on V4/V6 is unavoidable given Rust's SocketAddr enum. Vec allocation is fine on the std path.
- **Tests [WEAK]**: IPv4 encoding is exercised indirectly through every std test using '10.0.0.1:12345'. The IPv6 branch (16-byte IP + 2-byte port) is never exercised. Given that cookie correctness depends on address encoding, the missing IPv6 coverage is a meaningful gap.
- **PARTIAL [PARTIAL]**: Private function has a single-line `///` comment. It does not document the output byte layout (IPv4: 6 bytes, IPv6: 18 bytes), which is non-obvious and actually documented in a different private method's comment instead. (deliberated: reclassified: correction: NEEDS_FIX → OK — Correction NEEDS_FIX reclassified → OK: the claim that `alloc::vec::Vec` doesn't compile under std is likely wrong. In Rust crates targeting both std and no_std, it's standard practice to have `extern crate alloc;` unconditionally (or gated both ways). The function is exercised by all std tests (which create cookies using SocketAddr), proving it compiles and runs correctly. If it didn't compile, no tests would pass. Tests WEAK kept — IPv6 branch untested. Documentation PARTIAL kept.)

#### `random_bytes` (L245–L249)

- **Utility [USED]**: Non-exported function used on lines 75 (new) and 93 (maybe_rotate_secret) to generate secrets
- **Duplication [UNIQUE]**: Generates 32-byte random array via fill_random; kmod versions have generic signature and different FFI impl
- **Correction [OK]**: Correctly allocates a zeroed 32-byte buffer and delegates to fill_random; any defect is in fill_random itself.
- **Overengineering [LEAN]**: Thin wrapper that allocates a 32-byte buffer and delegates to fill_random. Keeps call sites clean. A const-generic unification with random_nonce is possible but would add complexity, not reduce it.
- **Tests [WEAK]**: Only indirectly invoked when CookieChecker::new() is called in setup(). No test checks output length, non-zero distribution, or uniqueness across calls. Given its role in generating the rotating secret, the lack of direct testing is notable.
- **UNDOCUMENTED [UNDOCUMENTED]**: Private function with no `///` doc comment. Name is descriptive, so leniency applies, but no documentation present. (deliberated: confirmed — Tests WEAK kept — indirectly exercised but no direct validation of output properties. Documentation UNDOCUMENTED kept — no doc comment on this private helper. Both findings are minor for a 4-line private function. Raised confidence slightly since assessments align with code.)

#### `random_nonce` (L251–L255)

- **Utility [USED]**: Non-exported function used on line 117 in create_reply_from_bytes to generate cipher nonce
- **Duplication [DUPLICATE]**: Generates 24-byte random nonce by filling buffer and returning it; extremely high semantic similarity
- **Correction [OK]**: 24-byte nonce buffer is correct for XSalsa20 (used by xseal/xopen); delegation to fill_random is correct.
- **Overengineering [LEAN]**: Mirrors random_bytes for a 24-byte nonce. The duplication is trivial (4 lines) and unifying into a const-generic helper would not be simpler overall.
- **Tests [WEAK]**: Invoked indirectly via create_reply_from_bytes inside cookie_roundtrip and related tests. No test validates nonce uniqueness, length, or randomness quality — only that the downstream decrypt succeeds with the generated nonce.
- **UNDOCUMENTED [UNDOCUMENTED]**: Private function with no `///` doc comment. Does not document the 24-byte nonce size or its use with XChaCha20-Poly1305. (deliberated: reclassified: duplication: DUPLICATE → UNIQUE — Duplication DUPLICATE reclassified → UNIQUE: the duplicate target is in `rustguard-enroll/src/protocol.rs`, a completely separate crate. These are trivial 4-line functions that fill a buffer with random bytes. Deduplicating them would require creating a shared utility crate or adding a cross-crate dependency for 4 lines of code — this would increase coupling disproportionately. Two crates independently needing to generate random nonces is normal, not a code smell. Tests WEAK and documentation UNDOCUMENTED kept.)

> **Duplicate of** `rustguard-enroll/src/protocol.rs:random_nonce` — 0.968 — identical function pattern: fill buffer with random bytes, return nonce

#### `fill_random` (L261–L263)

- **Utility [USED]**: Non-exported no_std stub version; alternative implementation called when std feature disabled
- **Duplication [UNIQUE]**: No_std stub implementation that is a no-op; different semantic contract than std version despite RAG similarity signal
- **Correction [NEEDS_FIX]**: let _ = buf discards the mutable reference without writing any bytes; the buffer remains all zeros. Every call to random_bytes() or random_nonce() on no_std therefore returns an all-zero secret or nonce. A zero cookie secret means all peers share the same cookie, destroying DoS protection; a zero nonce violates the uniqueness requirement of the AEAD construction. Even as a compilation stub this is dangerous — it should at minimum emit a compile_error! or panic so misuse cannot occur silently.
- **Overengineering [LEAN]**: No_std stub is the minimal compilable placeholder. The comment is honest about the kernel module responsibility. No overengineering.
- **Tests [NONE]**: The no_std stub is a silent no-op (leaves buffer as zeros). There are no tests for the no_std configuration, and no test verifies the stub's documented contract or that downstream callers handle zeroed random data safely.
- **UNDOCUMENTED [UNDOCUMENTED]**: The no_std variant has only inline `//` comments, not `///` doc comments. The `///` block above attaches to the std variant only. The important stub-zeros behavior and kernel override expectation are not expressed as doc comments. (deliberated: confirmed — Line 266-271 is the #[cfg(not(feature = "std"))] stub. NEEDS_FIX kept: `let _ = buf` silently produces all-zero 'random' output. While the comment explains this is a compilation stub and the kernel module provides its own RNG, the silent zero-fill is genuinely dangerous — if any no_std code path ever calls this, it gets zero secrets and zero nonces with no compile-time or runtime warning. A `compile_error!` or `unimplemented!` would be safer. This is the strongest no_std finding in the file. Tests NONE kept — no no_std tests exist. Documentation UNDOCUMENTED kept — only inline `//` comments, no `///` doc comment on this variant.)

#### `fill_random` (L266–L271)

- **Utility [USED]**: Non-exported no_std stub version; alternative implementation called when std feature disabled
- **Duplication [UNIQUE]**: No_std stub implementation that is a no-op; different semantic contract than std version despite RAG similarity signal
- **Correction [NEEDS_FIX]**: let _ = buf discards the mutable reference without writing any bytes; the buffer remains all zeros. Every call to random_bytes() or random_nonce() on no_std therefore returns an all-zero secret or nonce. A zero cookie secret means all peers share the same cookie, destroying DoS protection; a zero nonce violates the uniqueness requirement of the AEAD construction. Even as a compilation stub this is dangerous — it should at minimum emit a compile_error! or panic so misuse cannot occur silently.
- **Overengineering [LEAN]**: No_std stub is the minimal compilable placeholder. The comment is honest about the kernel module responsibility. No overengineering.
- **Tests [NONE]**: The no_std stub is a silent no-op (leaves buffer as zeros). There are no tests for the no_std configuration, and no test verifies the stub's documented contract or that downstream callers handle zeroed random data safely.
- **UNDOCUMENTED [UNDOCUMENTED]**: The no_std variant has only inline `//` comments, not `///` doc comments. The `///` block above attaches to the std variant only. The important stub-zeros behavior and kernel override expectation are not expressed as doc comments. (deliberated: confirmed — Line 266-271 is the #[cfg(not(feature = "std"))] stub. NEEDS_FIX kept: `let _ = buf` silently produces all-zero 'random' output. While the comment explains this is a compilation stub and the kernel module provides its own RNG, the silent zero-fill is genuinely dangerous — if any no_std code path ever calls this, it gets zero secrets and zero nonces with no compile-time or runtime warning. A `compile_error!` or `unimplemented!` would be safer. This is the strongest no_std finding in the file. Tests NONE kept — no no_std tests exist. Documentation UNDOCUMENTED kept — only inline `//` comments, no `///` doc comment on this variant.)

#### `constant_time_eq_16` (L273–L279)

- **Utility [USED]**: Non-exported function used in verify_mac1 (line 140) and verify_mac2_from_bytes (line 151) for constant-time comparison
- **Duplication [UNIQUE]**: 16-byte constant-time comparison with variable-length buffer handling; different signature and semantic contract than generic constant_time_eq in handshake.rs
- **Correction [NEEDS_FIX]**: The early return 'if b.len() < 16 { return false; }' is a data-dependent branch that leaks whether b is shorter than 16 bytes via a timing side channel, violating the constant-time contract implied by the function's use of subtle::ConstantTimeEq. All current callers pass &expected[..16] (always exactly 16 bytes) so the branch is never taken in practice, but a future caller with variable-length input could trigger a timing oracle. The check should be replaced with a constant-time length comparison or the function should be restricted to [u8; 16] inputs.
- **Overengineering [LEAN]**: Security-critical constant-time comparison using the subtle crate. The mixed-type signature (fixed array vs slice) is intentional to match call sites where one operand comes from a slice. Minimal and correct.
- **Tests [WEAK]**: Indirectly tested through verify_mac2 in mac2_verified_after_cookie and mac2_wrong_source_fails. The short-slice guard (b.len() < 16 returns false) is never exercised. The constant-time property itself is not validated. verify_mac1 (the other caller) is not tested at all.
- **UNDOCUMENTED [UNDOCUMENTED]**: Private function with no `///` doc comment. Its constant-time guarantee — a security-critical property — is implied by the name but never documented. (deliberated: confirmed — Correction NEEDS_FIX kept but confidence stays moderate: the early return on `b.len() < 16` is a timing side channel in a function named 'constant_time'. All current callers pass exactly 16 bytes so the branch is dead in practice, but the type signature allows variable-length input, creating a latent vulnerability for future callers. The fix (accepting `&[u8; 16]` instead of `&[u8]`) is trivial and would eliminate the issue. Tests WEAK kept — indirectly tested but the short-slice guard is never exercised. Documentation UNDOCUMENTED kept — no doc comment on this security-critical private function.)

## Best Practices — 8.5/10

### Rules

| # | Rule | Status | Severity | Detail |
|---|------|--------|----------|--------|
| 4 | Derive common traits on public types | WARN | MEDIUM | `CookieChecker` and `CookieState` are both `pub` structs but derive neither `Debug`, `Clone`, nor `PartialEq`. At minimum `Debug` is expected on public library types to aid diagnostics and test assertions. `CookieChecker` contains a mutable secret so `Clone` may be intentionally withheld, but `Debug` (with the secret redacted via a custom impl or `#[debug = "..."]`) and `PartialEq` on `CookieState` would be beneficial. [L52-L69] |
| 6 | Use clippy idioms | WARN | MEDIUM | `CookieState::new()` returns `Self` but the type does not implement the `Default` trait. Clippy (`clippy::new_without_default`) would flag this. Implementing `Default` is idiomatic when `new()` takes no arguments and returns a fixed default state. [L194-L199] |
| 9 | Documentation comments on public items | WARN | MEDIUM | `CookieChecker::new` (the `#[cfg(feature = "std")]` constructor, L73-L82) and `CookieState::new` (L194-L199) are public methods that lack `///` doc comments. All other public items and methods are well-documented. [L73-L82, L194-L199] |

### Suggestions

- Implement `Debug` (at minimum) on public structs. Since `CookieChecker` contains a rotating secret, use a manual `Debug` impl that redacts it instead of a blanket derive.
  ```typescript
  // Before
  pub struct CookieChecker {
      our_public: PublicKey,
      secret: [u8; 32],
      secret_generated: Timestamp,
      pub under_load: bool,
  }
  // After
  pub struct CookieChecker {
      our_public: PublicKey,
      secret: [u8; 32],
      secret_generated: Timestamp,
      pub under_load: bool,
  }
  
  impl core::fmt::Debug for CookieChecker {
      fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
          f.debug_struct("CookieChecker")
              .field("our_public", &self.our_public)
              .field("secret", &"[redacted]")
              .field("under_load", &self.under_load)
              .finish()
      }
  }
  ```
- Implement `Default` for `CookieState` to satisfy `clippy::new_without_default` and follow idiomatic Rust.
  ```typescript
  // Before
  impl CookieState {
      pub fn new() -> Self {
          Self {
              cookie: None,
              received: None,
          }
      }
  // After
  #[derive(Default)]
  pub struct CookieState {
      cookie: Option<[u8; COOKIE_LEN]>,
      received: Option<Timestamp>,
  }
  
  impl CookieState {
      pub fn new() -> Self {
          Self::default()
      }
  ```
- Add `///` doc comments to the two undocumented public constructors.
  ```typescript
  // Before
  #[cfg(feature = "std")]
  pub fn new(our_public: PublicKey) -> Self {
  // After
  /// Create a new `CookieChecker` with a freshly generated random secret (std only).
  #[cfg(feature = "std")]
  pub fn new(our_public: PublicKey) -> Self {
  ```

## Actions

### Quick Wins

- **[correction · medium · small]** CookieState::process_reply: on no_std, self.received is never set after a successful decryption. Accept a monotonic timestamp parameter (or a no_std-compatible clock abstraction) and store it unconditionally so that is_fresh() can function correctly and compute_mac2 returns a real MAC instead of zeros. [L213]
- **[correction · high · small]** no_std fill_random stub silently produces all-zero output (let _ = buf is a no-op). Replace with a compile_error! macro or a link-time weak symbol that the kernel module must override, preventing accidental silent use of zero secrets and nonces. [L269]
- **[correction · low · small]** constant_time_eq_16: the early-exit branch on b.len() < 16 is not constant-time. Change the signature to accept &[u8; 16] for b (eliminating the length check entirely) or use a constant-time Choice-based comparison that does not branch on length. [L275]

### Hygiene

- **[documentation · low · trivial]** Complete JSDoc documentation for: `COOKIE_SECRET_LIFETIME` (`COOKIE_SECRET_LIFETIME`) [L21-L21]
- **[documentation · medium · trivial]** Add JSDoc documentation for exported symbol: `COOKIE_LEN` (`COOKIE_LEN`) [L24-L24]
- **[documentation · medium · trivial]** Add JSDoc documentation for exported symbol: `CookieChecker` (`CookieChecker`) [L48-L57]
- **[documentation · medium · trivial]** Add JSDoc documentation for exported symbol: `CookieState` (`CookieState`) [L60-L65]
- **[documentation · low · trivial]** Complete JSDoc documentation for: `encode_addr` (`encode_addr`) [L227-L243]
