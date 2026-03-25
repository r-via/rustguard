# Design Decisions

> Architecture Decision Records (ADRs) capturing the key technical choices made during RustGuard's development phases.

## Overview

This page documents the significant design decisions across RustGuard's seven crates, organized by development phase. Each record covers the context that forced a choice, the decision taken, and its consequences. Security-critical decisions are marked **[SECURITY]**.

For the runtime paths these decisions govern, see [Data Flow](03-Data-Flow.md). For the cryptographic primitives and protocol concepts referenced here, see [Core Concepts](02-Core-Concepts.md).

---

## Phase 1 — Protocol Foundations

### ADR-001: Dual `std` / `no_std` for Shared Crates

**Status:** Accepted

**Context:** The kernel module (`rustguard-kmod`) must compile without the Rust standard library — the Linux kernel environment provides no heap allocator compatible with `std`. Userspace crates need `std` for I/O and threading.

**Decision:** `rustguard-crypto` and `rustguard-core` are written as `no_std`-compatible crates. All userspace crates (`rustguard-tun`, `rustguard-daemon`, `rustguard-enroll`, `rustguard-cli`) use `std` freely.

**Consequences:**
- `rustguard-kmod` reuses `rustguard-crypto` and `rustguard-core` without modification — no protocol code is duplicated between the userspace and kernel paths.
- Any new dependency added to `rustguard-crypto` or `rustguard-core` must be `no_std`-compatible, adding ongoing maintenance discipline.
- `rustguard-crypto` and `rustguard-core` cannot use `std::collections`, `std::io`, or heap-backed types without `no_std` equivalents.

---

### ADR-002: Multiple TUN Backends in `rustguard-tun`

**Status:** Accepted

**Context:** Different deployment environments favor different I/O models. macOS requires `utun` via kernel control sockets. Linux supports standard `/dev/net/tun`, multi-queue TUN for throughput, AF_XDP for zero-copy, and io_uring for async I/O.

**Decision:** `rustguard-tun` exposes a unified backend abstraction with five concrete implementations: macOS `utun`, Linux TUN (blocking), multi-queue TUN, AF_XDP, and io_uring. A BPF loader handles AF_XDP program management.

**Consequences:**
- AF_XDP and io_uring backends approach kernel module throughput on capable hardware.
- The abstraction layer's indirection cost is negligible compared to the cryptographic overhead on the hot path.

---

## Phase 2 — Security Audit

The following ADRs record fixes discovered during an OWASP-style adversarial audit. Each represents a deviation from the WireGuard whitepaper or a standard security practice that was corrected.

### ADR-003 [SECURITY]: MAC1 Verification Before DH Operations

**Status:** Accepted

**Context:** The original implementation performed X25519 DH operations before verifying MAC1 on incoming handshake initiations. An attacker could send a stream of malformed initiation packets, each triggering a full DH computation, exhausting CPU without authentication.

**Decision:** MAC1 is verified as the **first** operation on any received handshake message, before any DH or decryption occurs. A failed MAC1 causes an immediate drop.

**Consequences:**
- Unauthenticated packets are rejected at O(1) cost (a BLAKE2s MAC verify) rather than O(DH) cost.
- This matches the canonical WireGuard design; the original implementation deviated from the whitepaper.

---

### ADR-004 [SECURITY]: Replay Window Split Into `check()` / `update()`

**Status:** Accepted

**Context:** The original replay window advanced the 2048-bit sliding bitmap on packet receipt, before AEAD decryption. A packet with a valid counter but corrupt ciphertext would advance the window, permanently blocking the legitimate retransmission of that counter.

**Decision:** The replay window API exposes two distinct methods:
- `check(counter)` — validates the counter against the current window without mutating state; drops the packet if the counter was already seen or falls below the window floor.
- `update(counter)` — advances the window bitmap; called only after successful AEAD decryption.

**Consequences:**
- Replay window state is mutated only on authenticated, successfully decrypted packets.
- The `check()`/`update()` split is enforced at the API boundary — callers cannot advance the window before decryption succeeds.

---

### ADR-005 [SECURITY]: `encrypt()` Returns `Option` on Nonce Exhaustion

