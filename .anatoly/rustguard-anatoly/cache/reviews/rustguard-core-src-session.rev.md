# Review: `rustguard-core/src/session.rs`

**Verdict:** NEEDS_REFACTOR

## Symbols

| Symbol | Kind | Exported | Correction | Over-eng. | Utility | Duplication | Tests | Confidence |
|--------|------|----------|------------|-----------|---------|-------------|-------|------------|
| TransportSession | class | yes | OK | LEAN | USED | UNIQUE | WEAK | 90% |

### Details

#### `TransportSession` (L13–L30)

- **Utility [DEAD]**: Exported struct with 0 runtime importers and 0 type-only importers across all files. Pre-computed analysis confirms no external modules import this symbol.
- **Duplication [UNIQUE]**: No RAG similarity data available; no matching duplicates detected in codebase for this cryptographic transport session struct.
- **Correction [OK]**: The struct definition is correct. Fields are properly typed: keys are [u8; 32], counters are u64, and the replay window is a dedicated type. Zeroize/ZeroizeOnDrop are correctly applied to the key fields while index and counter fields are skipped (acceptable — they contain no secret material). No logic errors, type mismatches, or unsafe operations are present in the struct definition itself.
- **Overengineering [LEAN]**: The struct holds exactly the fields mandated by WireGuard-style transport: two session indices for packet routing, a send key, a receive key, an outgoing nonce counter, and an anti-replay window. Nothing extraneous. The selective `#[zeroize(skip)]` annotations are correctly applied only to non-secret fields (indices, counter, replay window), showing disciplined, purposeful use of the derive macros rather than cargo-cult application. No unnecessary generics, no abstraction layers, no configuration indirection — just the minimal state a bidirectional AEAD transport session requires.
- **Tests [WEAK]**: Inline #[cfg(test)] module provides solid coverage of the core API: encrypt/decrypt roundtrip, counter monotonicity, wrong-counter rejection, anti-replay window, out-of-order delivery, and bidirectional use. However, the two zero-alloc variants — encrypt_to and decrypt_in_place — have zero test coverage despite being public API surface. Nonce exhaustion (send_counter wrapping via checked_add returning None) is also untested. Empty plaintext and truncated/corrupted ciphertext edge cases are absent. The missing coverage of encrypt_to/decrypt_in_place is the most significant gap since those paths exercise a separate code branch (crypto::seal_to / crypto::open_to) that could silently misbehave.
- **DOCUMENTED [DOCUMENTED]**: The struct carries a meaningful type-level `///` block (lines 7–11) that explains its purpose, derivation from a completed handshake, and the initiator/responder key-role convention. Both public fields (`our_index`, `their_index`) have individual `///` comments describing their semantics in the context of the WireGuard index scheme. All private fields also carry `///` comments, which is above the minimum bar. No `# Examples` section is present, but the Rust doc-comment convention only requires that on public functions and methods, not on struct declarations, so this omission does not reduce the rating below DOCUMENTED. (deliberated: reclassified: utility: DEAD → USED, tests: NONE → WEAK, documentation: UNDOCUMENTED → DOCUMENTED — UTILITY DEAD→USED: This is a `pub struct` in a `-core` library crate (`rustguard-core`). Library crate public types are consumed by downstream crates in the workspace (e.g., `rustguard`). The utility evaluator likely only searched within the same crate and missed cross-crate imports, a classic false positive for workspace library crates. Reclassified to USED. TESTS NONE→WEAK: The merged field says NONE but the detail text itself says WEAK, and the source clearly contains 6 test functions in a `#[cfg(test)]` module covering encrypt/decrypt roundtrip, counter monotonicity, wrong-counter rejection, replay rejection, out-of-order delivery, and bidirectional use. The WEAK assessment is accurate: `encrypt_to` and `decrypt_in_place` (separate code paths using `seal_to`/`open_to`) have zero coverage, and nonce exhaustion / edge cases are untested. DOCUMENTATION UNDOCUMENTED→DOCUMENTED: The detail text itself concludes DOCUMENTED. The struct has a thorough `///` block (lines 7–11) explaining purpose, derivation, and key-role convention. Both public fields and all private fields carry `///` comments. Four of six methods have doc comments. Two public items (`new`, `send_counter`) lack doc comments (noted in best_practices rule 9), but the overall coverage is well above UNDOCUMENTED. The struct-level and field-level documentation is comprehensive.)

## Best Practices — 9/10

### Rules

| # | Rule | Status | Severity | Detail |
|---|------|--------|----------|--------|
| 4 | Derive common traits on public types | WARN | MEDIUM | `TransportSession` is a public struct that derives only `Zeroize` and `ZeroizeOnDrop`. `Debug`, `Clone`, and `PartialEq` are absent. Omitting `Clone` and `PartialEq` is defensible for a stateful cryptographic session — cloning would duplicate mutable counters and replay windows with shared state semantics. However, a custom `Debug` impl that redacts key material (printing `[REDACTED]` for `key_send`/`key_recv`) would aid debuggability without leaking secrets. [L11-L12] |
| 9 | Documentation comments on public items | WARN | MEDIUM | The struct and four of its methods carry `///` doc comments. However, `TransportSession::new` and `TransportSession::send_counter` are both `pub` and lack doc comments entirely. [L35-L46, L93-L95] |

### Suggestions

- Add a redacting `Debug` impl for `TransportSession` to aid diagnostics without exposing key material.
  ```typescript
  // Before
  #[derive(Zeroize, ZeroizeOnDrop)]
  pub struct TransportSession { ... }
  // After
  #[derive(Zeroize, ZeroizeOnDrop)]
  pub struct TransportSession { ... }
  
  impl core::fmt::Debug for TransportSession {
      fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
          f.debug_struct("TransportSession")
              .field("our_index", &self.our_index)
              .field("their_index", &self.their_index)
              .field("key_send", &"[REDACTED]")
              .field("key_recv", &"[REDACTED]")
              .field("send_counter", &self.send_counter)
              .finish()
      }
  }
  ```
- Add `///` doc comments to the two undocumented public items: `new` and `send_counter`.
  ```typescript
  // Before
  pub fn new(
      our_index: u32,
      their_index: u32,
      key_send: [u8; 32],
      key_recv: [u8; 32],
  ) -> Self {
  // After
  /// Creates a new `TransportSession` from the keys and index pair
  /// produced by a completed handshake.
  pub fn new(
      our_index: u32,
      their_index: u32,
      key_send: [u8; 32],
      key_recv: [u8; 32],
  ) -> Self {
  ```
- Document `send_counter` to clarify its semantics (next nonce value, not packets sent).
  ```typescript
  // Before
  pub fn send_counter(&self) -> u64 {
      self.send_counter
  }
  // After
  /// Returns the next outgoing nonce counter value.
  /// When this reaches `u64::MAX` the session must be rekeyed.
  pub fn send_counter(&self) -> u64 {
      self.send_counter
  }
  ```
