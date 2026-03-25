# Common Workflows

> Step-by-step guides for the most frequent RustGuard use cases: key generation, standard tunnel setup, and zero-config enrollment.

## Overview

RustGuard supports two deployment models. The **standard tunnel** model uses a `wg.conf` file compatible with the WireGuard specification — useful when integrating with existing infrastructure. The **zero-config enrollment** model uses `rustguard serve` and `rustguard join` to automate key exchange and IP assignment — useful for homelabs and peer bootstrapping without manual config editing.

Both models share the same underlying Noise_IKpsk2 handshake and transport layer. The workflows below cover the common happy path for each. For advanced options (pool sizing, PSK, IPv6, high-throughput transports) see [Advanced Configuration](02-Advanced-Configuration.md). For error recovery, see [Troubleshooting](03-Troubleshooting.md).

---

## Key Generation

All deployment models require an X25519 keypair per node. RustGuard provides two CLI subcommands for key management.

```bash
# Generate a new private key (base64-encoded, 32 bytes from CSPRNG)
rustguard genkey

# Derive the corresponding public key from a private key
rustguard genkey | rustguard pubkey
```

Store the private key securely. The public key is safe to share with peers and is required when writing `wg.conf` peer sections or when configuring the enrollment server.

---

## Standard Tunnel (wg.conf Mode)

The `rustguard up` subcommand reads a standard WireGuard configuration file and brings up a tunnel. This mode is handled by the `rustguard-daemon` crate.

### 1. Write the Configuration File

```ini
# wg0.conf — Interface + Peer configuration
[Interface]
PrivateKey = <base64-private-key>
Address    = 10.0.0.1/24
ListenPort = 51820

[Peer]
PublicKey  = <base64-peer-public-key>
AllowedIPs = 10.0.0.2/32
Endpoint   = 203.0.113.5:51820
```

Both IPv4 and IPv6 CIDR ranges are accepted in `AllowedIPs` and `Address`. The `Endpoint` field is optional for the listening side.

### 2. Bring Up the Tunnel

```bash
rustguard up wg0.conf
```

RustGuard creates the TUN device, configures routes for all `AllowedIPs` entries, and enters the tunnel loop. On Linux the device uses `/dev/net/tun` (`IFF_TUN | IFF_NO_PI`). On macOS it uses a `utun` interface via kernel control sockets.

### 3. Verify the Tunnel

```bash
# Linux
ip link show wg0
ip route show table main

# macOS
ifconfig utun3
```

Send a test packet through the tunnel:

```bash
ping 10.0.0.2
```

### 4. Shut Down

Send `SIGTERM` or `SIGINT`. RustGuard performs a clean shutdown: routes are removed and the TUN file descriptor is released with `O_CLOEXEC` semantics.

---

## Zero-Config Enrollment

The `rustguard-enroll` crate eliminates manual key exchange. The server manages a CIDR IP pool and issues addresses sequentially; the enrollment key exchange is encrypted with an XChaCha20 key derived from a shared token.

### 1. Start the Enrollment Server

```bash
rustguard serve --pool 10.150.0.0/24 --token mysecret
```

The server assigns itself the `.1` address from the pool (`10.150.0.1` for a `/24`). The enrollment window starts **closed** — no clients can join until the window is explicitly opened.

### 2. Open the Enrollment Window

On the server host, open the window for a bounded duration before the client attempts to join:

```bash
rustguard open 60      # open for 60 seconds
```

The command communicates with the running daemon over a UNIX domain control socket. The window closes automatically when the timer expires.

### 3. Enroll a Client

On the client machine, run:

```bash
rustguard join 1.2.3.4:51820 --token mysecret
```

Replace `1.2.3.4` with the server's public address. The client receives an IP from the pool (e.g. `10.150.0.2`) and a tunnel session is established. No config files are written on the client.

### 4. Verify Enrollment

On the server:

```bash
rustguard status
```

Output includes the current window state (`open` / `closed`) and the number of enrolled peers. The enrollment window can be closed immediately once the expected peer has joined:

```bash
rustguard close
```

Closing the window does not affect traffic for already-enrolled peers.

### 5. Server Restart Behaviour

Peer state (public keys and assigned IPs) is persisted to `~/.rustguard/state.json`. Restarting the server with the same `--pool` and `--token` restores all prior enrollments without requiring clients to re-join.

```bash
cat ~/.rustguard/state.json
```

---

## Managing the Enrollment Window

The enrollment window is controlled independently of the tunnel. The following commands all communicate over the UNIX domain control socket and require the server to be running.

| Command | Effect |
|---|---|
| `rustguard open <seconds>` | Opens the window for the specified duration |
| `rustguard close` | Closes the window immediately |
| `rustguard status` | Prints window state and enrolled peer count |

```bash
# Example: open window, enroll a peer, close immediately
rustguard open 120
# ... client runs: rustguard join 1.2.3.4:51820 --token mysecret
rustguard close
rustguard status
```

None of these commands interrupt active transport sessions.

---

## Examples

### Complete Two-Node Setup via Enrollment

```bash
# --- On the server (203.0.113.5) ---
rustguard serve --pool 10.150.0.0/24 --token homelab2026
rustguard open 60

# --- On the client (any machine) ---
rustguard join 203.0.113.5:51820 --token homelab2026

# --- Verify (server) ---
rustguard status
# window: closed
# peers:  1

# --- Test connectivity ---
ping 10.150.0.1       # client → server
```

### Complete Two-Node Setup via wg.conf

```bash
# --- Generate keys on each node ---

# Node A
PRIVKEY_A=$(rustguard genkey)
PUBKEY_A=$(echo "$PRIVKEY_A" | rustguard pubkey)

# Node B
PRIVKEY_B=$(rustguard genkey)
PUBKEY_B=$(echo "$PRIVKEY_B" | rustguard pubkey)

# --- Write config on Node A ---
cat > /etc/wireguard/wg0.conf <<EOF
[Interface]
PrivateKey = $PRIVKEY_A
Address    = 10.0.0.1/24
ListenPort = 51820

[Peer]
PublicKey  = $PUBKEY_B
AllowedIPs = 10.0.0.2/32
EOF

# --- Write config on Node B ---
cat > /etc/wireguard/wg0.conf <<EOF
[Interface]
PrivateKey = $PRIVKEY_B
Address    = 10.0.0.2/24

[Peer]
PublicKey  = $PUBKEY_A
AllowedIPs = 10.0.0.1/32
Endpoint   = 203.0.113.5:51820
EOF

# --- Bring up tunnels ---
rustguard up /etc/wireguard/wg0.conf   # run on each node
```

---

## See Also

- [Configuration](../01-Getting-Started/03-Configuration.md) — base configuration options and `wg.conf` field reference
- [Advanced Configuration](02-Advanced-Configuration.md) — IP pool sizing, PSK, IPv6 dual-stack, and high-throughput transports
- [Troubleshooting](03-Troubleshooting.md) — enrollment failures, TUN device errors, and diagnostic commands
- [System Overview](../02-Architecture/01-System-Overview.md) — component responsibilities and crate layout
- [Public API](../04-API-Reference/01-Public-API.md) — exported functions for programmatic tunnel control