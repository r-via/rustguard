# Review: `rustguard-daemon/src/tunnel.rs`

**Verdict:** NEEDS_REFACTOR

## Symbols

| Symbol | Kind | Exported | Correction | Over-eng. | Utility | Duplication | Tests | Confidence |
|--------|------|----------|------------|-----------|---------|-------------|-------|------------|
| TunnelState | class | no | OK | LEAN | USED | UNIQUE | NONE | 78% |
| MAX_PENDING_HANDSHAKES | constant | no | OK | LEAN | USED | UNIQUE | GOOD | 60% |
| HANDSHAKE_EXPIRY | constant | no | OK | LEAN | USED | UNIQUE | GOOD | 60% |
| RouteEntry | class | no | OK | LEAN | USED | UNIQUE | NONE | 78% |
| run | function | yes | NEEDS_FIX | ACCEPTABLE | USED | UNIQUE | NONE | 88% |
| rand_index | function | no | OK | LEAN | USED | DUPLICATE | NONE | 75% |
| fill_random | function | no | OK | ACCEPTABLE | USED | UNIQUE | NONE | 75% |
| ctrlc_handler | function | no | NEEDS_FIX | ACCEPTABLE | USED | UNIQUE | NONE | 88% |
| signal_noop | function | no | OK | LEAN | USED | UNIQUE | NONE | 60% |
| base64_key | function | no | OK | LEAN | USED | DUPLICATE | NONE | 60% |
| assign_v6_address | function | no | OK | LEAN | USED | UNIQUE | NONE | 60% |
| assign_v6_address | function | no | OK | LEAN | USED | UNIQUE | NONE | 60% |
| add_route | function | no | OK | LEAN | USED | UNIQUE | NONE | 60% |
| delete_route | function | no | OK | LEAN | USED | UNIQUE | NONE | 60% |
| add_route | function | no | OK | LEAN | USED | UNIQUE | NONE | 60% |
| delete_route | function | no | OK | LEAN | USED | UNIQUE | NONE | 60% |

### Details

#### `TunnelState` (L30–L35)

- **Utility [USED]**: Contains tunnel state (peers, handshakes, secrets) shared across threads; instantiated at L159
- **Duplication [UNIQUE]**: Struct definition with no similar types found in codebase
- **Correction [OK]**: Plain data struct with no logic; all fields are appropriate types. No correctness issues.
- **Overengineering [LEAN]**: Minimal struct grouping exactly the shared mutable state required by three threads: crypto key, peer list, and pending handshake map. No unnecessary generics or layering.
- **Tests [NONE]**: No test file exists for rustguard-daemon/src/tunnel.rs. This private struct is never instantiated or exercised in any test.
- **PARTIAL [PARTIAL]**: Struct has a /// summary line and one field-level comment on `pending_handshakes`, but `our_static` and `peers` fields carry no doc comments. Private struct so leniency applies, but the partial coverage is notable. (deliberated: confirmed — Tests NONE is accurate — no test file exists. Documentation PARTIAL is fair: struct-level doc exists but two of three fields lack doc comments. Private struct so severity is low. No changes.)

#### `MAX_PENDING_HANDSHAKES` (L37–L37)

- **Utility [USED]**: Constant used at L424 to limit pending handshakes and prevent memory exhaustion
- **Duplication [UNIQUE]**: Constant definition specific to tunnel state management; no similar constants found
- **Correction [OK]**: Reasonable cap constant; used correctly alongside Vec::remove(0) to evict oldest entry.
- **Overengineering [LEAN]**: Simple named constant preventing memory exhaustion. Appropriate and self-documenting.
- **Tests [GOOD]**: Compile-time constant with no runtime behavior. Per project evaluation policy, pure constants require no dedicated tests.
- **UNDOCUMENTED [UNDOCUMENTED]**: No /// doc comment present. Private constant; name is fairly self-descriptive but purpose (DoS cap) and rationale for value 64 are not documented. (deliberated: confirmed — Private constant with self-descriptive name but no doc comment explaining the specific value of 64 or its DoS-prevention rationale. UNDOCUMENTED is fair at low severity.)

