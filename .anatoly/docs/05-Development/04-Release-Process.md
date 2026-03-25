# Release Process

> Versioning, release checklist, and artifact publication for the RustGuard workspace.

## Overview

RustGuard is a Cargo workspace of seven crates. A release involves bumping versions across the workspace manifest, running the full test suite, producing release-mode binaries and the kernel module artifact, and tagging the repository. Because the project includes an out-of-tree kernel module targeting Linux 6.10+, the release pipeline has two distinct tracks: the userspace track (standard Cargo) and the kernel module track (out-of-tree `make` build).

## Prerequisites

All prerequisites are shared with the [Build and Test](02-Build-and-Test.md) page. The following must be available before cutting a release:

| Dependency | Requirement | Reason |
|---|---|---|
| Rust stable toolchain | stable | userspace crate compilation |
| Rust nightly toolchain | nightly | `rustguard-kmod` Rust-in-kernel compilation |
| C compiler (`cc`) | any system C compiler | `rustguard-kmod` C shim |
| Linux kernel headers | 6.10+ | `rustguard-kmod` out-of-tree build |
| `iperf3` | any recent version | pre-release benchmark verification |

## Versioning

RustGuard follows [Semantic Versioning](https://semver.org/). All seven crates share a synchronized version number declared in the workspace root `Cargo.toml`. A single version bump in the workspace manifest propagates to all members.

### Workspace `Cargo.toml` version field

```toml
[workspace.package]
version = "1.2.0"
```

Crate-level `Cargo.toml` files inherit this via:

```toml
[package]
version.workspace = true
```

### Version increment rules

| Change type | Increment |
|---|---|
| Backward-compatible bug fix | patch (`1.2.0` → `1.2.1`) |
| New backward-compatible functionality | minor (`1.2.0` → `1.3.0`) |
| Breaking public API or wire-format change | major (`1.2.0` → `2.0.0`) |

Wire-format changes in `rustguard-core/src/messages.rs` (Initiation, Response, CookieReply, Transport) always require a major version bump regardless of Rust API compatibility.

## Pre-Release Checklist

Run each step in order. Do not proceed past a failing step.

### 1. Verify the full test suite passes

```bash
cargo test --workspace
```

The workspace contains 80 tests distributed across all crates. All must pass.

### 2. Verify `no_std` compatibility for foundational crates

`rustguard-crypto` and `rustguard-core` are shared verbatim with `rustguard-kmod`. Confirm they compile against a bare-metal target before release:

```bash
cargo build -p rustguard-crypto --target thumbv7em-none-eabihf
cargo build -p rustguard-core   --target thumbv7em-none-eabihf
```

### 3. Run linting

```bash
cargo clippy --workspace -- -D warnings
cargo fmt --all -- --check
```

### 4. Build the full workspace in release mode

```bash
cargo build --workspace --release
```

Confirm the CLI binary is present:

```bash
./target/release/rustguard --help
```

Expected subcommands: `up`, `serve`, `join`, `open`, `close`, `status`, `genkey`, `pubkey`.

### 5. Build the kernel module

```bash
cd rustguard-kmod
make KERNELDIR=/lib/modules/$(uname -r)/build
```

This step requires a Linux 6.10+ kernel with matching headers installed and the nightly Rust toolchain configured in the module's `rust-toolchain.toml`.

## Building Release Artifacts

### Userspace binaries

```bash
cargo build --workspace --release
```

The CLI binary is produced at `target/release/rustguard`. Individual crate libraries are at `target/release/lib*.rlib` / `target/release/lib*.so`.

### Kernel module artifact

```bash
cd rustguard-kmod
make KERNELDIR=/lib/modules/$(uname -r)/build
```

The compiled kernel object is produced as `rustguard-kmod/rustguard.ko`.

### Packaging the CLI and kernel module together

```bash
VERSION=$(cargo metadata --no-deps --format-version 1 \
  | python3 -c "import sys,json; print(json.load(sys.stdin)['packages'][0]['version'])")

mkdir -p dist/rustguard-${VERSION}/bin
mkdir -p dist/rustguard-${VERSION}/kmod

cp target/release/rustguard      dist/rustguard-${VERSION}/bin/
cp rustguard-kmod/rustguard.ko   dist/rustguard-${VERSION}/kmod/

tar -czf dist/rustguard-${VERSION}.tar.gz -C dist rustguard-${VERSION}/
```

## Tagging the Release

After all pre-release steps pass and release artifacts are built, tag the commit:

```bash
git tag -a "v${VERSION}" -m "Release v${VERSION}"
git push origin "v${VERSION}"
```

Tag format is `v<MAJOR>.<MINOR>.<PATCH>` (e.g., `v1.3.0`).

## Post-Release Verification

After tagging, verify the release artifacts on a clean Debian 12 VM — the reference platform used throughout development — by running a basic tunnel smoke test:

```bash
# Generate a keypair to confirm the binary functions
./rustguard genkey | ./rustguard pubkey

# Bring up a tunnel using a wg.conf config
./rustguard up wg0.conf

# In a separate terminal, verify the enrollment server starts
./rustguard serve --pool 10.150.0.0/24 --token smoketest
```

For the kernel module, load and confirm it registers without errors:

```bash
sudo insmod rustguard-kmod/rustguard.ko
dmesg | tail -20
sudo rmmod rustguard
```

## Examples

### Full release workflow from version bump to tag

```bash
# 1. Bump workspace version
sed -i 's/^version = ".*"/version = "1.3.0"/' Cargo.toml

# 2. Regenerate lockfile
cargo update --workspace

# 3. Run tests
cargo test --workspace

# 4. Verify no_std targets
cargo build -p rustguard-crypto --target thumbv7em-none-eabihf
cargo build -p rustguard-core   --target thumbv7em-none-eabihf

# 5. Lint
cargo clippy --workspace -- -D warnings
cargo fmt --all -- --check

# 6. Release build
cargo build --workspace --release

# 7. Kernel module
cd rustguard-kmod && make KERNELDIR=/lib/modules/$(uname -r)/build && cd ..

# 8. Commit, tag, push
git add Cargo.toml Cargo.lock
git commit -m "chore: release v1.3.0"
git tag -a "v1.3.0" -m "Release v1.3.0"
git push origin main "v1.3.0"
```

## See Also

- [Build and Test](02-Build-and-Test.md) — full build, test, and benchmark reference
- [Source Tree](01-Source-Tree.md) — workspace layout and crate responsibilities
- [Code Conventions](03-Code-Conventions.md) — security patterns and style guide to verify before release
- [System Overview](../02-Architecture/01-System-Overview.md) — component architecture context