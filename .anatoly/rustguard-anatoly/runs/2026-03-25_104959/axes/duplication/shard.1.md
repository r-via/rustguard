# Duplication — Shard 1

## Findings

| File | Verdict | Duplication | Conf. | Details |
|------|---------|-------------|-------|---------|
| `rustguard-tun/src/linux_mq.rs` | CRITICAL | 19 | 88% | [details](../reviews/rustguard-tun-src-linux_mq.rev.md) |
| `rustguard-enroll/src/server.rs` | CRITICAL | 2 | 92% | [details](../reviews/rustguard-enroll-src-server.rev.md) |
| `rustguard-kmod/src/noise.rs` | NEEDS_REFACTOR | 4 | 92% | [details](../reviews/rustguard-kmod-src-noise.rev.md) |
| `rustguard-daemon/src/tunnel.rs` | NEEDS_REFACTOR | 2 | 88% | [details](../reviews/rustguard-daemon-src-tunnel.rev.md) |
| `rustguard-tun/src/linux.rs` | NEEDS_REFACTOR | 4 | 90% | [details](../reviews/rustguard-tun-src-linux.rev.md) |
| `rustguard-kmod/src/cookie.rs` | NEEDS_REFACTOR | 4 | 80% | [details](../reviews/rustguard-kmod-src-cookie.rev.md) |
| `rustguard-enroll/src/client.rs` | NEEDS_REFACTOR | 2 | 88% | [details](../reviews/rustguard-enroll-src-client.rev.md) |

## Symbol Details

### `rustguard-tun/src/linux_mq.rs`

