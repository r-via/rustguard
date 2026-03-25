# Review: `rustguard-enroll/src/state.rs`

**Verdict:** NEEDS_REFACTOR

## Symbols

| Symbol | Kind | Exported | Correction | Over-eng. | Utility | Duplication | Tests | Confidence |
|--------|------|----------|------------|-----------|---------|-------------|-------|------------|
| PersistedPeer | class | yes | OK | LEAN | USED | UNIQUE | GOOD | 92% |
| default_state_path | function | yes | OK | LEAN | USED | UNIQUE | NONE | 90% |
| save | function | yes | OK | LEAN | USED | UNIQUE | WEAK | 92% |
| load | function | yes | OK | LEAN | USED | UNIQUE | WEAK | 92% |

### Details

#### `PersistedPeer` (L13–L16)

- **Utility [DEAD]**: Exported struct with 0 runtime importers and 0 type-only importers; rule 2 applies.
- **Duplication [UNIQUE]**: Domain-specific struct for enrolled peer state; no similar types found in codebase
- **Correction [OK]**: Plain data struct with Clone derive. Fields are correctly typed ([u8;32] for a WireGuard public key, Ipv4Addr for the assigned address). No logic, no correctness concerns.
- **Overengineering [LEAN]**: Minimal two-field struct holding exactly the data needed to re-enroll a peer. No unnecessary generics, traits, or indirection. Derives only Clone, which is justified for collection manipulation.
- **Tests [GOOD]**: Plain data struct with no runtime behavior. Per rule 6, GOOD by default. Both inline tests directly construct and assert on PersistedPeer instances, exercising both public fields (public_key and assigned_ip).
- **PARTIAL [PARTIAL]**: Struct carries a /// doc comment ('A persisted peer — just enough to re-enroll on restart.') but neither public field (public_key, assigned_ip) has a field-level /// comment, and no # Examples section is present. (deliberated: reclassified: utility: DEAD → USED, tests: NONE → GOOD, documentation: UNDOCUMENTED → PARTIAL — Utility DEAD is a false positive: PersistedPeer is a pub struct in a library crate (rustguard-enroll) intended to be consumed by other workspace crates. Zero in-file importers is expected for a public API type — the evaluator only searched within the crate boundary. Tests reclassified from NONE to GOOD: the detail text itself states 'Both inline tests directly construct and assert on PersistedPeer instances, exercising both public fields' and concludes GOOD for a plain data struct — the NONE field value contradicts the detail. Documentation reclassified from UNDOCUMENTED to PARTIAL: source line 11 clearly shows `/// A persisted peer — just enough to re-enroll on restart.` and best_practices rule 9 confirms 'All public items carry /// doc comments'. Field-level docs are absent but struct-level doc exists.)

#### `default_state_path` (L19–L21)

- **Utility [DEAD]**: Exported function with 0 runtime importers and 0 type-only importers; rule 2 applies.
- **Duplication [UNIQUE]**: Trivial 3-line function returning hardcoded state path; RAG found no similar functions
- **Correction [OK]**: Returns a hardcoded PathBuf. No computation, no possible runtime error. Correct.
- **Overengineering [LEAN]**: Single-expression function returning a hardcoded PathBuf. Wrapping the literal in a named function is appropriate for testability and for giving callers a discoverable default without duplicating the string.
- **Tests [NONE]**: No inline or external test ever calls default_state_path(). Both inline tests construct their own custom temp paths and never invoke this function. It is a trivial one-liner but has zero direct test coverage.
- **PARTIAL [PARTIAL]**: Has a single-line /// doc ('Default state file path.') that conveys the basic intent, but lacks a # Examples section expected on a public function and omits any note about the hardcoded path or platform assumptions. (deliberated: reclassified: utility: DEAD → USED, documentation: UNDOCUMENTED → PARTIAL — Utility DEAD is a false positive: pub function in a library crate, designed for external consumers to call for the default path. No in-crate callers is expected — tests use temp dirs instead, and the binary crate would call this. Tests NONE is accurate: no test directly invokes this function, though for a trivial one-liner the risk is minimal. Documentation reclassified from UNDOCUMENTED to PARTIAL: source line 19 shows `/// Default state file path.` — the doc exists but is minimal (no examples, no platform notes).)

#### `save` (L25–L45)

- **Utility [DEAD]**: Exported function with 0 runtime importers and 0 type-only importers; rule 2 applies.
- **Duplication [UNIQUE]**: Specialized state persistence function with atomic write pattern; RAG found no similar functions
- **Correction [OK]**: The atomic-write pattern (write to .tmp then rename) is correct; both paths share the same parent directory so the rename is same-filesystem. base64::prelude::BASE64_STANDARD encodes [u8;32] without spaces, so the 'key ip' line format round-trips cleanly. path.parent() returning Some("") for a bare filename is harmless because create_dir_all("") is a no-op on Linux/macOS. If write succeeds but rename fails the .tmp file is orphaned but no data is lost or corrupted — acceptable for this pattern. No correctness bugs found.
- **Overengineering [LEAN]**: Three logical steps: ensure directory, build text content, atomic rename-write. The atomic write pattern (write-to-tmp then rename) is idiomatic and necessary for a state file to survive crashes. No unnecessary abstractions or configuration layers. Module doc claims JSON format but the implementation correctly uses a simpler line-based format — a doc inconsistency, not overengineering.
- **Tests [WEAK]**: Covered only by the save_and_load_roundtrip happy-path test with two peers. Missing: saving an empty slice, failure when the parent directory cannot be created, verification that the atomic tmp-then-rename write works correctly (e.g. no leftover .tmp file), and any I/O error path. Edge cases around the format (e.g. keys with unusual base64 padding) are untested.
- **PARTIAL [PARTIAL]**: Two-line /// block describes purpose and the line format ('base64_pubkey ip'), which is useful, but there are no parameter descriptions, no # Errors section documenting the io::Result failure modes (e.g., directory creation, atomic rename), and no # Examples. (deliberated: reclassified: utility: DEAD → USED, tests: NONE → WEAK, documentation: UNDOCUMENTED → PARTIAL — Utility DEAD is a false positive: pub function in a library crate meant for external consumers. Tests reclassified from NONE to WEAK: the detail text itself says 'Covered only by the save_and_load_roundtrip happy-path test with two peers' — the function IS called in the test at line 108, so NONE is factually incorrect. WEAK is appropriate given only the happy path is covered and error paths are untested. Documentation reclassified from UNDOCUMENTED to PARTIAL: lines 24-25 show a two-line /// doc comment describing purpose and format. It exists but lacks #Errors and parameter docs.)