#### `HANDSHAKE_EXPIRY` (L38–L38)

- **Utility [USED]**: Constant used at L397 to expire old handshakes after 30 seconds
- **Duplication [UNIQUE]**: Duration constant for handshake timeout; no similar constants found
- **Correction [OK]**: 30-second expiry is consistent with WireGuard protocol timers. No correctness issue.
- **Overengineering [LEAN]**: Named constant for the WireGuard handshake timeout, clear and appropriately scoped.
- **Tests [GOOD]**: Compile-time Duration constant with no runtime behavior. Per project evaluation policy, pure constants require no dedicated tests.
- **UNDOCUMENTED [UNDOCUMENTED]**: No /// doc comment present. Private constant. The 30-second value is a WireGuard protocol parameter and deserves at least a brief note tying it to the spec. (deliberated: confirmed — Private constant encoding a WireGuard protocol parameter (30s). A brief note linking to the spec would be valuable. UNDOCUMENTED is accurate.)

#### `RouteEntry` (L41–L45)

- **Utility [USED]**: Struct used at L113-128 to track added routes and clean them up at L482-484
- **Duplication [UNIQUE]**: Struct for route tracking on shutdown; no similar types found
- **Correction [OK]**: Plain record struct; no logic, no invariants to violate.
- **Overengineering [LEAN]**: Minimal data container for route cleanup tracking. Three fields encode exactly the information needed by the OS-level route delete calls.
- **Tests [NONE]**: No test file exists for this source file. This private data struct used for route cleanup bookkeeping has zero test coverage.
- **PARTIAL [PARTIAL]**: Has a single-line /// on the struct ('Routes we've added that need cleanup on shutdown.') but none of the three fields (route, interface, is_v6) carry doc comments. Private struct so leniency applies. (deliberated: confirmed — Tests NONE accurate — no tests exist for this file. Documentation PARTIAL correct: struct has a doc comment but fields do not. Private struct, low severity.)

#### `run` (L48–L489)

- **Utility [DEAD]**: Exported public function with 0 runtime or type-only importers per pre-computed analysis
- **Duplication [UNIQUE]**: Similar function found in server.rs (score 0.764) but with different semantic contract — tunnel daemon manages peer encryption/decryption; enrollment server manages client enrollment and multi-peer pool allocation. Different purposes, different threading models, not interchangeable despite structural similarities
- **Correction [NEEDS_FIX]**: Two distinct correctness bugs. (1) MSG_RESPONSE peer-matching heuristic is incorrect for multi-peer configurations: the find predicate uses '(p.endpoint.is_some() && !p.has_active_session()) || s.our_index == sender_index'. For a fresh handshake the second arm is never true (peer has no current session), so the first arm fires and returns the first peer without an active session — which may be the wrong peer. The peer index is never stored in pending_handshakes alongside the sender_index, making reliable lookup impossible. (2) In the timer thread's keepalive path, when session.encrypt(&[]) returns None (nonce exhausted), the code 'continue's without setting peer.session = None. The exhausted session remains Some, so peer.has_active_session() stays true and the rekey_requests collection is never triggered for that peer, permanently preventing rekeying via the timer path if there is no concurrent outbound traffic to clear it.
- **Overengineering [ACCEPTABLE]**: At ~340 lines the function is large, but it is an orchestration entry point for a WireGuard daemon: TUN setup, IPv6 addressing, UDP binding, route management, signal handling, and three protocol threads. Each section is clearly commented. The inline thread closures are idiomatic in Rust. Some extraction (e.g. outbound/inbound handlers into named functions) would improve readability, but the complexity is driven by the protocol, not by unnecessary abstraction.
- **Tests [NONE]**: No test file found. This is the most critical function in the daemon — orchestrating TUN creation, UDP binding, 3 worker threads, handshake initiation, route management, and graceful shutdown — yet it has zero test coverage of any kind.
- **PARTIAL [PARTIAL]**: Only a one-line /// comment ('Start the tunnel. Blocks until SIGINT/SIGTERM or fatal error.'). This is the sole pub function in the file: it takes a Config parameter with no description, returns io::Result<()> with no error-condition enumeration, and has no # Examples section. Substantially incomplete for a public API entry point. (deliberated: reclassified: utility: DEAD → USED, documentation: UNDOCUMENTED → PARTIAL — Correction NEEDS_FIX confirmed: (1) MSG_RESPONSE peer-matching at L296-301 uses a heuristic that breaks with multiple peers — the first peer without an active session is selected regardless of which peer the handshake was for. (2) Timer keepalive nonce exhaustion at ~L340 continues without clearing the session, preventing rekeying. Both are real bugs visible in the source. Utility reclassified DEAD→USED: this is `pub fn run()`, the sole public entry point of the daemon tunnel module. It is almost certainly called from main.rs or a binary crate entry point. The pre-computed analysis likely did not trace binary crate call graphs. A pub fn named 'run' in a daemon module being dead code would be nonsensical. Overengineering ACCEPTABLE kept — large but justified orchestration function. Tests NONE accurate. Documentation reclassified UNDOCUMENTED→PARTIAL: the function does have a /// doc comment ('Start the tunnel. Blocks until SIGINT/SIGTERM or fatal error.') as visible at L48, so it is not fully undocumented; it is partial (missing parameter/error docs).)

