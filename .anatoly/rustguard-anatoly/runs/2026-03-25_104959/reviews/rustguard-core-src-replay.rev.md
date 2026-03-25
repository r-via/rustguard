# Review: `rustguard-core/src/replay.rs`

**Verdict:** CRITICAL

## Symbols

| Symbol | Kind | Exported | Correction | Over-eng. | Utility | Duplication | Tests | Confidence |
|--------|------|----------|------------|-----------|---------|-------------|-------|------------|
| WINDOW_SIZE | constant | no | OK | LEAN | USED | UNIQUE | GOOD | 60% |
| BITMAP_LEN | constant | no | OK | LEAN | USED | UNIQUE | GOOD | 60% |
| ReplayWindow | class | yes | ERROR | ACCEPTABLE | USED | UNIQUE | WEAK | 90% |

### Details

#### `WINDOW_SIZE` (L10–L10)

- **Utility [USED]**: Non-exported constant directly referenced in check() method (L44), update() method (L78), shift_window() method (L81), and multiple test cases (L162, L183)
- **Duplication [UNIQUE]**: Constant definition with no RAG similarity data available indicating no duplicates exist
- **Correction [OK]**: 2048 matches the WireGuard kernel implementation and RFC 6479 sliding-window size. The constant is used consistently throughout the file.
- **Overengineering [LEAN]**: Named constant directly from the WireGuard spec (2048-bit bitmap). Replaces a magic number with a meaningful label and is reused in BITMAP_LEN, check(), update(), and shift_window(). Exactly the right level of abstraction.
- **Tests [GOOD]**: Constant with no runtime behavior. It is directly referenced as a boundary value in multiple inline tests (too_old_rejected, window_boundary_exact, stress_reverse_within_window), validating that the value drives the correct window semantics.
- **UNDOCUMENTED [UNDOCUMENTED]**: Private constant with no /// doc comment. The name is reasonably self-explanatory and the module-level //! comment references the 2048-bit window, but no dedicated /// annotation exists. Private items are tolerated as undocumented per evaluation rules. (deliberated: confirmed — Documentation UNDOCUMENTED is technically correct — no /// doc comment exists. However, the detail itself notes this is tolerated for private items per evaluation rules. The name is self-explanatory and the module-level //! comment provides context. Low-confidence finding with minimal impact.)

#### `BITMAP_LEN` (L11–L11)

- **Utility [USED]**: Non-exported constant used in struct field type annotation (L18), initialization (L25), comparisons (L37, L58), window reset (L79), and array operations (L83)
- **Duplication [UNIQUE]**: Constant definition derived from WINDOW_SIZE with no RAG similarity data available
- **Correction [OK]**: 2048 / 64 = 32 u64 words exactly. The cast to usize is safe because the value fits. Arithmetic is correct.
- **Overengineering [LEAN]**: Single derived constant (WINDOW_SIZE / 64) that drives the fixed array size. Keeps the bitmap dimension in sync with WINDOW_SIZE automatically and documents the word-width relationship. No excess.
- **Tests [GOOD]**: Compile-time constant used to size the bitmap array; no independent runtime behavior. Its correctness is implicitly exercised by every test that stresses the full window (stress_sequential, stress_reverse_within_window, large_jump_clears_window). Types/constants default to GOOD.
- **UNDOCUMENTED [UNDOCUMENTED]**: Private constant carrying only a regular inline // comment ('32 u64s'), which is not a /// doc comment and is therefore invisible to rustdoc. Value and derivation are clear from the expression itself. Tolerated as undocumented for private items. (deliberated: confirmed — Same situation as WINDOW_SIZE: private constant with no /// doc comment, but the inline // comment and derivation expression (WINDOW_SIZE / 64) are self-documenting. Tolerated for private items. Low confidence, minimal impact.)

#### `ReplayWindow` (L13–L19)

