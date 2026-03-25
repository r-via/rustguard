# Code Conventions

> Style guide, architectural patterns, and security-sensitive coding rules for the RustGuard workspace.

## Overview

RustGuard is a security-critical Rust workspace of seven crates. These conventions encode lessons learned during the security audit phase (Commits 5–6), where hand-rolled patterns were replaced with rigorously audited alternatives. Following them prevents the classes of vulnerability found and fixed in that phase.

The conventions are enforced at the tooling level where possible: `rustfmt` for style and `clippy -D warnings` for lint. The patterns documented here cover the cases tooling cannot automate.

## Formatting and Lint

All code is formatted with `rustfmt` using workspace defaults. CI rejects any diff that changes formatting output.

```bash
# Check formatting (fails on any diff)
cargo fmt --workspace -- --check

# Apply formatting
cargo fmt --workspace
```

All clippy warnings are treated as errors. Run the full workspace check before committing:

```bash
cargo clippy --workspace --all-targets -- -D warnings
```

No `#[allow(...)]` suppressions for the following lints:
- `clippy::suspicious_arithmetic_impl`
- `clippy::integer_arithmetic`
- `clippy::panic`

If a suppression is unavoidable, it must be accompanied by a `// SAFETY:` or `// RATIONALE:` comment explaining why.

## Crate Layering Rules

The workspace enforces a strict dependency direction. Violating it is a build error enforced by the Cargo dependency graph.

| Layer | Crates | Rule |
|-------|--------|------|
| Protocol | `rustguard-crypto`, `rustguard-core` | No I/O, no allocation required, `no_std` compatible |
| I/O | `rustguard-tun`, `rustguard-daemon`, `rustguard-enroll` | No protocol logic; calls into protocol crates only |
| CLI | `rustguard-cli` | Thin dispatch layer; no business logic |
| Kernel | `rustguard-kmod` | Reuses `rustguard-crypto` and `rustguard-core` verbatim via `no_std` feature |

**Protocol crates must never import I/O crates.** The separation enables the kernel module to compile the protocol stack without any `std` dependency.

## `no_std` Compatibility

`rustguard-crypto` and `rustguard-core` are dual `std`/`no_std`. Code in these crates must compile under both feature configurations.

Rules for `no_std`-compatible crates:

- Use `#![no_std]` at the crate root with a `std` feature flag that enables `extern crate std`.
- Heap allocation (`Vec`, `Box`, `String`) is prohibited unless gated behind a feature flag.
- Use `core::` and `alloc::` prefixes for standard types; never `std::` without a feature gate.
- Verify compatibility before committing:

```bash
cargo build -p rustguard-crypto --target thumbv7em-none-eabihf
cargo build -p rustguard-core   --target thumbv7em-none-eabihf
```

## Error Handling

### Prefer `Option` and `Result` over `panic!`

Functions that can fail must return `Option` or `Result`. Panicking on recoverable conditions (including exhausted counters) is prohibited in any crate that can be called from kernel context.

```rust
// WRONG — panics on nonce exhaustion, disallowed in rustguard-crypto/rustguard-core
pub fn encrypt(key: &Key, nonce: u64, plaintext: &[u8]) -> Vec<u8> {
    if nonce >= u64::MAX {
        panic!("nonce exhausted");
    }
    // ...
}

// CORRECT — returns None, lets the caller decide
pub fn encrypt(key: &Key, nonce: u64, plaintext: &[u8]) -> Option<Vec<u8>> {
    if nonce >= u64::MAX {
        return None;
    }
    // ...
}
```

### Operation Ordering in Security-Critical Paths

Authenticate before performing expensive operations. The ordering rule (from Commit 5):

1. Verify MAC1 first — before any Diffie-Hellman computation.
2. `check()` the replay window — before decryption.
3. `update()` the replay window — only after successful AEAD verification.

Reversing steps 2 and 3 allows an attacker to advance the replay window with unauthenticated garbage packets.

## Constant-Time Comparisons

All equality checks on secret or authentication material must use `subtle::ConstantTimeEq`. Hand-rolled alternatives using `black_box` or XOR loops are prohibited.

```rust
use subtle::ConstantTimeEq;

// WRONG — LLVM may optimize this to a short-circuit comparison
fn verify_mac(expected: &[u8; 16], received: &[u8; 16]) -> bool {
    expected == received
}

// CORRECT — constant-time, compiler-proof
fn verify_mac(expected: &[u8; 16], received: &[u8; 16]) -> bool {
    expected.ct_eq(received).into()
}
```

This applies to: MAC1, MAC2, cookie values, PSK comparisons, and any byte buffer derived from a secret key.

## Secret Type Hygiene