#### `rand_index` (L492–L496)

- **Utility [USED]**: Generates random u32 sender indices for handshakes; called at L173 and L421
- **Duplication [DUPLICATE]**: Identical implementation found in server.rs (score 0.946) and client.rs (score 0.938) — all three generate random u32 via fill_random() and u32::from_le_bytes(). Cross-crate duplication of trivial utility function
- **Correction [OK]**: Correctly reads 4 OS-random bytes and interprets them as a little-endian u32.
- **Overengineering [LEAN]**: Four-line wrapper that reads 4 OS-entropy bytes and converts them to u32. Clear intent, no unnecessary generics or patterns.
- **Tests [NONE]**: No test file found for this source file. The private helper function that generates random sender indices has no tests at all.
- **PARTIAL [PARTIAL]**: Has a single /// line explaining OS entropy usage. Private function; acceptable but missing return-semantics note (uniform u32, no bias guarantee claim). No # Examples required for private fn. (deliberated: confirmed — Duplication DUPLICATE is factually correct — identical 4-line function exists in server.rs and client.rs across crates. However, deduplication requires a shared utility crate which may be disproportionate effort for such a trivial function. Keeping the finding but noting practical effort. Tests NONE and documentation PARTIAL are both accurate.)

> **Duplicate of** `rustguard-enroll/src/server.rs:rand_index` — 100% identical — both fill 4-byte buffer with random data and convert to u32 for sender index generation

#### `fill_random` (L499–L501)

- **Utility [USED]**: Fills buffer with OS random bytes; called by rand_index() at L496
- **Duplication [UNIQUE]**: Wraps getrandom::getrandom(); trivial 1-line function with no similar functions found
- **Correction [OK]**: Delegates to getrandom crate; panicking on entropy failure is the correct behavior here since continuing without random bytes would be a security violation.
- **Overengineering [ACCEPTABLE]**: A one-liner wrapper around getrandom::getrandom with a single call site (rand_index). The indirection adds no real abstraction value and could be inlined, but the function name communicates intent and keeps the unsafe surface explicit. Minor over-separation, not harmful.
- **Tests [NONE]**: No test file found. This private OS-entropy wrapper has no test coverage, not even a smoke test verifying it produces non-zero output.
- **PARTIAL [PARTIAL]**: Has a single /// line noting getrandom crate and cross-platform nature. Private function; missing # Panics note (calls .expect() and will panic on getrandom failure, which can occur in no-std or sandboxed environments). (deliberated: confirmed — One-line wrapper with single call site — ACCEPTABLE is the right call since it communicates intent. Tests NONE and documentation PARTIAL (has a doc comment but missing #Panics note) are both accurate.)

#### `ctrlc_handler` (L504–L525)

