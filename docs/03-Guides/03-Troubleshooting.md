# Troubleshooting

> Common errors, diagnostics, and FAQ for RustGuard tunnel setup, enrollment, and runtime operation.

## Overview

This page documents known failure modes, their root causes, and corrective steps for RustGuard. Issues are grouped by subsystem: tunnel bring-up (`rustguard up`), zero-config enrollment (`serve`/`join`), runtime management (`open`/`close`/`status`), state persistence, and the kernel module.

For standard setup workflows see [Common Workflows](01-Common-Workflows.md). For advanced tuning options see [Advanced Configuration](02-Advanced-Configuration.md).

---

## Diagnostic Approach

Before consulting the sections below, collect baseline information:

```bash
# Confirm the binary is reachable and show version
rustguard --version

# Check the current enrollment server state and peer count
rustguard status

# Inspect persisted peer state
cat ~/.rustguard/state.json
```

RustGuard runs in the foreground by default. Redirect stderr to a file to capture handshake and timer events:

```bash
rustguard up wg0.conf 2>rustguard.log &
tail -f rustguard.log
```

---

## Tunnel Bring-Up (`rustguard up`)

### TUN Device Permission Denied

**Symptom:** `rustguard up wg0.conf` exits immediately with a permission error referencing `/dev/net/tun` (Linux) or a kernel control socket (macOS).

**Cause:** The process does not have permission to open the TUN character device.

**Fix (Linux):**
```bash
# Add the current user to the 'netdev' or 'tun' group (distro-dependent)
sudo usermod -aG netdev $USER

# Or run with elevated privileges
sudo rustguard up wg0.conf
```

**Fix (macOS):** The utun interface is opened via a kernel control socket. No `/dev/tun` node exists; the process requires no special group, but System Integrity Protection must not block the control socket. Running as root is the reliable path on macOS.

### Route Conflicts

**Symptom:** The tunnel comes up but traffic to certain destinations is routed incorrectly, or `rustguard up` logs a route installation error.

**Cause:** An `AllowedIPs` CIDR in `wg0.conf` overlaps with an existing route on the host.

**Fix:** Inspect the host routing table and choose non-overlapping CIDRs:

```bash
# Linux
ip route show

# macOS
netstat -rn
```

Narrow the `AllowedIPs` entry to avoid the conflict, or remove the conflicting host route if it is no longer needed. Both IPv4 and IPv6 CIDRs are installed; check both families.

### Handshake Never Completes

**Symptom:** The tunnel interface appears but pings time out. No traffic passes.

**Cause — Firewall blocking UDP 51820:** The WireGuard port is not reachable between peers.

```bash
# Verify UDP reachability from the initiator side
nc -u -vz <peer-endpoint> 51820
```

**Cause — Clock skew / timestamp rejected:** RustGuard enforces TAI64N timestamp freshness to prevent handshake replay. If the initiating peer's clock is skewed by more than the permitted window, the responder silently drops the initiation.

```bash
# Synchronise system time
sudo systemctl restart systemd-timesyncd    # Linux (systemd)
sudo sntp -sS time.apple.com               # macOS
```

**Cause — Key mismatch:** Verify that the `PublicKey` in each peer's `[Peer]` section matches the output of `rustguard pubkey <private-key>` run against the *other* peer's private key.

```bash
rustguard pubkey <base64-private-key>
```

### Pre-Shared Key (PSK) Mismatch

**Symptom:** Handshake fails when `PresharedKey` is set in `wg0.conf`.

**Cause:** The PSK value differs between the two peers, or one peer has a `PresharedKey` field and the other does not. The Noise_IKpsk2 handshake is not interoperable with the non-PSK variant when a PSK is present on only one side.

**Fix:** Ensure both peers either omit `PresharedKey` entirely (falls back gracefully to the non-PSK handshake) or carry the identical 32-byte base64-encoded value.

```ini
# Both peers must carry the same value
[Peer]
PublicKey    = <base64-peer-public-key>
PresharedKey = <base64-32-byte-psk>
AllowedIPs   = 10.0.0.2/32
```

### Nonce Exhaustion / Encrypt Returns None

**Symptom:** Long-running high-throughput tunnels silently stop encrypting after 2^60 messages on a single session.

**Cause:** By design, `encrypt()` returns `Option` and declines to encrypt once the nonce counter reaches 2^60. This triggers a rekey; the session expires and a new handshake is initiated automatically by the timer state machine.

**Action:** No operator intervention is required under normal operation. If rekeying appears to stall, confirm that UDP 51820 is still bidirectionally reachable (the new handshake must complete before traffic resumes).

---

## Zero-Config Enrollment

### `rustguard join` Is Rejected Immediately

**Symptom:** `rustguard join <server>:51820 --token mysecret` exits with an enrollment-closed or authentication error.

**Cause 1 — Enrollment window is closed.** The server starts with enrollment closed by default. An operator must run `rustguard open <seconds>` on the server before a new client can join.

```bash
# On the server — open enrollment for 60 seconds
rustguard open 60
```

**Cause 2 — Token mismatch.** The `--token` value is used to derive the XChaCha20 key that encrypts the enrollment key exchange. A mismatch causes authentication to fail silently. Confirm both sides use the identical token string.

### Enrollment Window Already Expired

**Symptom:** `rustguard join` was started after the open window elapsed.

**Fix:** Re-open the window on the server:

```bash
rustguard open 60
```

