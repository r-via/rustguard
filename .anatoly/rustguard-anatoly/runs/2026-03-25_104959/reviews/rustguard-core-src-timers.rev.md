# Review: `rustguard-core/src/timers.rs`

**Verdict:** NEEDS_REFACTOR

## Symbols

| Symbol | Kind | Exported | Correction | Over-eng. | Utility | Duplication | Tests | Confidence |
|--------|------|----------|------------|-----------|---------|-------------|-------|------------|
| now | function | no | OK | LEAN | USED | UNIQUE | WEAK | 70% |
| elapsed_since | function | no | NEEDS_FIX | LEAN | USED | UNIQUE | NONE | 82% |
| elapsed_since | function | no | NEEDS_FIX | LEAN | USED | UNIQUE | NONE | 82% |
| REKEY_AFTER_TIME | constant | yes | OK | LEAN | USED | UNIQUE | WEAK | 85% |
| REJECT_AFTER_TIME | constant | yes | OK | LEAN | USED | UNIQUE | WEAK | 85% |
| REKEY_TIMEOUT | constant | yes | NEEDS_FIX | LEAN | USED | UNIQUE | WEAK | 85% |
| REKEY_AFTER_MESSAGES | constant | yes | OK | LEAN | USED | UNIQUE | GOOD | 75% |
| REJECT_AFTER_MESSAGES | constant | yes | OK | LEAN | USED | UNIQUE | GOOD | 90% |
| REKEY_ATTEMPT_TIME | constant | yes | OK | LEAN | USED | UNIQUE | NONE | 88% |
| KEEPALIVE_TIMEOUT | constant | yes | OK | LEAN | LOW_VALUE | UNIQUE | NONE | 88% |
| DEAD_SESSION_TIMEOUT | constant | yes | OK | LEAN | USED | UNIQUE | WEAK | 85% |
| SessionTimers | class | yes | NEEDS_FIX | LEAN | USED | UNIQUE | WEAK | 90% |

### Details

#### `now` (L16–L18)

- **Utility [USED]**: Called locally in session_started (L79), packet_sent (L89), packet_received (L96). Internal-only function with clear local usage.
- **Duplication [UNIQUE]**: Trivial single-purpose function. No similar implementations found in codebase.
- **Correction [OK]**: Correctly delegates to std::time::Instant::now() under the std feature flag. No correctness issues.
- **Overengineering [LEAN]**: Minimal one-liner wrapper around std::time::Instant::now(), needed solely to enable the cfg-gated Timestamp abstraction used throughout the file. No unnecessary indirection.
- **Tests [WEAK]**: Indirectly invoked by session_started(), packet_sent(), and packet_received() in tests, but no assertion ever verifies a timing side-effect. The function is a trivial Instant::now() wrapper, yet the tests only confirm boolean state transitions, not that a timestamp was actually recorded.
- **UNDOCUMENTED [UNDOCUMENTED]**: Private function with no `///` doc comment. Applying leniency for private visibility; the name is self-explanatory and its purpose is implicitly covered by the `Timestamp` type alias comment above. (deliberated: confirmed — Tests WEAK: valid — now() is only exercised indirectly and no test ever verifies a timestamp was stored. Documentation UNDOCUMENTED: technically correct for a private one-liner, but the name is entirely self-descriptive and leniency for private visibility applies. Both findings stand but confidence raised slightly — the function is trivially correct and the findings are minor.)

#### `elapsed_since` (L21–L23)

