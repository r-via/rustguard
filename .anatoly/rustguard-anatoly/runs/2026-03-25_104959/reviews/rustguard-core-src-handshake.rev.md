# Review: `rustguard-core/src/handshake.rs`

**Verdict:** NEEDS_REFACTOR

## Symbols

| Symbol | Kind | Exported | Correction | Over-eng. | Utility | Duplication | Tests | Confidence |
|--------|------|----------|------------|-----------|---------|-------------|-------|------------|
| initial_chain_key | function | no | OK | LEAN | USED | UNIQUE | WEAK | 85% |
| initial_hash | function | no | OK | LEAN | USED | UNIQUE | WEAK | 85% |
| mix_hash | function | no | OK | LEAN | USED | UNIQUE | WEAK | 85% |
| mix_key | function | no | OK | LEAN | USED | UNIQUE | WEAK | 90% |
| encrypt_and_hash | function | no | OK | LEAN | USED | UNIQUE | WEAK | 72% |
| decrypt_and_hash | function | no | OK | LEAN | USED | UNIQUE | WEAK | 72% |
| compute_mac1 | function | yes | OK | LEAN | USED | UNIQUE | WEAK | 88% |
| InitiatorHandshake | class | yes | OK | LEAN | USED | UNIQUE | WEAK | 80% |
| create_initiation | function | yes | OK | LEAN | USED | UNIQUE | WEAK | 88% |
| create_initiation_psk | function | yes | OK | ACCEPTABLE | USED | UNIQUE | WEAK | 85% |
| create_initiation_with | function | yes | OK | LEAN | USED | UNIQUE | WEAK | 78% |
| process_response | function | yes | NEEDS_FIX | LEAN | USED | UNIQUE | WEAK | 76% |
| process_initiation | function | yes | OK | LEAN | USED | UNIQUE | WEAK | 88% |
| process_initiation_psk | function | yes | OK | ACCEPTABLE | USED | UNIQUE | WEAK | 85% |
| process_initiation_with | function | yes | OK | LEAN | USED | UNIQUE | WEAK | 77% |
| constant_time_eq | function | no | OK | LEAN | USED | UNIQUE | WEAK | 80% |

### Details

#### `initial_chain_key` (L23–L25)

- **Utility [USED]**: Called in create_initiation_with (line 119) and process_initiation_with (line 275)
- **Duplication [UNIQUE]**: Trivial 2-line utility function with no external duplicates found
- **Correction [OK]**: Correctly computes HASH(CONSTRUCTION) as the Noise protocol's initial chaining key. No issues found.
- **Overengineering [LEAN]**: Single-call wrapper that computes the Noise protocol constant exactly once. Naming it clearly communicates intent and isolates the magic constant.
- **Tests [WEAK]**: Private helper exercised only indirectly through the full handshake test. No isolated unit test verifying the output against a known constant derived from CONSTRUCTION, and no test that catches a regression if the constant changes.
- **PARTIAL [PARTIAL]**: Has a `///` doc comment describing purpose and rationale ('compute once'), but no return value description or `# Examples`. Tolerable for a private helper, but the comment is shared ambiguously with `initial_hash` above it. (deliberated: confirmed — Tests WEAK is fair — private crypto primitive with no known-answer test vectors, only exercised transitively. Documentation PARTIAL is accurate — the doc comment is present but ambiguously shared with initial_hash. Both findings stand as-is.)

#### `initial_hash` (L27–L29)

- **Utility [USED]**: Called in create_initiation_with (line 120) and process_initiation_with (line 276)
- **Duplication [UNIQUE]**: Trivial 2-line utility function with no external duplicates found
- **Correction [OK]**: Correctly computes HASH(ck || IDENTIFIER) for the initial handshake hash. No issues found.
- **Overengineering [LEAN]**: One-liner companion to initial_chain_key. The two helpers together mirror the Noise spec's initialisation step precisely.
- **Tests [WEAK]**: Private helper covered only transitively via full_handshake_and_transport. No direct test with known-good vectors for the CONSTRUCTION+IDENTIFIER hash, making silent regressions possible.
- **UNDOCUMENTED [UNDOCUMENTED]**: No `///` doc comment at all. The preceding doc block belongs to `initial_chain_key`, leaving `initial_hash` entirely undocumented. Even as a private function, some description of the `ck` parameter and the Noise protocol role would be helpful. (deliberated: confirmed — Tests WEAK is valid — no isolated test with known Noise protocol test vectors. UNDOCUMENTED is correct — the preceding doc block belongs to initial_chain_key, not this function. Both findings confirmed.)

