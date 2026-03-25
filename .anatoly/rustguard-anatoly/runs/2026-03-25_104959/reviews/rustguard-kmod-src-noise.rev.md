# Review: `rustguard-kmod/src/noise.rs`

**Verdict:** NEEDS_REFACTOR

## Symbols

| Symbol | Kind | Exported | Correction | Over-eng. | Utility | Duplication | Tests | Confidence |
|--------|------|----------|------------|-----------|---------|-------------|-------|------------|
| zeroize | function | yes | OK | LEAN | USED | UNIQUE | NONE | 85% |
| constant_time_eq | function | no | OK | LEAN | USED | DUPLICATE | NONE | 85% |
| CONSTRUCTION | constant | no | OK | LEAN | USED | UNIQUE | GOOD | 70% |
| IDENTIFIER | constant | no | OK | LEAN | USED | UNIQUE | GOOD | 70% |
| LABEL_MAC1 | constant | no | OK | LEAN | USED | UNIQUE | GOOD | 70% |
| TAI64_EPOCH_OFFSET | constant | no | OK | LEAN | USED | UNIQUE | GOOD | 85% |
| hash | function | no | NEEDS_FIX | LEAN | USED | DUPLICATE | NONE | 85% |
| mac | function | no | OK | LEAN | USED | DUPLICATE | NONE | 85% |
| hkdf | function | no | OK | LEAN | USED | UNIQUE | NONE | 80% |
| seal | function | no | OK | LEAN | USED | UNIQUE | NONE | 80% |
| open | function | no | OK | LEAN | USED | UNIQUE | NONE | 80% |
| dh | function | no | OK | LEAN | USED | UNIQUE | NONE | 82% |
| generate_keypair | function | no | OK | LEAN | USED | UNIQUE | NONE | 82% |
| random_bytes | function | no | OK | LEAN | DEAD | DUPLICATE | NONE | 75% |
| initial_chain_key | function | no | OK | LEAN | USED | UNIQUE | NONE | 70% |
| initial_hash | function | no | OK | LEAN | USED | UNIQUE | NONE | 70% |
| mix_hash | function | no | OK | LEAN | USED | UNIQUE | NONE | 70% |
| mix_key | function | no | OK | LEAN | USED | UNIQUE | NONE | 70% |
| encrypt_and_hash | function | no | OK | LEAN | USED | UNIQUE | NONE | 82% |
| decrypt_and_hash | function | no | OK | LEAN | USED | UNIQUE | NONE | 82% |
| compute_mac1 | function | no | OK | LEAN | USED | UNIQUE | NONE | 85% |
| tai64n_now | function | no | OK | LEAN | USED | UNIQUE | NONE | 85% |
| MSG_INITIATION | constant | yes | OK | LEAN | USED | UNIQUE | GOOD | 92% |
| MSG_RESPONSE | constant | yes | OK | LEAN | USED | UNIQUE | GOOD | 92% |
| MSG_TRANSPORT | constant | yes | OK | LEAN | USED | UNIQUE | GOOD | 90% |
| INITIATION_SIZE | constant | yes | OK | LEAN | USED | UNIQUE | GOOD | 90% |
| RESPONSE_SIZE | constant | yes | OK | LEAN | USED | UNIQUE | GOOD | 90% |
| TransportKeys | class | yes | OK | LEAN | USED | UNIQUE | NONE | 88% |
| InitiatorState | class | yes | OK | LEAN | USED | UNIQUE | GOOD | 85% |
| create_initiation | function | yes | OK | LEAN | USED | UNIQUE | NONE | 88% |
| process_response | function | yes | NEEDS_FIX | LEAN | USED | UNIQUE | NONE | 85% |
| process_initiation | function | yes | OK | LEAN | USED | UNIQUE | NONE | 88% |

### Details

#### `zeroize` (L46–L50)

- **Utility [USED]**: Exported pub(crate) function used extensively within the file: called in create_initiation (lines 308, 313, 323), process_response (lines 378, 381, 383, 385, 390, 391, 412, 414, 417, 419, 422), and process_initiation (lines 488, 492, 506, 510, 520) for secure wiping of sensitive key material.
- **Duplication [UNIQUE]**: No similar functions found in RAG results. Unique wrapper around wg_memzero.
- **Correction [OK]**: Correctly guards against empty slice before calling wg_memzero, preventing a null/dangling-pointer dereference in C for zero-length buffers.
- **Overengineering [LEAN]**: Minimal safe wrapper around the FFI memzero call with a correct empty-slice guard. No unnecessary abstraction.
- **Tests [NONE]**: No test file exists for rustguard-kmod/src/noise.rs. zeroize wraps wg_memzero (FFI) and is security-critical; no unit or integration tests verify it correctly clears sensitive buffers.
- **PARTIAL [PARTIAL]**: Has a single-line `///` doc describing purpose and the non-optimization guarantee. No parameter description for `buf`, no `# Examples` section. pub(crate) function warrants at least a `# Safety` or `# Panics` note. (deliberated: confirmed — Tests NONE is accurate — no test file exists for this module. Documentation PARTIAL is fair: has a `///` doc but lacks `# Safety` note for a pub(crate) function wrapping unsafe FFI. No reclassification needed.)

#### `constant_time_eq` (L53–L58)

- **Utility [USED]**: Non-exported helper function called in process_response (line 377) to verify MAC1 and process_initiation (line 451) to verify MAC1, implementing constant-time comparison for authentication tags.
- **Duplication [DUPLICATE]**: Identical implementation duplicated in rustguard-kmod/src/cookie.rs with score 0.982. Same signature, same FFI call to wg_crypto_memneq, same logic.
- **Correction [OK]**: Length mismatch short-circuit is acceptable since lengths are public; equal-length comparison is delegated to wg_crypto_memneq which provides constant-time behaviour.
- **Overengineering [LEAN]**: Thin wrapper delegating to kernel crypto_memneq with a length pre-check. Exactly the right amount of code.
- **Tests [NONE]**: No test file found. constant_time_eq is a security-critical primitive used in MAC verification; neither the equal-length/unequal-length branches nor the timing-safe property is tested.
- **PARTIAL [PARTIAL]**: Private function with a brief `///` doc stating purpose and underlying primitive. Length-mismatch behavior (returns false) is not documented. No examples. Leniency applied for private items. (deliberated: confirmed — DUPLICATE is correct — 98% identical to cookie.rs version, same signature and FFI call. Both files need this primitive; extracting to a shared crypto_utils module would be appropriate. Tests NONE accurate. Documentation PARTIAL fair for a private function with brief doc.)

> **Duplicate of** `rustguard-kmod/src/cookie.rs:constant_time_eq` — 98% identical — both use wg_crypto_memneq for constant-time comparison

#### `CONSTRUCTION` (L62–L62)

