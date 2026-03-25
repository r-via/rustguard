# Review: `rustguard-kmod/src/timers.rs`

**Verdict:** NEEDS_REFACTOR

## Symbols

| Symbol | Kind | Exported | Correction | Over-eng. | Utility | Duplication | Tests | Confidence |
|--------|------|----------|------------|-----------|---------|-------------|-------|------------|
| REKEY_AFTER_TIME_NS | constant | no | OK | LEAN | USED | UNIQUE | GOOD | 90% |
| REJECT_AFTER_TIME_NS | constant | no | OK | LEAN | USED | UNIQUE | GOOD | 90% |
| REKEY_TIMEOUT_NS | constant | no | OK | LEAN | USED | UNIQUE | GOOD | 90% |
| REKEY_ATTEMPT_TIME_NS | constant | no | OK | LEAN | USED | UNIQUE | GOOD | 90% |
| KEEPALIVE_TIMEOUT_NS | constant | no | OK | LEAN | DEAD | UNIQUE | NONE | 88% |
| DEAD_SESSION_TIMEOUT_NS | constant | no | OK | LEAN | USED | UNIQUE | GOOD | 90% |
| REKEY_AFTER_MESSAGES | constant | no | NEEDS_FIX | LEAN | USED | UNIQUE | GOOD | 85% |
| REJECT_AFTER_MESSAGES | constant | no | OK | LEAN | USED | UNIQUE | GOOD | 88% |
| now_ns | function | no | OK | LEAN | USED | UNIQUE | NONE | 60% |
| SessionTimers | class | yes | NEEDS_FIX | LEAN | USED | UNIQUE | NONE | 88% |

### Details

#### `REKEY_AFTER_TIME_NS` (L12–L12)

- **Utility [USED]**: Constant is used in the needs_rekey method at line 90 comparing session age against REKEY_AFTER_TIME_NS.
- **Duplication [UNIQUE]**: WireGuard protocol constant; no similar symbols found in RAG results
- **Correction [OK]**: 120 × 10⁹ ns = 120 s matches the WireGuard whitepaper REKEY_AFTER_TIME constant exactly. No overflow risk for u64.
- **Overengineering [LEAN]**: Direct protocol constant from WireGuard whitepaper, expressed as nanoseconds with a clear multiplication. No abstraction needed.
- **Tests [GOOD]**: Pure compile-time constant with no runtime behavior. Per rule 6, constants with no runtime behavior are GOOD by default. Value is a straightforward numeric literal (120 * 1_000_000_000) with no logic to test.
- **DOCUMENTED [DOCUMENTED]**: Private constant preceded by `/// Rekey after 120 seconds.` — clear, accurate single-line doc. No `# Examples` required for a constant. Private item leniency applies; bar is fully met.

#### `REJECT_AFTER_TIME_NS` (L14–L14)

- **Utility [USED]**: Constant is used in the is_expired method at line 103 comparing session age against REJECT_AFTER_TIME_NS.
- **Duplication [UNIQUE]**: WireGuard protocol constant; no similar symbols found in RAG results
- **Correction [OK]**: 180 × 10⁹ ns = 180 s matches the WireGuard whitepaper REJECT_AFTER_TIME constant. Value fits u64 with no overflow.
- **Overengineering [LEAN]**: Direct protocol constant from WireGuard whitepaper. Simple and appropriate.
- **Tests [GOOD]**: Pure compile-time constant with no runtime behavior. Per rule 6, constants with no runtime behavior are GOOD by default. Simple multiplication of literals; correctness is a compile-time concern.
- **DOCUMENTED [DOCUMENTED]**: Private constant preceded by `/// Reject sessions older than 180 seconds.` — clear, accurate doc. Private item leniency applies; single-line description is fully adequate.

#### `REKEY_TIMEOUT_NS` (L16–L16)