#### `mix_hash` (L32–L34)

- **Utility [USED]**: Called in encrypt_and_hash (line 46), decrypt_and_hash (line 54), and multiple times in both initiator and responder paths
- **Duplication [UNIQUE]**: Trivial 2-line utility function with no external duplicates found
- **Correction [OK]**: Correct Noise MixHash operation: h = HASH(h || data). No issues found.
- **Overengineering [LEAN]**: Trivial but semantically important: naming this 'mix_hash' makes every call site read like the Noise protocol spec, which is the right trade-off in cryptographic code.
- **Tests [WEAK]**: Called many times throughout the handshake but never tested in isolation. All coverage is implicit via higher-level tests; no edge-case input (empty slice, full 32-byte slice) is exercised directly.
- **PARTIAL [PARTIAL]**: Has a one-line `///` comment describing purpose, but the description says 'public key' while the parameter is `&[u8]` (generic data). No parameter or return documentation. Private function, so no `# Examples` required, but the inaccuracy reduces quality. (deliberated: confirmed — Tests WEAK valid — heavily used but never tested in isolation. Documentation PARTIAL correct — the comment inaccurately says 'public key' when it accepts generic &[u8]. Both findings confirmed.)

#### `mix_key` (L38–L41)

- **Utility [USED]**: Called multiple times in create_initiation_with (lines 132, 145) and process_initiation_with (lines 296, 318, 337, 341)
- **Duplication [DUPLICATE]**: Identical logic to mix_key in rustguard-kmod/src/noise.rs; both perform hkdf and return (new_ck, key) tuple. Parameter type differs (SharedSecret wrapper vs raw bytes) but semantic behavior is equivalent.
- **Correction [OK]**: Correctly implements HKDF2(ck, dh_result) returning (new_ck, key) and discarding the unused third output. Matches the Noise MixKey primitive.
- **Overengineering [LEAN]**: Wraps HKDF to produce (ck, key). Discarding the third HKDF output explicitly via '_' is correct and intentional per WireGuard spec.
- **Tests [WEAK]**: Indirect coverage only. The function discards the third HKDF output silently; no test verifies that the returned (ck, key) pair is derived correctly against a reference vector.
- **PARTIAL [PARTIAL]**: Two-line `///` comment describes purpose and the tuple return value `(new_ck, key)`. Good for a private helper, but the `dh_result` parameter and the dropped third HKDF output are not explained. No `# Examples`. (deliberated: reclassified: duplication: DUPLICATE → UNIQUE — Duplication reclassified DUPLICATE → UNIQUE: rustguard-core (userspace, x25519-dalek types, SharedSecret wrapper, alloc-based) and rustguard-kmod (kernel module, raw byte arrays, C FFI) are fundamentally different compilation targets. Sharing code between a userspace Rust crate and a kernel module is architecturally infeasible — the type differences (SharedSecret vs raw bytes) reflect different abstraction layers, not accidental duplication. The parallel implementations are intentional. Tests WEAK and docs PARTIAL stand as originally assessed.)

> **Duplicate of** `rustguard-kmod/src/noise.rs:mix_key` — 96% identical logic — both call hkdf(ck, dh_result) and return the key pair, differing only in parameter wrapper types

#### `encrypt_and_hash` (L44–L48)

