# Review: `rustguard-core/tests/integration.rs`

**Verdict:** NEEDS_REFACTOR

## Symbols

| Symbol | Kind | Exported | Correction | Over-eng. | Utility | Duplication | Tests | Confidence |
|--------|------|----------|------------|-----------|---------|-------------|-------|------------|
| do_handshake | function | no | OK | LEAN | USED | UNIQUE | GOOD | 88% |
| full_handshake_transport_roundtrip | function | no | OK | LEAN | USED | UNIQUE | GOOD | 90% |
| replay_attack_blocked | function | no | OK | LEAN | USED | UNIQUE | GOOD | 88% |
| out_of_order_delivery | function | no | OK | LEAN | USED | UNIQUE | WEAK | 82% |
| wire_format_roundtrip | function | no | OK | ACCEPTABLE | USED | UNIQUE | GOOD | 85% |
| multiple_independent_handshakes | function | no | OK | LEAN | USED | UNIQUE | GOOD | 85% |
| tampered_transport_rejected | function | no | OK | LEAN | USED | UNIQUE | GOOD | 90% |
| empty_transport_packet | function | no | OK | LEAN | USED | UNIQUE | GOOD | 93% |
| max_size_transport_packet | function | no | OK | LEAN | USED | UNIQUE | GOOD | 88% |

### Details

#### `do_handshake` (L11–L31)

- **Utility [USED]**: Helper function directly called by 6 test functions at lines 36, 55, 69, 172, 182, and 192. Usage is unambiguous in provided code.
- **Duplication [UNIQUE]**: No similar functions found by RAG. Helper function setting up complete handshake between two peers.
- **Correction [OK]**: Handshake flow is correct: create_initiation → process_initiation → process_response. Fresh StaticSecret::random() keys per call prevents any replay-window issues. The _ts return value from process_initiation is intentionally discarded (timestamp used for handshake-level replay protection is irrelevant when each call generates entirely new peer keys). assert_eq! on the peer public key is a valid sanity check.
- **Overengineering [LEAN]**: Appropriate DRY helper: used by 6 of 8 tests. Extracts identical boilerplate into one place without adding unnecessary generics or parameters. Return type is a plain tuple — no wrapper struct invented.
- **Tests [GOOD]**: Helper function exercised by 7 of the 8 test functions. Validates peer key identity assertion after handshake. All three handshake phases (create_initiation, process_initiation, process_response) are exercised and the returned sessions are actively used in downstream assertions, giving strong implicit coverage of the helper itself.
- **PARTIAL [PARTIAL]**: Has a `///` doc comment ('Helper: run a full handshake between two peers, return their sessions.') which captures purpose and return shape, but provides no detail on the tuple fields returned (initiator vs responder ordering), no explanation of the static-key setup, and no `# Examples` section. Adequate for a private test helper but incomplete by full Rust doc standards. (deliberated: reclassified: documentation: PARTIAL → DOCUMENTED — Documentation PARTIAL is too strict for a private test helper. The doc comment '/// Helper: run a full handshake between two peers, return their sessions.' clearly states purpose and return semantics. The return type signature `(TransportSession, TransportSession)` is self-documenting — the variable names at every call site (`(init, resp)` or `(init_session, resp_session)`) make tuple ordering unambiguous. All other test symbols received DOCUMENTED under test-module leniency; a helper used exclusively by those tests deserves the same leniency. Requiring `# Examples` or field-by-field tuple documentation for a 20-line private test helper is unreasonable. Reclassified to DOCUMENTED.)

#### `full_handshake_transport_roundtrip` (L34–L51)

