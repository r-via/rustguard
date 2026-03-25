# Review: `rustguard-crypto/src/tai64n.rs`

**Verdict:** NEEDS_REFACTOR

## Symbols

| Symbol | Kind | Exported | Correction | Over-eng. | Utility | Duplication | Tests | Confidence |
|--------|------|----------|------------|-----------|---------|-------------|-------|------------|
| Tai64n | class | yes | OK | LEAN | USED | UNIQUE | WEAK | 92% |
| TAI64_EPOCH_OFFSET | constant | no | OK | LEAN | USED | UNIQUE | WEAK | 88% |

### Details

#### `Tai64n` (L9–L9)

- **Utility [DEAD]**: Pre-computed analysis shows 0 runtime importers and 0 type-only importers. Follows rule: exported symbol with zero importers = DEAD. Confidence reduced from 95 to 85 due to known false-positive pattern: rustguard-crypto is a library crate (matching structure of rustguard-core), and similar public API symbols (TransportSession, CookieChecker, ReplayWindow) were previously misclassified as DEAD when workspace cross-crate imports were outside analysis scope.
- **Duplication [UNIQUE]**: No similar symbols found in RAG data. Tai64n is a newtype struct wrapping a 12-byte TAI64N timestamp.
- **Correction [OK]**: The struct correctly encodes TAI64N as 12 bytes in big-endian order (8 bytes TAI seconds, 4 bytes nanoseconds). Derived `PartialOrd`/`Ord` on `[u8; 12]` uses lexicographic byte comparison, which is numerically correct given big-endian encoding. `is_after` mirrors this with a direct byte-slice comparison, consistent with the derived ordering. `from_unix` addition cannot overflow u64 for any plausible Unix timestamp (current epoch ~1.7e9 + 2^62 ≈ 4.6e18, well below u64 max ~1.8e19). `subsec_nanos()` in `now()` is guaranteed in [0, 999_999_999] by the standard library. No correctness issues found.
- **Overengineering [LEAN]**: A minimal 12-byte newtype wrapper around a fixed-size byte array implementing the TAI64N format as required by the WireGuard protocol spec. No unnecessary generics, no abstraction layers, no factory patterns. Methods are minimal: now(), from_unix(), as_bytes(), from_bytes(), is_after(). Derives are appropriate. The struct does exactly one thing and no more.
- **Tests [WEAK]**: Three inline tests cover monotonicity of `now()`, a `now()`-based roundtrip, and a `from_unix` roundtrip. However, all roundtrip tests only assert identity (encode → decode → equal) and never verify the actual byte values against a known-correct TAI64N encoding. This means the encoding correctness — the core security property used to prevent replay — is never validated. `is_after` is only exercised between two `now()` calls; no test checks it with equal timestamps (expected false), or with hand-crafted `from_unix` inputs where the expected ordering is independently known. Edge cases (nanos=0, nanos=999_999_999, secs=0) are absent.
- **PARTIAL [PARTIAL]**: Struct carries four lines of /// doc comments (L3–L8) covering purpose, byte layout, WireGuard use-case, and the replay-monotonicity invariant — solid struct-level narrative. However, two public methods lack any doc comments at all: `as_bytes` and `from_bytes` have no /// annotations. `from_unix` has a brief comment but no parameter descriptions and no `# Examples` section. No `# Examples` block appears anywhere on the type or its methods. For a public cryptographic primitive in a library crate, the absence of examples and incomplete method coverage keeps this at PARTIAL. (deliberated: reclassified: utility: DEAD → USED, tests: NONE → WEAK, documentation: UNDOCUMENTED → PARTIAL — Utility DEAD → USED: This is a `pub struct` in `rustguard-crypto`, a library crate whose entire purpose is to export types for cross-crate consumption. The evaluator itself noted this false-positive pattern and reduced confidence. Cross-crate imports were outside analysis scope, and similar symbols (TransportSession, CookieChecker) were previously misclassified. Reclassified with high confidence. Tests NONE → WEAK: The symbol field says NONE but the detail section itself begins with '[WEAK] Three inline tests cover monotonicity...' — there are clearly three tests in the `mod tests` block (L50-L80). They exist but only check roundtrip equality, never assert against known-correct byte encodings. WEAK is the accurate classification. Documentation UNDOCUMENTED → PARTIAL: Lines 4-7 contain four `///` doc comments on the struct covering purpose, byte layout, WireGuard use-case, and replay-monotonicity invariant. The detail section even states 'Struct carries four lines of /// doc comments — solid struct-level narrative.' The struct is documented; only `as_bytes` and `from_bytes` methods lack docs. PARTIAL is correct.)

#### `TAI64_EPOCH_OFFSET` (L12–L12)