- **Utility [USED]**: Called in create_initiation_with (lines 139, 148) and process_initiation_with (line 351)
- **Duplication [UNIQUE]**: Similar structure to kmod version but different semantic contracts: returns owned Vec<u8> with immediate ciphertext vs Option<usize> with buffer-based output. Different memory models (allocation vs pre-allocated buffer) make them non-interchangeable despite 92% similarity score.
- **Correction [OK]**: AEAD seal with nonce 0 and h as AAD, followed by MixHash(ciphertext) is the correct Noise EncryptAndHash operation.
- **Overengineering [LEAN]**: Direct encoding of the Noise 'EncryptAndHash' operation. Two steps, no hidden logic.
- **Tests [WEAK]**: Covered transitively by tampered_initiation_rejected and tampered_response_rejected (which confirm AEAD authentication), but no unit test verifies ciphertext length, the hash update step, or encrypt-then-hash ordering in isolation.
- **PARTIAL [PARTIAL]**: Single-line `///` comment describes the combined operation. No parameter descriptions (key, h, plaintext), no return tuple explanation, no `# Examples`. Adequate for private use but incomplete. (deliberated: confirmed — Tests WEAK valid — encrypt-then-hash ordering never verified in isolation. Documentation PARTIAL correct — single-line comment without parameter or return docs. Confidence stays at 72 due to limited visibility into full test suite coverage.)

#### `decrypt_and_hash` (L51–L55)

- **Utility [USED]**: Called in process_response (line 222) and process_initiation_with (lines 308, 319)
- **Duplication [UNIQUE]**: Similar structure to kmod version but different semantic contracts: returns owned Vec<u8> plaintext vs Option<usize> with buffer-based output. Different memory models and return semantics make them non-interchangeable despite 92% similarity score.
- **Correction [OK]**: AEAD open with nonce 0 and h as AAD, followed by MixHash(ciphertext) on success is the correct Noise DecryptAndHash operation. Hash is updated with the ciphertext (not the plaintext), which is correct.
- **Overengineering [LEAN]**: Symmetric counterpart to encrypt_and_hash. Returns Option to propagate AEAD tag failures, which is idiomatic Rust for fallible decryption.
- **Tests [WEAK]**: Exercises the None path indirectly via tampered message tests, but no unit test confirms the hash is updated from the *ciphertext* (not plaintext) or checks behavior on zero-length input independently.
- **PARTIAL [PARTIAL]**: Single-line `///` comment mirrors `encrypt_and_hash`. The `Option` return indicating authentication failure is not documented. No parameter or return details, no `# Examples`. Adequate for private use but incomplete. (deliberated: confirmed — Tests WEAK valid — no test confirms hash is updated from ciphertext (not plaintext), which is a subtle correctness property. Documentation PARTIAL correct — Option return for auth failure undocumented. Both findings confirmed.)

#### `compute_mac1` (L59–L65)

- **Utility [USED]**: Called in create_initiation_with (line 162), process_initiation_with (lines 278, 356)
- **Duplication [DUPLICATE]**: Identical logic to compute_mac1 in rustguard-kmod/src/noise.rs; both hash LABEL_MAC1||responder_public, compute MAC, and extract first 16 bytes. Parameter type differs (PublicKey wrapper vs raw [u8; 32]) but implementation is equivalent.
- **Correction [OK]**: Correctly computes MAC1 = HMAC-BLAKE2s(HASH(LABEL_MAC1 || S_pub_r), msg)[:16] per WireGuard spec. Truncation to 16 bytes is correct.
- **Overengineering [LEAN]**: Faithfully implements the WireGuard MAC1 derivation. The temporary 'key' variable and slice copy are necessary since BLAKE2s-MAC produces 32 bytes and MAC1 is only 16.
- **Tests [WEAK]**: MAC1 rejection is exercised by wrong_responder_key_rejects (MAC1 mismatch causes None) but no test directly checks the 16-byte output value against a known vector, nor are there tests for tampered mac1 bytes specifically.
- **PARTIAL [PARTIAL]**: Public function with two-line `///` comment that includes an algorithmic formula — helpful. However, no `# Parameters`, `# Returns`, or `# Examples` sections. The `msg_bytes` parameter's expected content (pre-mac1 wire bytes) is not documented. (deliberated: reclassified: duplication: DUPLICATE → UNIQUE — Duplication reclassified DUPLICATE → UNIQUE for the same reason as mix_key: rustguard-core and rustguard-kmod target fundamentally different environments (userspace vs kernel). The parameter type difference (PublicKey wrapper vs raw [u8; 32]) is not superficial — it reflects different type systems mandated by the compilation target. Cross-crate deduplication into a shared dependency would force the kernel module to depend on x25519-dalek types, which is architecturally wrong. Tests WEAK valid — no known-vector test for the 16-byte output. Documentation PARTIAL correct — public function lacks #Parameters/#Returns sections.)