- **Utility [USED]**: Sets up signal handler for clean shutdown; called at L146 during initialization
- **Duplication [UNIQUE]**: Signal handler using libc for SIGINT/SIGTERM; no similar functions found in codebase
- **Correction [NEEDS_FIX]**: sigprocmask(SIG_BLOCK) is called only inside the spawned signal-handler thread and only affects that thread's signal mask. The main thread and every subsequently spawned thread (outbound, inbound, timer) inherit an unblocked mask. POSIX allows delivery of SIGINT/SIGTERM to any thread that does not have the signal blocked. If the kernel delivers the signal to one of those threads, signal_noop is called and the signal is consumed there; sigwait in the handler thread never unblocks, the shutdown closure is never invoked, and the process cannot be stopped cleanly. The correct fix is to block the signals in the main thread (pthread_sigmask/sigprocmask) before spawning any worker threads so all children inherit the blocked mask, reserving delivery exclusively for the sigwait thread.
- **Overengineering [ACCEPTABLE]**: Manually implements POSIX signal interception via raw libc (signal, sigprocmask, sigwait) plus a Mutex<Option<F>> dance to call a FnOnce across a thread boundary. The ctrlc crate (npm analogue: ~5M weekly downloads on crates.io) would handle most of this cleanly. However, the sigwait-blocking-thread pattern is the correct POSIX approach for multi-threaded signal handling and is not trivially replicated by ctrlc, so the manual implementation is justifiable. Suggest evaluating ctrlc or signal-hook as alternatives.
- **Tests [NONE]**: No test file found. The signal-handling setup that blocks on SIGINT/SIGTERM and triggers the shutdown callback has no test coverage whatsoever.
- **PARTIAL [PARTIAL]**: Has a single /// line ('Install a Ctrl-C / SIGTERM handler.'). Private function; missing description of the FnOnce parameter semantics (called exactly once, on first signal), the spawned thread side-effect, or the unsafe POSIX signal interaction. (deliberated: confirmed — Correction NEEDS_FIX confirmed with increased confidence: sigprocmask(SIG_BLOCK) at L420 runs only inside the spawned thread. The main thread and all worker threads (outbound, inbound, timer) inherit unblocked masks. POSIX allows delivery of SIGINT/SIGTERM to any unblocked thread. If a worker thread receives the signal, signal_noop consumes it, sigwait never returns, and clean shutdown never fires. This is a well-documented POSIX multi-threaded signal handling pitfall clearly present in the code. Tests NONE and documentation PARTIAL are both accurate.)

#### `signal_noop` (L527–L527)

- **Utility [USED]**: Empty signal handler used as placeholder; referenced at L516-517
- **Duplication [UNIQUE]**: Trivial 1-line C function extern stub; no similar functions found
- **Correction [OK]**: Valid async-signal-safe no-op handler with correct extern "C" ABI. No correctness issues.
- **Overengineering [LEAN]**: Required extern C no-op used as a signal handler placeholder before sigprocmask blocks the signals. Minimal and necessary.
- **Tests [NONE]**: No test file found. This extern C no-op signal handler stub has no test coverage.
- **UNDOCUMENTED [UNDOCUMENTED]**: No /// doc comment. Private extern "C" fn; name suggests a no-op signal handler but the rationale for its existence alongside sigwait is not explained. (deliberated: confirmed — Trivial extern C no-op signal handler. Tests NONE is expected — testing a no-op is not meaningful. UNDOCUMENTED is technically accurate though name is self-descriptive. Low severity for both.)

#### `base64_key` (L529–L532)

- **Utility [USED]**: Encodes keys as base64 for logging; used at L107, L277, L313, L439
- **Duplication [DUPLICATE]**: Identical implementation found in server.rs (score 0.988) and client.rs (score 0.988) — both encode 32-byte key to base64 using BASE64_STANDARD.encode(). Cross-crate duplication of trivial utility function
- **Correction [OK]**: Correctly encodes a fixed 32-byte slice using standard base64. No issues.
- **Overengineering [LEAN]**: Simple utility encoding a 32-byte key to base64 for logging. Minimal wrapper around base64 crate, appropriate scope.
- **Tests [NONE]**: No test file found. The private display helper for 32-byte keys has no tests; a trivial determinism or known-vector test would be straightforward to add.
- **UNDOCUMENTED [UNDOCUMENTED]**: No /// doc comment. Private helper function; name is clear enough for internal use but a brief note about the encoding variant (BASE64_STANDARD) would be helpful. (deliberated: confirmed — Duplication DUPLICATE is factually correct — near-identical implementations in server.rs and client.rs. Same cross-crate dedup consideration as rand_index. Tests NONE and UNDOCUMENTED are accurate for this private helper.)