- **Utility [USED]**: no_std variant implementing the same interface as std version. Used identically when feature=std is disabled, enabling kernel module support.
- **Duplication [DUPLICATE]**: No_std fallback implementation. Identical to cookie.rs no_std variant with semantic similarity score 1.000.
- **Correction [NEEDS_FIX]**: Returns Duration::ZERO unconditionally in no_std builds. Every time-based query method on SessionTimers — needs_rekey, is_expired, is_dead, needs_keepalive, should_retry_handshake, and handshake_timed_out — calls this function. In a no_std build, all elapsed comparisons evaluate to '0 >= threshold', which is false for every positive threshold, silently disabling session expiry, rekey triggering, dead-session detection, keepalive generation, and handshake retry logic. The doc comment states 'Kernel callers must use the explicit check methods with their own clock', but no no_std-aware variants of those six query methods exist in the impl block, leaving the no_std time-based path entirely broken.
- **Overengineering [LEAN]**: Necessary no_std stub returning Duration::ZERO with a clear comment directing kernel callers to override. Minimal and purposeful; the correct approach for a crate targeting both std and embedded/kernel environments.
- **Tests [NONE]**: This is the #[cfg(not(feature="std"))] variant. All tests run under the std feature, so this code path is never compiled or executed by the test suite. There are zero test cases for no_std behavior.
- **UNDOCUMENTED [UNDOCUMENTED]**: Private no_std fallback with explanatory `//` inline comments but no `///` doc comment. Inline comments do explain the zero-return sentinel behavior for kernel callers; lenient for private visibility. (deliberated: reclassified: duplication: DUPLICATE → UNIQUE — This is the no_std variant at L26-30. Correction NEEDS_FIX: the concern is legitimate — returning Duration::ZERO makes all time-based query methods (needs_rekey, is_expired, is_dead, needs_keepalive, should_retry_handshake, handshake_timed_out) silently non-functional in no_std builds. While the inline comment acknowledges the design, no _at() query variants exist for downstream kernel callers, leaving the no_std time path incomplete. Keep NEEDS_FIX. Duplication: two private cfg-gated stubs returning Duration::ZERO in different modules is a standard Rust no_std pattern, not meaningful duplication. Extracting to a shared module would create unnecessary coupling → reclassify to UNIQUE. Tests NONE: correct, no_std path is never compiled in tests. Documentation UNDOCUMENTED: fair for a private function, inline comments partially explain intent.)

> **Duplicate of** `rustguard-core/src/cookie.rs:elapsed_since` — Perfect match - both return Duration::ZERO for no_std builds as kernel-level fallback

#### `elapsed_since` (L26–L30)

- **Utility [USED]**: no_std variant implementing the same interface as std version. Used identically when feature=std is disabled, enabling kernel module support.
- **Duplication [DUPLICATE]**: No_std fallback implementation. Identical to cookie.rs no_std variant with semantic similarity score 1.000.
- **Correction [NEEDS_FIX]**: Returns Duration::ZERO unconditionally in no_std builds. Every time-based query method on SessionTimers — needs_rekey, is_expired, is_dead, needs_keepalive, should_retry_handshake, and handshake_timed_out — calls this function. In a no_std build, all elapsed comparisons evaluate to '0 >= threshold', which is false for every positive threshold, silently disabling session expiry, rekey triggering, dead-session detection, keepalive generation, and handshake retry logic. The doc comment states 'Kernel callers must use the explicit check methods with their own clock', but no no_std-aware variants of those six query methods exist in the impl block, leaving the no_std time-based path entirely broken.
- **Overengineering [LEAN]**: Necessary no_std stub returning Duration::ZERO with a clear comment directing kernel callers to override. Minimal and purposeful; the correct approach for a crate targeting both std and embedded/kernel environments.
- **Tests [NONE]**: This is the #[cfg(not(feature="std"))] variant. All tests run under the std feature, so this code path is never compiled or executed by the test suite. There are zero test cases for no_std behavior.
- **UNDOCUMENTED [UNDOCUMENTED]**: Private no_std fallback with explanatory `//` inline comments but no `///` doc comment. Inline comments do explain the zero-return sentinel behavior for kernel callers; lenient for private visibility. (deliberated: reclassified: duplication: DUPLICATE → UNIQUE — This is the no_std variant at L26-30. Correction NEEDS_FIX: the concern is legitimate — returning Duration::ZERO makes all time-based query methods (needs_rekey, is_expired, is_dead, needs_keepalive, should_retry_handshake, handshake_timed_out) silently non-functional in no_std builds. While the inline comment acknowledges the design, no _at() query variants exist for downstream kernel callers, leaving the no_std time path incomplete. Keep NEEDS_FIX. Duplication: two private cfg-gated stubs returning Duration::ZERO in different modules is a standard Rust no_std pattern, not meaningful duplication. Extracting to a shared module would create unnecessary coupling → reclassify to UNIQUE. Tests NONE: correct, no_std path is never compiled in tests. Documentation UNDOCUMENTED: fair for a private function, inline comments partially explain intent.)

> **Duplicate of** `rustguard-core/src/cookie.rs:elapsed_since` — Perfect match - both return Duration::ZERO for no_std builds as kernel-level fallback

