# Review: `rustguard-crypto/src/blake2s.rs`

**Verdict:** NEEDS_REFACTOR

## Symbols

| Symbol | Kind | Exported | Correction | Over-eng. | Utility | Duplication | Tests | Confidence |
|--------|------|----------|------------|-----------|---------|-------------|-------|------------|
| BLAKE2S_BLOCK_SIZE | constant | no | OK | LEAN | USED | UNIQUE | GOOD | 88% |
| hash | function | yes | OK | LEAN | USED | UNIQUE | WEAK | 92% |
| mac | function | yes | OK | LEAN | USED | UNIQUE | WEAK | 91% |
| hkdf | function | yes | OK | LEAN | USED | UNIQUE | WEAK | 92% |
| hmac_blake2s | function | no | OK | ACCEPTABLE | USED | UNIQUE | WEAK | 85% |

### Details

#### `BLAKE2S_BLOCK_SIZE` (L6–L6)

- **Utility [USED]**: Non-exported constant used 5 times in hmac_blake2s function (lines 72, 77, 79, 84, 86) for block-size array initialization and loop bounds.
- **Duplication [UNIQUE]**: Standard cryptographic constant for BLAKE2s-256 block size. No similar constants found in RAG results or codebase.
- **Correction [OK]**: BLAKE2s uses a 512-bit (64-byte) block size, which is the correct value to use when constructing HMAC with BLAKE2s as the underlying hash per RFC 2104.
- **Overengineering [LEAN]**: Named constant replacing the magic number 64 in a crypto context — essential for readability and correctness in the HMAC padding logic below. Exactly the right abstraction level.
- **Tests [GOOD]**: Pure private constant (value 64). No runtime behaviour to test directly. Indirectly exercised by every hmac_blake2s call in the embedded test suite — the padded-key and ipad/opad arrays are sized by this constant, so any wrong value would break determinism tests.
- **UNDOCUMENTED [UNDOCUMENTED]**: Private constant with no `///` doc comment. As a private item with a self-explanatory name and value, this is tolerable, but a brief note explaining why 64 bytes (BLAKE2s block size per spec) would improve clarity. (deliberated: reclassified: documentation: UNDOCUMENTED → DOCUMENTED — Documentation UNDOCUMENTED reclassified to DOCUMENTED. This is a private constant named BLAKE2S_BLOCK_SIZE with value 64 — the name is self-documenting for anyone with basic cryptographic knowledge. The original evaluator even acknowledged it is 'tolerable.' For a private constant whose name IS the specification reference, requiring a doc comment is excessive.)

#### `hash` (L9–L15)

- **Utility [DEAD]**: Exported function. Pre-computed analysis shows 0 runtime importers and 0 type-only importers. Used only within module tests; no external consumption detected.
- **Duplication [UNIQUE]**: High similarity (0.856, 0.849) with kernel module implementations, but fundamentally different. blake2s.rs is pure Rust library using Blake2s256::new(). Kernel versions (rustguard-kmod) use FFI wrappers (wg_blake2s_hash). Different compilation targets and type systems; architecturally distinct.
- **Correction [OK]**: Standard unkeyed BLAKE2s-256 over multiple chunks. Iterating chunks and calling update is semantically equivalent to concatenating them, and finalize().into() correctly converts GenericArray<u8, U32> to [u8; 32]. No correctness issues.
- **Overengineering [LEAN]**: Accepts &[&[u8]] for zero-copy multi-chunk hashing — idiomatic in Rust crypto code and directly validated by the hash_multi_chunk test. No unnecessary layers.
- **Tests [WEAK]**: Three embedded tests cover determinism (hash_deterministic), input-sensitivity (hash_different_inputs), and multi-chunk equivalence (hash_multi_chunk). However, no test validates output against a known BLAKE2s-256 reference vector, so a subtly wrong implementation (e.g., wrong digest size or initialisation) would pass all tests. Empty-slice input is also untested.
- **PARTIAL [PARTIAL]**: Has a one-line `///` doc comment ('BLAKE2s-256 hash.') but is otherwise bare. Missing: parameter description for the `data` slice-of-slices argument, return value description, and a required `# Examples` section for a public API function. (deliberated: reclassified: utility: DEAD → USED, tests: NONE → WEAK, documentation: UNDOCUMENTED → PARTIAL — Utility DEAD is a false positive: this is a library crate's public API. Exported functions in a crypto library ARE the product — absence of in-workspace importers does not make them dead. The crate is designed to be consumed externally (rustguard-crypto). Tests NONE contradicts the detail text which describes three existing tests (hash_deterministic, hash_different_inputs, hash_multi_chunk); reclassified to WEAK since tests exist but lack reference vectors. Documentation UNDOCUMENTED contradicts the detail and source: line 8 has `/// BLAKE2s-256 hash.` — it's PARTIAL (brief but present, missing params/examples).)