- **Utility [USED]**: Test function marked with #[test] attribute (line 33). Rust compiler and test runner execute all #[test] functions automatically.
- **Duplication [UNIQUE]**: No similar functions found by RAG. Tests bidirectional transport encryption/decryption with 100 packets per direction.
- **Correction [OK]**: Bidirectional 100-packet exchange with correct encrypt/decrypt pairing. Init session encrypts, resp session decrypts, and vice versa. Byte-level equality check against the original payload bytes is correct. No counter overflow risk at 100 packets.
- **Overengineering [LEAN]**: Sends 100 packets in each direction. The count is intentional for exercising the sliding-window replay counter; the test body is minimal and direct with no superfluous abstractions.
- **Tests [GOOD]**: Sends 100 packets in each direction with payload equality assertions. Covers the happy path thoroughly and with realistic volume. Both initiator→responder and responder→initiator directions are independently verified, catching any asymmetry in key derivation.
- **DOCUMENTED [DOCUMENTED]**: Integration test function in tests/ file. Per test-module leniency rule, all symbols in test contexts are DOCUMENTED by default. The function name is fully self-describing, and inline comments ('Send 100 packets each direction.') clarify intent within the body.

#### `replay_attack_blocked` (L54–L65)

- **Utility [USED]**: Test function marked with #[test] attribute (line 53). Executed by Rust test runner as part of test suite.
- **Duplication [UNIQUE]**: No similar functions found by RAG. Tests replay protection by rejecting duplicate counter values.
- **Correction [OK]**: Three assertions are logically correct: (1) first decrypt succeeds, (2) exact replay with same (ctr, ct) is rejected by the replay window, (3) ctr+1 with original ciphertext fails AEAD authentication because the tag was computed over the original counter value. The counter starts from 0/1, so ctr+1 has no overflow risk. Consistent use of is_some()/is_none() implies decrypt returns Option<Vec<u8>>.
- **Overengineering [LEAN]**: Three targeted assertions covering same-packet replay and AEAD-mismatch replay. Exactly as complex as the scenario requires.
- **Tests [GOOD]**: Tests exact-replay rejection (same counter + same ciphertext) and wrong-counter rejection (AEAD failure on counter+1 with original ciphertext). Covers the core security invariant. Minor gap: does not test a counter far in the past (outside the replay window) but the two cases present are the critical security paths.
- **DOCUMENTED [DOCUMENTED]**: Integration test function. Test-module leniency applies. Name precisely describes the invariant under test, and inline comments ('Replay the same packet — must be rejected.', 'Replay with different counter — AEAD will fail.') document expected behaviour for each assertion.

#### `out_of_order_delivery` (L68–L89)

- **Utility [USED]**: Test function marked with #[test] attribute (line 67). Executed by Rust test runner as part of test suite.
- **Duplication [UNIQUE]**: No similar functions found by RAG. Tests out-of-order packet decryption using sliding window mechanism.
- **Correction [OK]**: Encrypts 10 packets in order, delivers in reverse. Counter spread is 9, well within the standard WireGuard replay window of 64. The highest counter (9) arrives first and anchors the window; counters 8..0 fall within range. The second loop verifying that all replays are rejected is logically sound. Type annotation Vec<(u64, Vec<u8>, Vec<u8>)> is consistent with the encrypt return type; &Vec<u8> coerces to &[u8] for decrypt.
- **Overengineering [LEAN]**: Uses a Vec of tuples to hold (counter, ciphertext, plaintext) for reverse delivery — the minimum state needed to verify anti-replay after out-of-order acceptance. No unnecessary indirection.
- **Tests [WEAK]**: Delivers 10 packets in reverse order and then verifies all replays are rejected. Missing: window boundary cases (e.g., a counter exactly at the edge or outside the replay window), packets that arrive after the window has advanced past them, and large gaps between counters. The window-overflow scenario is a common source of bugs in WireGuard replay filters but is not exercised.
- **DOCUMENTED [DOCUMENTED]**: Integration test function. Test-module leniency applies. Function name is self-describing, and inline comments ('Encrypt 10 packets, deliver in reverse order.', 'Replaying any of them should fail now.') clearly annotate the two phases of the test scenario. (deliberated: confirmed — Tests WEAK is fair. The test covers the basic out-of-order delivery scenario (10 packets in reverse) and replay rejection, but omits window-boundary edge cases (counter at window edge, counter outside window after advancement, large counter gaps). These are known bug-prone areas in WireGuard replay filters. The finding is accurate and the confidence is appropriate.)

#### `wire_format_roundtrip` (L92–L135)

