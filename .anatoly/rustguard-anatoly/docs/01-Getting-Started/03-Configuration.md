# Configuration

> Reference for all RustGuard configuration methods: classic `wg.conf` files, enrollment-mode flags, runtime control commands, and persisted state.

## Overview

RustGuard supports two distinct configuration paths that can coexist in the same installation. **Config-file mode** reads a standard WireGuard `wg.conf` file and is fully backward-compatible with existing configurations. **Enrollment mode** requires no config files: a shared token and an IP pool flag on the server side are sufficient to bring up a tunnel and issue addresses to joining clients.

Runtime enrollment-window state is managed through a UNIX domain control socket and persisted across restarts in `~/.rustguard/state.json`.

For installation prerequisites, see [Installation](02-Installation.md). For step-by-step tunnel setup walkthroughs, see [Quick Start](04-Quick-Start.md).

---

## Config-File Mode

Invoke config-file mode with the `up` subcommand, passing the path to a standard WireGuard configuration file:

```bash
rustguard up wg0.conf
```

RustGuard parses the standard `[Interface]` / `[Peer]` INI format used by WireGuard. The `AllowedIPs` field accepts both IPv4 and IPv6 CIDR ranges; dual-stack routes are installed for both address families.

### Minimal wg.conf Example

```ini
[Interface]
PrivateKey = <base64-encoded-private-key>
ListenPort = 51820
Address = 10.0.0.1/24

[Peer]
PublicKey = <base64-encoded-peer-public-key>
AllowedIPs = 10.0.0.2/32
Endpoint = 192.0.2.2:51820
```

### Dual-Stack AllowedIPs

```ini
[Peer]
PublicKey = <base64-encoded-peer-public-key>
AllowedIPs = 10.0.0.2/32, fd00::2/128
Endpoint = 192.0.2.2:51820
```

Both address families are parsed, and routes are installed for each CIDR block individually.

---

## Enrollment Mode

Enrollment mode eliminates manual key exchange and IP planning. The server exposes a pairing endpoint; clients derive the session key from a shared token and receive an assigned tunnel address automatically.

### Server Configuration — `rustguard serve`

```bash
rustguard serve --pool <CIDR> --token <TOKEN>
```

| Flag | Type | Description |
|------|------|-------------|
| `--pool` | CIDR string | Address pool from which peer IPs are allocated. The server claims the `.1` address; clients receive sequential addresses starting at `.2`. |
| `--token` | string | Shared enrollment secret. The token derives an XChaCha20 key that encrypts the key exchange during enrollment. |

**Example:**

```bash
rustguard serve --pool 10.150.0.0/24 --token mysecret
```

The server allocates `10.150.0.1` for its own tunnel interface and hands out `10.150.0.2`, `10.150.0.3`, … to enrolling clients. State is written to `~/.rustguard/state.json` on each successful enrollment; see [State Persistence](#state-persistence) below.

### Client Configuration — `rustguard join`

```bash
rustguard join <SERVER_ADDRESS> --token <TOKEN>
```

| Argument | Type | Description |
|----------|------|-------------|
| `SERVER_ADDRESS` | `host:port` string | Public address and UDP port of the enrollment server. |
| `--token` | string | Must match the token passed to `rustguard serve`. |

**Example:**

```bash
rustguard join 192.0.2.1:51820 --token mysecret
```

No configuration files are written. The client receives its assigned address and the tunnel interface is brought up immediately upon successful enrollment.

> **Note:** The enrollment window on the server must be open before `rustguard join` is invoked. See [Enrollment Window Controls](#enrollment-window-controls) below.

---

## Enrollment Window Controls

The enrollment server starts with its pairing window **closed**. New clients cannot join until the window is explicitly opened. This physical-presence model prevents unattended enrollment.

Control commands communicate with the server over its UNIX domain socket:

| Command | Description |
|---------|-------------|
| `rustguard open <SECONDS>` | Opens the enrollment window for the specified duration. The window closes automatically when the timer expires. |
| `rustguard close` | Immediately closes the enrollment window, regardless of remaining time. |
| `rustguard status` | Prints the current window state and the number of active peers. |

**Example workflow:**

```bash
# Open window for 60 seconds, enroll a client, then close immediately
rustguard open 60
# (on client machine) rustguard join 192.0.2.1:51820 --token mysecret
rustguard close
rustguard status
```

Existing peers are never affected by window state changes. Only new enrollment attempts are gated.

---

## State Persistence

When running in enrollment mode, RustGuard persists server state to:

```
~/.rustguard/state.json
```

The state file records enrolled peer public keys and their assigned IP addresses. On server restart, previously enrolled peers reconnect without re-running the enrollment protocol.

The file is written on each successful enrollment. Do not edit it manually while the server is running. Back up this file before re-provisioning the server if peer assignments must be preserved.

---

## Key Generation

RustGuard includes built-in key generation utilities:

```bash
# Generate a new private key
rustguard genkey

# Derive the corresponding public key from a private key
rustguard genkey | rustguard pubkey

# Save keys to files
rustguard genkey | tee private.key | rustguard pubkey > public.key
```

Both commands operate on standard Base64-encoded WireGuard key material and are compatible with existing WireGuard tooling.

---

## CLI Subcommand Summary

| Subcommand | Purpose |
|------------|---------|
| `up <config>` | Bring up a tunnel from a `wg.conf` file |
| `serve --pool <CIDR> --token <TOKEN>` | Start an enrollment server |
| `join <addr:port> --token <TOKEN>` | Enroll as a client against a running server |
| `open <seconds>` | Open the enrollment window for N seconds |
| `close` | Close the enrollment window immediately |
| `status` | Show enrollment window state and peer count |
| `genkey` | Generate a new private key |
| `pubkey` | Derive a public key from a private key on stdin |

---

## Examples

### Zero-Config Enrollment (Full Flow)

```bash
# --- Server machine ---
rustguard serve --pool 10.150.0.0/24 --token supersecret &
rustguard open 120   # allow 2 minutes for clients to enroll

# --- Client machine ---
rustguard join 203.0.113.10:51820 --token supersecret

# --- Server machine: verify and lock down ---
rustguard status
rustguard close
```

### Config-File Tunnel with Dual-Stack Peers

```ini
[Interface]
PrivateKey = <private-key>
ListenPort = 51820
Address = 10.0.0.1/24, fd00::1/64

[Peer]
PublicKey = <peer-public-key>
AllowedIPs = 10.0.0.2/32, fd00::2/128
Endpoint = 198.51.100.5:51820
PresharedKey = <optional-psk>
```

```bash
rustguard up /etc/rustguard/wg0.conf
```

### Key Pair Generation

```bash
PRIVATE=$(rustguard genkey)
PUBLIC=$(echo "$PRIVATE" | rustguard pubkey)
echo "Private: $PRIVATE"
echo "Public:  $PUBLIC"
```

---

## See Also

- [Quick Start](04-Quick-Start.md) — end-to-end enrollment walkthrough
- [Installation](02-Installation.md) — build and install the `rustguard` binary
- [Common Workflows](../03-Guides/01-Common-Workflows.md) — step-by-step guides for frequent use cases
- [Advanced Configuration](../03-Guides/02-Advanced-Configuration.md) — tuning and advanced options
- [Configuration Schema](../04-API-Reference/02-Configuration-Schema.md) — complete config schema with defaults and validation