> **Duplicate of** `rustguard-enroll/src/server.rs:base64_key` — 99% identical logic — both encode [u8; 32] to base64 string for display

#### `assign_v6_address` (L536–L540)

- **Utility [USED]**: Assigns IPv6 address via ip command; called at L87 if IPv6 configured
- **Duplication [UNIQUE]**: Linux-specific IPv6 assignment via ip command; cfg-gated variant, no similar functions found
- **Correction [OK]**: Linux: 'ip -6 addr add <addr/prefix> dev <iface>' is the correct iproute2 command. No correctness issues.
- **Overengineering [LEAN]**: Linux-specific equivalent using ip -6. Same analysis as the macOS variant; idiomatic cfg-gated implementation.
- **Tests [NONE]**: No test file found. Linux-specific ip -6 addr add wrapper. No unit or integration tests exist for this platform variant.
- **UNDOCUMENTED [UNDOCUMENTED]**: Linux variant has no /// doc comment of its own. The macOS sibling at L536 carries the only doc comment. Private function; platform-variant doc gap. (deliberated: confirmed — Deliberating the L536-540 (macOS) variant. Tests NONE expected for platform-specific Command wrappers. The original detail says L536 carries the only doc comment (the macOS sibling), but looking at the source the macOS variant at L536 does have '/// Assign an IPv6 address to a TUN interface.' at L535. So it is documented, but the evaluator flagged UNDOCUMENTED. However, this symbol entry at L536-540 is the macOS variant which DOES have a doc comment. The detail text is contradictory. Keeping UNDOCUMENTED at low confidence since it's ambiguous which cfg-gated variant was intended.)

#### `assign_v6_address` (L543–L547)

- **Utility [USED]**: Assigns IPv6 address via ip command; called at L87 if IPv6 configured
- **Duplication [UNIQUE]**: Linux-specific IPv6 assignment via ip command; cfg-gated variant, no similar functions found
- **Correction [OK]**: Linux: 'ip -6 addr add <addr/prefix> dev <iface>' is the correct iproute2 command. No correctness issues.
- **Overengineering [LEAN]**: Linux-specific equivalent using ip -6. Same analysis as the macOS variant; idiomatic cfg-gated implementation.
- **Tests [NONE]**: No test file found. Linux-specific ip -6 addr add wrapper. No unit or integration tests exist for this platform variant.
- **UNDOCUMENTED [UNDOCUMENTED]**: Linux variant has no /// doc comment of its own. The macOS sibling at L536 carries the only doc comment. Private function; platform-variant doc gap. (deliberated: confirmed — Deliberating the L536-540 (macOS) variant. Tests NONE expected for platform-specific Command wrappers. The original detail says L536 carries the only doc comment (the macOS sibling), but looking at the source the macOS variant at L536 does have '/// Assign an IPv6 address to a TUN interface.' at L535. So it is documented, but the evaluator flagged UNDOCUMENTED. However, this symbol entry at L536-540 is the macOS variant which DOES have a doc comment. The detail text is contradictory. Keeping UNDOCUMENTED at low confidence since it's ambiguous which cfg-gated variant was intended.)

#### `add_route` (L551–L556)

- **Utility [USED]**: Adds routing rule via Linux ip command; called at L119 for each peer CIDR
- **Duplication [UNIQUE]**: Linux add_route found similar to client.rs version (score 0.864) but different semantic contract — uses ip command with -4/-6 subcmd, takes 3 parameters (route, interface, is_v6) vs client.rs 2 parameters. Different implementation for platform-specific tools
- **Correction [OK]**: Linux: 'ip -4|-6 route add <route> dev <iface>' is correct iproute2 usage. No issues.
- **Overengineering [LEAN]**: Linux route addition via ip command, cfg-gated. Minimal and direct.
- **Tests [NONE]**: No test file found. Linux-specific ip route add wrapper. Argument construction for both IPv4 (-4) and IPv6 (-6) variants is untested.
- **UNDOCUMENTED [UNDOCUMENTED]**: Linux variant has no /// doc comment. Private function; no documentation despite being a distinct platform implementation. (deliberated: confirmed — Deliberating L551-556 (macOS variant). Has the '/// Platform-specific route management.' doc comment at L550. Tests NONE expected for OS command wrappers. UNDOCUMENTED is borderline since a doc comment exists on the first cfg-gated variant, but keeping at low confidence.)