All types that hold secret key material must implement `ZeroizeOnDrop` via the `zeroize` crate. This ensures key bytes are scrubbed from memory when the value is dropped, regardless of whether the drop is explicit or triggered by a panic unwind.

```rust
use zeroize::ZeroizeOnDrop;

#[derive(ZeroizeOnDrop)]
pub struct HandshakeState {
    chaining_key: [u8; 32],
    hash: [u8; 32],
    psk: Option<[u8; 32]>,
}
```

Types requiring `ZeroizeOnDrop`:
- Handshake chaining key and hash
- Pre-shared keys (PSK)
- Ephemeral private keys
- Session encryption/decryption keys

Public key material and ciphertext do **not** require zeroing.

## Random Number Generation

Use the `getrandom` crate for all cryptographically secure randomness. Direct reads from `/dev/urandom` are prohibited — they are non-portable and lack the abstraction layer needed for `no_std` targets.

```rust
// WRONG — platform-specific, not portable to no_std
let mut buf = [0u8; 32];
File::open("/dev/urandom")?.read_exact(&mut buf)?;

// CORRECT — uses getrandom crate
let mut buf = [0u8; 32];
getrandom::getrandom(&mut buf)?;
```

## File Descriptor Hygiene

All file descriptors opened in `rustguard-tun` and `rustguard-daemon` must:

1. Be opened with `O_CLOEXEC` to prevent FD leaks into child processes.
2. Have error paths that close the FD before returning — no early-return leaks on TUN creation failure.

```rust
// Open with O_CLOEXEC
let fd = unsafe {
    libc::open(path.as_ptr(), libc::O_RDWR | libc::O_CLOEXEC)
};
if fd < 0 {
    return Err(io::Error::last_os_error());
}
```

## Struct Layout for `ioctl` / Wire Formats

When defining structs for kernel `ioctl` calls or wire-format serialization, all padding fields must be explicitly specified. Relying on implicit struct padding leads to uninitialized byte reads (the `ifreq` bug from Commit 5 — 12-byte padding instead of 20 caused stack garbage to be read for MTU).

```rust
// WRONG — implicit padding, size may differ between platforms
#[repr(C)]
struct Ifreq {
    ifr_name: [u8; 16],
    ifr_mtu: i32,
}

// CORRECT — explicit padding to match kernel ABI
#[repr(C)]
struct Ifreq {
    ifr_name: [u8; 16],
    ifr_mtu: i32,
    _pad: [u8; 20],   // must match kernel struct ifreq union size
}
```

Always verify struct sizes with a compile-time assertion:

```rust
const _: () = assert!(core::mem::size_of::<Ifreq>() == 40);
```

## Examples

The following example demonstrates the key conventions together: `Option`-returning encrypt, constant-time MAC comparison, and `ZeroizeOnDrop` on a composite secret type.

```rust
use subtle::ConstantTimeEq;
use zeroize::ZeroizeOnDrop;

#[derive(ZeroizeOnDrop)]
struct SessionKeys {
    send_key: [u8; 32],
    recv_key: [u8; 32],
}

/// Verifies a 16-byte MAC in constant time.
/// Returns true only if `received` matches `expected` in constant time.
fn verify_mac(expected: &[u8; 16], received: &[u8; 16]) -> bool {
    expected.ct_eq(received).into()
}

/// Encrypts a transport message, returning None on nonce exhaustion.
fn encrypt_transport(
    key: &[u8; 32],
    nonce: u64,
    plaintext: &[u8],
) -> Option<Vec<u8>> {
    // Nonce counter for ChaCha20-Poly1305 is limited to 2^64-1;
    // WireGuard renegotiates before 2^60 messages.
    if nonce >= (1u64 << 60) {
        return None;
    }
    // ... AEAD encryption ...
    Some(vec![]) // placeholder
}

fn main() {
    let keys = SessionKeys {
        send_key: [0u8; 32],
        recv_key: [0u8; 32],
    };

    let mac_expected = [0u8; 16];
    let mac_received = [0u8; 16];
    assert!(verify_mac(&mac_expected, &mac_received));

    let ciphertext = encrypt_transport(&keys.send_key, 0, b"hello");
    assert!(ciphertext.is_some());
    // keys is ZeroizeOnDrop — memory scrubbed here
}
```

## See Also

- [Source Tree](05-Development/01-Source-Tree.md) — crate responsibilities and module layout
- [Build and Test](05-Development/02-Build-and-Test.md) — running `clippy`, `fmt`, and the test suite
- [Release Process](05-Development/04-Release-Process.md) — pre-release checklist that enforces these conventions
- [Design Decisions](02-Architecture/04-Design-Decisions.md) — ADRs explaining why security patterns were chosen