> **Duplicate of** `rustguard-kmod/src/noise.rs:compute_mac1` — 95% identical implementation — both compute BLAKE2s MAC and slice first 16 bytes, differing only in parameter wrapper type

#### `InitiatorHandshake` (L72–L81)

- **Utility [USED]**: Returned by create_initiation_with (line 163) and accepted by process_response (line 174); used extensively in tests
- **Duplication [UNIQUE]**: Struct definition with no duplicate struct definitions found in RAG results or codebase
- **Correction [OK]**: Sensitive fields ck, h, ephemeral, and psk are correctly zeroized on drop. Non-sensitive sender_index and their_public are correctly skipped with #[zeroize(skip)].
- **Overengineering [LEAN]**: Appropriately uses ZeroizeOnDrop for key material and zeroize(skip) for non-sensitive index/pubkey fields. Exactly the fields needed to bridge msg1 and msg2 — nothing extraneous.
- **Tests [WEAK]**: The struct is instantiated and consumed in every test, confirming basic lifecycle. However, the ZeroizeOnDrop guarantee is never tested, and the psk field interaction is only tested with the all-zero default.
- **PARTIAL [PARTIAL]**: Public struct with a two-line `///` comment describing its lifecycle role and the zeroize security property. All fields are private so field-level docs are not strictly required, but there is no `# Examples` section showing how to obtain or use this state, and no mention of which functions consume it. (deliberated: confirmed — Tests WEAK valid — ZeroizeOnDrop guarantee untested, PSK field only tested with all-zeros. Documentation PARTIAL correct — lifecycle described but no usage examples or field docs. Both findings confirmed.)

#### `create_initiation` (L85–L91)

- **Utility [USED]**: Called in full_handshake_and_transport test (line 389) and other tests (lines 406, 427, 441)
- **Duplication [UNIQUE]**: Convenience wrapper that delegates to create_initiation_psk with default PSK parameter. Distinct function with clear caller-callee relationship rather than true duplication.
- **Correction [OK]**: Correctly delegates to create_initiation_psk with a zero PSK, matching WireGuard's no-PSK behavior.
- **Overengineering [LEAN]**: Minimal std-only shim that forwards to the PSK variant with a zero PSK. Provides the common no-PSK use case without duplicating logic.
- **Tests [WEAK]**: Used in all five inline tests covering happy path and several rejection scenarios. However it is only tested with the implicit zero PSK; no test distinguishes it from create_initiation_psk or verifies the OsRng / system-clock path specifically.
- **PARTIAL [PARTIAL]**: Public function with a single-line `///` comment noting it is a std convenience wrapper. No parameter descriptions, no return value explanation (`(Initiation, InitiatorHandshake)` tuple), and no `# Examples` section. The PSK defaulting to all-zeros is not documented. (deliberated: confirmed — Tests WEAK valid — only tested with implicit zero PSK, never distinguished from create_initiation_psk. Documentation PARTIAL correct — the zero-PSK defaulting behavior is undocumented. Both findings confirmed.)

#### `create_initiation_psk` (L95–L104)

