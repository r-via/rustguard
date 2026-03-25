# Review: `rustguard-crypto/src/aead.rs`

**Verdict:** NEEDS_REFACTOR

## Symbols

| Symbol | Kind | Exported | Correction | Over-eng. | Utility | Duplication | Tests | Confidence |
|--------|------|----------|------------|-----------|---------|-------------|-------|------------|
| AEAD_TAG_LEN | constant | yes | OK | LEAN | USED | UNIQUE | GOOD | 93% |
| MAX_PACKET_SIZE | constant | yes | NEEDS_FIX | LEAN | USED | UNIQUE | GOOD | 88% |
| seal | function | yes | OK | LEAN | USED | UNIQUE | WEAK | 90% |
| open | function | yes | OK | LEAN | USED | UNIQUE | GOOD | 92% |
| xseal | function | yes | OK | LEAN | USED | UNIQUE | NONE | 90% |
| xopen | function | yes | OK | LEAN | USED | UNIQUE | NONE | 90% |
| seal_to | function | yes | OK | LEAN | USED | UNIQUE | NONE | 90% |
| open_to | function | yes | NEEDS_FIX | LEAN | USED | UNIQUE | NONE | 88% |
| build_nonce | function | no | OK | LEAN | USED | UNIQUE | WEAK | 85% |

### Details

#### `AEAD_TAG_LEN` (L7–L7)

- **Utility [DEAD]**: Exported constant with 0 runtime/type-only importers. Public API constant in library crate (rustguard-crypto); pattern matches Known False Positives (COOKIE_LEN case). Likely consumed by downstream crates.
- **Duplication [UNIQUE]**: Simple constant (16). No similar constants reported by RAG.
- **Correction [OK]**: ChaCha20-Poly1305 produces a 16-byte (128-bit) authentication tag. Value is correct.
- **Overengineering [LEAN]**: A named constant for the Poly1305 tag size. Necessary to avoid magic numbers in packet-size calculations and in-place buffer sizing throughout the crate.
- **Tests [GOOD]**: Pure compile-time constant with no runtime behavior. Directly referenced in the `seal_then_open` assertion (`ciphertext.len() == plaintext.len() + AEAD_TAG_LEN`), providing indirect validation of its value (16).
- **UNDOCUMENTED [UNDOCUMENTED]**: No `///` doc comment present. Public constant with an opaque numeric value (16) would benefit from a brief description explaining it represents the Poly1305 authentication tag length in bytes. (deliberated: reclassified: utility: DEAD → USED, tests: NONE → GOOD — Utility DEAD→USED: This is a `pub const` in a library crate (`rustguard-crypto`). It is the fundamental tag length constant meant for downstream consumers. The evaluator itself noted it matches Known False Positives. Zero in-crate importers is expected for a library public API constant. Tests NONE→GOOD: Pure compile-time constant with no runtime behavior; indirectly validated in `seal_then_open` assertion (`ciphertext.len() == plaintext.len() + AEAD_TAG_LEN`). No dedicated test needed. Documentation UNDOCUMENTED confirmed: line 7 has no `///` comment, so UNDOCUMENTED is correct.)

#### `MAX_PACKET_SIZE` (L10–L10)