| Symbol | Lines | Duplication | Conf. | Detail |
|--------|-------|-------------|-------|--------|
| `TUNSETIFF` | L18–L18 | DUPLICATE | 65% | [USED] Non-exported constant used in ioctl call at line 100 for TUN device se... |
| `SIOCSIFADDR` | L19–L19 | DUPLICATE | 65% | [USED] Non-exported constant used in ioctl call at line 217 to set interface ... |
| `SIOCSIFDSTADDR` | L20–L20 | DUPLICATE | 65% | [USED] Non-exported constant used in ioctl call at line 222 to set destinatio... |
| `SIOCSIFNETMASK` | L21–L21 | DUPLICATE | 65% | [USED] Non-exported constant used in ioctl call at line 227 to set network ma... |
| `SIOCSIFMTU` | L22–L22 | DUPLICATE | 65% | [USED] Non-exported constant used in ioctl call at line 240 to set MTU | [DUP... |
| `SIOCSIFFLAGS` | L23–L23 | DUPLICATE | 65% | [USED] Non-exported constant used in ioctl call at line 254 to set interface ... |
| `SIOCGIFFLAGS` | L24–L24 | DUPLICATE | 65% | [USED] Non-exported constant used in ioctl call at line 250 to get interface ... |
| `IFF_TUN` | L26–L26 | DUPLICATE | 65% | [USED] Non-exported constant used in flag setup at lines 93 and 119 for TUN m... |
| `IFF_NO_PI` | L27–L27 | DUPLICATE | 65% | [USED] Non-exported constant used in flag setup at lines 93 and 119 to disabl... |
| `IFF_MULTI_QUEUE` | L28–L28 | DUPLICATE | 65% | [USED] Non-exported constant used in flag setup at lines 93 and 119 for multi... |
| `IFF_UP` | L30–L30 | DUPLICATE | 65% | [USED] Non-exported constant used at line 253 to bring up the interface | [DU... |
| `IFNAMSIZ` | L31–L31 | DUPLICATE | 65% | [USED] Non-exported constant used extensively for buffer sizing (lines 35, 39... |
| `IfreqFlags` | L34–L38 | DUPLICATE | 65% | [USED] Non-exported struct used for ioctl operations at lines 91, 117, and 24... |
| `IfreqAddr` | L41–L44 | DUPLICATE | 65% | [USED] Non-exported struct used at line 212 for address configuration via ioc... |
| `IfreqMtu` | L47–L51 | DUPLICATE | 65% | [USED] Non-exported struct used at line 231 for MTU configuration via ioctl |... |
| `set_name` | L53–L57 | DUPLICATE | 65% | [USED] Non-exported helper function used 5 times (lines 97, 122, 216, 234, 24... |
| `make_sockaddr_in` | L59–L68 | DUPLICATE | 65% | [USED] Non-exported helper function used 3 times (lines 214, 221, 226) to con... |
| `close_and_error` | L70–L74 | DUPLICATE | 65% | [USED] Non-exported helper function used 7 times (lines 101, 125, 218, 223, 2... |
| `configure_interface` | L206–L260 | DUPLICATE | 70% | [USED] Non-exported function called at line 136 to configure interface addres... |

### `rustguard-enroll/src/server.rs`

| Symbol | Lines | Duplication | Conf. | Detail |
|--------|-------|-------------|-------|--------|
| `rand_index` | L534–L538 | DUPLICATE | 85% | [USED] Non-exported function called directly at L272 during MSG_INITIATION ha... |
| `base64_key` | L541–L544 | DUPLICATE | 85% | [USED] Non-exported function called at L182 (enrollment logging with client_p... |

### `rustguard-kmod/src/noise.rs`

| Symbol | Lines | Duplication | Conf. | Detail |
|--------|-------|-------------|-------|--------|
| `constant_time_eq` | L53–L58 | DUPLICATE | 85% | [USED] Non-exported helper function called in process_response (line 377) to ... |
| `hash` | L72–L92 | DUPLICATE | 85% | [USED] Non-exported BLAKE2s hash function called in initial_chain_key (line 1... |
| `mac` | L95–L105 | DUPLICATE | 85% | [USED] Non-exported keyed BLAKE2s MAC function called in compute_mac1 (line 2... |
| `random_bytes` | L173–L177 | DUPLICATE | 75% | [DEAD] Non-exported generic function for random byte generation. Defined but ... |

### `rustguard-daemon/src/tunnel.rs`

| Symbol | Lines | Duplication | Conf. | Detail |
|--------|-------|-------------|-------|--------|
| `rand_index` | L492–L496 | DUPLICATE | 75% | [USED] Generates random u32 sender indices for handshakes; called at L173 and... |
| `base64_key` | L529–L532 | DUPLICATE | 60% | [USED] Encodes keys as base64 for logging; used at L107, L277, L313, L439 | [... |

### `rustguard-tun/src/linux.rs`

| Symbol | Lines | Duplication | Conf. | Detail |
|--------|-------|-------------|-------|--------|
| `close_and_error` | L55–L59 | DUPLICATE | 85% | [USED] Called at L105, L142, L149, L156, L171, L184 to clean up and propagate... |
| `make_sockaddr_in` | L61–L70 | DUPLICATE | 85% | [USED] Called at L139, L146, L153 to construct sockaddr_in structures from IP... |
| `set_name` | L72–L76 | DUPLICATE | 85% | [USED] Called at L97, L114, L135, L166, L174 to populate interface name buffe... |
| `configure_interface` | L124–L189 | DUPLICATE | 85% | [USED] Called at L117 from create() to configure interface addresses, netmask... |

### `rustguard-kmod/src/cookie.rs`

| Symbol | Lines | Duplication | Conf. | Detail |
|--------|-------|-------------|-------|--------|
| `hash` | L41–L51 | DUPLICATE | 75% | [USED] Helper function called in verify_mac1 (L119), create_reply (L131), and... |
| `mac` | L53–L57 | DUPLICATE | 75% | [USED] BLAKE2s MAC function called in make_cookie (L89), verify_mac1 (L120), ... |
| `random_bytes` | L59–L63 | DUPLICATE | 75% | [USED] RNG function called in new (L82), maybe_rotate_secret (L87), and creat... |
| `constant_time_eq` | L202–L205 | DUPLICATE | 80% | [USED] Constant-time equality check called in verify_mac1 (L122) and verify_m... |

### `rustguard-enroll/src/client.rs`

| Symbol | Lines | Duplication | Conf. | Detail |
|--------|-------|-------------|-------|--------|
| `rand_index` | L194–L198 | DUPLICATE | 75% | [USED] Non-exported internal function, called directly in run() at line 114 (... |
| `base64_key` | L200–L203 | DUPLICATE | 75% | [USED] Non-exported internal function, called directly in run() at line 67 (b... |

## Refactors

- [ ] <!-- ACT-58e5ff-4 --> **[duplication · high · small]** `rustguard-kmod/src/noise.rs`: Deduplicate: `constant_time_eq` duplicates `constant_time_eq` in `rustguard-kmod/src/cookie.rs` (`constant_time_eq`) [L53-L58]
- [ ] <!-- ACT-58e5ff-7 --> **[duplication · high · small]** `rustguard-kmod/src/noise.rs`: Deduplicate: `hash` duplicates `hash` in `rustguard-kmod/src/cookie.rs` (`hash`) [L72-L92]
- [ ] <!-- ACT-58e5ff-9 --> **[duplication · high · small]** `rustguard-kmod/src/noise.rs`: Deduplicate: `mac` duplicates `mac` in `rustguard-kmod/src/cookie.rs` (`mac`) [L95-L105]
- [ ] <!-- ACT-bd20cf-8 --> **[duplication · medium · small]** `rustguard-daemon/src/tunnel.rs`: Deduplicate: `rand_index` duplicates `rand_index` in `rustguard-enroll/src/server.rs` (`rand_index`) [L492-L496]
- [ ] <!-- ACT-bd20cf-12 --> **[duplication · medium · small]** `rustguard-daemon/src/tunnel.rs`: Deduplicate: `base64_key` duplicates `base64_key` in `rustguard-enroll/src/server.rs` (`base64_key`) [L529-L532]
- [ ] <!-- ACT-ca1d92-8 --> **[duplication · medium · small]** `rustguard-enroll/src/client.rs`: Deduplicate: `rand_index` duplicates `rand_index` in `rustguard-enroll/src/server.rs` (`rand_index`) [L194-L198]
- [ ] <!-- ACT-ca1d92-9 --> **[duplication · medium · small]** `rustguard-enroll/src/client.rs`: Deduplicate: `base64_key` duplicates `base64_key` in `rustguard-enroll/src/server.rs` (`base64_key`) [L200-L203]
- [ ] <!-- ACT-a732f2-6 --> **[duplication · medium · small]** `rustguard-enroll/src/server.rs`: Deduplicate: `rand_index` duplicates `rand_index` in `rustguard-enroll/src/client.rs` (`rand_index`) [L534-L538]
- [ ] <!-- ACT-a732f2-7 --> **[duplication · medium · small]** `rustguard-enroll/src/server.rs`: Deduplicate: `base64_key` duplicates `base64_key` in `rustguard-enroll/src/client.rs` (`base64_key`) [L541-L544]
- [ ] <!-- ACT-9105c7-2 --> **[duplication · medium · small]** `rustguard-kmod/src/cookie.rs`: Deduplicate: `hash` duplicates `hash` in `rustguard-kmod/src/noise.rs` (`hash`) [L41-L51]
- [ ] <!-- ACT-9105c7-3 --> **[duplication · medium · small]** `rustguard-kmod/src/cookie.rs`: Deduplicate: `mac` duplicates `mac` in `rustguard-kmod/src/noise.rs` (`mac`) [L53-L57]
- [ ] <!-- ACT-9105c7-4 --> **[duplication · medium · small]** `rustguard-kmod/src/cookie.rs`: Deduplicate: `random_bytes` duplicates `random_bytes` in `rustguard-kmod/src/noise.rs` (`random_bytes`) [L59-L63]
- [ ] <!-- ACT-9105c7-7 --> **[duplication · medium · small]** `rustguard-kmod/src/cookie.rs`: Deduplicate: `constant_time_eq` duplicates `constant_time_eq` in `rustguard-kmod/src/noise.rs` (`constant_time_eq`) [L202-L205]
- [ ] <!-- ACT-58e5ff-17 --> **[duplication · medium · small]** `rustguard-kmod/src/noise.rs`: Deduplicate: `random_bytes` duplicates `random_bytes` in `rustguard-kmod/src/cookie.rs` (`random_bytes`) [L173-L177]
- [ ] <!-- ACT-22946f-3 --> **[duplication · medium · small]** `rustguard-tun/src/linux_mq.rs`: Deduplicate: `TUNSETIFF` duplicates `TUNSETIFF` in `rustguard-tun/src/linux.rs` (`TUNSETIFF`) [L18-L18]
- [ ] <!-- ACT-22946f-4 --> **[duplication · medium · small]** `rustguard-tun/src/linux_mq.rs`: Deduplicate: `SIOCSIFADDR` duplicates `SIOCSIFADDR` in `rustguard-tun/src/linux.rs` (`SIOCSIFADDR`) [L19-L19]
- [ ] <!-- ACT-22946f-5 --> **[duplication · medium · small]** `rustguard-tun/src/linux_mq.rs`: Deduplicate: `SIOCSIFDSTADDR` duplicates `SIOCSIFDSTADDR` in `rustguard-tun/src/linux.rs` (`SIOCSIFDSTADDR`) [L20-L20]
- [ ] <!-- ACT-22946f-6 --> **[duplication · medium · small]** `rustguard-tun/src/linux_mq.rs`: Deduplicate: `SIOCSIFNETMASK` duplicates `SIOCSIFNETMASK` in `rustguard-tun/src/linux.rs` (`SIOCSIFNETMASK`) [L21-L21]
- [ ] <!-- ACT-22946f-7 --> **[duplication · medium · small]** `rustguard-tun/src/linux_mq.rs`: Deduplicate: `SIOCSIFMTU` duplicates `SIOCSIFMTU` in `rustguard-tun/src/linux.rs` (`SIOCSIFMTU`) [L22-L22]
- [ ] <!-- ACT-22946f-8 --> **[duplication · medium · small]** `rustguard-tun/src/linux_mq.rs`: Deduplicate: `SIOCSIFFLAGS` duplicates `SIOCSIFFLAGS` in `rustguard-tun/src/linux.rs` (`SIOCSIFFLAGS`) [L23-L23]
- [ ] <!-- ACT-22946f-9 --> **[duplication · medium · small]** `rustguard-tun/src/linux_mq.rs`: Deduplicate: `SIOCGIFFLAGS` duplicates `SIOCGIFFLAGS` in `rustguard-tun/src/linux.rs` (`SIOCGIFFLAGS`) [L24-L24]
- [ ] <!-- ACT-22946f-10 --> **[duplication · medium · small]** `rustguard-tun/src/linux_mq.rs`: Deduplicate: `IFF_TUN` duplicates `IFF_TUN` in `rustguard-tun/src/linux.rs` (`IFF_TUN`) [L26-L26]
- [ ] <!-- ACT-22946f-11 --> **[duplication · medium · small]** `rustguard-tun/src/linux_mq.rs`: Deduplicate: `IFF_NO_PI` duplicates `IFF_NO_PI` in `rustguard-tun/src/linux.rs` (`IFF_NO_PI`) [L27-L27]
- [ ] <!-- ACT-22946f-12 --> **[duplication · medium · small]** `rustguard-tun/src/linux_mq.rs`: Deduplicate: `IFF_MULTI_QUEUE` duplicates `IFF_MULTI_QUEUE` in `rustguard-tun/src/linux.rs` (`IFF_MULTI_QUEUE`) [L28-L28]
- [ ] <!-- ACT-22946f-13 --> **[duplication · medium · small]** `rustguard-tun/src/linux_mq.rs`: Deduplicate: `IFF_UP` duplicates `IFF_UP` in `rustguard-tun/src/linux.rs` (`IFF_UP`) [L30-L30]
- [ ] <!-- ACT-22946f-14 --> **[duplication · medium · small]** `rustguard-tun/src/linux_mq.rs`: Deduplicate: `IFNAMSIZ` duplicates `IFNAMSIZ` in `rustguard-tun/src/linux.rs` (`IFNAMSIZ`) [L31-L31]
- [ ] <!-- ACT-22946f-15 --> **[duplication · medium · small]** `rustguard-tun/src/linux_mq.rs`: Deduplicate: `IfreqFlags` duplicates `IfreqFlags` in `rustguard-tun/src/linux.rs` (`IfreqFlags`) [L34-L38]
- [ ] <!-- ACT-22946f-16 --> **[duplication · medium · small]** `rustguard-tun/src/linux_mq.rs`: Deduplicate: `IfreqAddr` duplicates `IfreqAddr` in `rustguard-tun/src/linux.rs` (`IfreqAddr`) [L41-L44]
- [ ] <!-- ACT-22946f-17 --> **[duplication · medium · small]** `rustguard-tun/src/linux_mq.rs`: Deduplicate: `IfreqMtu` duplicates `IfreqMtu` in `rustguard-tun/src/linux.rs` (`IfreqMtu`) [L47-L51]
- [ ] <!-- ACT-22946f-18 --> **[duplication · medium · small]** `rustguard-tun/src/linux_mq.rs`: Deduplicate: `set_name` duplicates `set_name` in `rustguard-tun/src/linux.rs` (`set_name`) [L53-L57]
- [ ] <!-- ACT-22946f-19 --> **[duplication · medium · small]** `rustguard-tun/src/linux_mq.rs`: Deduplicate: `make_sockaddr_in` duplicates `make_sockaddr_in` in `rustguard-tun/src/linux.rs` (`make_sockaddr_in`) [L59-L68]
- [ ] <!-- ACT-22946f-20 --> **[duplication · medium · small]** `rustguard-tun/src/linux_mq.rs`: Deduplicate: `close_and_error` duplicates `close_and_error` in `rustguard-tun/src/linux.rs` (`close_and_error`) [L70-L74]
- [ ] <!-- ACT-22946f-23 --> **[duplication · medium · small]** `rustguard-tun/src/linux_mq.rs`: Deduplicate: `configure_interface` duplicates `configure_interface` in `rustguard-tun/src/linux.rs` (`configure_interface`) [L206-L260]
- [ ] <!-- ACT-c62ee4-2 --> **[duplication · medium · small]** `rustguard-tun/src/linux.rs`: Deduplicate: `close_and_error` duplicates `close_and_error` in `rustguard-tun/src/linux_mq.rs` (`close_and_error`) [L55-L59]
- [ ] <!-- ACT-c62ee4-3 --> **[duplication · medium · small]** `rustguard-tun/src/linux.rs`: Deduplicate: `make_sockaddr_in` duplicates `make_sockaddr_in` in `rustguard-tun/src/linux_mq.rs` (`make_sockaddr_in`) [L61-L70]
- [ ] <!-- ACT-c62ee4-4 --> **[duplication · medium · small]** `rustguard-tun/src/linux.rs`: Deduplicate: `set_name` duplicates `set_name` in `rustguard-tun/src/linux_mq.rs` (`set_name`) [L72-L76]
- [ ] <!-- ACT-c62ee4-6 --> **[duplication · medium · small]** `rustguard-tun/src/linux.rs`: Deduplicate: `configure_interface` duplicates `configure_interface` in `rustguard-tun/src/linux_mq.rs` (`configure_interface`) [L124-L189]