#### `delete_route` (L559–L564)

- **Utility [USED]**: Removes routing rule via Linux ip command; called at L483 on shutdown
- **Duplication [UNIQUE]**: Linux delete_route paired with add_route at L567-572. Platform-specific route deletion (ip route del); complementary to add_route, not a duplicate. Intentional route cleanup counterpart
- **Correction [OK]**: Linux: 'ip -4|-6 route del <route> dev <iface>' correctly mirrors add_route. No issues.
- **Overengineering [LEAN]**: Linux route deletion, symmetric to Linux add_route. Minimal.
- **Tests [NONE]**: No test file found. Linux-specific ip route del wrapper called during shutdown cleanup. Has no test coverage at all.
- **UNDOCUMENTED [UNDOCUMENTED]**: Linux variant has no /// doc comment. Private function; entirely undocumented. (deliberated: confirmed — Deliberating L559-564 (macOS variant). No individual doc comment; shares the section-level comment. Tests NONE expected. Findings accurate at low severity.)

#### `add_route` (L567–L572)

- **Utility [USED]**: Adds routing rule via Linux ip command; called at L119 for each peer CIDR
- **Duplication [UNIQUE]**: Linux add_route found similar to client.rs version (score 0.864) but different semantic contract — uses ip command with -4/-6 subcmd, takes 3 parameters (route, interface, is_v6) vs client.rs 2 parameters. Different implementation for platform-specific tools
- **Correction [OK]**: Linux: 'ip -4|-6 route add <route> dev <iface>' is correct iproute2 usage. No issues.
- **Overengineering [LEAN]**: Linux route addition via ip command, cfg-gated. Minimal and direct.
- **Tests [NONE]**: No test file found. Linux-specific ip route add wrapper. Argument construction for both IPv4 (-4) and IPv6 (-6) variants is untested.
- **UNDOCUMENTED [UNDOCUMENTED]**: Linux variant has no /// doc comment. Private function; no documentation despite being a distinct platform implementation. (deliberated: confirmed — Deliberating L551-556 (macOS variant). Has the '/// Platform-specific route management.' doc comment at L550. Tests NONE expected for OS command wrappers. UNDOCUMENTED is borderline since a doc comment exists on the first cfg-gated variant, but keeping at low confidence.)

#### `delete_route` (L575–L580)

- **Utility [USED]**: Removes routing rule via Linux ip command; called at L483 on shutdown
- **Duplication [UNIQUE]**: Linux delete_route paired with add_route at L567-572. Platform-specific route deletion (ip route del); complementary to add_route, not a duplicate. Intentional route cleanup counterpart
- **Correction [OK]**: Linux: 'ip -4|-6 route del <route> dev <iface>' correctly mirrors add_route. No issues.
- **Overengineering [LEAN]**: Linux route deletion, symmetric to Linux add_route. Minimal.
- **Tests [NONE]**: No test file found. Linux-specific ip route del wrapper called during shutdown cleanup. Has no test coverage at all.
- **UNDOCUMENTED [UNDOCUMENTED]**: Linux variant has no /// doc comment. Private function; entirely undocumented. (deliberated: confirmed — Deliberating L559-564 (macOS variant). No individual doc comment; shares the section-level comment. Tests NONE expected. Findings accurate at low severity.)

## Best Practices — 1.5/10

### Rules