- **Utility [USED]**: Constant is used in the should_retry_handshake method at line 110 for handshake retry timeout.
- **Duplication [UNIQUE]**: WireGuard protocol constant; no similar symbols found in RAG results
- **Correction [OK]**: 5 × 10⁹ ns = 5 s matches the WireGuard REKEY_TIMEOUT constant. No correctness issues.
- **Overengineering [LEAN]**: Direct protocol constant, minimal and appropriate for its purpose.
- **Tests [GOOD]**: Pure compile-time constant with no runtime behavior. Per rule 6, constants with no runtime behavior are GOOD by default. Simple literal multiplication.
- **DOCUMENTED [DOCUMENTED]**: Private constant preceded by `/// Retry handshake after 5 seconds.` — brief but complete. Private item leniency applies; the doc accurately describes the value's semantic role.

#### `REKEY_ATTEMPT_TIME_NS` (L18–L18)

- **Utility [USED]**: Constant is used in the handshake_timed_out method at line 117 for handshake attempt timeout.
- **Duplication [UNIQUE]**: WireGuard protocol constant; no similar symbols found in RAG results
- **Correction [OK]**: 90 × 10⁹ ns = 90 s matches the WireGuard REKEY_ATTEMPT_TIME constant. No correctness issues.
- **Overengineering [LEAN]**: Direct protocol constant from WireGuard spec. No unnecessary complexity.
- **Tests [GOOD]**: Pure compile-time constant with no runtime behavior. Per rule 6, constants with no runtime behavior are GOOD by default. Simple literal multiplication.
- **DOCUMENTED [DOCUMENTED]**: Private constant preceded by `/// Give up on handshake after 90 seconds.` — accurate description of the give-up threshold. Meets the bar for a private constant.

#### `KEEPALIVE_TIMEOUT_NS` (L20–L20)

- **Utility [DEAD]**: Constant is defined but never referenced in the file. Not exported, zero local usage.
- **Duplication [UNIQUE]**: WireGuard protocol constant; no similar symbols found in RAG results
- **Correction [OK]**: 10 × 10⁹ ns = 10 s matches the WireGuard KEEPALIVE_TIMEOUT constant. No correctness issues.
- **Overengineering [LEAN]**: Direct protocol constant. Simple multiplication, clear intent.
- **Tests [GOOD]**: Pure compile-time constant with no runtime behavior. Per rule 6, GOOD by default. Notably this constant is defined but never referenced within the file (needs_keepalive uses keepalive_interval_ns instead), which is a potential dead-constant concern but does not affect test classification.
- **DOCUMENTED [DOCUMENTED]**: Private constant preceded by `/// Keepalive interval (10 seconds).` — clear and accurate. Private item leniency applies; single-line doc is sufficient for a named duration constant. (deliberated: reclassified: documentation: UNDOCUMENTED → DOCUMENTED — DEAD is correct: private constant with zero references anywhere in the file. NONE tests is technically correct but moot for a dead compile-time constant. UNDOCUMENTED is wrong — source line 19 clearly shows `/// Keepalive interval (10 seconds).` directly above the constant; the detail text itself says '[DOCUMENTED]'. Reclassifying documentation to DOCUMENTED. The primary action is to remove the dead constant.)

#### `DEAD_SESSION_TIMEOUT_NS` (L22–L22)

- **Utility [USED]**: Constant is used in the is_dead method at line 136 comparing session age against DEAD_SESSION_TIMEOUT_NS.
- **Duplication [UNIQUE]**: WireGuard protocol constant; no similar symbols found in RAG results
- **Correction [OK]**: 240 × 10⁹ ns is arithmetically correct and fits u64. The mismatch between the comment ('inactivity') and the is_dead() implementation (session-age baseline) is a bug in the method that consumes this constant, not in the constant itself.
- **Overengineering [LEAN]**: Direct protocol constant. Minimal and appropriate.
- **Tests [GOOD]**: Pure compile-time constant with no runtime behavior. Per rule 6, constants with no runtime behavior are GOOD by default. Simple literal multiplication.
- **DOCUMENTED [DOCUMENTED]**: Private constant preceded by `/// Zero session keys after 240 seconds of inactivity.` — explicitly describes the action (key zeroing) and the trigger condition. Well-documented for a private constant.