- **Utility [DEAD]**: Exported constant with 0 runtime/type-only importers. Public API constant in crypto library crate; no internal usage. Likely false positive for library public export.
- **Duplication [UNIQUE]**: Constant definition (1500 + AEAD_TAG_LEN). No duplicates found.
- **Correction [NEEDS_FIX]**: Doc comment states 'MTU 1420 + tag' but the expression uses 1500, yielding 1516 instead of 1436. WireGuard sets its interface MTU to ~1420 to leave room for IP/UDP/WireGuard headers on a 1500-byte Ethernet link. If callers use MAX_PACKET_SIZE as an upper-bound validator, they will accept packets ~80 bytes larger than the protocol allows.
- **Overengineering [LEAN]**: Single derived constant encoding the WireGuard MTU contract (1420 payload + 16 tag + 64 overhead headroom). Appropriate for a protocol-level boundary constant.
- **Tests [GOOD]**: Pure compile-time constant with no runtime behavior. Types, constants, and enums with no runtime logic are GOOD by default. No dedicated test needed.
- **PARTIAL [PARTIAL]**: Has a single-line `///` doc comment, but the description is slightly inaccurate: it claims 'MTU 1420' while the actual computation is `1500 + AEAD_TAG_LEN`. The discrepancy between the documented MTU (1420) and the code value (1500) is a documentation correctness issue. No examples expected for constants, but more precision is warranted. (deliberated: reclassified: utility: DEAD → USED, tests: NONE → GOOD, documentation: UNDOCUMENTED → PARTIAL — Correction NEEDS_FIX confirmed: the doc comment on L9 says 'MTU 1420 + tag' but the expression computes `1500 + AEAD_TAG_LEN = 1516`, a clear mismatch (~80 bytes difference). Either the doc or the value is wrong. Utility DEAD→USED: library public constant for downstream consumption, same false positive pattern as AEAD_TAG_LEN. Tests NONE→GOOD: compile-time constant with no runtime behavior, no dedicated test required. Documentation UNDOCUMENTED→PARTIAL: line 9 has a `///` doc comment present (`/// Maximum WireGuard transport payload (MTU 1420 + tag).`), so UNDOCUMENTED is incorrect. Reclassified to PARTIAL due to the inaccurate MTU value in the doc.)

#### `seal` (L16–L28)

- **Utility [DEAD]**: Exported core AEAD encryption function with 0 runtime/type-only importers. Primary public API of rustguard-crypto library; likely consumed by downstream crates. Pattern matches Known False Positives.
- **Duplication [UNIQUE]**: Encrypts with ChaCha20-Poly1305, returns allocated Vec<u8>. RAG shows similarity to seal in noise.rs (0.872, kernel FFI vs library), xseal (0.820, different cipher), seal_to (0.758, different return strategy). Different semantic contracts—cannot be substituted for one another.
- **Correction [OK]**: Key, nonce, and payload wiring are correct. The chacha20poly1305 crate's encrypt() method appends the 16-byte tag to the returned ciphertext, matching the expected WireGuard layout.
- **Overengineering [LEAN]**: Minimal allocating encrypt wrapper that delegates entirely to the `chacha20poly1305` crate. AAD support is required by the WireGuard handshake spec. No unnecessary abstraction.
- **Tests [WEAK]**: Happy path tested in `seal_then_open` (empty AAD, fixed counter 0, realistic plaintext). AAD path exercised indirectly via `aad_mismatch_fails`. Missing: empty plaintext, large/max-size plaintext, non-zero counter values as the sealing side, and explicit verification of ciphertext bytes (tests only check length and roundtrip). Edge cases like counter=u64::MAX are absent.
- **PARTIAL [PARTIAL]**: Has a `///` block describing purpose and nonce construction rationale, and briefly mentions AAD semantics. However, none of the four parameters (`key`, `counter`, `aad`, `plaintext`) are individually documented, the return type semantics are not described, and there is no `# Examples` section as expected for a public API function. (deliberated: reclassified: utility: DEAD → USED, tests: NONE → WEAK, documentation: UNDOCUMENTED → PARTIAL — Utility DEAD→USED: primary public encryption API of the library crate, consumed by downstream crates. Tests NONE→WEAK: `seal` is directly called in all 5 test functions (seal_then_open, wrong_key_fails, wrong_counter_fails, tampered_ciphertext_fails, aad_mismatch_fails), so NONE is factually incorrect. Reclassified to WEAK since edge cases (empty plaintext, large payloads, non-zero counter as primary input) are missing. Documentation UNDOCUMENTED→PARTIAL: has a multi-line `///` doc block on L12-15 describing purpose, nonce construction, and AAD semantics. Missing param-level docs and examples, hence PARTIAL not DOCUMENTED.)

#### `open` (L33–L45)

