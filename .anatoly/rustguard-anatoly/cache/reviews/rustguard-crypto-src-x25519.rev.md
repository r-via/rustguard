# Review: `rustguard-crypto/src/x25519.rs`

**Verdict:** NEEDS_REFACTOR

## Symbols

| Symbol | Kind | Exported | Correction | Over-eng. | Utility | Duplication | Tests | Confidence |
|--------|------|----------|------------|-----------|---------|-------------|-------|------------|
| StaticSecret | class | yes | NEEDS_FIX | LEAN | USED | UNIQUE | WEAK | 82% |
| EphemeralSecret | class | yes | NEEDS_FIX | LEAN | USED | UNIQUE | WEAK | 82% |
| PublicKey | class | yes | OK | LEAN | USED | UNIQUE | WEAK | 87% |
| SharedSecret | class | yes | OK | LEAN | USED | UNIQUE | WEAK | 87% |

### Details

#### `StaticSecret` (L7–L7)

- **Utility [USED]**: Pub struct in library crate; core X25519 type. Follows pattern of known false positives (TransportSession, CookieChecker, etc.): library crate public APIs are consumed by external/downstream crates despite zero in-workspace importers. Pre-computed analysis likely misses cross-crate imports.
- **Duplication [UNIQUE]**: No RAG duplicates found. Distinct semantic contract (long-lived key with to_bytes/from_bytes). Not interchangeable with EphemeralSecret despite implementation overlap.
- **Correction [NEEDS_FIX]**: StaticSecret::diffie_hellman (L46-49) does not validate that the DH output is non-zero. RFC 7748 §6.1 states implementations SHOULD check for the all-zero output and reject it, and the WireGuard specification explicitly requires aborting the handshake if any DHEX result is zero (a low-order or small-subgroup public key produces this). Without this check an adversary can supply a low-order point and obtain a predictable all-zero SharedSecret, collapsing the security of the handshake. The return type would need to become Result<SharedSecret, E> to surface this failure, or the check must be delegated and enforced at the call site.
- **Overengineering [LEAN]**: Thin newtype over x25519_dalek::StaticSecret. Adds Zeroize/ZeroizeOnDrop for security hygiene and exposes a minimal, controlled API surface. The distinction between StaticSecret and EphemeralSecret is mandated by the WireGuard noise protocol, making two separate types correct rather than premature.
- **Tests [WEAK]**: Happy-path DH exchange is tested in `dh_exchange_produces_shared_secret` and `public_key_roundtrip` covers `public_key()`. However, `from_bytes()`, `to_bytes()`, and `random_from_rng()` are never directly exercised, there are no RFC 7748 known-answer test vectors, zeroize-on-drop behavior is unverified, and low-order point inputs (all-zero DH result) are not tested. Security-critical code warrants spec vector coverage.
- **PARTIAL [PARTIAL]**: Struct has a concise `///` doc comment at L5. `random_from_rng` and `random` are documented, but `from_bytes`, `diffie_hellman`, `public_key`, and `to_bytes` lack any `///` doc comments. No `# Examples` section exists anywhere in the impl block, which is expected for a public cryptographic API. (deliberated: confirmed — Correction NEEDS_FIX is valid: RFC 7748 §6.1 SHOULD-level check and WireGuard spec requirement for zero-output DH rejection. While x25519_dalek deliberately omits this check (delegating to protocol layer), this is a WireGuard-specific crate (rustguard-crypto) where the check is protocol-mandated and should live either here or in the handshake layer — flagging it here is appropriate. Utility USED is correct: core public library type for the crate. Duplication UNIQUE confirmed: StaticSecret and EphemeralSecret have intentionally distinct semantic contracts (long-lived vs single-use), not interchangeable despite structural similarity. Overengineering LEAN correct: thin newtype with Zeroize is security-mandated. Tests WEAK is fair: only happy-path DH symmetry, no RFC 7748 vectors, no from_bytes/to_bytes coverage, no low-order point tests for crypto-critical code. Documentation PARTIAL is accurate: struct-level doc exists but from_bytes, diffie_hellman, public_key, to_bytes all lack /// comments. Raised confidence slightly since all axes are coherent and well-evidenced.)

#### `EphemeralSecret` (L11–L11)

- **Utility [USED]**: Pub struct in library crate for ephemeral keys. Identical pattern to known false positives. Essential crypto functionality exported by module; external consumers depend on this type.
- **Duplication [UNIQUE]**: No RAG duplicates found. Distinct semantic contract (ephemeral single-use key). Different invariants and public API from StaticSecret; not interchangeable.
- **Correction [NEEDS_FIX]**: EphemeralSecret::diffie_hellman (L75-78) has the identical missing zero-output check as StaticSecret::diffie_hellman. In WireGuard the ephemeral DH operations (e_i DH with the responder static key, and with the responder ephemeral) are equally required to abort on a zero result. A low-order public key will silently produce an all-zero SharedSecret with no error signal.
- **Overengineering [LEAN]**: Intentionally backed by x25519_dalek::StaticSecret (reason documented inline: WireGuard requires multiple DH ops with the same ephemeral key, which x25519_dalek::EphemeralSecret's consume-on-use design prohibits). The deviation is protocol-driven and clearly explained, not incidental complexity.
- **Tests [WEAK]**: `ephemeral_dh_works` covers the happy-path DH symmetry check. `random_from_rng()` is never called in any test (only `random()` is used), `public_key()` is called indirectly but not the standalone path, and there are no RFC 7748 test vectors. Zeroize behavior and low-order point edge cases are absent. The fact that `EphemeralSecret` reuses `StaticSecret` internally (allowing multi-use, a WireGuard-specific design choice) is not validated by any test.
- **PARTIAL [PARTIAL]**: Struct has a brief `///` doc comment at L9 and includes a useful implementation note in `random_from_rng` explaining the WireGuard-specific rationale for using `StaticSecret` internally. However, `diffie_hellman` and `public_key` lack `///` doc comments, and no `# Examples` section is present for any public method. (deliberated: confirmed — Same DH zero-check issue as StaticSecret applies identically at L78-81; NEEDS_FIX confirmed. Utility USED correct: essential ephemeral key type for WireGuard handshake. Duplication UNIQUE confirmed: despite wrapping StaticSecret internally, this is an intentional WireGuard design choice (documented in the random_from_rng comment at L64-66) since x25519_dalek::EphemeralSecret consumes itself on DH but WireGuard needs multiple DH ops with the same ephemeral. This is not duplication — it's a protocol-driven architectural decision. Overengineering LEAN confirmed: the deviation from x25519_dalek's EphemeralSecret is justified and documented inline. Tests WEAK valid: random_from_rng untested, no multi-DH test validating the WireGuard-specific reuse design, no edge cases. Documentation PARTIAL: struct doc and random_from_rng rationale are good, but diffie_hellman and public_key lack comments.)

#### `PublicKey` (L16–L16)

- **Utility [USED]**: Pub struct in library crate with security-critical constant-time equality comparison. Follows known false positive pattern. Core public key type for external cryptographic operations.
- **Duplication [UNIQUE]**: No RAG duplicates found. Wrapper for X25519 public key with constant-time comparison via PartialEq implementation.
- **Correction [OK]**: PartialEq delegates to ct_eq from the subtle crate (L20), correctly preventing timing side-channels in equality comparisons. from_bytes and as_bytes faithfully delegate to x25519_dalek::PublicKey. The AsRef<[u8]> impl is consistent with as_bytes. No correctness issues found.
- **Overengineering [LEAN]**: Minimal newtype. The custom PartialEq using subtle::ConstantTimeEq is a security requirement to prevent timing side-channels — not ceremony. AsRef<[u8]> and as_bytes() are standard ergonomics for a byte-key type. No excess.
- **Tests [WEAK]**: `public_key_roundtrip` covers `from_bytes()`, `as_bytes()`, and the `PartialEq` constant-time impl indirectly via `assert_eq!`. However, the `AsRef<[u8]>` impl is never exercised, inequality of distinct keys is not explicitly tested, and — most critically — the constant-time property of `PartialEq` (the main security claim in the doc comment) cannot be validated by the current tests. No negative roundtrip or adversarial byte arrays are tested.
- **PARTIAL [PARTIAL]**: Struct has a two-line `///` block at L13–L14 covering wire size and the constant-time comparison security property — the best struct-level doc among the four types. However, `from_bytes` and `as_bytes` carry no `///` doc comments, and no `# Examples` section exists on any public method. (deliberated: confirmed — Correction OK confirmed: PartialEq via ct_eq at L20 is correct constant-time implementation, from_bytes and as_bytes are faithful delegations. Tests WEAK valid: public_key_roundtrip covers the basic path but AsRef<[u8]> (L101-105) is untested, inequality case untested, and the constant-time property (the key security claim) is inherently difficult to test but should at least have a comment acknowledging this. Documentation PARTIAL valid: struct-level doc at L13-14 is the best among all types, mentioning wire size and CT comparison, but from_bytes and as_bytes lack method-level docs.)

#### `SharedSecret` (L28–L28)

- **Utility [USED]**: Pub struct in library crate with zeroization on drop. Follows known false positive pattern. Essential DH result type consumed by downstream crates despite zero in-workspace importers.
- **Duplication [UNIQUE]**: No RAG duplicates found. Wrapper for 32-byte DH shared secret with automatic zeroization on drop.
- **Correction [OK]**: SharedSecret wraps [u8; 32] with #[derive(Zeroize, ZeroizeOnDrop)]; [u8; 32] implements Zeroize so the derive is valid. as_bytes returns a reference to the inner array without copying secret material. No correctness issues found.
- **Overengineering [LEAN]**: Simplest possible zeroizing wrapper around [u8; 32]. Zeroize/ZeroizeOnDrop are mandatory in any crypto key type to prevent secret material lingering in memory. Single accessor method. Textbook minimal design.
- **Tests [WEAK]**: `SharedSecret` is only reached indirectly through the two DH tests; `as_bytes()` is called in both. There is no direct construction test, no test that the 32-byte value matches a known RFC 7748 vector, and zeroize-on-drop is never verified. The all-zero shared secret (small-subgroup / low-order point attack) is a known X25519 foot-gun and is completely untested.
- **PARTIAL [PARTIAL]**: Struct has a terse `///` doc comment at L26 noting it is a DH result that is zeroized on drop. The sole public method `as_bytes` has no `///` doc comment and no `# Examples` section, leaving callers with no guidance on intended usage patterns. (deliberated: confirmed — Correction OK confirmed: [u8; 32] with Zeroize/ZeroizeOnDrop derives are valid, as_bytes returns a reference without copying. Tests WEAK valid: only reached indirectly through DH tests, no direct construction test, no known-answer vectors, no all-zero shared secret test (the very scenario the DH zero-check actions address). Documentation PARTIAL valid: struct doc at L27 is terse but present; the sole public method as_bytes has no doc comment.)

## Best Practices — 9.5/10

### Rules

| # | Rule | Status | Severity | Detail |
|---|------|--------|----------|--------|
| 9 | Documentation comments on public items | WARN | MEDIUM | All four public structs (StaticSecret, EphemeralSecret, PublicKey, SharedSecret) have `///` doc comments. However, the following public methods are missing documentation: `StaticSecret::from_bytes`, `StaticSecret::diffie_hellman`, `StaticSecret::public_key`, `StaticSecret::to_bytes`; `EphemeralSecret::diffie_hellman`, `EphemeralSecret::public_key`; `PublicKey::from_bytes`, `PublicKey::as_bytes`; `SharedSecret::as_bytes`. These are core cryptographic operations that warrant documentation explaining behaviour and any security notes. [L42-L57, L72-L79, L83-L88, L91-L93] |

### Suggestions

- Add `///` doc comments to the undocumented public methods on StaticSecret. Crypto APIs especially benefit from documentation explaining security contracts.
  ```typescript
  // Before
  pub fn from_bytes(bytes: [u8; 32]) -> Self {
      Self(x25519_dalek::StaticSecret::from(bytes))
  }
  
  pub fn diffie_hellman(&self, their_public: &PublicKey) -> SharedSecret {
      let shared = self.0.diffie_hellman(&their_public.0);
      SharedSecret(shared.to_bytes())
  }
  // After
  /// Construct a static secret from raw bytes. Caller must ensure the
  /// bytes represent a valid, previously-generated X25519 scalar.
  pub fn from_bytes(bytes: [u8; 32]) -> Self {
      Self(x25519_dalek::StaticSecret::from(bytes))
  }
  
  /// Perform a Diffie-Hellman operation with the given public key.
  /// Returns a `SharedSecret` that is zeroized on drop.
  pub fn diffie_hellman(&self, their_public: &PublicKey) -> SharedSecret {
      let shared = self.0.diffie_hellman(&their_public.0);
      SharedSecret(shared.to_bytes())
  }
  ```
- Add `///` doc comments to the undocumented public methods on PublicKey and SharedSecret.
  ```typescript
  // Before
  pub fn from_bytes(bytes: [u8; 32]) -> Self {
      Self(x25519_dalek::PublicKey::from(bytes))
  }
  
  pub fn as_bytes(&self) -> &[u8; 32] {
      self.0.as_bytes()
  }
  // After
  /// Construct a `PublicKey` from its 32-byte wire representation.
  pub fn from_bytes(bytes: [u8; 32]) -> Self {
      Self(x25519_dalek::PublicKey::from(bytes))
  }
  
  /// Return the 32-byte encoding of this public key.
  pub fn as_bytes(&self) -> &[u8; 32] {
      self.0.as_bytes()
  }
  ```

## Actions

### Quick Wins

- **[correction · medium · small]** StaticSecret::diffie_hellman does not check for an all-zero DH result. Add a constant-time zero check (e.g. shared.to_bytes().ct_eq(&[0u8;32])) and either return an error or panic/abort, as required by RFC 7748 and the WireGuard specification. Consider changing the return type to Result<SharedSecret, ZeroSharedSecret> to propagate the failure. [L47]
- **[correction · medium · small]** EphemeralSecret::diffie_hellman has the same missing all-zero DH output check. Apply the identical fix: constant-time comparison against [0u8;32] and surface a failure when the result is zero. [L76]

### Hygiene

- **[documentation · low · trivial]** Complete JSDoc documentation for: `StaticSecret` (`StaticSecret`) [L7-L7]
- **[documentation · low · trivial]** Complete JSDoc documentation for: `EphemeralSecret` (`EphemeralSecret`) [L11-L11]
- **[documentation · low · trivial]** Complete JSDoc documentation for: `PublicKey` (`PublicKey`) [L16-L16]
- **[documentation · low · trivial]** Complete JSDoc documentation for: `SharedSecret` (`SharedSecret`) [L28-L28]