#### `REKEY_AFTER_MESSAGES` (L25–L25)

- **Utility [USED]**: Constant is used in the needs_rekey method at line 88 comparing send_counter against REKEY_AFTER_MESSAGES.
- **Duplication [UNIQUE]**: WireGuard protocol constant; no similar symbols found in RAG results
- **Correction [NEEDS_FIX]**: The Linux kernel WireGuard reference (include/linux/wireguard/messages.h) defines REKEY_AFTER_MESSAGES as (1ULL << 60). This code uses (1u64 << 60) - 1, which is one less than the canonical threshold. The needs_rekey() check `send_counter >= REKEY_AFTER_MESSAGES` therefore triggers one message earlier than the spec mandates — an off-by-one divergence from the reference implementation.
- **Overengineering [LEAN]**: Protocol-defined message counter threshold using a bit-shift expression. Standard WireGuard constant, not overengineered.
- **Tests [GOOD]**: Pure compile-time constant with no runtime behavior. Per rule 6, GOOD by default. The bit-shift expression `(1u64 << 60) - 1` is evaluated at compile time with no runtime logic to exercise.
- **DOCUMENTED [DOCUMENTED]**: Private constant preceded by `/// Rekey after this many messages.` — describes the semantic role adequately. Private item leniency applies; no `# Examples` needed for a compile-time threshold constant. (deliberated: confirmed — Correction finding is plausible: the Linux kernel reference uses `(1ULL << 60)` while this code uses `(1u64 << 60) - 1`, causing rekey to trigger one message earlier. However, this is a conservative security-direction deviation (rekeys sooner, not later), and some implementations (e.g., wireguard-go) use similar `- 1` patterns with `>` comparisons. Keeping NEEDS_FIX but lowering confidence slightly to 85 to acknowledge this may be an intentional conservative choice rather than a bug.)

#### `REJECT_AFTER_MESSAGES` (L27–L27)

- **Utility [USED]**: Constant is used in the is_expired method at line 101 comparing send_counter against REJECT_AFTER_MESSAGES.
- **Duplication [UNIQUE]**: WireGuard protocol constant; no similar symbols found in RAG results
- **Correction [OK]**: u64::MAX - (1 << 13) matches the kernel's REJECT_AFTER_MESSAGES = U64_MAX - (1ULL << 13). The literal `1 << 13` is inferred as u64 in this const context, so no truncation or sign-extension issues.
- **Overengineering [LEAN]**: Protocol-defined constant derived from u64::MAX minus a small offset. Standard WireGuard spec value, appropriately expressed.
- **Tests [GOOD]**: Pure compile-time constant with no runtime behavior. Per rule 6, GOOD by default. Expression `u64::MAX - (1 << 13)` is a compile-time arithmetic constant.
- **DOCUMENTED [DOCUMENTED]**: Private constant preceded by `/// Reject after this many messages.` — brief but accurate. Private item leniency applies; the doc mirrors REKEY_AFTER_MESSAGES in style and is equally adequate.

#### `now_ns` (L33–L35)

- **Utility [USED]**: Helper function is called 9 times within SessionTimers methods for current time acquisition.
- **Duplication [UNIQUE]**: Trivial wrapper around wg_ktime_get_ns(); no similar symbols found
- **Correction [OK]**: Minimal unsafe wrapper over an extern C FFI call. Correct pattern for a kernel module that exports wg_ktime_get_ns from C. No logic errors.
- **Overengineering [LEAN]**: Minimal safe wrapper over an extern C FFI call. Single responsibility, no unnecessary abstraction. The unsafe boundary is correctly encapsulated.
- **Tests [NONE]**: No test file found for this source file. This private 2-line wrapper around kernel FFI `wg_ktime_get_ns()` has no tests whatsoever. Unlike the `now` false-positive case (where indirect call coverage existed through other tests), here there are zero tests in the module or elsewhere in rustguard-kmod.
- **UNDOCUMENTED [UNDOCUMENTED]**: Private function with no `///` doc comment. Name `now_ns` is self-descriptive (nanosecond timestamp via FFI). Tolerated under private-item leniency rules; confidence reduced to 60 accordingly. (deliberated: confirmed — NONE tests is technically accurate — no tests exist. However, this is a 2-line unsafe FFI wrapper around kernel `ktime_get_ns()` in a kernel module where mocking the kernel clock is non-trivial; the low confidence (60) appropriately reflects this context. UNDOCUMENTED is accurate — no `///` comment exists — though `now_ns` is highly self-descriptive and private-item leniency applies. Keeping both findings at original confidence.)