- **Utility [DEAD]**: Exported core AEAD decryption function with 0 runtime/type-only importers. Primary public API of rustguard-crypto library; likely consumed by downstream crates. Pattern matches Known False Positives.
- **Duplication [UNIQUE]**: Decrypts with ChaCha20-Poly1305, returns Option<Vec<u8>>. RAG shows similarity to open in noise.rs (0.891, FFI vs library), xopen (0.863, different cipher), open_to (0.758, different return strategy). Different contracts—xopen uses XChaCha20, open_to uses mutable buffer.
- **Correction [OK]**: Nonce reconstructed identically to seal(). Authentication failures are correctly mapped to None via .ok(). No issues.
- **Overengineering [LEAN]**: Direct mirror of `seal` for decryption. Returns Option to signal auth failure without panicking. Clean and minimal.
- **Tests [GOOD]**: Five tests exercise `open` thoroughly: happy path (seal_then_open), wrong key (wrong_key_fails), wrong counter (wrong_counter_fails), bit-flipped ciphertext (tampered_ciphertext_fails), and AAD mismatch (aad_mismatch_fails). All four primary authentication-failure modes are covered. Minor gap: no test for ciphertext shorter than AEAD_TAG_LEN, but the core contract is well validated.
- **PARTIAL [PARTIAL]**: Has a `///` block with a purpose line and a useful `Returns None` explanation covering three failure conditions. However, no individual parameter descriptions exist, and no `# Examples` section is provided, which is expected for public decryption APIs. (deliberated: reclassified: utility: DEAD → USED, tests: NONE → GOOD, documentation: UNDOCUMENTED → PARTIAL — Utility DEAD→USED: primary public decryption API of the library crate. Tests NONE→GOOD: five tests thoroughly exercise `open` — happy path, wrong key, wrong counter, tampered ciphertext, and AAD mismatch. All four primary authentication-failure modes are covered. NONE is clearly wrong. Documentation UNDOCUMENTED→PARTIAL: has a `///` block on L31-33 describing purpose and three failure conditions for `None` return. Missing param-level docs and examples, hence PARTIAL.)

#### `xseal` (L48–L54)

- **Utility [DEAD]**: Exported extended-nonce encryption function with 0 runtime/type-only importers. Public API variant in library crate; no local usage. Likely false positive.
- **Duplication [UNIQUE]**: Encrypts with XChaCha20-Poly1305 (24-byte nonce), returns Vec<u8>. RAG shows 0.820 similarity to seal but uses different cipher and explicit nonce parameter. Semantic contract differs—designed for cookie encryption with fixed nonce, not counter-based transport.
- **Correction [OK]**: XChaCha20-Poly1305 path is correct. XNonce::from_slice is safe because nonce is &[u8; 24] and XNonce is 24 bytes.
- **Overengineering [LEAN]**: XChaCha20 variant is protocol-mandated for WireGuard cookie encryption (24-byte random nonce). A separate function rather than a generic abstraction is the correct, minimal choice here.
- **Tests [NONE]**: No test in the inline `#[cfg(test)]` module covers `xseal`. Used for WireGuard cookie encryption — a security-critical path — with zero test coverage. Neither happy path nor authentication-failure scenarios are tested.
- **PARTIAL [PARTIAL]**: Has a single-line `///` comment noting the algorithm, nonce size, and intended use (cookie encryption). No parameter descriptions, no return value description, and no `# Examples` section. The doc is minimal but not absent. (deliberated: reclassified: utility: DEAD → USED, documentation: UNDOCUMENTED → PARTIAL — Utility DEAD→USED: public API variant for WireGuard cookie encryption, consumed by downstream crates. Tests NONE confirmed: no test covers xseal — this is a genuine gap on a security-critical path. Documentation UNDOCUMENTED→PARTIAL: has a single-line `///` on L48 noting algorithm, nonce size, and use case. Minimal but present, so UNDOCUMENTED is incorrect.)

#### `xopen` (L57–L63)