- **Utility [USED]**: Test function marked with #[test] attribute (line 91). Executed by Rust test runner as part of test suite.
- **Duplication [UNIQUE]**: No similar functions found by RAG. Tests serialization/deserialization of Initiation, Response, and Transport messages.
- **Correction [OK]**: Two separate create_initiation calls using the same init_static are independent (each generates a fresh ephemeral key). The first call's init_state is correctly discarded with _. The second call produces a complete handshake. Transport struct is constructed from init_session fields and round-tripped through to_bytes/from_bytes before decryption by resp_session. The decrypt result is compared against b"test payload" via PartialEq<[u8]> on Vec<u8>, which is valid.
- **Overengineering [ACCEPTABLE]**: At 43 lines this test covers three distinct wire formats (Initiation, Response, Transport) in one function, requiring an embedded full handshake to obtain a valid Response. Splitting into three tests would be cleaner, but the interdependency of message types partially justifies the combined approach. Complexity is real but not gratuitous.
- **Tests [GOOD]**: Exercises serialization and deserialization of all three message types (Initiation, Response, Transport). Verifies byte-length constants (INITIATION_SIZE, RESPONSE_SIZE), field preservation (sender_index, ephemeral), and end-to-end decryption after wire-format round-trip. Covers the happy path thoroughly with meaningful field-level assertions.
- **DOCUMENTED [DOCUMENTED]**: Integration test function. Test-module leniency applies. The name describes the property being verified. Inline comments ('Serialize/deserialize initiation.', 'Now do a real handshake and test transport wire format.', 'Transport message.') segment and annotate each phase of a complex multi-step test. (deliberated: confirmed — Overengineering ACCEPTABLE is a fair assessment. At 43 lines covering three message types (Initiation, Response, Transport) with an embedded handshake, the test is longer than ideal but the interdependency between message types partially justifies the combined approach. Not OVER (it serves a clear purpose with no gratuitous abstractions), but not LEAN either (could theoretically be split). ACCEPTABLE is the right middle ground.)

#### `multiple_independent_handshakes` (L138–L167)