**Status:** Accepted

**Context:** ChaCha20-Poly1305 nonces are 64-bit counters. Exhaustion at 2⁶⁰ messages must trigger a rekey; continuing past exhaustion reuses nonces and breaks AEAD security. The original implementation panicked on exhaustion — a denial-of-service vector in long-lived sessions.

**Decision:** `encrypt()` returns `Option<Ciphertext>`, yielding `None` on nonce exhaustion. The timer state machine in `rustguard-core` also initiates a proactive rekey before reaching exhaustion.

**Consequences:**
- No panic in the encryption path regardless of session age or message volume.
- Callers that do not handle `None` fail the packet silently rather than crashing the process.

---

### ADR-006 [SECURITY]: HMAC-BLAKE2s Over Keyed BLAKE2s

**Status:** Accepted

**Context:** The initial implementation used keyed BLAKE2s (BLAKE2s with a key parameter) in place of HMAC-BLAKE2s (RFC 2104 ipad/opad double-hash). These constructions are **not** equivalent. The WireGuard protocol specifies HMAC-BLAKE2s explicitly; using keyed BLAKE2s produced incorrect KDF output and broke interoperability with the reference implementation.

**Decision:** All KDF calls use a correct RFC 2104 HMAC-BLAKE2s implementation. Keyed BLAKE2s is not exposed in the public API.

**Consequences:**
- KDF output is correct and interoperable with the WireGuard reference implementation.
- Sessions established with an unfixed peer would have failed to decrypt — this was a protocol-level correctness bug, not merely a performance issue.

---

### ADR-007 [SECURITY]: `subtle::ConstantTimeEq` for MAC Comparisons

**Status:** Accepted

**Context:** Hand-rolled constant-time comparison functions using `black_box` are brittle — LLVM can still collapse the comparison under specific optimization contexts, turning a MAC equality check into a timing oracle.

**Decision:** All MAC and key comparisons use `subtle::ConstantTimeEq` from the `subtle` crate.

**Consequences:**
- Timing side-channel risk on MAC comparisons is eliminated by a maintained, audited external crate.
- `subtle` is `no_std`-compatible and used directly in `rustguard-crypto`.

---

### ADR-008 [SECURITY]: `getrandom` Crate for CSPRNG

**Status:** Accepted

**Context:** The original implementation read from `/dev/urandom` directly for generating sender indices. This approach is Linux-only, bypasses the platform's preferred secure random API, and requires an extra file descriptor.

**Decision:** All random values, including sender indices, are generated via the `getrandom` crate, which delegates to the OS CSPRNG on each platform (`getrandom(2)` on Linux, `CCRandom` on macOS).

**Consequences:**
- Cross-platform correctness without platform-specific device paths.
- `getrandom` supports `no_std` via its `custom` feature, making it usable in `rustguard-kmod`.

---

### ADR-009 [SECURITY]: `ZeroizeOnDrop` on Handshake State

**Status:** Accepted

**Context:** Handshake state — the chaining key, handshake hash, and PSK — contains derived key material that must not persist in memory after the handshake completes. Without explicit zeroing, this material may remain in freed heap allocations.

**Decision:** Handshake state structs in `rustguard-core` derive `ZeroizeOnDrop`. The chaining key, handshake hash, and PSK fields are annotated with `#[zeroize]`.

**Consequences:**
- Key material lifetime in memory is bounded to the handshake duration.
- `zeroize` is `no_std`-compatible and incurs no runtime overhead on the fast path.

---

## Phase 3 — Zero-Config Enrollment

### ADR-010: Token-Derived Key for Enrollment Channel

**Status:** Accepted

**Context:** The standard WireGuard flow requires out-of-band key exchange, manual public key distribution, IP assignment, and config file authoring on both sides. This is impractical for homelab deployments.

**Decision:** `rustguard-enroll` implements a custom enrollment protocol. A shared token derives an XChaCha20-Poly1305 key that encrypts an in-band key exchange. The server assigns IPs from a CIDR pool. `rustguard up wg0.conf` remains functional — enrollment is additive.

