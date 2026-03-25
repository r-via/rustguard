# Correction — Shard 2

## Findings

| File | Verdict | Correction | Conf. | Details |
|------|---------|------------|-------|---------|
| `rustguard-cli/src/main.rs` | NEEDS_REFACTOR | 2 | 92% | [details](../reviews/rustguard-cli-src-main.rev.md) |
| `rustguard-core/src/timers.rs` | NEEDS_REFACTOR | 4 | 90% | [details](../reviews/rustguard-core-src-timers.rev.md) |
| `rustguard-enroll/src/control.rs` | NEEDS_REFACTOR | 1 | 90% | [details](../reviews/rustguard-enroll-src-control.rev.md) |
| `rustguard-crypto/src/aead.rs` | NEEDS_REFACTOR | 2 | 93% | [details](../reviews/rustguard-crypto-src-aead.rev.md) |
| `rustguard-daemon/src/config.rs` | NEEDS_REFACTOR | 1 | 90% | [details](../reviews/rustguard-daemon-src-config.rev.md) |
| `rustguard-tun/src/linux.rs` | NEEDS_REFACTOR | 2 | 90% | [details](../reviews/rustguard-tun-src-linux.rev.md) |
| `rustguard-tun/src/bpf_loader.rs` | NEEDS_REFACTOR | 3 | 88% | [details](../reviews/rustguard-tun-src-bpf_loader.rev.md) |
| `rustguard-tun/src/macos.rs` | NEEDS_REFACTOR | 4 | 90% | [details](../reviews/rustguard-tun-src-macos.rev.md) |
| `rustguard-kmod/src/cookie.rs` | NEEDS_REFACTOR | 1 | 80% | [details](../reviews/rustguard-kmod-src-cookie.rev.md) |
| `rustguard-enroll/src/client.rs` | NEEDS_REFACTOR | 2 | 88% | [details](../reviews/rustguard-enroll-src-client.rev.md) |

## Symbol Details

### `rustguard-cli/src/main.rs`

| Symbol | Lines | Correction | Conf. | Detail |
|--------|-------|------------|-------|--------|
| `cmd_serve` | L68–L152 | NEEDS_FIX | 92% | [USED] Called from main() at line 10 as command handler for 'serve' subcomman... |
| `cmd_join` | L154–L199 | NEEDS_FIX | 90% | [USED] Called from main() at line 11 as command handler for 'join' subcommand... |

### `rustguard-core/src/timers.rs`

| Symbol | Lines | Correction | Conf. | Detail |
|--------|-------|------------|-------|--------|
| `elapsed_since` | L21–L23 | NEEDS_FIX | 82% | [USED] no_std variant implementing the same interface as std version. Used id... |
| `elapsed_since` | L26–L30 | NEEDS_FIX | 82% | [USED] no_std variant implementing the same interface as std version. Used id... |
| `REKEY_TIMEOUT` | L39–L39 | NEEDS_FIX | 85% | [USED] Exported constant used locally in should_retry_handshake method (L150)... |
| `SessionTimers` | L57–L68 | NEEDS_FIX | 90% | [USED] Public struct exported from rustguard-core library crate. Library publ... |

### `rustguard-enroll/src/control.rs`

| Symbol | Lines | Correction | Conf. | Detail |
|--------|-------|------------|-------|--------|
| `open_window` | L44–L51 | NEEDS_FIX | 72% | [USED] Called internally by handle_client at L128. Implements OPEN protocol c... |

### `rustguard-crypto/src/aead.rs`

| Symbol | Lines | Correction | Conf. | Detail |
|--------|-------|------------|-------|--------|
| `MAX_PACKET_SIZE` | L10–L10 | NEEDS_FIX | 88% | [DEAD] Exported constant with 0 runtime/type-only importers. Public API const... |
| `open_to` | L83–L96 | NEEDS_FIX | 88% | [DEAD] Exported in-place decryption function with 0 runtime/type-only importe... |

### `rustguard-daemon/src/config.rs`

| Symbol | Lines | Correction | Conf. | Detail |
|--------|-------|------------|-------|--------|
| `parse_interface` | L176–L226 | NEEDS_FIX | 85% | [USED] Helper called by Config::parse on line 215 to parse interface configur... |