#### `mac` (L21–L29)

- **Utility [DEAD]**: Exported function. Pre-computed analysis shows 0 runtime importers and 0 type-only importers. Used only within module tests; no external consumption detected.
- **Duplication [UNIQUE]**: High similarity (0.930, 0.915) with kernel versions, but different semantic contracts. blake2s.rs takes data: &[&[u8]] (slice of slices), kernel versions take data: &[u8] (single slice). Different implementations: blake2s uses Blake2sMac keyed mode; kernel uses FFI. Different compilation targets preclude substitutability.
- **Correction [OK]**: Uses BLAKE2s built-in keying mode via Blake2sMac<U32>. new_from_slice accepts 1–32 byte keys; the panic message is slightly imprecise about the minimum (omits '>= 1') but that is a documentation concern, not a runtime bug for valid WireGuard usage. result.into_bytes().into() correctly converts GenericArray<u8, U32> to [u8; 32].
- **Overengineering [LEAN]**: Thin wrapper over Blake2sMac with multi-chunk input. The explicit CtOutput binding is a minor verbosity imposed by the type system, not a design choice. Function is minimal and purposeful.
- **Tests [WEAK]**: Only one direct test (mac_differs_with_key) plus indirect coverage via hmac_differs_from_keyed_blake2s. Missing: determinism check, multi-chunk behaviour, empty-data, and—critically—no known-good test vector against a BLAKE2s-MAC reference value. A wrong keying or output truncation would not be caught.
- **PARTIAL [PARTIAL]**: Has a multi-line `///` block with a useful purpose description and an important design note distinguishing keyed BLAKE2s from HMAC. However, neither the `key` nor `data` parameters are documented, the return value is not described, and there is no `# Examples` section expected for a public function. (deliberated: reclassified: utility: DEAD → USED, tests: NONE → WEAK, documentation: UNDOCUMENTED → PARTIAL — Utility DEAD is a false positive for the same library-crate reason as hash. Tests NONE contradicts the detail text which describes mac_differs_with_key and indirect coverage via hmac_differs_from_keyed_blake2s; reclassified to WEAK (tests exist but no reference vectors). Documentation UNDOCUMENTED contradicts the source: lines 17-20 have a multi-line /// block explaining keyed MAC vs HMAC distinction. Reclassified to PARTIAL per the detail's own assessment.)

#### `hkdf` (L37–L57)

- **Utility [DEAD]**: Exported function. Pre-computed analysis shows 0 runtime importers and 0 type-only importers. Used only within module tests; no external consumption detected.
- **Duplication [UNIQUE]**: High similarity (0.911) with kernel module hkdf, same signature, but fundamentally different implementations. blake2s.rs is full Rust implementation with explicit HKDF extract-expand using hmac_blake2s. Kernel version uses FFI wrapper (wg_hkdf). Different compilation targets (userspace library vs kernel module).
- **Correction [OK]**: Extract step is HMAC(key=key, msg=input) producing PRK, then expand steps correctly implement RFC 5869 with empty info: T1=HMAC(PRK,0x01), T2=HMAC(PRK,T1||0x02), T3=HMAC(PRK,T2||0x03). This matches the WireGuard noise protocol HKDF exactly.
- **Overengineering [LEAN]**: Direct, faithful implementation of WireGuard's 3-output HKDF. Returning a fixed triple is appropriate because the WireGuard spec defines exactly three expansion outputs. No abstraction layers are added beyond what the protocol requires.
- **Tests [WEAK]**: Two tests cover output distinctness (T1 ≠ T2 ≠ T3) and determinism. However, there are no test vectors derived from the WireGuard specification or any reference implementation, so a bug in the expansion step ordering or HMAC invocation would pass. Different-input differentiation and empty-input edge cases are also absent. For a KDF used in a cryptographic protocol, spec-level test vectors are essential.
- **PARTIAL [PARTIAL]**: Has a multi-line `///` block with a description of the algorithm, a note about always using BLAKE2s, and a return-value summary ('callers use what they need'). Missing: explicit `key` and `input` parameter descriptions, and a `# Examples` section. The return note is informal rather than structured. (deliberated: reclassified: utility: DEAD → USED, tests: NONE → WEAK, documentation: UNDOCUMENTED → PARTIAL — Utility DEAD is a false positive: hkdf is a core public function of a crypto library crate, essential for the WireGuard KDF. Tests NONE contradicts the detail which describes two tests (hkdf_produces_different_outputs, hkdf_deterministic); reclassified to WEAK since no RFC 5869/WireGuard spec test vectors are present. Documentation UNDOCUMENTED contradicts source: lines 31-36 have a substantial /// block describing the algorithm and return semantics. Reclassified to PARTIAL.)

#### `hmac_blake2s` (L64–L88)

- **Utility [USED]**: Non-exported function. Called 4 times in hkdf (lines 48, 51, 56, 61) and 3 times in tests (lines 116, 121, 124). Critical to HKDF expansion logic.
- **Duplication [UNIQUE]**: All similar functions score below 0.82 threshold (0.728, 0.723, 0.717). Implements RFC 2104 standard HMAC-BLAKE2s construction. Intentionally different from mac() in same file, which uses BLAKE2s built-in keyed mode. Code documentation explicitly distinguishes these as separate algorithms.
- **Correction [OK]**: Correct RFC 2104 HMAC construction: key (32 bytes) is zero-padded to block size (64 bytes) since 32 < 64 so no key hashing is needed. ipad = padded_key XOR 0x36..., opad = padded_key XOR 0x5c... are computed correctly. Inner hash = H(ipad || data) and outer hash = H(opad || inner_hash) are both correct. Blake2s256::new() creates an unkeyed hasher, which is required for HMAC. No correctness issues.
- **Overengineering [ACCEPTABLE]**: Hand-rolled HMAC per RFC 2104. The RustCrypto `hmac` crate (>10M weekly downloads) is already in the same ecosystem as the `blake2` dep and would provide `Hmac<Blake2s256>` with less risk of subtle implementation error. Since `hmac` does not appear to be installed, this is ACCEPTABLE, but adding it would reduce the crypto surface area. If `hmac` is already a transitive dep, this becomes a mild NIH case (OVER).
- **Tests [WEAK]**: Two tests exist: hmac_differs_from_keyed_blake2s (verifies HMAC ≠ keyed-BLAKE2s mode) and hmac_test_vector (despite the name, only checks determinism and key-sensitivity — it carries no known-correct HMAC output). No test validates against an RFC 2104 reference vector computed with BLAKE2s. The ipad/opad XOR logic and key-padding are critical correctness points that are entirely unverified against ground truth.
- **PARTIAL [PARTIAL]**: Private function with a meaningful `///` block that includes the HMAC formula notation, an RFC 2104 reference, and a critical design distinction from keyed BLAKE2s. For a private function this is solid; however, the key parameter is typed as `&[u8; 32]` while the doc refers to it only in the formula, and no param/return annotations are present. Confidence reduced slightly due to private-item leniency. (deliberated: confirmed — All three findings are correct and coherent. Tests WEAK: two tests exist but no RFC 2104 reference vector validates the ipad/opad XOR logic — critical gap for a hand-rolled HMAC. Documentation PARTIAL: the /// block is solid for a private function (includes formula and RFC reference) but lacks param annotations. Overengineering ACCEPTABLE: hand-rolled HMAC is justified if the hmac crate isn't a dependency, though using it would reduce crypto surface area. Confidence raised from 70 to 85 as assessment is well-supported.)

## Best Practices — 8.5/10

### Rules

| # | Rule | Status | Severity | Detail |
|---|------|--------|----------|--------|
| 6 | Use clippy idioms | WARN | MEDIUM | hmac_blake2s contains two manual indexed for-loops (lines 70-73 and 78-81) to XOR key pads. Clippy would flag these in favour of iterator zip chains, which are more idiomatic, avoid index bounds checks, and are easier to read. [L70-L73, L78-L81] |
| 7 | No panic in library/production code | WARN | CRITICAL | rustguard-crypto is a library crate (has lib.rs). The public function mac() calls .expect("BLAKE2s-MAC key must be <= 32 bytes") at line 23, which panics if the caller supplies a key longer than 32 bytes. This exposes callers to an unrecoverable panic without any way to handle the error. The function should return Result<[u8; 32], Blake2sError> and propagate the error instead. [L22-L24] |

### Suggestions

- Make mac() panic-safe for library use by returning Result instead of calling .expect(). This eliminates the silent panic footgun for callers who pass oversized keys.
  ```typescript
  // Before
  pub fn mac(key: &[u8], data: &[&[u8]]) -> [u8; 32] {
      let mut m = Blake2sMac::<U32>::new_from_slice(key)
          .expect("BLAKE2s-MAC key must be <= 32 bytes");
      ...
      result.into_bytes().into()
  }
  // After
  pub fn mac(key: &[u8], data: &[&[u8]]) -> Result<[u8; 32], blake2::digest::InvalidLength> {
      let mut m = Blake2sMac::<U32>::new_from_slice(key)?;
      for chunk in data {
          m.update(chunk);
      }
      let result: CtOutput<Blake2sMac<U32>> = m.finalize();
      Ok(result.into_bytes().into())
  }
  ```
- Replace manual indexed XOR loops in hmac_blake2s with idiomatic iterator zip chains to satisfy clippy::needless_range_loop and improve readability.
  ```typescript
  // Before
  let mut ipad = [0x36u8; BLAKE2S_BLOCK_SIZE];
  for i in 0..BLAKE2S_BLOCK_SIZE {
      ipad[i] ^= padded_key[i];
  }
  // ...
  let mut opad = [0x5cu8; BLAKE2S_BLOCK_SIZE];
  for i in 0..BLAKE2S_BLOCK_SIZE {
      opad[i] ^= padded_key[i];
  }
  // After
  let mut ipad = [0x36u8; BLAKE2S_BLOCK_SIZE];
  ipad.iter_mut().zip(padded_key.iter()).for_each(|(a, b)| *a ^= b);
  // ...
  let mut opad = [0x5cu8; BLAKE2S_BLOCK_SIZE];
  opad.iter_mut().zip(padded_key.iter()).for_each(|(a, b)| *a ^= b);
  ```

## Actions

### Hygiene

- **[documentation · medium · trivial]** Add JSDoc documentation for exported symbol: `hash` (`hash`) [L9-L15]
- **[documentation · medium · trivial]** Add JSDoc documentation for exported symbol: `mac` (`mac`) [L21-L29]
- **[documentation · medium · trivial]** Add JSDoc documentation for exported symbol: `hkdf` (`hkdf`) [L37-L57]
- **[documentation · low · trivial]** Complete JSDoc documentation for: `hmac_blake2s` (`hmac_blake2s`) [L64-L88]