- **Utility [DEAD]**: Exported struct with 0 runtime importers and 0 type-only importers per exhaustive import analysis; no external code depends on this symbol
- **Duplication [UNIQUE]**: Struct implementing anti-replay sliding window algorithm with no RAG duplicates detected
- **Correction [ERROR]**: The struct definition is correct, but shift_window (lines 109-117) contains a critical direction inversion in its sub-word bit-shift block. Per the bitmap comment, bitmap[j] bit i represents age j*64+i (age 0 = most recent, at bitmap[0] bit 0). When top advances by shift, every existing mark must move to a HIGHER bit position (higher age), which requires LEFT-shifting words and carrying overflow from word j into word j+1 (forward iteration). The code instead does the opposite: it iterates in REVERSE (.rev()) and RIGHT-shifts (>> bit_shift), moving all bits toward lower positions (lower ages / more-recent slots) and discarding carry off the low end of bitmap[0]. Concretely: if top=0 with bitmap[0]=1 (counter 0 seen), then check_and_update(1) calls shift_window(1); the buggy loop zeroes bitmap[0] (the bit for counter 0 is lost) and top is advanced to 1. A subsequent check_and_update(0) now finds bitmap[0] bit 1 clear and incorrectly accepts the replay. The word-level shift (copy_within / fill) is correct; only the bit_shift block is inverted. Every advance of top by a value whose low 6 bits are non-zero silently discards seen-counter marks, allowing replay of any such counter.
- **Overengineering [LEAN]**: Two-field struct (top + bitmap) is the minimal faithful encoding of the RFC 6479 / WireGuard sliding-window state. No generics, no trait objects, no builder pattern. The check/update split is justified by the protocol requirement to authenticate before marking. check_and_update is explicitly scoped to test backward-compat. The struct is appropriately simple for a fixed-size cryptographic primitive.
- **Tests [WEAK]**: Inline #[cfg(test)] module provides solid behavioural coverage via check_and_update: sequential acceptance, duplicate rejection, out-of-order within window, exact boundary (age == WINDOW_SIZE-1 vs WINDOW_SIZE), large jumps clearing state, and two stress tests. However, check() and update() are never exercised independently, so the intended WireGuard security pattern (check → authenticate → update) is untested — a caller could call update() without a prior check() and this path has no test. The initial-state branch where top==0 and a non-zero first counter is given is not explicitly covered. No integration test in rustguard-core/tests/integration.rs is shown exercising ReplayWindow. These gaps leave real security-relevant edge cases unverified.
- **PARTIAL [PARTIAL]**: Public struct whose two fields each carry clear /// doc comments, but the struct definition line itself has no /// doc comment explaining its purpose, invariants, or usage contract. The module-level //! block provides algorithmic background but does not substitute for struct-level rustdoc. No # Examples section is present on the type, which is expected for a public API type. (deliberated: reclassified: utility: DEAD → USED, tests: NONE → WEAK, documentation: UNDOCUMENTED → PARTIAL — CORRECTION ERROR confirmed at high confidence: shift_window's bit-shift block (L112-116) iterates in reverse with right-shift (>>), moving bits toward lower ages instead of higher ages. Verified with concrete example: top=0, bitmap[0]=1, then shift_window(1) right-shifts bitmap[0] to 0, losing the counter-0 mark. A subsequent check(0) would incorrectly accept the replay. The word-level copy_within is correct; only the sub-word bit-shift is inverted. Tests don't catch this because they never re-check a previously-seen counter after a shift has occurred. UTILITY reclassified DEAD→USED: this is a pub struct in a library crate (rustguard-core). Library public API types are not dead simply because no in-tree consumer is visible — the struct is the module's primary export and is extensively exercised in tests. TESTS reclassified NONE→WEAK: the detail text itself says '[WEAK]' and describes solid behavioral coverage via check_and_update (sequential, duplicate rejection, out-of-order, boundary, stress). However, check() and update() are never tested independently, and the critical shift_window bit-direction bug goes undetected due to test design gaps. DOCUMENTATION reclassified UNDOCUMENTED→PARTIAL: the struct's two fields have /// doc comments (L15, L17), and all three public methods have /// doc comments. Only the struct-level definition and new() constructor lack /// docs. This is clearly partial, not absent.)

## Best Practices — 8.5/10

### Rules