| # | Rule | Status | Severity | Detail |
|---|------|--------|----------|--------|
| 1 | No unwrap in production code | FAIL | CRITICAL | Multiple .unwrap() calls across all production code paths: state.lock().unwrap() appears in the main thread (initial handshake setup), all three spawned threads (outbound, inbound, timer), and in ctrlc_handler. buf[..4].try_into().unwrap() appears in the hot UDP receive path. outbound.join().unwrap(), inbound.join().unwrap(), timer.join().unwrap() are called at shutdown. Mutex poison and unexpected thread failures are completely unhandled. [L153, L192, L248, L313, L383-L385, L424] |
| 2 | No unsafe blocks without clear justification comment | FAIL | CRITICAL | ctrlc_handler contains three unsafe blocks covering libc::signal(), std::mem::zeroed() for sigset_t, sigemptyset/sigaddset/sigprocmask/sigwait calls, and a signal_noop extern "C" function cast. None carry a // SAFETY: comment explaining which operations are async-signal-safe, why the cast of signal_noop is valid, or what invariants ensure the sigwait pattern is correct in a multithreaded program. [L407-L431] |
| 3 | Proper error handling with Result/Option | WARN | HIGH | Three uses of `let _ = ...` silently discard errors: UDP send for keepalive packets in the timer thread, UDP send for handshake retry packets, and delete_route at shutdown. Keepalive drops may be operationally acceptable, but silent handshake-retry failures make debugging connectivity issues very difficult. Route deletion failures at shutdown are not reported. [L342, L370, L389] |
| 6 | Use clippy idioms | WARN | MEDIUM | Three clippy-style issues: (1) `for i in 0..st.peers.len()` with subsequent indexing in the initial handshake loop is less idiomatic than enumerate(); (2) `format!("{}", cidr)` can be simplified to cidr.to_string(); (3) `st.pending_handshakes.remove(0)` is O(n) shift on Vec — the drop-oldest ring-buffer pattern is the canonical use-case for VecDeque::pop_front(). [L153-L165, L110, L365] |
| 7 | No panic! in library/production code | WARN | CRITICAL | No explicit panic! macros are present, but fill_random uses .expect("failed to get random bytes") which panics if getrandom fails — crashing the daemon rather than propagating an error through the call chain. The thread join .unwrap() calls at shutdown would also propagate a thread panic as a second unhandled panic rather than logging and shutting down gracefully. [L401, L383-L385] |
| 11 | Memory safety | WARN | HIGH | std::mem::zeroed() is used inside an unsafe block to initialise libc::sigset_t. While zeroing a sigset_t is conventional in C, using zeroed() in Rust on an opaque FFI type is technically undefined behaviour if the type contains non-zero-valid padding. No mem::forget, Box::into_raw leaks, or missing Drop implementations are visible. All thread handles are properly joined. [L413] |
| 12 | Concurrency safety | WARN | HIGH | Two concerns: (1) The outbound thread holds the TunnelState Mutex while calling udp_out.send_to(), a potentially blocking network I/O operation, starving the inbound and timer threads for the duration. The inbound thread correctly calls drop(st) before tun_in.write(), making it the better pattern. (2) sigprocmask(SIG_BLOCK) is called only inside the spawned ctrlc_handler thread; POSIX requires signals to be blocked in all threads before sigwait is reliable — signals may be delivered to the outbound or inbound thread instead, silently invoking signal_noop and never triggering shutdown. [L192-L215, L416-L422] |

### Suggestions

- Replace Mutex::lock().unwrap() with poison-tolerant recovery
  ```typescript
  // Before
  let mut st = state.lock().unwrap();
  // After
  let mut st = state.lock().unwrap_or_else(|poisoned| {
      eprintln!("state mutex poisoned, recovering");
      poisoned.into_inner()
  });
  ```
- Add // SAFETY: comments to unsafe blocks in ctrlc_handler
  ```typescript
  // Before
  unsafe {
      libc::signal(libc::SIGINT, signal_noop as *const () as libc::sighandler_t);
      libc::signal(libc::SIGTERM, signal_noop as *const () as libc::sighandler_t);
  }
  // After
  // SAFETY: signal_noop is an extern "C" fn performing no operations, satisfying
  // async-signal-safety requirements. We register it before blocking the signal
  // to prevent the default termination handler from firing between registration
  // and the subsequent sigwait call in the spawned thread.
  unsafe {
      libc::signal(libc::SIGINT, signal_noop as *const () as libc::sighandler_t);
      libc::signal(libc::SIGTERM, signal_noop as *const () as libc::sighandler_t);
  }
  ```
