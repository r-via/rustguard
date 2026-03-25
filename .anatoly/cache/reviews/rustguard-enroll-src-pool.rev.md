# Review: `rustguard-enroll/src/pool.rs`

**Verdict:** NEEDS_REFACTOR

## Symbols

| Symbol | Kind | Exported | Correction | Over-eng. | Utility | Duplication | Tests | Confidence |
|--------|------|----------|------------|-----------|---------|-------------|-------|------------|
| IpPool | class | yes | NEEDS_FIX | LEAN | USED | UNIQUE | WEAK | 90% |

### Details

#### `IpPool` (L9–L20)

- **Utility [USED]**: Exported pub struct in library crate rustguard-enroll. Zero in-crate importers is a known false positive for library public APIs (see deliberation overturns for TransportSession, Peer, is_open). This is the primary module export with multiple pub methods, pub fields, and comprehensive test coverage. Would be consumed by downstream workspace crates for peer IP pool management.
- **Duplication [UNIQUE]**: No RAG data available; no semantic duplicates detected in the codebase. This is the sole IP pool management structure for dynamic peer address assignment.
- **Correction [NEEDS_FIX]**: The struct definition itself is correct, but the `contains` method in the impl block computes `u32::MAX << (32 - self.prefix_len)` without guarding against `prefix_len == 0`. The `new` constructor explicitly special-cases this (`if prefix_len == 0 { 0 } else { u32::MAX << (32 - prefix_len) }`), but `contains` omits the guard. Since `new` accepts `prefix_len` in `0..=30`, an `IpPool` with `prefix_len == 0` is constructable. Calling `contains` on it attempts to shift a `u32` left by 32, which panics in debug mode (shift-overflow) and returns an incorrect mask in release mode on x86 (the shift amount is masked to 5 bits, so 32 % 32 = 0, leaving `mask = u32::MAX` instead of `0`), causing `contains` to return the wrong result for every address.
- **Overengineering [LEAN]**: The struct carries exactly five fields, each with a clear purpose: `network` (u32 for efficient bitmasking), `prefix_len` (CIDR notation), `capacity` (precomputed mask complement to avoid hot-path recalculation), `assigned` (HashSet for O(1) membership checks), and `server_addr` (cached for O(1) guard in `release`). No unnecessary generics, no trait abstraction, no builder pattern — just a minimal, purpose-fit data structure for a /n CIDR pool. The impl methods map 1-to-1 onto genuine operations (allocate, release, contains, restore). Linear scan in `allocate` is appropriate for typical WireGuard subnet sizes (/24 or smaller).
- **Tests [WEAK]**: Five inline #[cfg(test)] tests cover the primary behaviours: sequential allocation, release-and-reuse, /30 exhaustion, /29 exhaustion, contains() membership, and server-address release protection. However, several important paths are untested: (1) IpPool::new() returning None when prefix_len > 30 is never exercised; (2) allocate_specific() has zero test coverage despite being a public method used for restoring persisted peers; (3) assigned_count() is never called in any test; (4) the address-normalization logic in new() (net & mask) is not tested with an unnormalized input; (5) contains() is only verified for a /24 and never checks boundary addresses (network address itself, broadcast). Happy-path and release-cycle coverage is solid, but the missing error-path and method coverage makes this WEAK.
- **PARTIAL [PARTIAL]**: All five fields carry individual `///` doc comments (lines 10, 12, 14, 16, 18), which is thorough field-level coverage. However, the struct type itself at line 9 has no preceding `///` doc comment — only module-level `//!` comments exist above it. Per the evaluation rule, DOCUMENTED requires `///` on the type *and* on each public field/variant. The missing struct-level `///` and the absence of an `# Examples` section keep this at PARTIAL. (deliberated: confirmed — Correction NEEDS_FIX confirmed: The `contains` method at L81 computes `u32::MAX << (32 - self.prefix_len)` without the `prefix_len == 0` guard that `new` has at L32-36. Since `new` allows prefix_len 0 (only rejects >30), constructing an IpPool with prefix_len=0 and then calling `contains` will shift u32 left by 32, which panics in debug and produces incorrect results in release. This is a real, verifiable bug in the source. Tests WEAK confirmed: `allocate_specific()` and `assigned_count()` have zero test coverage, `new()` returning None for invalid prefix_len is untested, and `contains()` boundary addresses are unchecked — these are genuine gaps in a public API. Documentation PARTIAL confirmed: The struct at L9 has no `///` doc comment; only module-level `//!` comments exist (L1-4), which do not render as type-level documentation in rustdoc. All fields have `///` comments but the type itself does not.)

## Best Practices — 9/10

### Rules

| # | Rule | Status | Severity | Detail |
|---|------|--------|----------|--------|
| 4 | Derive common traits on public types | WARN | MEDIUM | The public struct IpPool (L11) derives no standard traits. At minimum Debug should be derived for diagnostics and usability. Clone and PartialEq are feasible since all fields implement those traits (u32, u8, HashSet<Ipv4Addr>). Absence of Debug makes it impossible to use IpPool in format strings, log messages, or test assertion output. [L11-L21] |
| 9 | Documentation comments on public items | WARN | MEDIUM | All six public methods (new, allocate, allocate_specific, release, contains, assigned_count) and the two public fields (prefix_len, server_addr) carry /// doc comments. However, the public struct IpPool itself (L11) is missing a /// doc comment. The module-level //! comment (L1-L5) describes the pool conceptually but does not substitute for a struct-level doc that would appear in rustdoc output for the type. [L11] |

### Suggestions

- Add standard trait derives to IpPool for diagnostics, clonability, and equality checks
  ```typescript
  // Before
  pub struct IpPool {
      /// Network address (e.g. 10.150.0.0).
      network: u32,
  // After
  #[derive(Debug, Clone)]
  pub struct IpPool {
      /// Network address (e.g. 10.150.0.0).
      network: u32,
  ```
- Add a /// doc comment directly on the IpPool struct so rustdoc renders it as the type summary
  ```typescript
  // Before
  pub struct IpPool {
  // After
  /// Manages a CIDR-scoped IPv4 address pool for dynamic WireGuard peer assignment.
  ///
  /// The first host address (`.1`) is reserved for the server and is never returned
  /// by [`IpPool::allocate`].
  pub struct IpPool {
  ```

## Actions

### Quick Wins

- **[correction · low · small]** Mirror the prefix_len == 0 guard from `new` inside `contains`: `let mask = if self.prefix_len == 0 { 0 } else { u32::MAX << (32 - self.prefix_len) };` to prevent a shift-overflow panic in debug mode and incorrect results in release mode. [L74]

### Hygiene

- **[documentation · low · trivial]** Complete JSDoc documentation for: `IpPool` (`IpPool`) [L9-L20]