- **Utility [USED]**: Called by create_initiation (line 91) and used as entry point for PSK-based handshakes
- **Duplication [UNIQUE]**: Convenience wrapper that generates ephemeral key and timestamp, then delegates to create_initiation_with. Distinct function serving specific purpose.
- **Correction [OK]**: Correctly generates a fresh random ephemeral secret and current TAI64n timestamp before delegating to the core function.
- **Overengineering [ACCEPTABLE]**: Middle tier in a three-layer delegation: plain → psk → with. The split between this and create_initiation_with is justified: this layer injects OS entropy and the system clock (std-only concerns), while the _with variant is deterministic for no_std and testing. Slight verbosity but the separation of concerns is sound for a crypto library.
- **Tests [WEAK]**: Only reached through create_initiation (zero PSK). No test passes a non-zero PSK through this wrapper, so the psk parameter branch is untested. The core matching-PSK / mismatched-PSK scenarios are absent.
- **PARTIAL [PARTIAL]**: Public function with a minimal one-line `///` comment. No documentation for the `psk` parameter semantics, no return description, and no `# Examples`. The PSK length constraint (32 bytes) is implied by the type but not stated. (deliberated: confirmed — Overengineering ACCEPTABLE is correct — the three-tier delegation (plain → psk → with) separates std concerns (OsRng, system clock) from the deterministic core, which is sound design for a crypto library needing testability and no_std support. Tests WEAK valid — no non-zero PSK test path. Documentation PARTIAL correct — PSK semantics undocumented.)

#### `create_initiation_with` (L108–L170)

- **Utility [USED]**: Called by create_initiation_psk (line 103); core function implementing initiator handshake message creation
- **Duplication [UNIQUE]**: Core handshake initiation logic similar to kmod version (0.898 score) but context-specific: uses PublicKey/StaticSecret wrapper types, struct-based message format, and different error handling. Non-interchangeable due to different abstraction levels.
- **Correction [OK]**: Correctly implements Noise_IKpsk2 initiation: MixHash(S_pub_r), MixHash(e_pub_i), MixKey(DH(e_i, S_r)), EncryptAndHash(S_pub_i), MixKey(DH(S_i, S_r)), EncryptAndHash(timestamp). MAC1 is computed over the first 116 wire bytes (1+3+4+32+48+28), which is correct for WireGuard's Initiation message layout.
- **Overengineering [LEAN]**: Core no_std-compatible function. Length is proportional to the Noise_IKpsk2 spec: every statement maps to a named protocol step. No unnecessary abstraction.
- **Tests [WEAK]**: Core initiator logic exercised transitively by all handshake tests, covering the full happy path. Missing: test with non-zero PSK, test with a controlled deterministic ephemeral (no_std path), and no test verifying the mac1 slice boundary (wire[..116]).
- **PARTIAL [PARTIAL]**: Most complex public function; has a two-line `///` comment noting it is the core no_std-compatible entry point. None of the six parameters are documented, the return tuple is not described, and there is no `# Examples` section. The inline comments inside the body are helpful but do not substitute for API-level documentation. (deliberated: confirmed — Tests WEAK valid — core function missing non-zero PSK test, deterministic ephemeral test, and mac1 boundary verification. Documentation PARTIAL correct — most complex public function with six undocumented parameters and no examples. Both findings confirmed.)

#### `process_response` (L173–L221)

