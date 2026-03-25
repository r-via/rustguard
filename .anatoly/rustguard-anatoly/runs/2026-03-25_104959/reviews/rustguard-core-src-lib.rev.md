# Review: `rustguard-core/src/lib.rs`

**Verdict:** CLEAN

## Best Practices — 9.5/10

### Rules

| # | Rule | Status | Severity | Detail |
|---|------|--------|----------|--------|
| 9 | Documentation comments on public items | WARN | MEDIUM | None of the six public module declarations or the `pub use rustguard_crypto as crypto` re-export have `///` documentation comments. As the public API entry point for the crate, each public item should carry at least a one-line summary to aid users of the library. [L5-L11] |

### Suggestions

- Add `///` doc comments to each public module declaration and the crypto re-export so that `cargo doc` generates meaningful API documentation for library consumers.
  ```typescript
  // Before
  pub mod cookie;
  pub mod handshake;
  pub mod messages;
  pub mod replay;
  pub mod session;
  pub mod timers;
  
  pub use rustguard_crypto as crypto;
  // After
  /// Cookie reply and MAC computation utilities.
  pub mod cookie;
  
  /// Noise protocol handshake state machine.
  pub mod handshake;
  
  /// Wire-format message types and serialization.
  pub mod messages;
  
  /// Anti-replay sliding-window counter.
  pub mod replay;
  
  /// Established session key material and state.
  pub mod session;
  
  /// Keepalive and expiry timer logic.
  pub mod timers;
  
  /// Re-exported cryptographic primitives (AEAD, X25519, BLAKE2s, TAI64N).
  pub use rustguard_crypto as crypto;
  ```
