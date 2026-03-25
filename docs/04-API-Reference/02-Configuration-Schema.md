# Configuration Schema

> Complete reference for RustGuard's configuration file format, persistence schema, and CLI configuration flags.

## Overview

RustGuard accepts configuration through two mechanisms:

- **`wg.conf` files** — Standard WireGuard INI-format configuration, consumed by `rustguard up`. Fully compatible with the WireGuard specification's `[Interface]` and `[Peer]` sections.
- **`~/.rustguard/state.json`** — Server-side persistence file written and read by `rustguard serve`. Stores enrolled peer keys and assigned IP addresses across restarts.

CLI flags passed to `rustguard serve`, `rustguard join`, `rustguard open`, and `rustguard close` are documented in the [Public API Reference](./01-Public-API.md). Types referenced throughout this page are defined in [Types and Interfaces](./03-Types-and-Interfaces.md).

---

## `wg.conf` Format

Used exclusively by `rustguard up <CONFIG_FILE>`. Follows the standard WireGuard configuration syntax.

### `[Interface]` Section

Defines the local tunnel endpoint.

| Field | Type | Required | Description |
|---|---|---|---|
| `PrivateKey` | Base64 string (32 bytes) | Yes | The local peer's X25519 private key. See `rustguard genkey`. |
| `Address` | CIDR string | Yes | The IP address (IPv4 or IPv6) assigned to the TUN interface. Multiple addresses are comma-separated for dual-stack. |
| `ListenPort` | `u16` | Yes | UDP port to listen on for incoming WireGuard packets. |
| `DNS` | IP string | No | DNS server to configure when the tunnel is up. |
| `MTU` | `u32` | No | MTU for the TUN interface. Defaults to 1420 if omitted. |

### `[Peer]` Section

One `[Peer]` block per remote peer. Multiple blocks are supported.

| Field | Type | Required | Description |
|---|---|---|---|
| `PublicKey` | Base64 string (32 bytes) | Yes | The remote peer's X25519 public key. Used in the Noise_IKpsk2 handshake. |
| `PresharedKey` | Base64 string (32 bytes) | No | Optional 256-bit pre-shared key (PSK) for post-quantum resistance. Both peers must specify the same value. |
| `AllowedIPs` | CIDR list | Yes | Comma-separated IPv4 and/or IPv6 CIDR ranges routed through this peer. Accepts both `0.0.0.0/0` (full tunnel) and specific subnets. |
| `Endpoint` | `ADDR:PORT` | No | The peer's publicly reachable UDP socket address. Omit for passive (server) peers that initiate connections inbound. |
| `PersistentKeepalive` | `u16` (seconds) | No | Interval in seconds for keepalive packets. Recommended when the local peer is behind NAT. |

### Example — Standard Two-Peer Tunnel

```ini
[Interface]
PrivateKey = aBcd1234aBcd1234aBcd1234aBcd1234aBcd1234aBc=
Address = 10.0.0.1/24, fd00::1/64
ListenPort = 51820

[Peer]
PublicKey = xYz5678xYz5678xYz5678xYz5678xYz5678xYz56=
PresharedKey = psk1234psk1234psk1234psk1234psk1234psk123=
AllowedIPs = 10.0.0.2/32, fd00::2/128
Endpoint = 203.0.113.42:51820
PersistentKeepalive = 25
```

```bash
rustguard up /etc/wireguard/wg0.conf
```

### Example — Dual-Stack Full Tunnel (Road Warrior)

```ini
[Interface]
PrivateKey = clientPrivKey1234clientPrivKey1234clientP=
Address = 10.0.0.50/24
ListenPort = 0

[Peer]
PublicKey = serverPubKey5678serverPubKey5678serverPub=
AllowedIPs = 0.0.0.0/0, ::/0
Endpoint = vpn.example.com:51820
PersistentKeepalive = 20
```

---

## `~/.rustguard/state.json` Schema