- **Utility [USED]**: Called in full_handshake_and_transport test (line 392) and failure scenario tests (lines 450, 465)
- **Duplication [UNIQUE]**: Similar protocol logic to kmod version (0.846 score) but different semantic contracts: takes InitiatorHandshake struct and Response type instead of raw state arrays. Different input/output abstractions make functions non-interchangeable.
- **Correction [NEEDS_FIX]**: The function performs no MAC1 verification of the Response message before executing two expensive DH operations. The initiator's sender_index is transmitted in plaintext in the Initiation message, so an observer can craft a fake Response with the correct receiver_index, bypassing the only cheap early-exit check (line 183) and forcing full DH computation. compute_mac1(&our_static.public_key(), &wire[..60]) should be verified against msg.mac1 before any DH work, mirroring the responder's check in process_initiation_with.
- **Overengineering [LEAN]**: Implements the initiator's response-processing path step-by-step per spec. Consuming InitiatorHandshake by value ensures the ephemeral is single-use and the struct is zeroized after use — idiomatic and correct.
- **Tests [WEAK]**: Three tests cover the happy path, tampered encrypted_empty, and mismatched receiver_index. Missing: PSK mismatch (different PSK on each side), wrong static key for DH2, and verifying the initiator-sends/responder-receives key assignment.
- **PARTIAL [PARTIAL]**: Public function with a single-line `///` comment. The `Option` return (indicating handshake failure) is not documented, nor are the conditions that cause `None` (index mismatch, AEAD failure). No `# Parameters`, `# Returns`, or `# Examples` sections. (deliberated: confirmed — Correction NEEDS_FIX is legitimate: the Response message carries a mac1 field (computed in process_initiation_with at line 354 over resp_wire[..60]), but process_response never verifies it. Since the Initiation's sender_index is transmitted in cleartext, an observer can forge Response messages with matching receiver_index, bypassing the only cheap guard (line 183) and forcing two DH operations. The responder side (process_initiation_with) correctly verifies MAC1 first — the asymmetry is a real oversight. This is DoS-grade, not authentication-breaking (AEAD would still reject), hence medium severity is appropriate. Tests WEAK valid — missing PSK mismatch and wrong-static-key scenarios. Documentation PARTIAL correct — Option return conditions undocumented.)

#### `process_initiation` (L227–L233)

- **Utility [USED]**: Called in tests (lines 390, 408, 428, 442); convenience wrapper for responder initiation processing
- **Duplication [UNIQUE]**: Convenience wrapper that delegates to process_initiation_psk with default PSK and no last_timestamp. Distinct function with clear delegating relationship.
- **Correction [OK]**: Correctly delegates to process_initiation_psk with a zero PSK and no prior timestamp, which is the correct WireGuard default behavior.
- **Overengineering [LEAN]**: Thin std convenience that delegates to the PSK variant with zero PSK and no timestamp check. Provides clean API for the common case.
- **Tests [WEAK]**: Convenience wrapper tested in all five inline tests. Because it always passes zero PSK and no last_timestamp, the replay-protection branch and non-zero PSK paths inside process_initiation_with are never exercised through this entry point.
- **PARTIAL [PARTIAL]**: Public function with a single-line `///` comment. The four-element return tuple `(PublicKey, Tai64n, Response, TransportSession)` is not explained, failure conditions for `None` are not listed, and there is no `# Examples` section. (deliberated: confirmed — Tests WEAK valid — always passes zero PSK and no last_timestamp, leaving replay-protection and PSK branches untested. Documentation PARTIAL correct — four-element return tuple unexplained. Both findings confirmed.)

#### `process_initiation_psk` (L237–L246)

- **Utility [USED]**: Called by process_initiation (line 232); PSK-based responder handshake processing
- **Duplication [UNIQUE]**: Convenience wrapper that generates random ephemeral key and delegates to process_initiation_with. Distinct function serving specific purpose.
- **Correction [OK]**: Correctly generates a fresh random responder ephemeral before delegating to the core function. No issues found.
- **Overengineering [ACCEPTABLE]**: Same three-tier delegation rationale as create_initiation_psk. This layer owns the std-specific random ephemeral generation, keeping process_initiation_with testable and no_std-compatible. Justified, if slightly verbose.
- **Tests [WEAK]**: Only called transitively with psk=[0u8;32] and last_timestamp=None. No test verifies PSK matching, PSK mismatch rejection, or the replay-protection branch activated by a non-None last_timestamp.
- **PARTIAL [PARTIAL]**: Public function with a one-line `///` comment. The `last_timestamp` replay-protection parameter is significant but undocumented. No parameter, return, or `# Examples` sections. (deliberated: confirmed — Overengineering ACCEPTABLE correct — same three-tier rationale as create_initiation_psk, justified for std/no_std separation. Tests WEAK valid — PSK matching/mismatch and replay protection untested. Documentation PARTIAL correct — critical last_timestamp parameter undocumented.)

#### `process_initiation_with` (L249–L357)

- **Utility [USED]**: Called by process_initiation_psk (line 245); core function implementing responder initiation processing
- **Duplication [UNIQUE]**: Core responder-side handshake logic similar to kmod version (0.878 score) but context-specific: uses PublicKey/StaticSecret wrappers, struct-based messages (Initiation/Response), and Option-based timestamp validation. Different abstraction levels and input/output handling make non-interchangeable.
- **Correction [OK]**: Correctly implements the responder side: MAC1 verified first via constant_time_eq, then full Noise_IKpsk2 state evolution (DH(S_r,e_i), decrypt static, DH(S_r,S_i), decrypt timestamp, replay check), response construction (MixHash(e_r), DH(e_r,e_i), DH(e_r,S_i), PSK phase, EncryptAndHash(empty)), and transport key derivation with correct key ordering (recv=first, send=second).
- **Overengineering [LEAN]**: Long but not complex: each block maps directly to a Noise protocol step. MAC1 is checked first (cheap before DH), timestamp replay protection is inline. No gratuitous abstractions.
- **Tests [WEAK]**: Covers MAC1 rejection (wrong key), AEAD failure (tampered encrypted_static), and happy path. Critically absent: timestamp replay protection (last_timestamp branch never tested), PSK mismatch scenario, tampered mac1 field, and tampered ephemeral or encrypted_timestamp fields.
- **PARTIAL [PARTIAL]**: Largest public function in the file with a single-line `///` comment. None of the six parameters (including the critical `last_timestamp` replay guard and `resp_eph`) are documented. The complex four-element return tuple and all `None` failure conditions are undescribed. No `# Examples` section. (deliberated: confirmed — Tests WEAK valid — timestamp replay protection (last_timestamp branch) is never exercised, PSK mismatch absent, tampered mac1/ephemeral/encrypted_timestamp fields untested. These are critical gaps for security-sensitive code. Documentation PARTIAL correct — largest public function with six undocumented parameters including the critical last_timestamp replay guard. Both findings confirmed.)

#### `constant_time_eq` (L360–L363)

- **Utility [USED]**: Called in process_initiation_with (line 279) for constant-time MAC comparison; encapsulates subtle::ConstantTimeEq conversion
- **Duplication [UNIQUE]**: Uses subtle crate's ConstantTimeEq trait wrapper, while kmod version uses unsafe wg_crypto_memneq. Different implementation approaches (library vs raw unsafe code) despite similar purpose.
- **Correction [OK]**: Correctly uses the subtle crate's ConstantTimeEq trait for timing-safe MAC comparison. ct_eq on fixed-size arrays and conversion via Into<bool> is the idiomatic correct usage.
- **Overengineering [LEAN]**: Correct use of the 'subtle' crate for constant-time MAC comparison. The thin wrapper gives a readable call site without hiding what it does.
- **Tests [WEAK]**: Indirectly exercised by wrong_responder_key_rejects (MAC1 values differ). No direct unit test with equal arrays, one-bit-different arrays, or any assertion that the subtle crate's ct_eq is being used rather than a naive comparison.
- **UNDOCUMENTED [UNDOCUMENTED]**: Private function with no `///` doc comment. The name is self-explanatory and it delegates to `subtle::ConstantTimeEq`, but the security rationale (MAC comparison must not short-circuit) is worth documenting even for private helpers in a cryptographic codebase. (deliberated: confirmed — Tests WEAK valid — only exercised via MAC1 mismatch path, no direct test with equal/near-equal arrays. UNDOCUMENTED correct — private crypto function with security rationale worth documenting (MAC comparison must not short-circuit). Both findings confirmed.)

## Best Practices — 9.5/10

### Rules

| # | Rule | Status | Severity | Detail |
|---|------|--------|----------|--------|
| 4 | Derive common traits on public types | WARN | MEDIUM | InitiatorHandshake (pub) derives only Zeroize and ZeroizeOnDrop, omitting Debug, Clone, and PartialEq. While intentional omission of Clone (prevents accidental key duplication) and PartialEq (avoids non-constant-time comparison) is a sound security decision, the absence of a custom Debug impl means users cannot inspect the struct during development at all. A redacting Debug impl (masking key fields) would satisfy the rule's intent without leaking key material. [L73-L81] |
| 11 | Memory safety | WARN | HIGH | InitiatorHandshake correctly derives ZeroizeOnDrop, ensuring ck, h, ephemeral, and psk are cleared on drop. However, in process_response the fields are copied out of the struct into local variables (let mut ck = state.ck; let psk = state.psk;). Because [u8; 32] is Copy, these stack locals hold live key material but are not wrapped in Zeroizing<T>, so they will not be scrubbed when the function returns. The original struct fields will still be zeroized via ZeroizeOnDrop, but the copies persist on the stack frame until overwritten by the OS/allocator. [L176-L181] |

### Suggestions

- Wrap sensitive local stack copies in Zeroizing<T> inside process_response to ensure they are cleared on scope exit, matching the intent of ZeroizeOnDrop on the parent struct.
  ```typescript
  // Before
  let mut ck = state.ck;
  let mut h = state.h;
  let psk = state.psk;
  // After
  use zeroize::Zeroizing;
  let mut ck = Zeroizing::new(state.ck);
  let mut h = Zeroizing::new(state.h);
  let psk = Zeroizing::new(state.psk);
  ```
- Add a custom Debug impl for InitiatorHandshake that redacts key material, preserving debuggability without leaking secrets.
  ```typescript
  // Before
  #[derive(Zeroize, ZeroizeOnDrop)]
  pub struct InitiatorHandshake { .. }
  // After
  #[derive(Zeroize, ZeroizeOnDrop)]
  pub struct InitiatorHandshake { .. }
  
  impl core::fmt::Debug for InitiatorHandshake {
      fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
          f.debug_struct("InitiatorHandshake")
              .field("sender_index", &self.sender_index)
              .field("ck", &"[redacted]")
              .field("h", &"[redacted]")
              .field("psk", &"[redacted]")
              .finish()
      }
  }
  ```

## Actions

### Quick Wins

- **[correction · medium · small]** Add MAC1 verification in process_response before any DH operations: compute expected_mac1 = compute_mac1(&our_static.public_key(), &msg.to_bytes()[..60]) and return None if it does not constant-time-equal msg.mac1. Without this check, an attacker who observed the plaintext sender_index in the Initiation can craft fake Response messages that survive the receiver_index guard and force two DH exponentiations per fake packet. [L183]

### Hygiene

- **[documentation · low · trivial]** Complete JSDoc documentation for: `initial_chain_key` (`initial_chain_key`) [L23-L25]
- **[documentation · low · trivial]** Complete JSDoc documentation for: `mix_hash` (`mix_hash`) [L32-L34]
- **[documentation · low · trivial]** Complete JSDoc documentation for: `mix_key` (`mix_key`) [L38-L41]
- **[documentation · low · trivial]** Complete JSDoc documentation for: `encrypt_and_hash` (`encrypt_and_hash`) [L44-L48]
- **[documentation · low · trivial]** Complete JSDoc documentation for: `decrypt_and_hash` (`decrypt_and_hash`) [L51-L55]
- **[documentation · low · trivial]** Complete JSDoc documentation for: `compute_mac1` (`compute_mac1`) [L59-L65]
- **[documentation · low · trivial]** Complete JSDoc documentation for: `InitiatorHandshake` (`InitiatorHandshake`) [L72-L81]
- **[documentation · low · trivial]** Complete JSDoc documentation for: `create_initiation` (`create_initiation`) [L85-L91]
- **[documentation · low · trivial]** Complete JSDoc documentation for: `create_initiation_psk` (`create_initiation_psk`) [L95-L104]
- **[documentation · low · trivial]** Complete JSDoc documentation for: `create_initiation_with` (`create_initiation_with`) [L108-L170]
- **[documentation · low · trivial]** Complete JSDoc documentation for: `process_response` (`process_response`) [L173-L221]
- **[documentation · low · trivial]** Complete JSDoc documentation for: `process_initiation` (`process_initiation`) [L227-L233]
- **[documentation · low · trivial]** Complete JSDoc documentation for: `process_initiation_psk` (`process_initiation_psk`) [L237-L246]
- **[documentation · low · trivial]** Complete JSDoc documentation for: `process_initiation_with` (`process_initiation_with`) [L249-L357]