#### `REKEY_AFTER_TIME` (L33–L33)

- **Utility [USED]**: Exported constant used locally in needs_rekey method (L115) to check if session exceeds rekey threshold.
- **Duplication [UNIQUE]**: WireGuard protocol constant (120 seconds). No semantic duplicates in codebase.
- **Correction [OK]**: 120 seconds matches REKEY-AFTER-TIME from the WireGuard whitepaper §6.3.
- **Overengineering [LEAN]**: Direct transcription of the WireGuard whitepaper §6 constant. Single-line, protocol-mandated, no abstraction involved.
- **Tests [WEAK]**: Used in the time branch of needs_rekey. The only needs_rekey test (message_count_triggers_rekey) exercises the counter branch. The time-based rekey trigger path is entirely untested — no test advances or mocks the clock past REKEY_AFTER_TIME.
- **DOCUMENTED [DOCUMENTED]**: Has a `///` comment describing the trigger for a new handshake. Minor phrasing imprecision ('this many seconds' for a Duration value) but intent is unambiguous. No `# Examples` section required for a protocol constant. (deliberated: confirmed — Tests WEAK is correct: the time-based rekey trigger path using this constant is never exercised — only the counter branch of needs_rekey is tested. Finding stands unchanged.)

#### `REJECT_AFTER_TIME` (L36–L36)

- **Utility [USED]**: Exported constant used locally in is_expired method (L122) to determine session expiration.
- **Duplication [UNIQUE]**: WireGuard protocol constant (180 seconds). No semantic duplicates found.
- **Correction [OK]**: 180 seconds matches REJECT-AFTER-TIME from the WireGuard whitepaper §6.3.
- **Overengineering [LEAN]**: Direct WireGuard protocol constant. No complexity.
- **Tests [WEAK]**: Used in the time branch of is_expired. Tests cover the no-session case and the REJECT_AFTER_MESSAGES counter case, but no test drives elapsed time past REJECT_AFTER_TIME to verify the time-based expiry path.
- **DOCUMENTED [DOCUMENTED]**: Has a `///` comment accurately describing that data using a keypair older than this value is rejected. Concise and correct. (deliberated: confirmed — Tests WEAK is correct: the time-based expiry path using this constant is untested. Only the counter branch and no-session branch of is_expired are covered. Finding stands.)

#### `REKEY_TIMEOUT` (L39–L39)

- **Utility [USED]**: Exported constant used locally in should_retry_handshake method (L150) for handshake retry logic.
- **Duplication [UNIQUE]**: WireGuard protocol constant (5 seconds). Unique value not replicated elsewhere.
- **Correction [NEEDS_FIX]**: The doc comment reads 'Don't try to send with a keypair older than this (REJECT_AFTER_TIME + padding)', which is semantically wrong. REKEY_TIMEOUT (5 s) is used exclusively in should_retry_handshake as the between-attempt retry interval, not as a keypair-age threshold for suppressing sends. The misleading description would cause callers reading the API docs to misapply the constant (e.g., as a send-gating guard instead of a retry window). The numeric value 5 s is correct per the WireGuard spec.
- **Overengineering [LEAN]**: Direct WireGuard protocol constant. No complexity.
- **Tests [WEAK]**: Used in should_retry_handshake to compare elapsed time since last handshake. The only test for that method checks the 'never sent a handshake' early-return path; the time-comparison branch that actually uses REKEY_TIMEOUT is never exercised.
- **PARTIAL [PARTIAL]**: Has a `///` comment but the description is factually misleading: '(REJECT_AFTER_TIME + padding)' implies a large derived value, whereas REKEY_TIMEOUT is 5 seconds and represents the per-attempt handshake-retry wait interval. The parenthetical contradicts the constant's actual semantics. (deliberated: confirmed — Correction NEEDS_FIX: the doc comment says 'Don't try to send with a keypair older than this (REJECT_AFTER_TIME + padding)' but REKEY_TIMEOUT (5s) is used exclusively in should_retry_handshake as a between-attempt retry interval, not a keypair-age threshold. The parenthetical '(REJECT_AFTER_TIME + padding)' is factually wrong for a 5-second value. The numeric value itself is correct per spec; only the documentation is misleading. This is a genuine doc-level NEEDS_FIX. Tests WEAK: the time-comparison branch using this constant is untested. Documentation PARTIAL: consistent with correction — the doc exists but is misleading.)