Written by `rustguard serve` on peer enrollment and updated on restart. The file is read on startup to restore previously enrolled peers without requiring re-enrollment.

```json
{
  "pool": "10.150.0.0/24",
  "peers": [
    {
      "public_key": "<base64-encoded X25519 public key>",
      "assigned_ip": "10.150.0.2"
    },
    {
      "public_key": "<base64-encoded X25519 public key>",
      "assigned_ip": "10.150.0.3"
    }
  ]
}
```

| Field | Type | Description |
|---|---|---|
| `pool` | CIDR string | The address pool passed to `--pool`. The server always holds `.1`; this value establishes the allocation range. |
| `peers` | Array of peer objects | One entry per successfully enrolled client. |
| `peers[].public_key` | Base64 string (32 bytes) | The client's X25519 public key, received during token-encrypted key exchange. |
| `peers[].assigned_ip` | IP string | The IP address from `pool` assigned to this peer. Addresses are allocated sequentially starting from `.2`. |

**Location:** `~/.rustguard/state.json`. The directory `~/.rustguard/` is created automatically if absent.

**Persistence behavior:** Existing peers in `state.json` are re-admitted to the tunnel on server restart without a new enrollment exchange. New enrollments are appended to the `peers` array.

---

## Enrollment CLI Flags

These flags configure the zero-config enrollment subsystem (`rustguard-enroll`). They are not used with `rustguard up`.

### `rustguard serve`

```bash
rustguard serve --pool <CIDR> --token <SECRET>
```

| Flag | Type | Description |
|---|---|---|
| `--pool` | IPv4 CIDR | Address pool for enrolled clients. Server always occupies `.1`. |
| `--token` | String | Shared secret used to derive the XChaCha20Poly1305 key that encrypts the key exchange. Must match `--token` on all joining clients. |

### `rustguard join`

```bash
rustguard join <ADDR:PORT> --token <SECRET>
```

| Argument/Flag | Type | Description |
|---|---|---|
| `ADDR:PORT` | Socket address | Address of the enrollment server. |
| `--token` | String | Must match the token on the server. |

### `rustguard open`

```bash
rustguard open <SECONDS>
```

| Argument | Type | Description |
|---|---|---|
| `SECONDS` | `u64` | Duration in seconds to hold the enrollment window open. The window closes automatically on expiry. |

### `rustguard close`

```bash
rustguard close
```

No flags. Closes the enrollment window immediately via the UNIX domain control socket. Has no effect on peers already enrolled.

---

## Key Generation

RustGuard provides built-in key generation for use in `wg.conf` files.

```bash
# Generate a private key
rustguard genkey

# Derive the corresponding public key
rustguard genkey | rustguard pubkey
```

Output is a Base64-encoded 32-byte value suitable for direct use in `PrivateKey` and `PublicKey` fields.

---

## Validation Rules

The following constraints are enforced at parse time or tunnel startup:

- `PrivateKey` and `PublicKey` values must decode to exactly 32 bytes.
- `PresharedKey`, if present, must decode to exactly 32 bytes.
- `AllowedIPs` entries must be valid CIDR notation (IPv4 or IPv6). Overlapping ranges across peers are rejected.
- `ListenPort` of `0` is accepted; the OS assigns an ephemeral port (useful for road-warrior clients).
- `Address` fields accept both `/32` (host) and subnet masks; both are applied to the TUN interface.
- `PersistentKeepalive` is interpreted in seconds; `0` disables keepalives.

---

## See Also

- [Public API Reference](./01-Public-API.md) — CLI command signatures and library interfaces
- [Types and Interfaces](./03-Types-and-Interfaces.md) — Rust types for keys, sessions, and handshake state
- [Quick Start](../01-Getting-Started/04-Quick-Start.md) — End-to-end tunnel setup tutorial
- [Common Workflows](../03-Guides/01-Common-Workflows.md) — Step-by-step guides including enrollment workflows
- [Advanced Configuration](../03-Guides/02-Advanced-Configuration.md) — Tuning and override options