#### `load` (L48–L84)

- **Utility [DEAD]**: Exported function with 0 runtime importers and 0 type-only importers; rule 2 applies.
- **Duplication [UNIQUE]**: Specialized state deserialization from custom format; RAG found no similar functions
- **Correction [OK]**: NotFound is correctly short-circuited to an empty vec. Lines are trimmed and empty lines are skipped, so the trailing newline appended by save produces no spurious entry. split_once(' ') is safe because BASE64_STANDARD output never contains spaces. key_bytes.try_into::<[u8;32]>() correctly rejects any key that is not exactly 32 bytes. ip_str.parse::<Ipv4Addr>() properly rejects malformed addresses. Error messages propagate with InvalidData kind. No correctness bugs found.
- **Overengineering [LEAN]**: Straightforward line-by-line parser with proper not-found short-circuit and clear per-field error mapping. Error conversions are direct and necessary for meaningful diagnostics; no trait objects, no intermediate parser types, no builder patterns. Length is driven by thorough error handling, not structural complexity.
- **Tests [WEAK]**: Two tests exist: save_and_load_roundtrip (happy path with 2 peers) and load_nonexistent_returns_empty (NotFound branch). The three InvalidData error paths — malformed line (no space separator), invalid base64 key, and key with wrong byte length (not 32) — are entirely untested. The invalid-IP error path is also untested. Empty/whitespace-only line handling is exercised only incidentally via the format produced by save, not via an explicit edge-case test.
- **PARTIAL [PARTIAL]**: Single-line /// doc ('Load peers from the state file.') is present but minimal. Missing parameter docs, # Errors section (notably the silent Ok(vec![]) on NotFound is a meaningful behavior worth documenting), and # Examples. (deliberated: reclassified: utility: DEAD → USED, tests: NONE → WEAK, documentation: UNDOCUMENTED → PARTIAL — Utility DEAD is a false positive: pub function in a library crate for external consumption. Tests reclassified from NONE to WEAK: both test functions call load() — save_and_load_roundtrip (line 109) and load_nonexistent_returns_empty (line 116). The detail text itself describes WEAK coverage with specific gaps (error paths untested). NONE is factually wrong. Documentation reclassified from UNDOCUMENTED to PARTIAL: line 48 shows `/// Load peers from the state file.` — the doc exists but is minimal, missing #Errors section and the important NotFound → Ok(vec![]) behavior.)

## Best Practices — 9/10

### Rules

| # | Rule | Status | Severity | Detail |
|---|------|--------|----------|--------|
| 4 | Derive common traits on public types | WARN | MEDIUM | `PersistedPeer` is a public struct that derives only `Clone`. It is missing `Debug` (needed for logging/debugging) and `PartialEq` (useful for tests and comparisons). The test at L99 manually compares individual fields rather than the struct directly, which would be cleaner with `PartialEq`. [L13-L17] |
| 6 | Clippy idioms | WARN | MEDIUM | Line 38 uses `content.push_str(&format!(...))`, which Clippy flags via `clippy::format_push_string` because it allocates an intermediate `String` before pushing. The idiomatic alternative is to use `use std::fmt::Write; write!(content, ...).unwrap()` (in test context acceptable) or collect an iterator of formatted strings. This is a minor but real lint violation. [L36-L39] |

### Suggestions

- Add missing trait derives to `PersistedPeer` for ergonomics, testability, and logging
  ```typescript
  // Before
  #[derive(Clone)]
  pub struct PersistedPeer {
      pub public_key: [u8; 32],
      pub assigned_ip: Ipv4Addr,
  }
  // After
  #[derive(Debug, Clone, PartialEq)]
  pub struct PersistedPeer {
      pub public_key: [u8; 32],
      pub assigned_ip: Ipv4Addr,
  }
  ```
- Replace `push_str(&format!(...))` with the `write!` macro to avoid an intermediate allocation (fixes `clippy::format_push_string`)
  ```typescript
  // Before
  use std::fmt::Write; // add this
  // ...
  let key = BASE64_STANDARD.encode(peer.public_key);
  content.push_str(&format!("{} {}\n", key, peer.assigned_ip));
  // After
  use std::fmt::Write;
  // ...
  let key = BASE64_STANDARD.encode(peer.public_key);
  write!(content, "{} {}\n", key, peer.assigned_ip)
      .expect("String write is infallible");
  ```
- With `PartialEq` derived, the roundtrip test can compare whole structs instead of individual fields
  ```typescript
  // Before
  assert_eq!(loaded[0].public_key, [0x42; 32]);
  assert_eq!(loaded[0].assigned_ip, Ipv4Addr::new(10, 150, 0, 2));
  // After
  assert_eq!(loaded[0], PersistedPeer { public_key: [0x42; 32], assigned_ip: Ipv4Addr::new(10, 150, 0, 2) });
  ```