- **Utility [DEAD]**: Exported extended-nonce decryption function with 0 runtime/type-only importers. Public API variant in library crate; no local usage. Likely false positive.
- **Duplication [UNIQUE]**: Decrypts with XChaCha20-Poly1305 (24-byte nonce), returns Option<Vec<u8>>. RAG shows 0.863 similarity to open and 0.793 to open in noise.rs, but uses different cipher (XChaCha20 vs ChaCha20). Semantically distinct—handles explicit 24-byte nonces, not counter-derived 12-byte nonces.
- **Correction [OK]**: Symmetric counterpart to xseal. Authentication failure correctly returns None.
- **Overengineering [LEAN]**: Symmetric counterpart to `xseal` for cookie decryption. Same justification — protocol-required, no unnecessary abstraction.
- **Tests [NONE]**: No test in the inline `#[cfg(test)]` module covers `xopen`. The XChaCha20-Poly1305 decryption path (used for cookie decryption) has zero test coverage; roundtrip, wrong key, tampered data, and nonce mismatch are all untested.
- **PARTIAL [PARTIAL]**: Has a single-line `///` comment noting the algorithm, nonce size, and use case. Missing parameter descriptions, return-value semantics (e.g., when `None` is returned), and a `# Examples` section expected for public APIs. (deliberated: reclassified: utility: DEAD → USED, documentation: UNDOCUMENTED → PARTIAL — Utility DEAD→USED: public API variant for WireGuard cookie decryption, consumed by downstream crates. Tests NONE confirmed: no test covers xopen — roundtrip, wrong key, tampered data, and nonce mismatch all untested. Documentation UNDOCUMENTED→PARTIAL: has a single-line `///` on L57 noting algorithm, nonce size, and use case. Present but lacks param docs and return semantics.)

#### `seal_to` (L68–L78)

- **Utility [DEAD]**: Exported in-place encryption function with 0 runtime/type-only importers. Zero-allocation API variant in crypto library; no local usage. Likely false positive.
- **Duplication [UNIQUE]**: In-place encryption: copies plaintext to buffer, encrypts, appends tag. Returns usize (total length). RAG shows 0.762/0.758 similarity to seal/kmod operations, but different contract—uses provided mutable buffer instead of allocating Vec<u8>. Zero-allocation design.
- **Correction [OK]**: Preconditions (buf >= plaintext.len() + 16) are documented and panicking on contract violation is idiomatic for hot-path Rust. The encrypt_in_place_detached + manual tag append is correct. Empty AAD matches WireGuard transport usage.
- **Overengineering [LEAN]**: Zero-allocation in-place encryption for the transport hot path. In a VPN daemon encrypting every outbound packet, avoiding heap allocation per packet is a legitimate, necessary optimization, not premature complexity.
- **Tests [NONE]**: No test in the inline `#[cfg(test)]` module covers `seal_to`. The zero-allocation in-place encryption path is entirely untested: output length, correct tag placement in buf, and buffer-size semantics are all unverified.
- **PARTIAL [PARTIAL]**: Has a three-line `///` block describing purpose, buffer size precondition, and the zero-allocation property. Lacks individual parameter documentation, missing a `# Panics` section despite calling `.expect()` unconditionally, and no `# Examples` section. (deliberated: reclassified: utility: DEAD → USED, documentation: UNDOCUMENTED → PARTIAL — Utility DEAD→USED: zero-allocation public API for hot-path transport encryption, consumed by downstream crates. Tests NONE confirmed: no test covers seal_to — output length, correct tag placement, and buffer semantics all unverified. Documentation UNDOCUMENTED→PARTIAL: has a three-line `///` block on L66-68 describing purpose, buffer precondition, and zero-allocation property. Missing `# Panics` section for `.expect()` and param-level docs.)

#### `open_to` (L83–L96)