**Consequences:**
- Token security determines enrollment security. Weak or reused tokens weaken the enrollment channel.
- XChaCha20 (not ChaCha20) is used to eliminate nonce-reuse risk across concurrent enrollment requests on the same token.

---

### ADR-011: Zigbee-Style Pairing Window via UNIX Domain Socket

**Status:** Accepted

**Context:** An always-open enrollment endpoint is a persistent attack surface. A server that accepts new peers indefinitely can be enrolled by any party that learns the token.

**Decision:** The enrollment server starts with enrollment closed. A UNIX domain socket accepts `open <seconds>`, `close`, and `status` commands from a local operator. `open` sets an atomic expiry timestamp; the enrollment handler rejects requests after expiry without modifying tunnel state for existing peers.

**Consequences:**
- Physical or SSH access to the server is required to open an enrollment window, approximating a physical-presence security model.
- The UNIX socket is accessible only to local users — there is no network exposure for the control plane.
- Existing peers and the running tunnel are unaffected by window state changes.

---

## Security Decision Chain

The diagram below shows the inbound packet path, with each decision point corresponding to a specific ADR.

```mermaid
flowchart TD
    A[UDP Datagram Received] --> B{"MAC1 valid?\nADR-003"}
    B -- No --> DROP1[Drop — O(1) cost]
    B -- Yes --> C{"Replay window\ncheck(counter)\nADR-004"}
    C -- Seen or stale --> DROP2[Drop — window unchanged]
    C -- New counter --> D["ChaCha20-Poly1305\nAEAD Decrypt\nrustguard-crypto"]
    D -- Auth failure --> DROP3[Drop — window NOT advanced]
    D -- Auth success --> E["Replay window\nupdate(counter)\nADR-004"]
    E --> F{AllowedIPs verify}
    F -- Rejected --> DROP4[Drop]
    F -- Accepted --> G["Write to TUN\nrustguard-tun"]
```

---

## Examples

The following illustrates the `check()`/`update()` replay window API (ADR-004) and the `Option`-returning `encrypt()` API (ADR-005) as used in `rustguard-core`'s inbound and outbound paths.

```rust
use rustguard_core::replay::ReplayWindow;
use rustguard_crypto::aead::{encrypt, decrypt};

// Outbound: encrypt returns None on nonce exhaustion at 2^60 (ADR-005)
fn send_packet(session: &mut Session, plaintext: &[u8]) -> Option<Vec<u8>> {
    let nonce = session.next_nonce()?;  // None triggers caller-side rekey
    encrypt(&session.send_key, nonce, plaintext)
}

// Inbound: check before decrypt, update only on success (ADR-004)
fn recv_packet(
    window: &mut ReplayWindow,
    session: &Session,
    counter: u64,
    ciphertext: &[u8],
) -> Option<Vec<u8>> {
    // MAC1 verified upstream (ADR-003)
    window.check(counter).ok()?;                               // drop if replayed
    let plaintext = decrypt(&session.recv_key, counter, ciphertext)?; // drop if auth fails
    window.update(counter);                                    // advance only after success
    Some(plaintext)
}
```

Enrollment workflow (ADR-010, ADR-011):

```bash
# Start the enrollment server with an IP pool and shared token
rustguard serve --pool 10.150.0.0/24 --token mysecret

# Open a 60-second enrollment window (Zigbee-style physical-presence model)
rustguard open 60

# Enroll a client — no config files required on either side
rustguard join 1.2.3.4:51820 --token mysecret

# Close the enrollment window immediately
rustguard close

# Inspect window state and enrolled peer count
rustguard status
```

---

## See Also

- [System Overview](01-System-Overview.md) — Crate map and the two runtime execution paths
- [Core Concepts](02-Core-Concepts.md) — Definitions for ChaCha20-Poly1305, HMAC-BLAKE2s, replay window, and Noise_IKpsk2
- [Data Flow](03-Data-Flow.md) — Runtime packet and handshake paths governed by the decisions above
- [Public API](../04-API-Reference/01-Public-API.md) — `ReplayWindow`, `encrypt`, `decrypt`, and session types
- [Common Workflows](../03-Guides/01-Common-Workflows.md) — Using `serve`, `join`, `open`, `close`, and `status` in practice