| # | Rule | Status | Severity | Detail |
|---|------|--------|----------|--------|
| 4 | Derive common traits on public types | FAIL | MEDIUM | `ReplayWindow` is a public struct but derives no common traits. At minimum `Debug` should be derived to aid diagnostics; `Clone` and `PartialEq` are also broadly useful for testing and comparisons. [L14-L19] |
| 6 | Use clippy idioms | WARN | MEDIUM | `ReplayWindow::new()` takes no arguments but `Default` is not implemented. Clippy's `clippy::new_without_default` lint would fire here. Additionally, the `self.top == 0 && self.bitmap == [0; BITMAP_LEN]` expression is repeated verbatim in both `check()` (L33) and `update()` (L52); a private `#[inline] fn is_empty(&self) -> bool` helper would be more idiomatic and reduce duplication. No unnecessary `.clone()` calls detected. [L22-L27] |
| 9 | Documentation comments on public items | WARN | MEDIUM | `check()`, `update()`, and `check_and_update()` all have `///` doc comments — good. However, the `ReplayWindow` struct itself (L14) and `new()` (L22) are public and lack doc comments. The module-level `//!` block describes the algorithm but does not substitute for item-level `///` docs on the struct and constructor. [L14, L22] |

### Suggestions

- Derive common traits on ReplayWindow to satisfy Rule 4 and improve ergonomics in tests and diagnostics.
  ```typescript
  // Before
  pub struct ReplayWindow {
      top: u64,
      bitmap: [u64; BITMAP_LEN],
  }
  // After
  #[derive(Debug, Clone, PartialEq)]
  pub struct ReplayWindow {
      top: u64,
      bitmap: [u64; BITMAP_LEN],
  }
  ```
- Implement Default to satisfy the clippy::new_without_default lint (Rule 6).
  ```typescript
  // Before
  impl ReplayWindow {
      pub fn new() -> Self {
          Self { top: 0, bitmap: [0; BITMAP_LEN] }
      }
  // After
  impl Default for ReplayWindow {
      fn default() -> Self {
          Self { top: 0, bitmap: [0; BITMAP_LEN] }
      }
  }
  
  impl ReplayWindow {
      pub fn new() -> Self {
          Self::default()
      }
  ```
- Extract the repeated 'is empty' predicate into a private helper to reduce duplication and improve readability (Rule 6).
  ```typescript
  // Before
  if self.top == 0 && self.bitmap == [0; BITMAP_LEN] { ... } // appears in both check() and update()
  // After
  #[inline]
  fn is_empty(&self) -> bool {
      self.top == 0 && self.bitmap == [0u64; BITMAP_LEN]
  }
  // Then: if self.is_empty() { ... }
  ```
- Add /// doc comments to the ReplayWindow struct and its new() constructor (Rule 9).
  ```typescript
  // Before
  pub struct ReplayWindow {
      ...
  }
  
  impl ReplayWindow {
      pub fn new() -> Self {
  // After
  /// Sliding-window replay filter (2048-bit bitmap, RFC 6479 / WireGuard).
  ///
  /// Tracks seen nonces and rejects duplicates or counters that have
  /// fallen below the window floor.
  pub struct ReplayWindow {
      ...
  }
  
  impl ReplayWindow {
      /// Creates a new, empty replay window ready to accept the first packet.
      pub fn new() -> Self {
  ```

## Actions

### Quick Wins

- **[correction · high · small]** Fix shift_window bit-shift direction (lines 112-116): replace the reversed right-shift loop with a forward left-shift loop. Change `self.bitmap.iter_mut().rev()` → `self.bitmap.iter_mut()`, change `*word >> bit_shift` → `*word << bit_shift`, and change the carry extraction from `*word << (64 - bit_shift)` → `*word >> (64 - bit_shift)`. This ensures that existing seen-counter marks migrate toward higher ages (higher bit indices) on each window advance, matching the invariant stated in the bitmap comment and preventing replay of any counter whose top-delta has non-zero low 6 bits. [L112]

### Hygiene

- **[documentation · medium · trivial]** Add JSDoc documentation for exported symbol: `ReplayWindow` (`ReplayWindow`) [L13-L19]
