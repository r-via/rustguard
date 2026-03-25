# Build and Test

> Complete reference for building, testing, linting, and benchmarking the RustGuard workspace.

## Overview

RustGuard is a Cargo workspace of seven crates totalling approximately 8,500 lines of Rust, covered by 80 tests distributed across the workspace. Two foundational crates (`rustguard-crypto` and `rustguard-core`) support dual `std`/`no_std` targets. The `rustguard-kmod` crate is an out-of-tree kernel module that requires a Linux 6.10+ build environment.

This page covers day-to-day development workflows. For the full pre-release checklist that must pass before tagging a version, see [Release Process](04-Release-Process.md).

## Prerequisites

| Requirement | Notes |
|-------------|-------|
| Rust toolchain | Installed via [rustup](https://rustup.rs/). Stable channel. |
| `thumbv7em-none-eabihf` target | Required only for `no_std` build verification. Install with `rustup target add thumbv7em-none-eabihf`. |
| Linux 6.10+ kernel headers | Required only to build `rustguard-kmod`. Must be present on the build host or inside a dedicated VM/container. |

## Building

### Full Workspace

Build all crates in debug mode:

```bash
cargo build --workspace
```

Build all crates in release mode:

```bash
cargo build --workspace --release
```

### Single Crate

Build only a specific crate (useful during development to keep iteration times short):

```bash
cargo build -p rustguard-crypto
cargo build -p rustguard-core
cargo build -p rustguard-daemon
cargo build -p rustguard-enroll
cargo build -p rustguard-cli
cargo build -p rustguard-tun
```

### Kernel Module

`rustguard-kmod` is an out-of-tree kernel module targeting Linux 6.10+. It must be built on a host that has matching kernel headers installed, or inside a suitably configured VM or container:

```bash
cargo build -p rustguard-kmod --release
```

If the host does not run Linux 6.10+, this step must be performed in a separate environment. All other crates build and test cleanly without a Linux 6.10+ host.

### `no_std` Builds

`rustguard-crypto` and `rustguard-core` are dual `std`/`no_std` and are reused verbatim inside `rustguard-kmod`. Verify that both crates compile cleanly for a bare-metal target:

```bash
cargo build -p rustguard-crypto --no-default-features --target thumbv7em-none-eabihf
cargo build -p rustguard-core   --no-default-features --target thumbv7em-none-eabihf
```

`--no-default-features` disables the `std` feature flag so the compiler uses the `no_std` code paths. See [Source Tree](01-Source-Tree.md) for the `cfg_attr` pattern used in these crates.

## Running Tests

### All Tests

```bash
cargo test --workspace
```

This runs all 80 tests across `rustguard-crypto`, `rustguard-core`, `rustguard-tun`, `rustguard-daemon`, `rustguard-enroll`, and `rustguard-cli`. All must pass green before opening a pull request.

### Single Crate

```bash
cargo test -p rustguard-crypto
cargo test -p rustguard-core
cargo test -p rustguard-enroll
```

### Single Test by Name

```bash
cargo test -p rustguard-core replay_window
```

### With Output

By default Cargo captures test output. Pass `--nocapture` to print directly to stdout (useful when debugging a failing test):

```bash
cargo test --workspace -- --nocapture
```

## Linting

### Clippy

All warnings are treated as errors. Run Clippy against every crate and every target (lib, bin, tests, benches):

```bash
cargo clippy --workspace --all-targets -- -D warnings
```

Fix all Clippy diagnostics before submitting. The `-D warnings` flag matches the CI gate — a locally passing `cargo clippy` without `-D warnings` may still fail in CI.

### Formatting

Check that all source files conform to `rustfmt` without modifying them:

```bash
cargo fmt --workspace -- --check
```

Apply formatting in-place during development:

```bash
cargo fmt --workspace
```

## Benchmarks

Benchmark infrastructure is included in the workspace. Results are informational and are used to track regressions across releases, not as a hard pass/fail gate:

```bash
cargo bench --workspace
```

Run a specific benchmark suite:

```bash
cargo bench -p rustguard-crypto
```

## Dependency Audit

Scan the dependency tree against the RustSec advisory database:

```bash
cargo audit
```

Resolve any advisories that appear before merging security-sensitive changes. The release process treats any open advisory as a blocker — see [Release Process](04-Release-Process.md).

## Examples

The following sequence replicates a full local CI pass against the userspace crates (excluding the kernel module, which requires a Linux 6.10+ host):

```bash
# 1. Format check
cargo fmt --workspace -- --check

# 2. Lint
cargo clippy --workspace --all-targets -- -D warnings

# 3. Tests
cargo test --workspace

# 4. no_std verification
cargo build -p rustguard-crypto --no-default-features --target thumbv7em-none-eabihf
cargo build -p rustguard-core   --no-default-features --target thumbv7em-none-eabihf

# 5. Dependency audit
cargo audit
```

On a Linux 6.10+ host, add the kernel module build:

```bash
cargo build -p rustguard-kmod --release
```

## See Also

- [Source Tree](01-Source-Tree.md) — Crate layout and module responsibilities, including the `no_std` pattern used in `rustguard-crypto` and `rustguard-core`
- [Code Conventions](03-Code-Conventions.md) — Style guide and security patterns that Clippy and review enforce
- [Release Process](04-Release-Process.md) — Ordered pre-release checklist that incorporates all of the above steps