#### `REKEY_AFTER_MESSAGES` (L42–L42)

- **Utility [USED]**: Exported constant used in needs_rekey (L113) and test assertion (L198) to check message count rekey threshold.
- **Duplication [UNIQUE]**: WireGuard protocol constant with bitwise expression. No duplicates detected.
- **Correction [OK]**: Value is (2^60)–1. The WireGuard whitepaper specifies REKEY-AFTER-MESSAGES = 2^60; this constant is one less, so the >= check in needs_rekey triggers one message earlier than the spec mandates. The deviation is overly conservative and harmless in practice, but is technically an off-by-one versus the specification.
- **Overengineering [LEAN]**: Protocol-defined message-count threshold expressed as a bit-shift literal, consistent with how WireGuard specifies it. No overengineering.
- **Tests [GOOD]**: Directly referenced in the message_count_triggers_rekey test, which asserts needs_rekey(REKEY_AFTER_MESSAGES) returns true. The constant's boundary value is explicitly used and validated.
- **DOCUMENTED [DOCUMENTED]**: Has a `///` comment clearly describing the maximum message count before rekeying is triggered.

#### `REJECT_AFTER_MESSAGES` (L45–L45)

- **Utility [USED]**: Exported constant used in is_expired (L120) and test assertion (L203) to check message count expiration.
- **Duplication [UNIQUE]**: WireGuard protocol constant using u64::MAX. Unique specification not replicated.
- **Correction [OK]**: u64::MAX – (1 << 13) = 2^64 – 2^13 – 1. Rust correctly infers the integer literal as u64 in this context, avoiding overflow. Value matches the WireGuard spec REJECT-AFTER-MESSAGES.
- **Overengineering [LEAN]**: Protocol-mandated ceiling (u64::MAX minus a small buffer). Directly mirrors WireGuard spec arithmetic.
- **Tests [GOOD]**: Directly referenced in the message_count_triggers_reject test, which asserts is_expired(REJECT_AFTER_MESSAGES) returns true. The boundary value is explicitly tested.
- **DOCUMENTED [DOCUMENTED]**: Has a `///` comment accurately capturing the reject-on-count policy independent of elapsed time.

#### `REKEY_ATTEMPT_TIME` (L48–L48)

- **Utility [USED]**: Exported constant used locally in handshake_timed_out method (L155) for handshake timeout check.
- **Duplication [UNIQUE]**: WireGuard protocol constant (90 seconds). No duplicates in codebase.
- **Correction [OK]**: 90 seconds matches REKEY-ATTEMPT-TIME from the WireGuard whitepaper §6.3 and is used correctly in handshake_timed_out.
- **Overengineering [LEAN]**: Direct WireGuard protocol constant. No complexity.
- **Tests [NONE]**: Used exclusively in handshake_timed_out(). There is no test at all for handshake_timed_out — neither the false path (no timestamp), the true path (time exceeded), nor any boundary condition. REKEY_ATTEMPT_TIME is therefore completely untested.
- **DOCUMENTED [DOCUMENTED]**: Has a `///` comment describing the overall timeout before giving up on a handshake. Clear and accurate. (deliberated: confirmed — Tests NONE is correct: handshake_timed_out() has zero test coverage — no test exercises either the false or true path. Finding stands.)

#### `KEEPALIVE_TIMEOUT` (L51–L51)

