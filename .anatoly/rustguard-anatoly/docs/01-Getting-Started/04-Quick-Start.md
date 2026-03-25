# Quick Start

> An end-to-end tutorial demonstrating the two primary RustGuard tunnel modes, from key generation to a running encrypted tunnel.

## Overview

This guide walks through two complete scenarios: the **zero-config enrollment** path (`serve` / `join`) and the **standard config-file** path (`up`). Each scenario ends with an encrypted WireGuard tunnel ready to carry traffic.

Before proceeding, ensure the `rustguard` binary is installed and accessible on `PATH`. See [Installation](02-Installation.md) for build instructions.

---

## Scenario A — Zero-Config Enrollment (Recommended)

The enrollment path requires no manual key exchange or IP planning. The server manages an IP pool; each client that joins receives the next available address. Server state is persisted to `~/.rustguard/state.json` and survives restarts.

### Step 1 — Start the enrollment server

On the machine that will act as the VPN gateway, start the daemon in server mode:

```bash
rustguard serve --pool 10.150.0.0/24 --token mysecret
```

The server claims `10.150.0.1` within the pool and begins listening on UDP port 51820. The enrollment window starts **closed** — no new peers can join until the window is explicitly opened.

### Step 2 — Open the enrollment window

On the **server** machine, open the enrollment window while a client joins. The argument is the window duration in seconds:

```bash
rustguard open 60
```

The window closes automatically after 60 seconds. To close it immediately:

```bash
rustguard close
```

To inspect the current enrollment state and connected peer count:

```bash
rustguard status
```

### Step 3 — Join from the client

On the client machine, while the server's enrollment window is open:

```bash
rustguard join 203.0.113.1:51820 --token mysecret
```

The client performs an encrypted key exchange with the server, receives an allocated IP address (e.g., `10.150.0.2`), and brings up the tunnel automatically. No config file is written or required.

### Step 4 — Verify connectivity

From the client, ping the server's tunnel address:

```bash
ping 10.150.0.1
```

For a complete reference to all `serve` and `join` flags, see [Configuration](03-Configuration.md).

---

## Scenario B — Standard Config-File Mode

This mode is compatible with existing `wg.conf` files produced by upstream WireGuard tooling.

### Step 1 — Generate key pairs

Generate a private key and derive the corresponding public key on **each peer**:

```bash
# Peer A
rustguard genkey | tee peer-a.key | rustguard pubkey
```

```bash
# Peer B
rustguard genkey | tee peer-b.key | rustguard pubkey
```

Each command writes the private key to a file and prints the Base64-encoded public key to stdout. Record both public keys — each peer needs the other's public key in its config.

### Step 2 — Write configuration files

**Peer A** (`wg0.conf`):

```ini
[Interface]
PrivateKey = <peer-a private key>
Address    = 10.0.0.1/24
ListenPort = 51820

[Peer]
PublicKey  = <peer-b public key>
Endpoint   = 203.0.113.2:51820
AllowedIPs = 10.0.0.2/32
```

**Peer B** (`wg0.conf`):

```ini
[Interface]
PrivateKey = <peer-b private key>
Address    = 10.0.0.2/24
ListenPort = 51820

[Peer]
PublicKey  = <peer-a public key>
Endpoint   = 203.0.113.1:51820
AllowedIPs = 10.0.0.1/32
```

For the full field reference — including `PresharedKey`, IPv6 `Address` entries, and multi-peer configs — see [Configuration](03-Configuration.md).

### Step 3 — Bring up the tunnel

Run on both peers:

```bash
rustguard up wg0.conf
```

The daemon reads the config file, creates the TUN interface, configures the routes, and enters the tunnel loop. On SIGINT or SIGTERM, the daemon removes the routes it added and tears down the interface cleanly.

### Step 4 — Verify connectivity

From Peer A, ping Peer B's tunnel address:

```bash
ping 10.0.0.2
```

---

## Adding a Pre-Shared Key (PSK)

Both modes support an optional pre-shared key for post-quantum resistance. In config-file mode, add `PresharedKey` to the `[Peer]` block:

```ini
[Peer]
PublicKey    = <peer-b public key>
Endpoint     = 203.0.113.2:51820
AllowedIPs   = 10.0.0.2/32
PresharedKey = <base64-encoded 32-byte PSK>
```

The PSK activates the `psk2` layer of the Noise_IKpsk2 handshake. Both peers must specify the same PSK. Generate a random 32-byte PSK with:

```bash
rustguard genkey
```

---

## Quick Reference — CLI Subcommands

| Subcommand | Mode | Description |
|---|---|---|
| `up <config>` | Config-file | Bring up a tunnel from a `wg.conf` file |
| `serve` | Enrollment | Start an enrollment server with an IP pool |
| `join <host:port>` | Enrollment | Join an enrollment server |
| `open <seconds>` | Enrollment | Open the enrollment window on a running server |
| `close` | Enrollment | Close the enrollment window immediately |
| `status` | Enrollment | Show enrollment window state and peer count |
| `genkey` | Utility | Generate a random X25519 private key |
| `pubkey` | Utility | Derive the public key from a private key on stdin |

---

## See Also

- [Installation](02-Installation.md) — Build prerequisites and kernel module setup
- [Configuration](03-Configuration.md) — Complete flag reference for all subcommands and the `wg.conf` schema
- [Common Workflows](../03-Guides/01-Common-Workflows.md) — Multi-peer setups, route configuration, and other step-by-step guides
- [System Overview](../02-Architecture/01-System-Overview.md) — Crate responsibilities and component architecture