### `rustguard-tun/src/linux.rs`

| Symbol | Lines | Correction | Conf. | Detail |
|--------|-------|------------|-------|--------|
| `IfreqAddr` | L39–L42 | NEEDS_FIX | 85% | [USED] Instantiated at L138 and reused for address, destination, and netmask ... |
| `configure_interface` | L124–L189 | NEEDS_FIX | 85% | [USED] Called at L117 from create() to configure interface addresses, netmask... |

### `rustguard-tun/src/bpf_loader.rs`

| Symbol | Lines | Correction | Conf. | Detail |
|--------|-------|------------|-------|--------|
| `XdpProgram` | L37–L41 | NEEDS_FIX | 88% | [DEAD] Exported pub struct with 0 runtime importers per pre-computed analysis... |
| `bpf_prog_load` | L124–L175 | NEEDS_FIX | 75% | [USED] Called in load_and_attach at L70 to load BPF bytecode into kernel | [U... |
| `attach_xdp_netlink` | L235–L342 | NEEDS_FIX | 80% | [USED] Called in attach_xdp at L231 and in detach_xdp at L345; core netlink-b... |

### `rustguard-tun/src/macos.rs`

| Symbol | Lines | Correction | Conf. | Detail |
|--------|-------|------------|-------|--------|
| `IfreqMtu` | L65–L68 | NEEDS_FIX | 85% | [USED] Instantiated in set_mtu for interface MTU setting | [UNIQUE] Platform-... |
| `create` | L95–L192 | NEEDS_FIX | 90% | [DEAD] Exported with 0 runtime importers in crate analysis | [UNIQUE] macOS u... |
| `configure_address` | L194–L218 | NEEDS_FIX | 88% | [USED] Called from create to configure interface IP address | [UNIQUE] macOS ... |
| `set_mtu` | L220–L242 | NEEDS_FIX | 88% | [USED] Called from create to set interface MTU size | [UNIQUE] Similarity sco... |

### `rustguard-kmod/src/cookie.rs`

| Symbol | Lines | Correction | Conf. | Detail |
|--------|-------|------------|-------|--------|
| `hash` | L41–L51 | NEEDS_FIX | 75% | [USED] Helper function called in verify_mac1 (L119), create_reply (L131), and... |

### `rustguard-enroll/src/client.rs`

| Symbol | Lines | Correction | Conf. | Detail |
|--------|-------|------------|-------|--------|
| `run` | L28–L192 | NEEDS_FIX | 85% | [DEAD] Exported function with 0 imports per pre-computed analysis (Rule 2 → D... |
| `add_route` | L205–L226 | NEEDS_FIX | 72% | [USED] Non-exported internal function, called directly in run() at line 91 (a... |

## Quick Wins

- [x] <!-- ACT-8bab2a-1 --> **[correction · high · small]** `rustguard-cli/src/main.rs`: In cmd_serve, replace `args.get(i).cloned().unwrap_or_default()` for --token (line 86) with an explicit check: if args.get(i) is None or the value starts with '-', print a usage error and exit. An empty token disables authentication on the enrollment server. [L86]
- [x] <!-- ACT-fb2f2d-1 --> **[correction · high · small]** `rustguard-core/src/timers.rs`: The no_std elapsed_since always returns Duration::ZERO, rendering all time-based SessionTimers query methods (needs_rekey, is_expired, is_dead, needs_keepalive, should_retry_handshake, handshake_timed_out) non-functional. Add no_std-aware _at(now_ns: u64) variants for each of these query methods that compute the duration inline from stored u64 timestamp fields, mirroring the _at pattern already used for session_started_at, packet_sent_at, and packet_received_at. [L29]
- [x] <!-- ACT-efa3ec-2 --> **[correction · high · small]** `rustguard-crypto/src/aead.rs`: open_to must guard against ct_len > buf.len() before indexing buf[..ct_len]. Add `if ct_len > buf.len() { return None; }` immediately after the AEAD_TAG_LEN check to prevent a panic when adversarial or malformed inputs arrive with an inflated ct_len. [L86]
- [x] <!-- ACT-8bab2a-2 --> **[correction · medium · small]** `rustguard-cli/src/main.rs`: In cmd_join, replace `args.get(i).cloned().unwrap_or_default()` for --token (line 163) with an explicit missing-value check; an empty token string is forwarded to the client and bypasses token validation. [L163]
- [x] <!-- ACT-fb2f2d-2 --> **[correction · medium · small]** `rustguard-core/src/timers.rs`: needs_keepalive incorrectly falls back to last_send_time = received when last_sent is None, creating contradictory guard conditions. Fix by handling the None case explicitly: if last_sent is None and elapsed_since(received) >= interval, return true; otherwise when last_sent is Some, apply the dual-condition check (elapsed_since(sent) >= interval && elapsed_since(received) < interval). [L163]
- [x] <!-- ACT-efa3ec-1 --> **[correction · medium · small]** `rustguard-crypto/src/aead.rs`: MAX_PACKET_SIZE uses 1500 in its expression but the doc comment documents the limit as 'MTU 1420 + tag'. Change the constant to 1420 + AEAD_TAG_LEN (= 1436) to match the stated WireGuard transport MTU, or update the comment to reflect that 1500 is intentional (e.g. Ethernet MTU before WireGuard header subtraction). As-is, callers using this constant for size validation will accept oversized packets. [L10]
- [x] <!-- ACT-ca1d92-1 --> **[correction · medium · small]** `rustguard-enroll/src/client.rs`: Introduce a shutdown mechanism so `running` can be set to `false`: install a Ctrl+C / SIGTERM handler (e.g. via the `ctrlc` crate) that calls `running.store(false, Ordering::SeqCst)`, or propagate errors from the worker threads and store `false` on exit. Without this, both `join()` calls block forever and the function never returns. [L89]
- [x] <!-- ACT-ca1d92-3 --> **[correction · medium · small]** `rustguard-enroll/src/client.rs`: Add a `#[cfg(not(any(target_os = "macos", target_os = "linux")))]` arm in `add_route` that either emits `compile_error!("add_route: unsupported OS")` or a runtime no-op, to prevent 'cannot find value `result`' compilation failure on other targets. [L218]
- [x] <!-- ACT-9105c7-1 --> **[correction · medium · small]** `rustguard-kmod/src/cookie.rs`: In hash(), change chunks.len() as u32 to chunks.len().min(4) as u32 when passing num_chunks to wg_blake2s_hash. The ptrs/lens arrays have a fixed capacity of 4; passing a larger value causes the C function to read beyond the stack-allocated arrays, which is undefined behaviour. [L49]
- [x] <!-- ACT-62f2c7-1 --> **[correction · medium · small]** `rustguard-tun/src/bpf_loader.rs`: Fix file descriptor leak in load_and_attach: xsks_map_fd is not closed when parse_and_patch_elf or bpf_prog_load fails via ?. Both xsks_map_fd and prog_fd are not closed when attach_xdp fails. Use an explicit close guard (e.g., scopeguard crate, or a local struct with Drop) so that partially acquired FDs are always released on error paths. [L48]
- [x] <!-- ACT-b59607-1 --> **[correction · medium · small]** `rustguard-tun/src/macos.rs`: Pad IfreqMtu to 32 bytes by adding `_pad: [u8; 12]` (or `_pad: [u8; 12], initialised to zero`) so the kernel's copyin for SIOCSIFMTU reads only allocated memory instead of uninitialized stack bytes. [L65]
- [x] <!-- ACT-b59607-2 --> **[correction · medium · small]** `rustguard-tun/src/macos.rs`: In configure_address, capture errno before calling close(): save `let err = io::Error::last_os_error();` immediately after the ioctl call, then close the socket, then return `Err(err)` if ret < 0. Mirrors the close_and_error helper already present in this file. [L210]
- [x] <!-- ACT-b59607-3 --> **[correction · medium · small]** `rustguard-tun/src/macos.rs`: In set_mtu, apply the same errno-before-close fix: capture last_os_error() before libc::close(sock), then return that captured error if ret < 0. [L234]
- [x] <!-- ACT-8bab2a-3 --> **[correction · low · small]** `rustguard-cli/src/main.rs`: In cmd_serve, replace `args.get(i).cloned().unwrap_or_default()` for --pool (line 82) with an explicit missing-value check to produce a correct usage error instead of a misleading 'bad pool address' parse failure downstream. [L82]
- [x] <!-- ACT-8bab2a-4 --> **[correction · low · small]** `rustguard-cli/src/main.rs`: In cmd_serve, replace `args.get(i).cloned().unwrap_or_default()` for --xdp (line 93) with an explicit missing-value check to prevent an empty interface name from being silently passed to ServeConfig. [L93]
- [x] <!-- ACT-fb2f2d-3 --> **[correction · low · small]** `rustguard-core/src/timers.rs`: The doc comment on REKEY_TIMEOUT incorrectly describes it as a keypair-age send threshold ('REJECT_AFTER_TIME + padding') rather than a between-attempt handshake retry interval. Correct the doc comment to match the actual usage in should_retry_handshake to prevent API misuse. [L39]
- [x] <!-- ACT-dbf131-1 --> **[correction · low · small]** `rustguard-daemon/src/config.rs`: Replace the hardcoded default prefix '24' with a type-aware default: parse the address first, then choose '32' for IPv4 and '128' for IPv6 when no prefix notation is present—mirroring the logic already used in parse_cidr. [L196]
- [x] <!-- ACT-ca1d92-2 --> **[correction · low · small]** `rustguard-enroll/src/client.rs`: Release the session Mutex guard before calling `udp_out.send_to()` in the outbound thread. Holding the lock across the send syscall blocks the inbound thread from acquiring the session lock for decryption during that window. Shadow the encrypted payload into a local variable, drop the guard, then send. [L148]
- [x] <!-- ACT-18d76d-1 --> **[correction · low · small]** `rustguard-enroll/src/control.rs`: In open_window, clamp or validate duration_secs before casting to i64. Values above i64::MAX wrap to negative when cast, producing a deadline in the past (enrollment silently never opens). Values in the range (i64::MAX - current_epoch_secs, i64::MAX] cause an i64 overflow that panics in debug builds and wraps to an incorrect deadline in release builds. Fix: use duration_secs.min(i64::MAX as u64) before casting, or perform checked/saturating arithmetic. [L49]
- [x] <!-- ACT-62f2c7-2 --> **[correction · low · small]** `rustguard-tun/src/bpf_loader.rs`: Fix NLMSG_ERROR bounds check in attach_xdp_netlink: change 'if n >= 16' to 'if n >= 20' before reading resp[16..19]. The nlmsgerr struct requires 20 bytes (nlmsghdr=16 + error=4); reading with only n>=16 causes a silent false-success when a short NLMSG_ERROR is received. [L322]
- [x] <!-- ACT-62f2c7-3 --> **[correction · low · small]** `rustguard-tun/src/bpf_loader.rs`: Fix uninitialized trailing padding in BpfAttrProgLoad: the #[repr(C)] struct is 256 bytes (252 of named fields + 4 bytes compiler trailing padding). The trailing 4 bytes are not covered by _pad and may be uninitialized stack bytes when passed to the bpf() syscall. Zero-initialize the entire struct with MaybeUninit::zeroed() before filling fields, or extend _pad to [u8; 212] so std::mem::size_of equals exactly 256 with no implicit trailing padding. [L138]
- [x] <!-- ACT-c62ee4-1 --> **[correction · low · small]** `rustguard-tun/src/linux.rs`: Add an 8-byte trailing padding field to IfreqAddr so its size equals sizeof(struct ifreq) = 40 bytes: add `_pad: [u8; 8]` after ifr_addr. Without it the kernel's copy_from_user reads 8 bytes beyond the struct allocation for every SIOCSIFADDR/SIOCSIFDSTADDR/SIOCSIFNETMASK ioctl call. [L39]
- [x] <!-- ACT-b59607-4 --> **[correction · low · small]** `rustguard-tun/src/macos.rs`: In create, replace `num + 1` with `num.checked_add(1).ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "utun unit number too large"))?` to prevent silent wrap-to-zero or debug panic for extreme utun indices. [L131]