Then retry `rustguard join` within the window. See [Advanced Configuration — Enrollment Window Management](02-Advanced-Configuration.md#enrollment-window-management) for recommended window durations.

### IP Pool Exhausted

**Symptom:** Enrollment succeeds on the wire but the server logs indicate no addresses are available to assign.

**Cause:** The CIDR block passed to `--pool` has been fully allocated. For a `/24` pool, the server holds `.1` and up to 253 clients can be assigned; a 254th join attempt will fail.

**Fix:** Restart the server with a larger pool or a different CIDR:

```bash
rustguard serve --pool 172.20.0.0/16 --token mysecret
```

Existing peer assignments are stored in `~/.rustguard/state.json`. If stale entries are present from decommissioned peers, remove them from the state file before restarting to reclaim addresses.

---

## Runtime Management (`open` / `close` / `status`)

### Commands Hang or Return "Connection Refused"

**Symptom:** `rustguard open 60`, `rustguard close`, or `rustguard status` hang or fail immediately.

**Cause:** The `rustguard serve` daemon is not running, or its UNIX domain control socket is not accessible (wrong user, wrong working directory, or the socket file was not cleaned up after an unclean shutdown).

**Diagnostic:**

```bash
# Confirm the daemon is running
pgrep -a rustguard

# Check for a stale socket file (path is implementation-defined; check /tmp or ~/.rustguard/)
ls -la /tmp/rustguard*.sock 2>/dev/null || ls -la ~/.rustguard/*.sock 2>/dev/null
```

**Fix:** If the daemon crashed and left a stale socket file, remove it and restart the server:

```bash
rm -f /tmp/rustguard.sock        # adjust path as needed
rustguard serve --pool 10.150.0.0/24 --token mysecret
```

---

## State and Persistence

### Peers Lost After Server Restart

**Symptom:** After restarting `rustguard serve`, previously enrolled clients must re-enroll.

**Cause:** The state file `~/.rustguard/state.json` was deleted, is corrupted, or was written with insufficient permissions.

**Diagnostic:**

```bash
cat ~/.rustguard/state.json
```

A valid state file is a JSON document containing peer public keys and their assigned IP addresses. An empty file, missing file, or JSON parse error causes the server to start with no peers.

**Fix:** If the file is corrupted beyond repair, remove it and re-enroll all clients:

```bash
rm ~/.rustguard/state.json
rustguard serve --pool 10.150.0.0/24 --token mysecret
rustguard open 60    # then re-run 'rustguard join' on each client
```

To prevent data loss, back up `~/.rustguard/state.json` before upgrading or migrating the server.

### State File Not Written

**Symptom:** Server appears to run correctly but `~/.rustguard/state.json` does not appear or is always empty.

**Cause:** The `~/.rustguard/` directory does not exist or is not writable.

```bash
mkdir -p ~/.rustguard
chmod 700 ~/.rustguard
```

---

## Kernel Module (`rustguard-kmod`)

### Module Fails to Load

**Symptom:** `insmod` or `modprobe rustguard_kmod` fails with a kernel version mismatch or symbol error.

**Cause:** The `rustguard-kmod` crate targets Linux kernel **6.10+**. Loading the module on an older kernel is not supported.

```bash
# Verify running kernel version
uname -r
```

If the kernel is older than 6.10, use the userspace daemon (`rustguard up`) instead. The userspace path uses the standard Linux TUN device (`/dev/net/tun`) and does not require the kernel module.

### Module Loads but No Performance Gain

**Symptom:** The kernel module loads successfully but throughput is similar to the userspace daemon.

**Cause:** The kernel module eliminates the TUN device overhead but is otherwise subject to the same NIC and driver constraints. Confirm the module is actually handling traffic and that the userspace daemon is not also running on the same interface.

---

## Frequently Asked Questions

**Q: Does RustGuard interoperate with the official WireGuard tools?**

The `rustguard up wg0.conf` path uses the standard WireGuard wire format (Noise_IKpsk2, ChaCha20-Poly1305, BLAKE2s/HKDF, TAI64N) and is designed to be wire-compatible. The `serve`/`join` enrollment protocol is RustGuard-specific and has no equivalent in the reference implementation.

**Q: Why does the server reject my initiation packets under high load?**

RustGuard implements WireGuard's cookie mechanism: MAC1 is always required, and MAC2 (XChaCha20-encrypted cookie) is required when the server is under load. A client that does not handle the Cookie Reply message (type 3) will be unable to complete the handshake under load. This is expected protocol behavior, not a bug.

**Q: How do I cleanly shut down the tunnel?**

Send `SIGTERM` or `SIGINT` to the `rustguard up` process. The signal handler removes installed routes and closes the TUN file descriptor before exiting.

```bash
# Graceful shutdown
kill -TERM <rustguard-pid>
```

**Q: Can I run multiple tunnels simultaneously?**

Each `rustguard up` invocation manages one TUN interface. Multiple concurrent tunnels require separate config files, non-overlapping `Address` and `AllowedIPs` ranges, and separate UDP listen ports.

---

## See Also

- [Common Workflows](01-Common-Workflows.md) — step-by-step guides for tunnel setup and enrollment
- [Advanced Configuration](02-Advanced-Configuration.md) — IP pool sizing, PSK configuration, enrollment window tuning
- [Configuration](../01-Getting-Started/03-Configuration.md) — `wg.conf` field reference
- [Quick Start](../01-Getting-Started/04-Quick-Start.md) — end-to-end tutorial
- [System Overview](../02-Architecture/01-System-Overview.md) — crate responsibilities and component boundaries