- **Utility [USED]**: Test function marked with #[test] attribute (line 137). Executed by Rust test runner as part of test suite.
- **Duplication [UNIQUE]**: No similar functions found by RAG. Tests responder managing multiple independent initiators concurrently.
- **Correction [OK]**: Each iteration creates a fresh init_static, so the responder treats them as distinct peers — no timestamp replay-protection collision. sessions.iter_mut() yields &mut (TransportSession, TransportSession); destructuring as (init, resp) produces &mut TransportSession for each, satisfying the mut self requirement of encrypt/decrypt. String::from_utf8 roundtrip is correct since the original message is ASCII.
- **Overengineering [LEAN]**: Straightforward loop over 5 peers stored in a plain Vec. No factory, no trait objects, no generics — exactly what is needed to verify session isolation.
- **Tests [GOOD]**: Simulates a responder handling 5 separate initiators, verifying session isolation. Each session pair can encrypt/decrypt independently. Catches state-sharing bugs between concurrent sessions. Could be strengthened by attempting cross-session decryption (using one session's ciphertext in another), but core isolation is validated.
- **DOCUMENTED [DOCUMENTED]**: Integration test function. Test-module leniency applies. The name is self-describing. Inline comments ('Simulate a responder handling multiple initiators.', 'Each pair can communicate independently.') state the test scenario and its key assertion clearly.

#### `tampered_transport_rejected` (L170–L178)

- **Utility [USED]**: Test function marked with #[test] attribute (line 169). Executed by Rust test runner as part of test suite.
- **Duplication [UNIQUE]**: No similar functions found by RAG. Tests AEAD authentication failure on ciphertext bit flips.
- **Correction [OK]**: Flipping bit 0 of the ciphertext causes the ChaCha20-Poly1305 AEAD authentication tag to fail, so decrypt correctly returns None. The ciphertext is at minimum 16 bytes (AEAD tag) for any non-empty plaintext, so ct[0] is always a valid index. is_none() assertion is logically correct.
- **Overengineering [LEAN]**: Minimal: single encrypt, single XOR of one byte, single decrypt assertion. Cannot be simplified further.
- **Tests [GOOD]**: Flips the first byte of ciphertext with XOR 0xFF and asserts decryption returns None. Because AEAD authentication is all-or-nothing, testing a single-byte flip is sufficient to validate the integrity protection; the specific byte location does not affect correctness of the finding.
- **DOCUMENTED [DOCUMENTED]**: Integration test function. Test-module leniency applies. Name directly describes the security invariant. The inline comment 'Tamper with ciphertext.' makes the mutation step explicit and readable.

#### `empty_transport_packet` (L181–L188)

- **Utility [USED]**: Test function marked with #[test] attribute (line 180). Executed by Rust test runner as part of test suite.
- **Duplication [UNIQUE]**: No similar functions found by RAG. Tests empty payload encryption as WireGuard keepalive mechanism.
- **Correction [OK]**: Encrypting an empty slice produces a 16-byte AEAD tag ciphertext. Decrypting it must yield an empty Vec<u8>. assert!(pt.is_empty()) is the correct check. This correctly models WireGuard keepalive semantics.
- **Overengineering [LEAN]**: Single keepalive roundtrip with one assertion. Perfectly minimal for the scenario it covers.
- **Tests [GOOD]**: Explicitly tests the WireGuard keepalive case (zero-length payload). Encrypts an empty slice, decrypts, and asserts the result is empty. This is an important edge case that can fail if AEAD implementations mishandle zero-length associated data or plaintext.
- **DOCUMENTED [DOCUMENTED]**: Integration test function. Test-module leniency applies. The inline comment 'WireGuard uses empty transport packets as keepalives.' provides valuable protocol-level rationale explaining why a zero-byte payload is a meaningful test case, going beyond what the name alone conveys.

#### `max_size_transport_packet` (L191–L199)

- **Utility [USED]**: Test function marked with #[test] attribute (line 190). Executed by Rust test runner as part of test suite.
- **Duplication [UNIQUE]**: No similar functions found by RAG. Tests maximum MTU-constrained payload roundtrip encryption/decryption.
- **Correction [OK]**: 1400-byte payload is a realistic maximum WireGuard transport payload. vec![0xAA; 1400] is a deterministic, pattern-filled buffer. The round-trip encrypt/decrypt equality check is correct. No integer overflow or allocation issues at this size.
- **Overengineering [LEAN]**: 1400-byte vec filled with a constant byte; one encrypt/decrypt roundtrip. The magic number is documented inline (MTU calculation comment). No unnecessary abstraction.
- **Tests [GOOD]**: Tests a 1400-byte payload (close to the WireGuard MTU limit) with full encrypt/decrypt round-trip and byte-equality assertion. Validates correct handling at the practical upper boundary. Missing a test for payloads larger than the MTU, but 1400 bytes is the documented target maximum and the most operationally relevant boundary.
- **DOCUMENTED [DOCUMENTED]**: Integration test function. Test-module leniency applies. The inline comment 'MTU 1420 minus IP/UDP headers, this is roughly the max WireGuard payload.' provides protocol-level justification for the magic constant 1400, which is the most important documentation needed here.

## Best Practices — 9.5/10

### Rules

| # | Rule | Status | Severity | Detail |
|---|------|--------|----------|--------|
| 6 | Use clippy idioms | WARN | MEDIUM | In `wire_format_roundtrip` at L115, `ct.clone()` is used when constructing the `Transport` struct, but `ct` is never accessed after that point. The value could be moved directly, avoiding an unnecessary heap allocation. This would be flagged by `clippy::redundant_clone`. [L113-L120] |

### Suggestions

- In `wire_format_roundtrip`, `ct` is cloned when constructing `Transport` but never used again after that point. Move `ct` directly into the struct instead of cloning to avoid an unnecessary heap allocation. This removes a `clippy::redundant_clone` warning.
  ```typescript
  // Before
  let (ctr, ct) = init_session.encrypt(b"test payload").unwrap();
  let transport = Transport {
      receiver_index: init_session.their_index,
      counter: ctr,
      payload: ct.clone(),
  };
  // After
  let (ctr, ct) = init_session.encrypt(b"test payload").unwrap();
  let transport = Transport {
      receiver_index: init_session.their_index,
      counter: ctr,
      payload: ct,  // move: ct is not used after this point
  };
  ```