- **Utility [DEAD]**: Exported constant with zero local or external usage. Not referenced in any method, no local test usage, and import analysis shows zero imports. Appears to be unused placeholder.
- **Duplication [UNIQUE]**: WireGuard protocol constant (10 seconds). Unique timing value not duplicated.
- **Correction [OK]**: 10 seconds matches KEEPALIVE-TIMEOUT from the WireGuard whitepaper §6.3.
- **Overengineering [LEAN]**: Direct WireGuard protocol constant. No complexity.
- **Tests [NONE]**: Exported constant intended for callers, but not used internally by needs_keepalive (which accepts interval as a parameter). No test in the module passes KEEPALIVE_TIMEOUT as an argument, and no integration test referencing it is visible in the provided context.
- **DOCUMENTED [DOCUMENTED]**: Has a `///` comment indicating its role as the keepalive interval when configured. Brief but sufficient for a named protocol constant. (deliberated: reclassified: utility: DEAD → LOW_VALUE, documentation: UNDOCUMENTED → DOCUMENTED — Utility DEAD → LOW_VALUE: this is an exported WireGuard protocol constant (§6.3). The needs_keepalive method deliberately takes keepalive_interval as an Option<Duration> parameter, making KEEPALIVE_TIMEOUT the canonical default value for external callers to pass in. For a library crate, exported protocol constants serve as public API even without internal usage — downstream consumers may use it. Removing it would break the API contract. Tests NONE: stands — no test references it. Documentation: the source clearly has '/// Keepalive interval when configured.' on L51, and the detail text itself says '[DOCUMENTED]'. The UNDOCUMENTED classification contradicts the evaluator's own detail — reclassify to DOCUMENTED.)

#### `DEAD_SESSION_TIMEOUT` (L54–L54)

- **Utility [USED]**: Exported constant used locally in is_dead method (L130) to determine if session should be zeroed.
- **Duplication [UNIQUE]**: WireGuard protocol constant (240 seconds). No semantic duplicates found.
- **Correction [OK]**: 240 seconds (60 s beyond REJECT_AFTER_TIME) provides a reasonable key-zeroing window after expiry. A session already expired at 180 s should have been replaced before 240 s, making is_dead a sound last-resort cleanup. Not explicitly specified in the whitepaper but internally consistent with the surrounding constants.
- **Overengineering [LEAN]**: Direct WireGuard protocol constant. No complexity.
- **Tests [WEAK]**: Used in is_dead() to compare elapsed time. The only test (new_timers_not_expired) verifies is_dead() returns false for a brand-new timer with no session — the trivially-false branch. The actual time-elapsed path where DEAD_SESSION_TIMEOUT triggers a true return is never tested.
- **DOCUMENTED [DOCUMENTED]**: Has a `///` comment clearly describing the threshold after which session keys are zeroed if no new handshake occurs. (deliberated: confirmed — Tests WEAK is correct: the only test (new_timers_not_expired) checks is_dead() returns false for a timer with no session — the trivially-false branch. The time-elapsed path where DEAD_SESSION_TIMEOUT would trigger a true return is never tested. Finding stands.)

#### `SessionTimers` (L57–L68)

- **Utility [USED]**: Public struct exported from rustguard-core library crate. Library public types are consumed by downstream crates (matches known false positive pattern for TransportSession). Also used in local tests (L184+) demonstrating the pattern's viability.
- **Duplication [UNIQUE]**: Struct defining peer session lifecycle state. No similar data structures found in codebase.
- **Correction [NEEDS_FIX]**: needs_keepalive has a logical contradiction: when last_sent is None, last_send_time is set to received via unwrap_or(received). The return expression then becomes elapsed_since(received) >= interval && elapsed_since(received) < interval — both sides measure elapsed time from the same instant, making them mutually exclusive. The function always returns false when no outbound packet has ever been sent, silently suppressing keepalives for exactly the scenario they exist to handle: the peer has sent data but the local side has not yet replied.
- **Overengineering [LEAN]**: Five fields — four Option<Timestamp> slots and one boolean — map precisely onto the WireGuard session-lifecycle events the struct must track. No unnecessary generics, no inheritance, no factory indirection. The pub fields are appropriate for an internal core crate. Lean data model for a non-trivial protocol requirement.
- **Tests [WEAK]**: Six inline tests cover new(), session_started(), packet_received(), needs_rekey/is_expired counter paths, needs_keepalive(None/Zero), and should_retry_handshake() early return. Untested: handshake_timed_out() entirely; all no_std methods (session_started_at, packet_sent_at, packet_received_at); the rekey_requested=true short-circuit in needs_rekey; all time-based code branches in needs_rekey/is_expired/is_dead; the positive (true) path of needs_keepalive; and should_retry_handshake() when a handshake was recently sent.
- **DOCUMENTED [DOCUMENTED]**: The struct carries a `///` type-level comment and every public field has its own `///` comment describing its role. No `# Examples` section expected for a plain data struct. Note: `new()`, `packet_sent_at()`, and `packet_received_at()` in the impl block lack doc comments, but those are outside the struct declaration range and the struct definition itself is comprehensively documented. (deliberated: confirmed — Correction NEEDS_FIX confirmed with higher confidence: the needs_keepalive logic has a real bug. When last_sent is None, `sent.unwrap_or(received)` sets last_send_time = received, making the return expression `elapsed_since(received) >= interval && elapsed_since(received) < interval` — this is `x >= i && x < i` which is always false. This silently suppresses keepalives precisely when they're most needed (peer sent data, we never replied). This is a genuine logic error. Tests WEAK: many significant paths untested including handshake_timed_out, rekey_requested short-circuit, all time-based branches, the positive needs_keepalive path, and should_retry_handshake time comparison. Both findings confirmed.)

## Best Practices — 9/10

### Rules

| # | Rule | Status | Severity | Detail |
|---|------|--------|----------|--------|
| 4 | Derive common traits on public types | WARN | MEDIUM | `SessionTimers` is a public struct but derives no traits at all. At minimum `Debug` and `Clone` should be derived. `std::time::Instant` implements both `Debug` and `Clone`; `u64` does as well, so both `cfg` branches support these derives. `PartialEq` is not derivable for `Instant` in all Rust versions, so that can be omitted, but `Debug` and `Clone` are straightforward. [L64] |
| 9 | Documentation comments on public items | WARN | MEDIUM | Three public methods lack `///` doc comments: `new()` (L70), `packet_sent_at()` (L103), and `packet_received_at()` (L113). All public constants, the struct itself, and the majority of methods are well-documented. The missing docs are on the no_std variants and the constructor. [L70, L103, L113] |

### Suggestions

- Derive Debug and Clone on the public SessionTimers struct so consumers can log, inspect, and clone timer state without resorting to manual implementations.
  ```typescript
  // Before
  pub struct SessionTimers {
      pub session_established: Option<Timestamp>,
      ...
  }
  // After
  #[derive(Debug, Clone)]
  pub struct SessionTimers {
      pub session_established: Option<Timestamp>,
      ...
  }
  ```
- Add doc comments to the three undocumented public methods: new(), packet_sent_at(), and packet_received_at().
  ```typescript
  // Before
  pub fn new() -> Self {
      Self { ... }
  }
  
  #[cfg(not(feature = "std"))]
  pub fn packet_sent_at(&mut self, now_ns: u64) {
      self.last_sent = Some(now_ns);
  }
  
  #[cfg(not(feature = "std"))]
  pub fn packet_received_at(&mut self, now_ns: u64) {
      self.last_received = Some(now_ns);
  }
  // After
  /// Creates a new `SessionTimers` with no active session.
  pub fn new() -> Self {
      Self { ... }
  }
  
  /// Record that we sent a packet (no_std: caller provides nanosecond timestamp).
  #[cfg(not(feature = "std"))]
  pub fn packet_sent_at(&mut self, now_ns: u64) {
      self.last_sent = Some(now_ns);
  }
  
  /// Record that we received a valid packet (no_std: caller provides nanosecond timestamp).
  #[cfg(not(feature = "std"))]
  pub fn packet_received_at(&mut self, now_ns: u64) {
      self.last_received = Some(now_ns);
  }
  ```

## Actions

### Quick Wins

- **[correction · high · small]** The no_std elapsed_since always returns Duration::ZERO, rendering all time-based SessionTimers query methods (needs_rekey, is_expired, is_dead, needs_keepalive, should_retry_handshake, handshake_timed_out) non-functional. Add no_std-aware _at(now_ns: u64) variants for each of these query methods that compute the duration inline from stored u64 timestamp fields, mirroring the _at pattern already used for session_started_at, packet_sent_at, and packet_received_at. [L29]
- **[correction · medium · small]** needs_keepalive incorrectly falls back to last_send_time = received when last_sent is None, creating contradictory guard conditions. Fix by handling the None case explicitly: if last_sent is None and elapsed_since(received) >= interval, return true; otherwise when last_sent is Some, apply the dual-condition check (elapsed_since(sent) >= interval && elapsed_since(received) < interval). [L163]
- **[correction · low · small]** The doc comment on REKEY_TIMEOUT incorrectly describes it as a keypair-age send threshold ('REJECT_AFTER_TIME + padding') rather than a between-attempt handshake retry interval. Correct the doc comment to match the actual usage in should_retry_handshake to prevent API misuse. [L39]

### Hygiene

- **[documentation · low · trivial]** Complete JSDoc documentation for: `REKEY_TIMEOUT` (`REKEY_TIMEOUT`) [L39-L39]