- **Utility [DEAD]**: Exported in-place decryption function with 0 runtime/type-only importers. Zero-allocation API variant in crypto library; no local usage. Likely false positive.
- **Duplication [UNIQUE]**: In-place decryption: validates length, decrypts in buffer, returns plaintext length. RAG shows 0.783/0.758 similarity to open variants, but different contract—takes mutable buffer instead of returning Option<Vec<u8>>. Zero-allocation design with length-based output.
- **Correction [NEEDS_FIX]**: The function already performs partial input validation (ct_len < AEAD_TAG_LEN → None), signalling defensive intent. However it never checks ct_len > buf.len(). If a caller passes ct_len larger than buf's actual length, buf[..ct_len] panics instead of returning None. Because open_to processes potentially adversarial ciphertext and advertises an Option return for failure, the missing upper-bound guard is an inconsistency that causes a crash rather than a graceful failure.
- **Overengineering [LEAN]**: Zero-allocation in-place decryption mirroring `seal_to`. The early guard for `ct_len < AEAD_TAG_LEN` is a minimal, correct safety check. Justified for the same hot-path reasons as `seal_to`.
- **Tests [NONE]**: No test in the inline `#[cfg(test)]` module covers `open_to`. The short-buffer guard (`ct_len < AEAD_TAG_LEN → None`), the happy-path decryption, and authentication-failure behavior are all completely untested.
- **PARTIAL [PARTIAL]**: Has a three-line `///` block describing purpose, buffer in/out contract, and zero-allocation guarantee. Missing formal parameter descriptions (e.g., semantics of `ct_len`), no `# Panics` for the internal `expect`, and no `# Examples` section required for public API. (deliberated: reclassified: utility: DEAD → USED, documentation: UNDOCUMENTED → PARTIAL — Correction NEEDS_FIX confirmed: the function guards `ct_len < AEAD_TAG_LEN` (defensive intent) but never checks `ct_len > buf.len()`. Since `buf[..ct_len]` on L91 will panic if ct_len exceeds buf length, and this function processes potentially adversarial input while advertising Option return, the missing upper-bound guard is a genuine inconsistency. Utility DEAD→USED: zero-allocation public API for hot-path transport decryption. Tests NONE confirmed: completely untested. Documentation UNDOCUMENTED→PARTIAL: has a three-line `///` block on L81-83 describing purpose, buffer contract, and zero-allocation guarantee.)

#### `build_nonce` (L99–L103)

- **Utility [USED]**: Non-exported private function called 4 times locally: seal (L22), open (L39), seal_to (L73), open_to (L86). Required helper for WireGuard nonce construction.
- **Duplication [UNIQUE]**: Helper: constructs WireGuard nonce (4 zero bytes || 8-byte LE counter). No similar functions reported by RAG. Private utility with specific WireGuard nonce format.
- **Correction [OK]**: Follows the WireGuard spec exactly: 96-bit nonce = 32 zero bits || 64-bit little-endian counter (section 5.4.6 of the WireGuard paper). nonce[4..] receives the 8 LE counter bytes, bytes 0-3 remain zero.
- **Overengineering [LEAN]**: Four-line private helper encoding exactly the WireGuard nonce layout (4 zero bytes || 8-byte LE counter). Factoring it out avoids duplication across `seal`, `open`, `seal_to`, and `open_to`. Appropriately scoped.
- **Tests [WEAK]**: Private helper indirectly exercised by all five inline tests via `seal`/`open`. However, the WireGuard-specific nonce layout (4 zero bytes prepended to 8-byte LE counter) is never explicitly verified — no test inspects the nonce bytes or confirms the encoding of counter=1 vs counter=0. The `wrong_counter_fails` test only proves different counters produce different nonces, not that the byte layout matches the WireGuard spec.
- **DOCUMENTED [DOCUMENTED]**: Private function with a clear, precise `///` comment describing the exact WireGuard nonce wire format (4 zero bytes || 8-byte LE counter). For a private helper with a descriptive name, this level of documentation is appropriate and complete. No `# Examples` required. (deliberated: confirmed — Tests WEAK confirmed: build_nonce is indirectly exercised by all 5 tests via seal/open, but no test explicitly verifies the WireGuard nonce byte layout (4 zero bytes || 8-byte LE counter). The `wrong_counter_fails` test only proves different counters produce different ciphertexts, not that the nonce encoding matches the spec. WEAK is appropriate.)

## Best Practices — 7.5/10

### Rules