- **Utility [USED]**: Non-exported const used locally within from_unix() constructor (line 29) to add TAI epoch offset to UNIX seconds. Direct internal usage confirmed in method implementation.
- **Duplication [UNIQUE]**: No similar symbols found in RAG data. TAI64_EPOCH_OFFSET is a domain-specific constant for TAI/UTC epoch conversion.
- **Correction [OK]**: `0x4000_0000_0000_000a` equals `2^62 + 10`, which correctly encodes the TAI64 base offset (`2^62`) plus the 10-second TAI–UTC difference that existed at the Unix epoch (1970-01-01). This is the exact value used in the WireGuard reference implementation and the TAI64N spec as applied to WireGuard. Subsequent leap seconds are intentionally not tracked: WireGuard only requires strictly monotonic, consistent timestamps between peers for replay prevention — not absolute UTC accuracy — so both sides using the same fixed offset is correct by design.
- **Overengineering [LEAN]**: A single named constant encoding the TAI64 epoch offset (2^62 + 10), which is a protocol-mandated magic value from the TAI64N specification. Naming it rather than inlining the literal is the correct engineering choice for readability and auditability. No complexity here whatsoever.
- **Tests [WEAK]**: This private constant encodes the TAI-vs-UTC epoch offset (2^62 + 10), which is the protocol-critical value in WireGuard's timestamp scheme. It is consumed only inside `from_unix`. All three tests use roundtrip equality, meaning any wrong constant value would still pass — a test that asserts `Tai64n::from_unix(0, 0).as_bytes()[..8] == 0x400000000000000a_u64.to_be_bytes()` would be the minimal check, but it does not exist. The constant's correctness is entirely untested.
- **UNDOCUMENTED [UNDOCUMENTED]**: Private constant (`const`, no `pub`) with no /// doc comment. The hex literal `0x4000_0000_0000_000a` encodes the 2^62 TAI64 bias plus the 10-second UTC–TAI offset, which is non-obvious to readers unfamiliar with TAI64; a brief doc comment would add value. Tolerated as UNDOCUMENTED at reduced confidence per leniency rule for private items — the name `TAI64_EPOCH_OFFSET` is moderately self-descriptive and the struct-level doc on `Tai64n` (L3–L8) hints at the '10 seconds + 2^62 offset' rationale, providing indirect context. (deliberated: reclassified: documentation: UNDOCUMENTED → DOCUMENTED — Tests WEAK: Kept as-is. The constant is only exercised through roundtrip tests that would pass even with an incorrect value. A test asserting `from_unix(0,0).as_bytes()[..8]` against the known hex value would be the minimal correctness check but is absent. However, as a private constant this is lower severity. Documentation UNDOCUMENTED → DOCUMENTED: Line 11 contains `/// TAI is ahead of UTC by 10 seconds (1970 epoch) + 2^62 offset.` — a `///` doc comment directly above the constant that clearly explains the composition of the hex literal (10-second TAI-UTC offset + 2^62 base). The evaluator's own detail acknowledges the name is 'moderately self-descriptive' and the struct-level docs provide indirect context. With an explicit doc comment present, UNDOCUMENTED is incorrect. Confidence raised from 60 to 88 since the doc comment is plainly visible in the source.)

## Best Practices — 9/10

### Rules

| # | Rule | Status | Severity | Detail |
|---|------|--------|----------|--------|
| 3 | Proper error handling with Result/Option | WARN | HIGH | The `now()` function (L18-L22) calls `.expect()` on `duration_since`, which panics instead of propagating the error. While a system clock before UNIX epoch is rare, this is a library crate (`rustguard-crypto`) and the caller cannot handle this case. Returning `Result<Self, std::time::SystemTimeError>` would be more idiomatic for a library. [L18-L22] |
| 9 | Documentation comments on public items | WARN | MEDIUM | `now()` and `is_after()` have doc comments, but `as_bytes()` (L36-L38) and `from_bytes()` (L40-L42) are public methods missing `///` documentation. Both are part of the public API and should explain their purpose, safety invariants, and byte layout. [L36-L42] |

### Suggestions

- Make `now()` return `Result` instead of panicking via `.expect()`, allowing callers in the library to handle clock errors gracefully.
  ```typescript
  // Before
  pub fn now() -> Self {
      let duration = SystemTime::now()
          .duration_since(SystemTime::UNIX_EPOCH)
          .expect("system clock before UNIX epoch");
      Self::from_unix(duration.as_secs(), duration.subsec_nanos())
  }
  // After
  pub fn now() -> Result<Self, std::time::SystemTimeError> {
      let duration = SystemTime::now()
          .duration_since(SystemTime::UNIX_EPOCH)?;
      Ok(Self::from_unix(duration.as_secs(), duration.subsec_nanos()))
  }
  ```
- Add `///` documentation to the undocumented public methods `as_bytes` and `from_bytes`.
  ```typescript
  // Before
  pub fn as_bytes(&self) -> &[u8; 12] {
      &self.0
  }
  
  pub fn from_bytes(bytes: [u8; 12]) -> Self {
      Self(bytes)
  }
  // After
  /// Returns the raw 12-byte TAI64N encoding: 8 bytes big-endian TAI seconds
  /// followed by 4 bytes big-endian nanoseconds.
  pub fn as_bytes(&self) -> &[u8; 12] {
      &self.0
  }
  
  /// Reconstruct a `Tai64n` from its 12-byte wire representation.
  /// The caller is responsible for ensuring the bytes are a valid TAI64N value.
  pub fn from_bytes(bytes: [u8; 12]) -> Self {
      Self(bytes)
  }
  ```