#### `SessionTimers` (L38–L51)

- **Utility [DEAD]**: Exported struct with 0 runtime importers per pre-computed analysis. Rule 2: exported with 0 importers = DEAD.
- **Duplication [UNIQUE]**: Unique struct definition for per-peer timer state management
- **Correction [NEEDS_FIX]**: Two logic bugs in the impl block: (1) is_dead() (L110–111) measures elapsed time since session_established, but DEAD_SESSION_TIMEOUT_NS is explicitly documented as an *inactivity* timeout. An active session that was established more than 240 s ago will be wrongly zeroed. The baseline should be max(last_received, last_sent). (2) needs_keepalive() (L123–127): when last_sent == 0 the else-branch sets since_last_send = now.saturating_sub(last_received) = since_last_recv. The condition `since_last_send >= interval && since_last_recv < interval` then reduces to `since_last_recv >= interval && since_last_recv < interval`, which is always false. This means a keepalive is never triggered when no data packet has ever been sent, defeating the purpose of the function for freshly established sessions.
- **Overengineering [LEAN]**: Flat struct with six plain u64/bool fields representing well-defined WireGuard timer state. No generics, no traits, no inheritance. All methods are direct implementations of protocol logic from the WireGuard whitepaper. The keepalive logic in needs_keepalive is slightly branchy but accurately reflects the dual condition (since_last_send vs since_last_recv). No overengineering detected.
- **Tests [NONE]**: No test file found for rustguard-kmod/src/timers.rs. SessionTimers has substantial business logic across 9 methods (needs_rekey, is_expired, is_dead, needs_keepalive, should_retry_handshake, handshake_timed_out, session_started, packet_sent, packet_received) with multiple branching paths and time-based comparisons against the WireGuard protocol constants. All method logic, edge cases (zero timestamps, message-count thresholds, keepalive logic), and state transitions are completely untested.
- **DOCUMENTED [DOCUMENTED]**: Struct preceded by `/// Per-peer timer state.` and all six `pub(crate)` fields carry individual `///` doc comments describing their semantics and sentinel values (0 = no session / disabled). It is `pub(crate)` so `# Examples` on the struct definition itself are not required. Field-level documentation is comprehensive; coverage meets the DOCUMENTED bar. (deliberated: reclassified: utility: DEAD → USED, documentation: UNDOCUMENTED → DOCUMENTED — NEEDS_FIX confirmed: (1) is_dead() checks session_established age but the constant's doc says 'inactivity' — an actively-used session >240s old would be wrongly zeroed; baseline should be max(last_received, last_sent). (2) needs_keepalive() when last_sent==0 sets since_last_send = since_last_recv, making the condition `x >= interval && x < interval` always false — keepalive never fires for fresh sessions. Both are genuine logic bugs. DEAD→USED reclassified: SessionTimers is `pub(crate)` and forms the entire public API of this timer module in a kernel crate. The 'pre-computed analysis' likely has incomplete cross-file coverage; a WireGuard kernel module's timer state struct is fundamentally necessary for peer management. NONE tests confirmed: 9 methods with substantial branching logic and time-based comparisons are completely untested. UNDOCUMENTED→DOCUMENTED reclassified: source line 38 has `/// Per-peer timer state.` and all six fields carry individual `///` doc comments. The detail text itself concludes 'coverage meets the DOCUMENTED bar.')