| # | Rule | Status | Severity | Detail |
|---|------|--------|----------|--------|
| 3 | Proper error handling with Result/Option | WARN | HIGH | `seal()`, `xseal()`, and `seal_to()` return `Vec<u8>`/`usize` directly and convert encrypt errors to panics via `.expect()`, silencing them from the caller's perspective. A library crate should surface errors via `Result<Vec<u8>, AeadError>` so callers can handle them. The decrypt paths (`open`, `open_to`, `xopen`) correctly use `Option` for auth failures and `.ok()` for conversion. [L17-L28, L47-L55, L65-L77] |
| 7 | No panic in library/production code | WARN | CRITICAL | This is a library crate (`rustguard-crypto`). Three public functions — `seal()` (L26), `xseal()` (L52), and `seal_to()` (L72) — call `.expect()`, which panics on error. While ChaCha20-Poly1305 encryption with correctly typed fixed-size keys should never fail in practice, a library must not impose panics on its callers. The idiomatic fix is to return `Result<Vec<u8>, chacha20poly1305::Error>` and use `?`. No literal `panic!` macro is used, so this is WARN rather than FAIL. [L26, L52, L72] |
| 9 | Documentation comments on public items | WARN | MEDIUM | `AEAD_TAG_LEN` (L7) is a public constant with no `///` documentation comment. All six public functions (`seal`, `open`, `xseal`, `xopen`, `seal_to`, `open_to`) and `MAX_PACKET_SIZE` are well-documented. One public constant is missing a doc comment. [L7] |

### Suggestions

- Return `Result` from `seal` instead of panicking, so callers can handle encryption errors without risking a library-triggered panic.
  ```typescript
  // Before
  pub fn seal(key: &[u8; 32], counter: u64, aad: &[u8], plaintext: &[u8]) -> Vec<u8> {
      let cipher = ChaCha20Poly1305::new(key.into());
      let nonce = build_nonce(counter);
      cipher
          .encrypt(&nonce, Payload { msg: plaintext, aad })
          .expect("encryption failed")
  }
  // After
  pub fn seal(
      key: &[u8; 32],
      counter: u64,
      aad: &[u8],
      plaintext: &[u8],
  ) -> Result<Vec<u8>, chacha20poly1305::Error> {
      let cipher = ChaCha20Poly1305::new(key.into());
      let nonce = build_nonce(counter);
      cipher.encrypt(&nonce, Payload { msg: plaintext, aad })
  }
  ```
- Add a `///` doc comment to the public constant `AEAD_TAG_LEN` to document its role and value.
  ```typescript
  // Before
  pub const AEAD_TAG_LEN: usize = 16;
  // After
  /// Length in bytes of the Poly1305 authentication tag appended by ChaCha20-Poly1305 encryption.
  pub const AEAD_TAG_LEN: usize = 16;
  ```
- Return `Result` from `seal_to` so in-place encryption errors are surfaced to callers instead of panicking inside a library function.
  ```typescript
  // Before
  pub fn seal_to(key: &[u8; 32], counter: u64, plaintext: &[u8], buf: &mut [u8]) -> usize {
      ...
      let tag = cipher
          .encrypt_in_place_detached(&nonce, &[], &mut buf[..len])
          .expect("encryption failed");
      ...
  }
  // After
  pub fn seal_to(
      key: &[u8; 32],
      counter: u64,
      plaintext: &[u8],
      buf: &mut [u8],
  ) -> Result<usize, chacha20poly1305::Error> {
      ...
      let tag = cipher
          .encrypt_in_place_detached(&nonce, &[], &mut buf[..len])?;
      ...
      Ok(len + AEAD_TAG_LEN)
  }
  ```

## Actions

### Quick Wins

- **[correction · medium · small]** MAX_PACKET_SIZE uses 1500 in its expression but the doc comment documents the limit as 'MTU 1420 + tag'. Change the constant to 1420 + AEAD_TAG_LEN (= 1436) to match the stated WireGuard transport MTU, or update the comment to reflect that 1500 is intentional (e.g. Ethernet MTU before WireGuard header subtraction). As-is, callers using this constant for size validation will accept oversized packets. [L10]
- **[correction · high · small]** open_to must guard against ct_len > buf.len() before indexing buf[..ct_len]. Add `if ct_len > buf.len() { return None; }` immediately after the AEAD_TAG_LEN check to prevent a panic when adversarial or malformed inputs arrive with an inflated ct_len. [L86]

### Hygiene

- **[documentation · medium · trivial]** Add JSDoc documentation for exported symbol: `AEAD_TAG_LEN` (`AEAD_TAG_LEN`) [L7-L7]