- Propagate getrandom error instead of panicking
  ```typescript
  // Before
  fn fill_random(buf: &mut [u8]) {
      getrandom::getrandom(buf).expect("failed to get random bytes");
  }
  // After
  fn fill_random(buf: &mut [u8]) -> io::Result<()> {
      getrandom::getrandom(buf)
          .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
  }
  ```
- Use VecDeque instead of Vec for the pending-handshakes ring-buffer to get O(1) pop_front
  ```typescript
  // Before
  if st.pending_handshakes.len() >= MAX_PENDING_HANDSHAKES {
      st.pending_handshakes.remove(0); // Drop oldest.
  }
  // After
  // Change field type to VecDeque<(u32, Instant, InitiatorHandshake)>
  if st.pending_handshakes.len() >= MAX_PENDING_HANDSHAKES {
      st.pending_handshakes.pop_front(); // O(1) drop of oldest.
  }
  ```
- Avoid holding the TunnelState mutex across UDP send in the outbound thread
  ```typescript
  // Before
  let mut st = state_out.lock().unwrap();
  // ... build transport ...
  if let Err(e) = udp_out.send_to(&wire, endpoint) {
  // After
  let wire = {
      let mut st = state_out.lock().unwrap_or_else(|p| p.into_inner());
      // ... build transport, return wire bytes ...
      transport.to_bytes()
  }; // lock released before I/O
  if let Err(e) = udp_out.send_to(&wire, endpoint) {
  ```

## Actions

### Quick Wins

- **[correction · medium · small]** Store the originating peer index alongside the sender_index in pending_handshakes (change the tuple type to (u32, Instant, InitiatorHandshake, usize) where the last element is the peer index). In the MSG_RESPONSE handler, after removing the entry, use the stored peer index directly instead of the fragile heuristic that finds 'any peer without an active session', which silently assigns the session to the wrong peer when multiple peers are configured. [L265]
- **[correction · medium · small]** Block SIGINT and SIGTERM in the main thread before calling ctrlc_handler (and before spawning any worker threads) using sigprocmask/pthread_sigmask so that all spawned threads inherit the blocked mask. This ensures that the only thread eligible to receive those signals via sigwait is the dedicated signal-handler thread, preventing the shutdown closure from being silently dropped when the signal is delivered to a worker thread. [L504]
- **[correction · low · small]** In the timer thread's keepalive path, when session.encrypt(&[]) returns None (nonce counter exhausted), set peer.session = None before executing 'continue'. Without this, the exhausted session remains Some, peer.has_active_session() continues to return true, and the rekey_requests collection loop never fires for that peer, making rekeying impossible via the timer path when there is no concurrent outbound packet traffic to clear the session. [L340]

### Refactors

- **[duplication · medium · small]** Deduplicate: `rand_index` duplicates `rand_index` in `rustguard-enroll/src/server.rs` (`rand_index`) [L492-L496]
- **[duplication · medium · small]** Deduplicate: `base64_key` duplicates `base64_key` in `rustguard-enroll/src/server.rs` (`base64_key`) [L529-L532]

### Hygiene

- **[documentation · low · trivial]** Complete JSDoc documentation for: `TunnelState` (`TunnelState`) [L30-L35]
- **[documentation · low · trivial]** Complete JSDoc documentation for: `RouteEntry` (`RouteEntry`) [L41-L45]
- **[documentation · medium · trivial]** Add JSDoc documentation for exported symbol: `run` (`run`) [L48-L489]
- **[documentation · low · trivial]** Complete JSDoc documentation for: `rand_index` (`rand_index`) [L492-L496]
- **[documentation · low · trivial]** Complete JSDoc documentation for: `fill_random` (`fill_random`) [L499-L501]
- **[documentation · low · trivial]** Complete JSDoc documentation for: `ctrlc_handler` (`ctrlc_handler`) [L504-L525]