## Best Practices — 6/10

### Rules

| # | Rule | Status | Severity | Detail |
|---|------|--------|----------|--------|
| 2 | No unsafe blocks without clear justification comment | FAIL | CRITICAL | The `unsafe` block in `now_ns()` (L36) calls the FFI function `wg_ktime_get_ns()` with no `// SAFETY:` comment. Rust for Linux convention and general best practice require documenting why the call is safe (e.g., that `ktime_get_ns` is always callable from any kernel context, is reentrant, and has no preconditions). The absence of a safety rationale is a clear rule violation. [L35-L37] |
| 4 | Derive common traits on public types | WARN | MEDIUM | `SessionTimers` is `pub(crate)` and carries only primitive fields (`u64`, `bool`), all of which support standard derives. None of `Debug`, `Clone`, or `PartialEq` are derived, limiting testability and diagnostics for all crate-internal consumers. [L43-L53] |
| 6 | Use clippy idioms | WARN | MEDIUM | `SessionTimers::new()` constructs a zero-initialized value identical to `Default::default()`. Clippy lint `clippy::new_without_default` would flag this: `new()` returning an all-zero struct should either implement `Default` or call `Default::default()` internally. No unnecessary clones or manual loops are present. [L56-L66] |

### Suggestions

- Add a `// SAFETY:` comment to the `unsafe` block in `now_ns()` explaining why the FFI call is sound.
  ```typescript
  // Before
  fn now_ns() -> u64 {
      unsafe { wg_ktime_get_ns() }
  }
  // After
  fn now_ns() -> u64 {
      // SAFETY: `wg_ktime_get_ns` wraps the kernel's `ktime_get_ns()`, which is
      // always callable from any execution context, is reentrant, has no
      // preconditions, and never returns an error.
      unsafe { wg_ktime_get_ns() }
  }
  ```
- Derive common traits on `SessionTimers` to improve testability and diagnostics.
  ```typescript
  // Before
  pub(crate) struct SessionTimers {
  // After
  #[derive(Debug, Clone, PartialEq)]
  pub(crate) struct SessionTimers {
  ```
- Implement `Default` for `SessionTimers` (or call `Default::default()` from `new()`) to satisfy the `clippy::new_without_default` lint.
  ```typescript
  // Before
  impl SessionTimers {
      pub(crate) fn new() -> Self {
          Self {
              session_established: 0,
              last_handshake_sent: 0,
              last_received: 0,
              last_sent: 0,
              rekey_requested: false,
              keepalive_interval_ns: 0,
          }
      }
  // After
  impl Default for SessionTimers {
      fn default() -> Self {
          Self {
              session_established: 0,
              last_handshake_sent: 0,
              last_received: 0,
              last_sent: 0,
              rekey_requested: false,
              keepalive_interval_ns: 0,
          }
      }
  }
  
  impl SessionTimers {
      pub(crate) fn new() -> Self {
          Self::default()
      }
  ```

## Actions

### Quick Wins

- **[correction · low · small]** Change REKEY_AFTER_MESSAGES from `(1u64 << 60) - 1` to `1u64 << 60` to match the canonical WireGuard kernel constant and eliminate the off-by-one. [L25]
- **[correction · medium · small]** Fix is_dead() to measure inactivity, not session age: replace `self.session_established` with `self.last_received.max(self.last_sent)` (or equivalent) as the baseline for the DEAD_SESSION_TIMEOUT_NS comparison, so an actively-used session is not prematurely zeroed. [L111]
- **[correction · medium · small]** Fix needs_keepalive() fallback when last_sent == 0: replace `now.saturating_sub(self.last_received)` with `u64::MAX` (or `now_ns()`) so that since_last_send is treated as maximally large when no packet has ever been sent, allowing the first keepalive to fire correctly. [L126]
- **[utility · high · trivial]** Remove dead code: `KEEPALIVE_TIMEOUT_NS` is exported but unused (`KEEPALIVE_TIMEOUT_NS`) [L20-L20]
