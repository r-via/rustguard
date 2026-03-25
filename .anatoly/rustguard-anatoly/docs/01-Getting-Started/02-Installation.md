# Installation

> How to build RustGuard from source, install the CLI binary, and optionally build the out-of-tree kernel module.

## Overview

RustGuard is distributed as source code organized in a Cargo workspace of seven crates. Installation requires building from source with the standard Rust toolchain. The userspace daemon runs on macOS and Linux; the kernel module targets Linux 6.10+ only.

## Prerequisites

### Userspace daemon (all platforms)

| Requirement | Notes |
|---|---|
| Rust toolchain (`rustc`, `cargo`) | Stable channel is sufficient for all userspace crates |
| Linux or macOS | Linux uses `/dev/net/tun`; macOS uses `utun` via kernel control sockets |
| Root or `CAP_NET_ADMIN` | Required at runtime to create TUN interfaces and add routes |

### Kernel module (`rustguard-kmod` only)

| Requirement | Notes |
|---|---|
| Linux kernel 6.10+ | The kernel must be built with `CONFIG_RUST=y` |
| Matching kernel headers | Required for out-of-tree module compilation |
| C compiler (`gcc` or `clang`) | Required for the C shim component of `rustguard-kmod` |
| Rust toolchain matching kernel's bindgen version | Out-of-tree Rust kernel modules require a specific Rust version matching the kernel build |

## Building from Source

### Clone the repository

```bash
git clone https://github.com/your-org/rustguard.git
cd rustguard
```

### Build all userspace crates

```bash
cargo build --release
```

This compiles all crates in the workspace. The `rustguard-kmod` crate is excluded from the default workspace build because it requires the kernel build system.

The resulting CLI binary is placed at:

```
target/release/rustguard
```

### Install the CLI binary

Copy the binary to a directory on `PATH`:

```bash
install -m 0755 target/release/rustguard /usr/local/bin/rustguard
```

### Build individual crates

To build only a specific crate — for example, to verify the `no_std` crypto layer in isolation:

```bash
cargo build --release -p rustguard-crypto
cargo build --release -p rustguard-core
```

Both `rustguard-crypto` and `rustguard-core` are dual `std`/`no_std` and compile without the standard library when the `no_std` feature is selected.

## Building the Kernel Module

The `rustguard-kmod` crate is an out-of-tree Linux kernel module composed of Rust code and a C shim. It must be built against a Linux 6.10+ source tree with `CONFIG_RUST=y`.

### Verify kernel Rust support

```bash
cat /boot/config-$(uname -r) | grep CONFIG_RUST
```

The output must include:

```
CONFIG_RUST=y
```

### Build against the running kernel

```bash
cd rustguard-kmod
make KDIR=/lib/modules/$(uname -r)/build
```

### Load the module

```bash
sudo insmod rustguard.ko
```

### Unload the module

```bash
sudo rmmod rustguard
```

> **Note:** The kernel module eliminates the TUN overhead present in the userspace daemon by operating entirely within kernel context. For most deployments, the userspace daemon is sufficient. The kernel module is intended for performance-critical scenarios where the TUN bottleneck has been measured.

## Verifying the Installation

### Check the binary is reachable

```bash
rustguard --version
```

### Generate a key pair

```bash
rustguard genkey
rustguard genkey | rustguard pubkey
```

Expected output (example — keys are randomly generated):

```
wHaQ7fP2gK3mNxLtVoUyRiDcBsEjZnM4ApCqW6Xb8/0=
```

### Confirm the subcommands are available

```bash
rustguard --help
```

The following subcommands should be listed: `up`, `serve`, `join`, `open`, `close`, `status`, `genkey`, `pubkey`.

## Runtime State Directory

RustGuard writes enrollment server state to:

```
~/.rustguard/state.json
```

This file is created automatically on first run of `rustguard serve`. It stores peer public keys and assigned IP addresses so that peer state survives server restarts. The directory and file do not need to be created manually.

## Examples

### Full build and smoke test

```bash
# Build
git clone https://github.com/your-org/rustguard.git
cd rustguard
cargo build --release

# Install
sudo install -m 0755 target/release/rustguard /usr/local/bin/rustguard

# Verify: generate a keypair
PRIVATE=$(rustguard genkey)
PUBLIC=$(echo "$PRIVATE" | rustguard pubkey)
echo "Private: $PRIVATE"
echo "Public:  $PUBLIC"

# Verify: check enrollment status (no server running — exits with error, confirms binary works)
rustguard status
```

### Kernel module build (Linux 6.10+)

```bash
# Confirm Rust support in running kernel
grep CONFIG_RUST /boot/config-$(uname -r)

# Build the out-of-tree module
cd rustguard/rustguard-kmod
make KDIR=/lib/modules/$(uname -r)/build

# Load
sudo insmod rustguard.ko

# Confirm loaded
lsmod | grep rustguard

# Unload
sudo rmmod rustguard
```

## See Also

- [Overview](01-Overview.md) — Crate structure and feature summary
- [Quick Start](04-Quick-Start.md) — End-to-end tunnel setup after installation
- [Configuration](03-Configuration.md) — `wg.conf` format and enrollment flags
- [Source Tree](../05-Development/01-Source-Tree.md) — Annotated workspace layout
- [Build and Test](../05-Development/02-Build-and-Test.md) — Developer build targets, test suite, and CI