- **Utility [USED]**: Non-exported constant used in initial_chain_key (line 182) via hash function call, forming the WireGuard Noise protocol construction string.
- **Duplication [UNIQUE]**: Protocol constant specific to Noise_IKpsk2. No duplicates flagged in RAG.
- **Correction [OK]**: Correct Noise_IKpsk2 construction string per WireGuard specification.
- **Overengineering [LEAN]**: Direct wire-format protocol constant from the Noise spec. No abstraction needed.
- **Tests [GOOD]**: Compile-time constant with no runtime behavior. Per rule 6, constants are GOOD by default.
- **UNDOCUMENTED [UNDOCUMENTED]**: Private constant with no `///` doc comment. Self-descriptive name and section header comment exist, but no `///` is present. Leniency applied for private items. (deliberated: confirmed — Private protocol constant with no `///` doc. Name is self-descriptive to WireGuard/Noise protocol implementers, and leniency applies for private items. Keeping UNDOCUMENTED but raising confidence slightly since it's a standard protocol string.)

#### `IDENTIFIER` (L63–L63)

- **Utility [USED]**: Non-exported constant used in initial_hash (line 186) via hash function call to incorporate WireGuard protocol identifier.
- **Duplication [UNIQUE]**: Protocol constant specific to WireGuard. No duplicates in RAG.
- **Correction [OK]**: Correct WireGuard protocol identifier string per specification.
- **Overengineering [LEAN]**: Direct wire-format protocol constant from the WireGuard spec. No abstraction needed.
- **Tests [GOOD]**: Compile-time constant with no runtime behavior. Per rule 6, constants are GOOD by default.
- **UNDOCUMENTED [UNDOCUMENTED]**: Private constant with no `///` doc comment. Name alone does not explain the WireGuard protocol role. Leniency applied for private items. (deliberated: confirmed — Same rationale as CONSTRUCTION — private protocol constant, no `///` doc, self-descriptive name. UNDOCUMENTED is technically correct.)

#### `LABEL_MAC1` (L64–L64)

- **Utility [USED]**: Non-exported constant used in compute_mac1 (line 215) to derive MAC1 key material for message authentication.
- **Duplication [UNIQUE]**: Protocol constant used for MAC1 derivation. No duplicates in RAG.
- **Correction [OK]**: Correct eight-byte MAC1 label constant per WireGuard spec.
- **Overengineering [LEAN]**: Direct protocol label constant. Exactly what the spec requires.
- **Tests [GOOD]**: Compile-time constant with no runtime behavior. Per rule 6, constants are GOOD by default.
- **UNDOCUMENTED [UNDOCUMENTED]**: Private constant with no `///` doc comment. Name is self-descriptive but no explanation of use in MAC1 derivation is provided. Leniency applied for private items. (deliberated: confirmed — Private protocol label constant with no doc. Self-descriptive name. UNDOCUMENTED is correct but low priority given private scope and domain convention.)

#### `TAI64_EPOCH_OFFSET` (L67–L67)

- **Utility [USED]**: Non-exported constant used in tai64n_now (line 234) for TAI64N timestamp calculation, adding epoch offset to system time.
- **Duplication [UNIQUE]**: Specific TAI64N epoch offset constant. No duplicates in RAG.
- **Correction [OK]**: 0x4000_0000_0000_000a equals 2^62 + 10, the correct TAI64 base with 10 leap-second offset used by WireGuard.
- **Overengineering [LEAN]**: Single named constant for the TAI64 epoch offset. Correct and minimal.
- **Tests [GOOD]**: Compile-time constant with no runtime behavior. Per rule 6, constants are GOOD by default.
- **PARTIAL [PARTIAL]**: Has `///` doc 'TAI64N epoch offset (2^62 + 10 leap seconds).' which explains the hex literal composition. Brief but adequate for a private constant; no reference to leap-second count or TAI-UTC relationship beyond the parenthetical. (deliberated: confirmed — Has `///` doc explaining the hex literal composition. PARTIAL is fair — the doc is brief but adequate for a private constant. No reclassification needed.)

#### `hash` (L72–L92)

- **Utility [USED]**: Non-exported BLAKE2s hash function called in initial_chain_key (line 182), initial_hash (line 186), mix_hash (line 190), and compute_mac1 (lines 215, 215).
- **Duplication [DUPLICATE]**: Duplicated in rustguard-kmod/src/cookie.rs with score 0.966. Identical C FFI wrapper around wg_blake2s_hash with same array construction and chunking logic.
- **Correction [NEEDS_FIX]**: Both inner loops are capped at 8 iterations via .take(8), so ptrs and lens only have valid entries for indices 0..min(chunks.len(),8). However, chunks.len() as u32 is unconditionally passed as num_chunks to wg_blake2s_hash. If chunks.len() > 8, the C function receives a count larger than the populated portion of the arrays and will read beyond them—out-of-bounds in C memory. All current call sites pass at most 2 chunks so the bug is latent, but the function provides no assertion or cap on num_chunks to enforce safety.
- **Overengineering [LEAN]**: The fixed-size [8] arrays for FFI pointer/length passing are verbose but necessary in a no_std kernel context without Vec. The silent take(8) truncation cap is a minor implicit limit but the WireGuard protocol never exceeds it. Overall this is the minimal viable FFI adapter.
- **Tests [NONE]**: No test file found for this module. hash delegates to wg_blake2s_hash (FFI); no tests verify chunk concatenation semantics, boundary behavior (empty chunks, 8-chunk limit), or output determinism.
- **PARTIAL [PARTIAL]**: Private function with `///` doc 'BLAKE2s-256 hash of concatenated chunks.' Describes algorithm and input shape. No parameter docs, no return description beyond type, no examples. Leniency applied for private items. (deliberated: confirmed — Correction NEEDS_FIX is valid: chunks.len() is passed to the C function but only 8 array slots are populated via .take(8). If chunks.len()>8, OOB read occurs in C. All current callers pass ≤2 chunks so it's latent, but a debug_assert or min() is warranted. DUPLICATE with cookie.rs is genuine — identical FFI wrapper pattern. Tests NONE and docs PARTIAL both accurate.)

> **Duplicate of** `rustguard-kmod/src/cookie.rs:hash` — 97% identical — both build ptr/len arrays and call wg_blake2s_hash

#### `mac` (L95–L105)

- **Utility [USED]**: Non-exported keyed BLAKE2s MAC function called in compute_mac1 (line 217) to compute authentication tags for MAC1 and MAC2.
- **Duplication [DUPLICATE]**: Duplicated in rustguard-kmod/src/cookie.rs with score 0.983. Identical C FFI wrapper around wg_blake2s_256_mac.
- **Correction [OK]**: Correctly delegates to wg_blake2s_256_mac with accurate key and data length parameters.
- **Overengineering [LEAN]**: Straightforward thin wrapper over the C BLAKE2s MAC shim with no unnecessary indirection.
- **Tests [NONE]**: No test file found. mac wraps wg_blake2s_256_mac (FFI); no tests verify key-sensitivity, data-sensitivity, or known-vector correctness.
- **PARTIAL [PARTIAL]**: Private function with `///` doc 'Keyed BLAKE2s MAC (for MAC1/MAC2).' States algorithm and use sites but omits parameter semantics, output truncation behavior, and examples. (deliberated: confirmed — DUPLICATE with cookie.rs is genuine — 98% identical FFI wrapper. Tests NONE accurate. Documentation PARTIAL fair — has `///` doc stating algorithm and use but lacks parameter descriptions.)

> **Duplicate of** `rustguard-kmod/src/cookie.rs:mac` — 98% identical — both call wg_blake2s_256_mac with same FFI pattern

#### `hkdf` (L108–L119)

- **Utility [USED]**: Non-exported key derivation function called in mix_key (line 194), process_response (lines 407, 421), and process_initiation (line 516).
- **Duplication [UNIQUE]**: C FFI wrapper unique to kernel module. RAG shows 0.911 match with rustguard-crypto pure-Rust HKDF, but fundamentally different — kernel uses C function, crypto crate uses Rust crypto libraries. Different implementations warrant separate versions.
- **Correction [OK]**: Correctly delegates to wg_hkdf; callers passing an empty slice produce a valid zero-length pointer which is safe in C with a corresponding zero length argument.
- **Overengineering [LEAN]**: Clean FFI wrapper returning a named triple. The C shim owns the complexity; this is appropriately thin.
- **Tests [NONE]**: No test file found. hkdf is the core KDF used in all handshake key derivation; no tests verify output independence, determinism, or agreement with WireGuard test vectors.
- **PARTIAL [PARTIAL]**: Private function with `///` doc 'HKDF extract + expand.' Very terse; does not explain the three output keys, WireGuard-specific usage, or whether out3 is ever used. Leniency applied for private items. (deliberated: confirmed — Tests NONE accurate — core KDF with no tests. Documentation PARTIAL fair — very terse doc. No reclassification needed.)

#### `seal` (L122–L136)

- **Utility [USED]**: Non-exported AEAD encryption function called in encrypt_and_hash (line 201) for ChaCha20-Poly1305 encryption during handshake.
- **Duplication [UNIQUE]**: C FFI wrapper for ChaCha20-Poly1305 encryption. RAG scores (0.872, 0.762) indicate similar semantics to rustguard-crypto versions, but different underlying implementations (C FFI vs pure Rust) and different buffer handling patterns (mutable buffer output vs Vec return).
- **Correction [OK]**: Correctly computes output length as plaintext.len() + AEAD_TAG_SIZE on success; delegates encrypt to C shim with correct parameter mapping.
- **Overengineering [LEAN]**: Minimal AEAD encrypt wrapper. Converts C int return to Option and computes output length. No unnecessary abstraction.
- **Tests [NONE]**: No test file found. seal performs AEAD encryption; no tests cover happy path, empty plaintext, wrong-key detection, or return-value semantics.
- **PARTIAL [PARTIAL]**: Private function with `///` doc mentioning algorithm, tag size, and None failure condition. Return semantics (Some(len) = plaintext_len + tag) are stated inline. Missing parameter descriptions and examples. Leniency applied for private items. (deliberated: confirmed — Tests NONE accurate for untested AEAD encrypt wrapper. Documentation PARTIAL correct — has doc mentioning algorithm and None failure but missing parameter descriptions.)

#### `open` (L139–L153)

- **Utility [USED]**: Non-exported AEAD decryption function called in decrypt_and_hash (line 208) for ChaCha20-Poly1305 decryption during handshake.
- **Duplication [UNIQUE]**: C FFI wrapper for ChaCha20-Poly1305 decryption. RAG score 0.891 with rustguard-crypto::open reflects semantic similarity but different implementation (C FFI vs pure Rust) and buffer handling.
- **Correction [OK]**: Returns ciphertext.len() - AEAD_TAG_SIZE on success. All call sites pass ciphertext buffers whose lengths are at least AEAD_TAG_SIZE (16 bytes), so no usize underflow occurs in practice. The C shim rejects short ciphertext before returning 0.
- **Overengineering [LEAN]**: Minimal AEAD decrypt wrapper mirroring seal. Correctly subtracts tag size from output length.
- **Tests [NONE]**: No test file found. open performs AEAD decryption; no tests cover authentication failure, tampered ciphertext, or roundtrip with seal.
- **PARTIAL [PARTIAL]**: Private function with `///` doc 'AEAD decrypt. Returns plaintext length or None on auth failure.' Concise but no parameter docs, no note on ciphertext minimum length, no examples. (deliberated: confirmed — Mirror of seal — tests NONE and documentation PARTIAL both accurate. No reclassification needed.)

#### `dh` (L156–L160)

- **Utility [USED]**: Non-exported Curve25519 DH function called in create_initiation (lines 304, 315), process_response (lines 396, 403), and process_initiation (lines 484, 494, 507, 513).
- **Duplication [UNIQUE]**: No similar functions found in RAG results. Unique Curve25519 DH wrapper.
- **Correction [OK]**: Correctly wraps wg_curve25519 and propagates low-order-point rejection (non-zero return) as None.
- **Overengineering [LEAN]**: Single-line FFI call wrapped in Option. Cannot be simpler.
- **Tests [NONE]**: No test file found. dh wraps wg_curve25519; the critical zero-result (low-order point) failure path and happy-path DH agreement are untested.
- **PARTIAL [PARTIAL]**: Private function with `///` doc 'Curve25519 DH.' Extremely brief. Does not explain the None return (low-order point / all-zero output), which is security-relevant. Leniency applied for private items. (deliberated: confirmed — Tests NONE accurate — untested DH wrapper. Documentation PARTIAL fair — doc says 'Curve25519 DH.' but doesn't explain None return for low-order points.)

#### `generate_keypair` (L163–L171)

- **Utility [USED]**: Non-exported ephemeral keypair generation called in create_initiation (line 299) and process_initiation (line 475).
- **Duplication [UNIQUE]**: No similar functions found in RAG results. Unique ephemeral key generation wrapper.
- **Correction [OK]**: Correctly generates the scalar secret first then derives the public key from it via wg_curve25519_generate_public.
- **Overengineering [LEAN]**: Straightforward two-call FFI sequence to generate a Curve25519 keypair. No abstraction overhead.
- **Tests [NONE]**: No test file found. generate_keypair calls FFI key generation; no tests verify the returned public key is the correct scalar multiple of the secret.
- **PARTIAL [PARTIAL]**: Private function with `///` doc 'Generate ephemeral keypair.' States purpose; does not document return tuple order (secret, public) which callers must know. Leniency applied. (deliberated: confirmed — Tests NONE accurate. Documentation PARTIAL fair — doc says purpose but omits return tuple order (secret, public). Both findings consistent.)

#### `random_bytes` (L173–L177)

- **Utility [DEAD]**: Non-exported generic function for random byte generation. Defined but never called anywhere in the file. May be dead code or intended for future use.
- **Duplication [DUPLICATE]**: Duplicated in rustguard-kmod/src/cookie.rs with score 0.971. Identical generic function generating random byte arrays via wg_get_random_bytes.
- **Correction [OK]**: Correctly fills a const-generic stack buffer from the kernel CSPRNG.
- **Overengineering [LEAN]**: Const-generic size parameter is idiomatic Rust and removes the need for caller-side array sizing boilerplate. Not over-generalized.
- **Tests [NONE]**: No test file found. random_bytes wraps wg_get_random_bytes; no tests verify the buffer is populated or that the generic const parameter N is handled correctly.
- **UNDOCUMENTED [UNDOCUMENTED]**: Private generic function with no `///` doc comment. Name is self-descriptive. Leniency applied for private items. (deliberated: confirmed — DEAD is correct — private function with no callers in this file and no pub visibility for cross-module use. DUPLICATE with cookie.rs is real. If it's dead here, removal resolves both issues simultaneously. Tests NONE and UNDOCUMENTED both accurate. Raising confidence since the function is genuinely unreferenced.)

> **Duplicate of** `rustguard-kmod/src/cookie.rs:random_bytes` — 97% identical — both fill buffer with wg_get_random_bytes

#### `initial_chain_key` (L181–L183)

- **Utility [USED]**: Non-exported helper called in create_initiation (line 291) and process_initiation (line 469) to initialize Noise chain key.
- **Duplication [UNIQUE]**: Trivial one-liner helper. No similar functions in RAG.
- **Correction [OK]**: Correctly computes Ci = HASH(CONSTRUCTION) per the Noise protocol specification.
- **Overengineering [LEAN]**: Named one-liner encoding a specific Noise protocol step. Improves readability over inlining.
- **Tests [NONE]**: No test file found. initial_chain_key computes the protocol-defined hash of CONSTRUCTION; no tests verify the output against a known WireGuard reference vector.
- **UNDOCUMENTED [UNDOCUMENTED]**: Private function with no `///` doc comment. Name is meaningful to Noise protocol readers but is undocumented. Leniency applied for private items. (deliberated: confirmed — Tests NONE accurate. UNDOCUMENTED correct — no `///` doc. Name is meaningful to Noise protocol readers. Low priority given private scope and domain knowledge.)

#### `initial_hash` (L185–L187)

- **Utility [USED]**: Non-exported helper called in create_initiation (line 292) and process_initiation (line 470) to initialize Noise hash state.
- **Duplication [UNIQUE]**: Trivial one-liner helper. No similar functions in RAG.
- **Correction [OK]**: Correctly computes Hi = HASH(Ci || IDENTIFIER); the &[u8; 32] reference coerces to &[u8] in the array literal via unsized coercion.
- **Overengineering [LEAN]**: Named one-liner for the Noise initial hash step. Lean and matches protocol structure.
- **Tests [NONE]**: No test file found. initial_hash mixes IDENTIFIER into the chain key; no tests verify against WireGuard protocol test vectors.
- **UNDOCUMENTED [UNDOCUMENTED]**: Private function with no `///` doc comment. Role in Noise handshake initialization is clear by name but undocumented. Leniency applied. (deliberated: confirmed — Same pattern as initial_chain_key — tests NONE and UNDOCUMENTED both accurate. Private Noise protocol helper with self-descriptive name.)

#### `mix_hash` (L189–L191)

- **Utility [USED]**: Non-exported helper called in create_initiation (lines 295, 298, 309), process_response (line 393), and process_initiation (lines 472, 476, 497, 504, 518, 527) for Noise hash mixing.
- **Duplication [UNIQUE]**: Trivial one-liner helper. No similar functions in RAG.
- **Correction [OK]**: Correctly delegates to hash with h coerced from &[u8;32] to &[u8]; produces HASH(h || data) as required.
- **Overengineering [LEAN]**: Thin named alias for a hash operation that appears repeatedly in the protocol. Reduces repetition without over-abstracting.
- **Tests [NONE]**: No test file found. mix_hash is the core Noise state accumulator; no tests verify it changes the hash value or matches reference implementations.
- **UNDOCUMENTED [UNDOCUMENTED]**: Private function with no `///` doc comment. Noise protocol operation; self-descriptive but no formal documentation. Leniency applied. (deliberated: confirmed — Tests NONE accurate. UNDOCUMENTED correct. Core Noise state accumulator called extensively; name conveys purpose to protocol implementers.)

#### `mix_key` (L193–L196)

- **Utility [USED]**: Non-exported KDF wrapper called in create_initiation (lines 306, 316), process_response (lines 397, 404), and process_initiation (lines 485, 495, 508, 514).
- **Duplication [UNIQUE]**: RAG score 0.961 with rustguard-core, but deliberation found architectural differences: kernel module uses raw [u8; 32] while userspace uses SharedSecret wrapper type. Type differences reflect different compilation targets (kernel vs userspace). Per prior deliberation, architecturally distinct.
- **Correction [OK]**: Correctly performs KDF2: extracts new chain key and intermediate AEAD key from HKDF, discards unused third output.
- **Overengineering [LEAN]**: Wraps the HKDF call pattern that recurs in every DH step. Correctly discards the third output. Minimal and justified.
- **Tests [NONE]**: No test file found. mix_key drives all handshake key ratcheting; no tests verify output independence or HKDF expansion correctness.
- **UNDOCUMENTED [UNDOCUMENTED]**: Private function with no `///` doc comment. Returns (new_ck, key) tuple; tuple element order is not documented anywhere. Leniency applied for private items. (deliberated: confirmed — Tests NONE accurate. UNDOCUMENTED correct — no doc explaining the returned tuple order (new_ck, key) or that third HKDF output is discarded. Private scope leniency applies.)

#### `encrypt_and_hash` (L199–L203)

- **Utility [USED]**: Non-exported helper that encrypts and mixes ciphertext into hash, called in create_initiation (lines 311, 321) and process_initiation (line 525).
- **Duplication [UNIQUE]**: RAG score 0.922 with rustguard-core version, but different semantic contract: kernel version uses mutable buffer parameter and returns Option<usize>, while core version uses owned Vec return. These are not interchangeable APIs.
- **Correction [OK]**: Correctly encrypts then mixes the ciphertext (not plaintext) into the hash, consistent with Noise encrypt-and-hash semantics.
- **Overengineering [LEAN]**: Encodes a mandatory Noise pattern (encrypt then mix ciphertext into transcript hash) that occurs three times in the handshake. Factoring it out is correct and lean.
- **Tests [NONE]**: No test file found. encrypt_and_hash combines AEAD encryption with hash mixing; no tests verify the combined semantics or that hash is updated only on success.
- **PARTIAL [PARTIAL]**: Private function with `///` doc 'Encrypt-and-hash: AEAD encrypt, mix ciphertext into hash.' Pattern name explains the composition. Missing return tuple documentation and examples. Leniency applied. (deliberated: confirmed — Tests NONE accurate. Documentation PARTIAL fair — has `///` doc naming the pattern but missing return tuple description.)

#### `decrypt_and_hash` (L206–L210)

- **Utility [USED]**: Non-exported helper that decrypts and mixes ciphertext into hash, called in process_response (line 408) and process_initiation (lines 489, 500, 529).
- **Duplication [UNIQUE]**: RAG score 0.918 with rustguard-core version, but different semantic contract: kernel uses mutable buffer for output and returns usize, core returns owned Vec. Architectural differences in memory handling.
- **Correction [OK]**: Correctly mixes the original ciphertext into the hash (not the recovered plaintext), consistent with Noise decrypt-and-hash semantics.
- **Overengineering [LEAN]**: Symmetric counterpart to encrypt_and_hash. Factoring the repeated decrypt+mix pattern is appropriate.
- **Tests [NONE]**: No test file found. decrypt_and_hash combines AEAD decryption with hash mixing; no tests verify auth-failure propagation or hash-update correctness.
- **PARTIAL [PARTIAL]**: Private function with `///` doc 'Decrypt-and-hash: AEAD decrypt, mix ciphertext into hash.' Symmetric to encrypt_and_hash. Same gaps: no parameter or return tuple docs. Leniency applied. (deliberated: confirmed — Symmetric to encrypt_and_hash. Tests NONE and PARTIAL docs both accurate.)

#### `compute_mac1` (L213–L219)

- **Utility [USED]**: Non-exported function called in create_initiation (line 328), process_response (line 373), and process_initiation (lines 451, 536) to compute MAC1 authentication tag.
- **Duplication [UNIQUE]**: RAG score 0.960 with rustguard-core, but deliberation found architectural differences: kernel uses raw [u8; 32] while userspace uses PublicKey wrapper. Type difference reflects compilation target distinction (kernel vs userspace). Per prior deliberation, not a practical duplication.
- **Correction [OK]**: Correctly computes MAC1 = BLAKE2s-MAC(HASH('mac1----'||pubkey), msg)[0..16] per WireGuard specification.
- **Overengineering [LEAN]**: Encodes the WireGuard MAC1 derivation (hash label+pubkey, then MAC). Used in both create and process paths. Factoring is justified.
- **Tests [NONE]**: No test file found. compute_mac1 is the outer DoS-protection MAC; no tests verify the key derivation from LABEL_MAC1 or the 16-byte truncation.
- **PARTIAL [PARTIAL]**: Private function with `///` doc 'Compute MAC1 over a message.' Does not document the hash-then-truncate construction, the 16-byte output truncation from a 32-byte MAC, or parameter roles. Leniency applied. (deliberated: confirmed — Tests NONE accurate — MAC computation untested. Documentation PARTIAL fair — doc states purpose but omits the hash-then-truncate construction details.)

#### `tai64n_now` (L224–L238)

- **Utility [USED]**: Non-exported function called in create_initiation (line 319) to generate current TAI64N timestamp for encrypted timestamp payload.
- **Duplication [UNIQUE]**: No similar functions found in RAG results. Unique TAI64N timestamp generation specific to kernel wall clock.
- **Correction [OK]**: Correctly converts kernel wall-clock to TAI64N big-endian 12-byte format. The nsecs value from ktime_get_real is always in [0, 999999999] so the i64-to-u32 cast is safe and lossless.
- **Overengineering [LEAN]**: The inner extern "C" block for wg_ktime_get_real is unusual placement (all other FFI is at module top), but it correctly scopes a single-use kernel API to its only call site. The TAI64N encoding is per-spec. Overall lean.
- **Tests [NONE]**: No test file found. tai64n_now encodes kernel wall-clock time in TAI64N format; no tests verify the epoch offset, byte ordering, or nanosecond packing.
- **PARTIAL [PARTIAL]**: Private function with `///` doc 'Generate a TAI64N timestamp from kernel wall clock.' Describes source clock. Does not document the 12-byte output layout or that it calls an FFI wall-clock function. Leniency applied. (deliberated: confirmed — Tests NONE accurate. Documentation PARTIAL fair — has `///` doc but doesn't describe output layout or FFI dependency.)

#### `MSG_INITIATION` (L243–L243)

- **Utility [USED]**: Exported pub(crate) constant used in create_initiation (line 325) to set message type field in handshake initiation.
- **Duplication [UNIQUE]**: Protocol message type constant. No duplicates in RAG.
- **Correction [OK]**: Correct WireGuard type-1 message identifier.
- **Overengineering [LEAN]**: Protocol wire-type constant. Required and minimal.
- **Tests [GOOD]**: Compile-time constant with no runtime behavior. Per rule 6, constants are GOOD by default.
- **DOCUMENTED [DOCUMENTED]**: pub(crate) constant with `///` doc 'Message type 1: Handshake Initiation (148 bytes).' Includes wire value, message name, and wire size. Adequate for a protocol constant.

#### `MSG_RESPONSE` (L245–L245)

- **Utility [USED]**: Exported pub(crate) constant used in process_initiation (line 532) to set message type field in handshake response.
- **Duplication [UNIQUE]**: Protocol message type constant. No duplicates in RAG.
- **Correction [OK]**: Correct WireGuard type-2 message identifier.
- **Overengineering [LEAN]**: Protocol wire-type constant. Required and minimal.
- **Tests [GOOD]**: Compile-time constant with no runtime behavior. Per rule 6, constants are GOOD by default.
- **DOCUMENTED [DOCUMENTED]**: pub(crate) constant with `///` doc 'Message type 2: Handshake Response (92 bytes).' Includes wire value, message name, and wire size. Adequate for a protocol constant.

#### `MSG_TRANSPORT` (L247–L247)

- **Utility [DEAD]**: Exported pub(crate) constant for transport message type. Not used anywhere in this file and has 0 known importers. Likely dead protocol constant or intended for future use.
- **Duplication [UNIQUE]**: Protocol message type constant. No duplicates in RAG.
- **Correction [OK]**: Correct WireGuard type-4 message identifier.
- **Overengineering [LEAN]**: Protocol wire-type constant. Required and minimal.
- **Tests [GOOD]**: Compile-time constant with no runtime behavior. Per rule 6, constants are GOOD by default.
- **DOCUMENTED [DOCUMENTED]**: pub(crate) constant with `///` doc 'Message type 4: Transport Data.' Wire value and role documented. No size given but transport packets are variable-length, so omission is acceptable. (deliberated: reclassified: utility: DEAD → USED, tests: NONE → GOOD, documentation: UNDOCUMENTED → DOCUMENTED — Utility DEAD is a false positive: MSG_TRANSPORT is a pub(crate) protocol constant for WireGuard transport data messages (type 4). It is almost certainly imported by other modules handling the data path (e.g., transport/data handling). The 0-importer count reflects incomplete cross-module analysis. Tests reclassified to GOOD per rule 6 (compile-time constant). Documentation reclassified to DOCUMENTED: source line 247 clearly shows `/// Message type 4: Transport Data.` preceding the constant definition. The merged JSON had an internal inconsistency where the field said UNDOCUMENTED but the detail text said DOCUMENTED.)

#### `INITIATION_SIZE` (L249–L249)

- **Utility [USED]**: Exported pub(crate) constant used in create_initiation (line 289) for message buffer allocation and process_initiation (line 437) for parameter type.
- **Duplication [UNIQUE]**: Wire format size constant. No duplicates in RAG.
- **Correction [OK]**: 148 = 4+4+32+48+28+16+16 bytes, exactly matching the WireGuard initiation wire format.
- **Overengineering [LEAN]**: Named wire-format size constant avoids magic numbers. Correct and minimal.
- **Tests [GOOD]**: Compile-time constant with no runtime behavior. Per rule 6, constants are GOOD by default.
- **UNDOCUMENTED [UNDOCUMENTED]**: pub(crate) constant with no `///` doc comment. The value 148 is used in function signatures; a brief doc explaining wire layout or referencing MSG_INITIATION would be appropriate for a public constant. (deliberated: confirmed — pub(crate) constant with no `///` doc. Value 148 is non-obvious without documentation explaining the wire format breakdown. UNDOCUMENTED is correct.)

#### `RESPONSE_SIZE` (L250–L250)

- **Utility [USED]**: Exported pub(crate) constant used in process_response (line 350) for parameter type and process_initiation (line 438) for return value.
- **Duplication [UNIQUE]**: Wire format size constant. No duplicates in RAG.
- **Correction [OK]**: 92 = 4+4+4+32+16+16+16 bytes, exactly matching the WireGuard response wire format.
- **Overengineering [LEAN]**: Named wire-format size constant. Correct and minimal.
- **Tests [GOOD]**: Compile-time constant with no runtime behavior. Per rule 6, constants are GOOD by default.
- **UNDOCUMENTED [UNDOCUMENTED]**: pub(crate) constant with no `///` doc comment. Paired with INITIATION_SIZE; same gap applies — no doc explaining the 92-byte wire format size. (deliberated: confirmed — Same as INITIATION_SIZE — pub(crate) constant with no `///` doc. The 92-byte size warrants a brief doc. UNDOCUMENTED is correct.)

#### `TransportKeys` (L255–L266)

- **Utility [USED]**: Exported pub(crate) struct returned from process_response (line 350) and process_initiation (line 438) as the primary output of completed handshakes.
- **Duplication [UNIQUE]**: Struct representing derived transport keys. No similar struct definitions in RAG results.
- **Correction [OK]**: Struct correctly holds the two symmetric transport keys, both peer indices, and an atomic outgoing nonce counter.
- **Overengineering [LEAN]**: Plain data struct grouping exactly the fields needed after a completed handshake. No unnecessary methods or generics.
- **Tests [NONE]**: No test file found. TransportKeys is a plain data struct (no methods defined), which qualifies as GOOD per rule 6. However, the AtomicU64 send_counter field has initialization semantics (starts at 0) that are security-relevant and could benefit from testing; because no tests exist at all for this module, NONE is the accurate classification.
- **DOCUMENTED [DOCUMENTED]**: pub(crate) struct with `///` doc at struct level ('Derived transport keys from a completed handshake.') and individual `///` comments on all five fields explaining their roles. Comprehensive for an opaque key-material struct. (deliberated: confirmed — Tests NONE is technically accurate — no test file exists. The struct has no methods but holds security-sensitive fields (keys, AtomicU64 counter) whose initialization semantics are important. NONE reflects the module-wide test gap.)

#### `InitiatorState` (L271–L278)

- **Utility [USED]**: Exported pub(crate) struct returned from create_initiation (line 289) and used as parameter in process_response (line 350) to maintain handshake state.
- **Duplication [UNIQUE]**: Struct for initiator handshake state. No similar struct definitions in RAG results.
- **Correction [OK]**: Correctly captures all Noise state needed between sending the initiation and processing the response.
- **Overengineering [LEAN]**: Minimal carrier struct for the mid-handshake state the initiator must retain between send and receive. No extraneous fields.
- **Tests [GOOD]**: Plain data struct with no methods; constructed and consumed entirely within create_initiation/process_response. Per rule 6, types with no runtime behavior are GOOD by default.
- **PARTIAL [PARTIAL]**: pub(crate) struct with struct-level `///` doc 'State held between sending initiation and receiving response.' All six fields are private (no pub) so field-level docs are not required. Doc is brief; no guidance on lifetime or ownership transfer to process_response. Leniency applied for private fields. (deliberated: confirmed — Has struct-level `///` doc but no guidance on lifetime or security contract for the sensitive key material it holds. PARTIAL is accurate for a pub(crate) type carrying secret state.)

#### `create_initiation` (L284–L347)

- **Utility [DEAD]**: Exported pub(crate) function with 0 known importers and no local calls in file. Matches known false-positive pattern for public APIs in kernel module crates, but pre-computed analysis found 0 importers. Confidence reduced from 95 due to expected cross-file usage.
- **Duplication [UNIQUE]**: RAG scores 0.898, 0.870, 0.807 with rustguard-core variants, but architectural differences: kernel uses raw [u8; 32] parameters while core uses PublicKey/EphemeralSecret/Tai64n wrappers. Also uses C FFI functions. Different type systems reflect different compilation targets.
- **Correction [OK]**: Correctly implements Noise_IKpsk2 initiator send path: mixes responder static public, generates ephemeral, performs es and ss DH operations in the right order, encrypts static key and timestamp, and applies MAC1. All intermediate keys (key, key2) are zeroed after use. The local eph_secret copy is zeroed while the state's copy is preserved. ck and h local copies are not explicitly zeroed after moving into InitiatorState, a minor hygiene inconsistency but not a functional bug since [u8;32] is Copy and the values persist only in the returned state.
- **Overengineering [LEAN]**: Directly implements the 6-step Noise initiator message construction: mix public key, ephemeral, two DH operations, encrypt static key, encrypt timestamp, append MAC1. Every line corresponds to a protocol step. The repeated zeroize calls on early-return paths are necessary for security. Not over-engineered.
- **Tests [NONE]**: No test file found. create_initiation is the security-critical handshake initiator; no tests cover happy path, DH-zero rejection, MAC1 correctness, wire format layout, or zeroization of ephemeral secrets.
- **PARTIAL [PARTIAL]**: pub(crate) function with a multi-line `///` block describing purpose, return type semantics, and the DH zero-result failure condition (low-order point attack). No formal `# Parameters` section, no `# Examples`, and the psk role is not documented. Above-average for this codebase but still incomplete. (deliberated: reclassified: utility: DEAD → USED, documentation: UNDOCUMENTED → PARTIAL — Utility DEAD is a false positive: this is the primary pub(crate) API for creating handshake initiation messages. It is the entry point for the initiator handshake flow and must be called by other crate modules (e.g., peer/device management). The 0-importer count reflects incomplete cross-crate analysis. Documentation reclassified UNDOCUMENTED→PARTIAL: source lines 284-287 show a multi-line `///` block describing purpose, return semantics, and DH failure condition. The merged JSON field contradicts its own detail text which says PARTIAL.)

#### `process_response` (L350–L425)

- **Utility [DEAD]**: Exported pub(crate) function with 0 known importers and no local calls in file. Matches known false-positive pattern for public APIs, but analysis found 0 importers. Expected to be called from other crate modules, confidence reduced due to pattern.
- **Duplication [UNIQUE]**: RAG score 0.846 with rustguard-core version, but architectural differences: kernel operates on raw byte arrays and InitiatorState while core uses typed PublicKey/Tai64n wrappers and InitiatorHandshake. Different parameter and return types reflect kernel vs userspace design.
- **Correction [NEEDS_FIX]**: The PSK-HKDF-derived AEAD key bound as 'key' in 'let (new_ck, mut t, key) = hkdf(&ck, &state.psk)' is never zeroed after decrypt_and_hash. Because [u8;32] is Copy it is not mut, so zeroize cannot be called on it as written. The equivalent variable 'key3' in process_initiation is correctly declared mut and explicitly zeroed via 'zeroize(&mut key3)'. This inconsistency leaves sensitive intermediate cryptographic key material (derived from the pre-shared key) in stack memory after the function returns.
- **Overengineering [LEAN]**: Implements the complete initiator response-processing path: MAC1 verification, receiver index check, two DH operations, PSK phase, empty-payload decryption, key derivation. Repeated early-exit zeroization is a security requirement, not overengineering. Re-deriving our_static_public from secret for MAC1 check is a minor inefficiency but keeps the API surface simple.
- **Tests [NONE]**: No test file found. process_response is the security-critical response processor; no tests cover MAC1 validation, receiver-index mismatch, DH-zero rejection, PSK mixing, or derived transport key correctness.
- **PARTIAL [PARTIAL]**: pub(crate) function with a single-line `///` doc 'Process a handshake response (type 2) and derive transport keys.' Does not document MAC1 verification step, parameter roles, the receiver-index check, or None conditions. Security-critical function deserves fuller documentation. (deliberated: reclassified: utility: DEAD → USED, documentation: UNDOCUMENTED → PARTIAL — Correction NEEDS_FIX is valid: the PSK-derived AEAD key at line 407 (`let (new_ck, mut t, key) = hkdf(...)`) is not declared mut and never zeroed, unlike the equivalent `key3` in process_initiation which is properly zeroed. This leaves sensitive key material on the stack. Utility DEAD→USED: this is the primary pub(crate) API for processing handshake responses, necessarily called by peer management code. Documentation UNDOCUMENTED→PARTIAL: source line 350 has `/// Process a handshake response (type 2) and derive transport keys.` — the merged field contradicts the detail text. Confidence raised since findings are now coherent.)

#### `process_initiation` (L434–L539)

- **Utility [DEAD]**: Exported pub(crate) function with 0 known importers and no local calls in file. Matches known false-positive pattern for public APIs, but analysis found 0 importers. Confidence reduced from 95 due to expected cross-file usage in kernel module.
- **Duplication [UNIQUE]**: RAG scores 0.878, 0.843 with rustguard-core variants, but architectural differences: kernel uses raw bytes and C FFI while core uses typed PublicKey/Tai64n wrappers and Rust crypto library. Type system differences are fundamental, not superficial.
- **Correction [OK]**: Correctly implements Noise_IKpsk2 responder path: verifies MAC1 first, derives chain/hash state, decrypts initiator static and timestamp, generates response ephemeral, performs ee and se DH in correct order, runs PSK phase, encrypts empty payload, and derives transport keys with recv=first/send=second. All intermediate keys (key, key2, key3) and the response ephemeral secret are properly zeroed.
- **Overengineering [LEAN]**: Largest function in the file but necessarily so: it performs MAC1 check, four DH operations (two to process initiator message, two for response ephemeral), PSK phase, encrypt-empty, key derivation, and wire-message assembly — all mandatory steps of the Noise_IKpsk2 responder role. No unnecessary abstraction layers.
- **Tests [NONE]**: No test file found. process_initiation is the security-critical responder path; no tests cover MAC1 verification, decrypted initiator public key recovery, timestamp extraction, response wire format, or end-to-end key agreement with create_initiation.
- **PARTIAL [PARTIAL]**: pub(crate) function with a multi-line `///` block naming the return tuple elements and explicitly requiring the caller to perform replay protection (H3). Good security guidance. Missing formal `# Parameters`, `# Returns` section, None conditions, and examples. Return tuple field ordering must be inferred from doc prose alone. (deliberated: reclassified: utility: DEAD → USED, documentation: UNDOCUMENTED → PARTIAL — Utility DEAD→USED: this is the primary pub(crate) API for the responder handshake path, necessarily called by device/peer management modules. Documentation UNDOCUMENTED→PARTIAL: source lines 434-439 show a multi-line `///` block describing return tuple elements and requiring caller replay protection. The merged JSON field contradicts the detail text. Tests NONE is accurate — no test coverage for this critical path.)

## Best Practices — 7/10

### Rules

| # | Rule | Status | Severity | Detail |
|---|------|--------|----------|--------|
| 2 | No unsafe blocks without clear justification comment | WARN | CRITICAL | There are 12+ unsafe blocks across the file (in hash, mac, hkdf, seal, open, dh, generate_keypair, random_bytes, zeroize, tai64n_now, process_response, etc.). None carry idiomatic // SAFETY: comments explaining the invariants upheld (valid pointers, correct lengths, etc.). A module-level note mentions the C shim but does not substitute per-block rationale. Clippy's undocumented_unsafe_blocks lint would flag every occurrence. [L54-L55, L66-L68, L80-L85, L91-L95, L101-L108, L117-L124, L131-L136, L142-L144, L148-L151, L156-L157, L169-L170, L313-L313] |
| 4 | Derive common traits on public types | WARN | MEDIUM | TransportKeys (pub(crate)) and InitiatorState (pub(crate)) derive no traits at all. AtomicU64 prevents auto-derive of Clone/PartialEq for TransportKeys, but Debug could still be derived (AtomicU64 implements Debug). InitiatorState could derive Debug with a redacted custom impl or at minimum Clone. Omitting Debug makes these types harder to use in logging and tests. [L229-L238, L243-L251] |
| 6 | Use clippy idioms | WARN | MEDIUM | In the hash() function, chunks is iterated twice with the same enumerate().take(8) pattern to build ptrs and lens arrays separately. A single pass with zip or a combined iterator would be more idiomatic and avoid the duplicate traversal. Clippy's clippy::needless_range_loop and manual loop over index patterns apply here. [L73-L86] |
| 9 | Documentation comments on public items | WARN | MEDIUM | Several pub(crate) items lack /// doc comments: INITIATION_SIZE, RESPONSE_SIZE, and InitiatorState. The struct InitiatorState in particular is part of the public API surface (returned by create_initiation, consumed by process_response) but has no documentation explaining its purpose, lifetime, or security contract. [L220-L221, L243-L251] |
| 11 | Memory safety | WARN | HIGH | TransportKeys and InitiatorState both hold sensitive key material ([u8; 32] fields: key_send, key_recv, eph_secret, ck, h, psk, their_public) but neither implements Drop with zeroization. Any early-exit code path that propagates None via ? (e.g., dh() returning None at L340 or L345 in process_response) will drop the structs without zeroizing the fields, leaving key bytes on the stack. The explicit zeroize() calls at the happy-path exit are correct but incomplete without a Drop guard. [L229-L238, L243-L251, L336-L345] |

### Suggestions

- Add // SAFETY: comments to every unsafe block explaining the invariants upheld
  ```typescript
  // Before
  unsafe {
      wg_blake2s_hash(ptrs.as_ptr(), lens.as_ptr(), chunks.len() as u32, out.as_mut_ptr());
  }
  // After
  // SAFETY: ptrs and lens are valid stack-allocated arrays sized to chunks.len() (≤8).
  // out is a 32-byte stack buffer. All pointers remain valid for the duration of the call.
  unsafe {
      wg_blake2s_hash(ptrs.as_ptr(), lens.as_ptr(), chunks.len() as u32, out.as_mut_ptr());
  }
  ```
- Implement Drop for TransportKeys and InitiatorState to guarantee zeroization on all exit paths including panics and ? propagation
  ```typescript
  // Before
  pub(crate) struct TransportKeys {
      pub(crate) key_send: [u8; 32],
      pub(crate) key_recv: [u8; 32],
      // ...
  }
  // After
  pub(crate) struct TransportKeys {
      pub(crate) key_send: [u8; 32],
      pub(crate) key_recv: [u8; 32],
      // ...
  }
  
  impl Drop for TransportKeys {
      fn drop(&mut self) {
          zeroize(&mut self.key_send);
          zeroize(&mut self.key_recv);
      }
  }
  
  impl Drop for InitiatorState {
      fn drop(&mut self) {
          zeroize(&mut self.ck);
          zeroize(&mut self.h);
          zeroize(&mut self.eph_secret);
          zeroize(&mut self.psk);
          zeroize(&mut self.their_public);
      }
  }
  ```
- Combine the two separate iteration passes in hash() into a single pass using zip
  ```typescript
  // Before
  let ptrs: [*const u8; 8] = {
      let mut p = [core::ptr::null(); 8];
      for (i, c) in chunks.iter().enumerate().take(8) {
          p[i] = c.as_ptr();
      }
      p
  };
  let lens: [u32; 8] = {
      let mut l = [0u32; 8];
      for (i, c) in chunks.iter().enumerate().take(8) {
          l[i] = c.len() as u32;
      }
      l
  };
  // After
  let mut ptrs = [core::ptr::null::<u8>(); 8];
  let mut lens = [0u32; 8];
  for (i, c) in chunks.iter().enumerate().take(8) {
      ptrs[i] = c.as_ptr();
      lens[i] = c.len() as u32;
  }
  ```
- Add documentation to undocumented pub(crate) items
  ```typescript
  // Before
  pub(crate) const INITIATION_SIZE: usize = 148;
  pub(crate) const RESPONSE_SIZE: usize = 92;
  // After
  /// Wire size of a Handshake Initiation message (bytes).
  pub(crate) const INITIATION_SIZE: usize = 148;
  /// Wire size of a Handshake Response message (bytes).
  pub(crate) const RESPONSE_SIZE: usize = 92;
  ```
- Add doc comment to InitiatorState explaining its purpose and security contract
  ```typescript
  // Before
  pub(crate) struct InitiatorState {
  // After
  /// Intermediate state held by the initiator between sending an initiation
  /// message and receiving the response. Contains secret ephemeral key material
  /// that MUST be zeroized after use. Drop is implemented to guarantee this.
  pub(crate) struct InitiatorState {
  ```

## Actions

### Quick Wins

- **[correction · medium · small]** Fix hash() to pass min(chunks.len(), 8) as num_chunks to wg_blake2s_hash, or add a debug_assert!(chunks.len() <= 8). Currently chunks.len() is passed unconditionally while only 8 array slots are populated, causing out-of-bounds reads in the C shim if the function is ever called with more than 8 chunks. [L89]
- **[correction · low · small]** In process_response, declare the PSK HKDF output key as 'mut' (e.g. 'let (new_ck, mut t, mut key) = hkdf(&ck, &state.psk)') and call zeroize(&mut key) after decrypt_and_hash, mirroring the zeroize(&mut key3) call in process_initiation. Without this, the PSK-derived AEAD key persists on the stack after the function returns. [L398]

### Refactors

- **[duplication · high · small]** Deduplicate: `constant_time_eq` duplicates `constant_time_eq` in `rustguard-kmod/src/cookie.rs` (`constant_time_eq`) [L53-L58]
- **[duplication · high · small]** Deduplicate: `hash` duplicates `hash` in `rustguard-kmod/src/cookie.rs` (`hash`) [L72-L92]
- **[duplication · high · small]** Deduplicate: `mac` duplicates `mac` in `rustguard-kmod/src/cookie.rs` (`mac`) [L95-L105]
- **[utility · medium · trivial]** Remove dead code: `random_bytes` is exported but unused (`random_bytes`) [L173-L177]
- **[duplication · medium · small]** Deduplicate: `random_bytes` duplicates `random_bytes` in `rustguard-kmod/src/cookie.rs` (`random_bytes`) [L173-L177]

### Hygiene

- **[documentation · low · trivial]** Complete JSDoc documentation for: `zeroize` (`zeroize`) [L46-L50]
- **[documentation · low · trivial]** Complete JSDoc documentation for: `constant_time_eq` (`constant_time_eq`) [L53-L58]
- **[documentation · low · trivial]** Complete JSDoc documentation for: `TAI64_EPOCH_OFFSET` (`TAI64_EPOCH_OFFSET`) [L67-L67]
- **[documentation · low · trivial]** Complete JSDoc documentation for: `hash` (`hash`) [L72-L92]
- **[documentation · low · trivial]** Complete JSDoc documentation for: `mac` (`mac`) [L95-L105]
- **[documentation · low · trivial]** Complete JSDoc documentation for: `hkdf` (`hkdf`) [L108-L119]
- **[documentation · low · trivial]** Complete JSDoc documentation for: `seal` (`seal`) [L122-L136]
- **[documentation · low · trivial]** Complete JSDoc documentation for: `open` (`open`) [L139-L153]
- **[documentation · low · trivial]** Complete JSDoc documentation for: `dh` (`dh`) [L156-L160]
- **[documentation · low · trivial]** Complete JSDoc documentation for: `generate_keypair` (`generate_keypair`) [L163-L171]
- **[documentation · low · trivial]** Complete JSDoc documentation for: `encrypt_and_hash` (`encrypt_and_hash`) [L199-L203]
- **[documentation · low · trivial]** Complete JSDoc documentation for: `decrypt_and_hash` (`decrypt_and_hash`) [L206-L210]
- **[documentation · low · trivial]** Complete JSDoc documentation for: `compute_mac1` (`compute_mac1`) [L213-L219]
- **[documentation · low · trivial]** Complete JSDoc documentation for: `tai64n_now` (`tai64n_now`) [L224-L238]
- **[documentation · medium · trivial]** Add JSDoc documentation for exported symbol: `INITIATION_SIZE` (`INITIATION_SIZE`) [L249-L249]
- **[documentation · medium · trivial]** Add JSDoc documentation for exported symbol: `RESPONSE_SIZE` (`RESPONSE_SIZE`) [L250-L250]
- **[documentation · low · trivial]** Complete JSDoc documentation for: `InitiatorState` (`InitiatorState`) [L271-L278]
- **[documentation · medium · trivial]** Add JSDoc documentation for exported symbol: `create_initiation` (`create_initiation`) [L284-L347]
- **[documentation · medium · trivial]** Add JSDoc documentation for exported symbol: `process_response` (`process_response`) [L350-L425]
- **[documentation · medium · trivial]** Add JSDoc documentation for exported symbol: `process_initiation` (`process_initiation`) [L434-L539]
