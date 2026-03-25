# Review: `rustguard-tun/src/macos.rs`

**Verdict:** NEEDS_REFACTOR

## Symbols

| Symbol | Kind | Exported | Correction | Over-eng. | Utility | Duplication | Tests | Confidence |
|--------|------|----------|------------|-----------|---------|-------------|-------|------------|
| AF_SYSTEM | constant | no | OK | LEAN | USED | UNIQUE | GOOD | 88% |
| AF_SYS_CONTROL | constant | no | OK | LEAN | USED | UNIQUE | GOOD | 88% |
| SYSPROTO_CONTROL | constant | no | OK | LEAN | USED | UNIQUE | GOOD | 88% |
| UTUN_OPT_IFNAME | constant | no | OK | LEAN | USED | UNIQUE | GOOD | 88% |
| UTUN_CONTROL_NAME | constant | no | OK | LEAN | USED | UNIQUE | GOOD | 88% |
| CTLIOCGINFO | constant | no | OK | LEAN | USED | UNIQUE | GOOD | 88% |
| SIOCSIFMTU | constant | no | OK | LEAN | USED | UNIQUE | GOOD | 88% |
| SIOCAIFADDR | constant | no | OK | LEAN | USED | UNIQUE | GOOD | 88% |
| AF_INET | constant | no | OK | LEAN | USED | UNIQUE | GOOD | 88% |
| AF_INET6 | constant | no | OK | LEAN | USED | UNIQUE | GOOD | 88% |
| CtlInfo | class | no | OK | LEAN | USED | UNIQUE | GOOD | 88% |
| SockaddrCtl | class | no | OK | LEAN | USED | UNIQUE | GOOD | 88% |
| SockaddrIn | class | no | OK | LEAN | USED | UNIQUE | GOOD | 88% |
| IfAliasReq | class | no | OK | LEAN | USED | UNIQUE | GOOD | 88% |
| IfreqMtu | class | no | NEEDS_FIX | LEAN | USED | UNIQUE | GOOD | 85% |
| last_os_error | function | no | OK | LEAN | USED | UNIQUE | GOOD | 88% |
| close_and_error | function | no | OK | LEAN | USED | UNIQUE | NONE | 88% |
| make_sockaddr_in | function | no | OK | LEAN | USED | UNIQUE | NONE | 85% |
| create | function | yes | NEEDS_FIX | LEAN | USED | UNIQUE | NONE | 90% |
| configure_address | function | no | NEEDS_FIX | LEAN | USED | UNIQUE | NONE | 88% |
| set_mtu | function | no | NEEDS_FIX | LEAN | USED | UNIQUE | NONE | 88% |
| read | function | yes | OK | LEAN | USED | UNIQUE | NONE | 90% |
| write | function | yes | OK | LEAN | USED | UNIQUE | NONE | 90% |

### Details

#### `AF_SYSTEM` (L13–L13)

- **Utility [USED]**: Used in socket creation and SockaddrCtl initialization
- **Duplication [UNIQUE]**: Platform-specific Darwin constant, no similar symbols found in RAG
- **Correction [OK]**: Darwin AF_SYSTEM is 32; value matches Darwin headers.
- **Overengineering [LEAN]**: Darwin-specific constant absent from libc; required for kernel control socket creation.
- **Tests [GOOD]**: Compile-time constant with no runtime behavior. Per rule 6, constants are GOOD by default. No test file exists but none is required for a plain numeric constant.
- **UNDOCUMENTED [UNDOCUMENTED]**: Private constant with no `///` doc comment. Name is self-descriptive (Darwin AF constant), tolerated under private-item leniency. (deliberated: reclassified: documentation: UNDOCUMENTED → DOCUMENTED — Private constant whose name is an exact Darwin header identifier (AF_SYSTEM=32). Self-descriptive for any systems programmer; `///` doc would be redundant. Reclassified under private-item leniency.)

#### `AF_SYS_CONTROL` (L14–L14)

- **Utility [USED]**: Used in SockaddrCtl.ss_sysaddr field initialization
- **Duplication [UNIQUE]**: Platform-specific Darwin constant, no similar symbols found in RAG
- **Correction [OK]**: Darwin AF_SYS_CONTROL is 2; value matches Darwin headers.
- **Overengineering [LEAN]**: Darwin-specific sysaddr constant not provided by libc; necessary for SockaddrCtl.
- **Tests [GOOD]**: Compile-time constant with no runtime behavior. Per rule 6, constants are GOOD by default.
- **UNDOCUMENTED [UNDOCUMENTED]**: Private constant with no `///` doc comment. Tolerated under private-item leniency; name reflects Darwin origin. (deliberated: reclassified: documentation: UNDOCUMENTED → DOCUMENTED — Private constant matching Darwin AF_SYS_CONTROL. Name is self-documenting for the target audience. Reclassified under private-item leniency.)

#### `SYSPROTO_CONTROL` (L15–L15)

- **Utility [USED]**: Used in socket creation for kernel control socket
- **Duplication [UNIQUE]**: Platform-specific Darwin constant, no similar symbols found in RAG
- **Correction [OK]**: SYSPROTO_CONTROL = 2 matches Darwin kernel headers.
- **Overengineering [LEAN]**: Darwin kernel control protocol constant; not in libc, required for socket() call.
- **Tests [GOOD]**: Compile-time constant with no runtime behavior. Per rule 6, constants are GOOD by default.
- **UNDOCUMENTED [UNDOCUMENTED]**: Private constant with no `///` doc comment. Well-known Darwin symbol; private-item leniency applies. (deliberated: reclassified: documentation: UNDOCUMENTED → DOCUMENTED — Private constant matching Darwin SYSPROTO_CONTROL. Standard kernel constant name serves as documentation. Reclassified under private-item leniency.)

#### `UTUN_OPT_IFNAME` (L16–L16)

- **Utility [USED]**: Used in getsockopt call to retrieve interface name
- **Duplication [UNIQUE]**: Platform-specific Darwin constant, no similar symbols found in RAG
- **Correction [OK]**: UTUN_OPT_IFNAME = 2 matches Darwin utun setsockopt option.
- **Overengineering [LEAN]**: Platform-specific getsockopt option; needed to retrieve the assigned utun interface name.
- **Tests [GOOD]**: Compile-time constant with no runtime behavior. Per rule 6, constants are GOOD by default.
- **UNDOCUMENTED [UNDOCUMENTED]**: Private constant with no `///` doc comment. Name communicates purpose adequately; private-item leniency applies. (deliberated: reclassified: documentation: UNDOCUMENTED → DOCUMENTED — Private constant matching Darwin UTUN_OPT_IFNAME. Name communicates purpose clearly. Reclassified under private-item leniency.)

#### `UTUN_CONTROL_NAME` (L17–L17)

- **Utility [USED]**: Used to identify utun kernel control in ioctl
- **Duplication [UNIQUE]**: Platform-specific Darwin control name constant, no similar symbols found in RAG
- **Correction [OK]**: Correct null-terminated control name for macOS utun; 27 bytes fits within CtlInfo.ctl_name[96].
- **Overengineering [LEAN]**: Kernel control name string required for CTLIOCGINFO lookup; single-purpose and correctly null-terminated.
- **Tests [GOOD]**: Compile-time constant (byte-string literal) with no runtime behavior. Per rule 6, constants are GOOD by default.
- **UNDOCUMENTED [UNDOCUMENTED]**: Private constant with no `///` doc comment. The byte-string value is self-documenting; private-item leniency applies. (deliberated: reclassified: documentation: UNDOCUMENTED → DOCUMENTED — Private constant containing the literal Darwin kernel control name string. The byte literal is self-documenting. Reclassified under private-item leniency.)

#### `CTLIOCGINFO` (L20–L20)

- **Utility [USED]**: Used in ioctl call to get utun control ID
- **Duplication [UNIQUE]**: Platform-specific Darwin ioctl constant, no similar symbols found in RAG
- **Correction [OK]**: _IOWR('N', 3, struct ctl_info) where ctl_info = 100 bytes yields 0xC0000000 | (0x64<<16) | (0x4E<<8) | 3 = 0xC0644E03. Matches.
- **Overengineering [LEAN]**: Darwin ioctl request constant derived from _IOWR macro; not in libc and required for retrieving the utun control ID.
- **Tests [GOOD]**: Compile-time ioctl constant with no runtime behavior. Per rule 6, constants are GOOD by default.
- **UNDOCUMENTED [UNDOCUMENTED]**: Private constant with no `///` doc comment. Has an inline `//` comment showing the macro expansion, but `//` is not a Rust doc comment. Tolerated under private-item leniency. (deliberated: reclassified: documentation: UNDOCUMENTED → DOCUMENTED — Private ioctl constant with an inline // comment showing the _IOWR macro expansion. Name matches Darwin header exactly. Reclassified under private-item leniency; the inline comment provides sufficient context.)

#### `SIOCSIFMTU` (L23–L23)

- **Utility [USED]**: Used in ioctl call to set network interface MTU
- **Duplication [UNIQUE]**: Platform-specific Darwin ioctl constant, no similar symbols found in RAG
- **Correction [OK]**: _IOW('i', 52, struct ifreq) = 0x80000000|(32<<16)|(0x69<<8)|52 = 0x80206934. Constant is correct; the struct-size mismatch is tracked separately on IfreqMtu.
- **Overengineering [LEAN]**: Standard BSD ioctl for setting interface MTU; libc does not expose it as a typed constant for macOS.
- **Tests [GOOD]**: Compile-time ioctl constant with no runtime behavior. Per rule 6, constants are GOOD by default.
- **UNDOCUMENTED [UNDOCUMENTED]**: Private constant with no `///` doc comment. Section header comment above it uses `//`, not `///`. Private-item leniency applies. (deliberated: reclassified: documentation: UNDOCUMENTED → DOCUMENTED — Standard BSD ioctl name. Section header comment above provides context. Reclassified under private-item leniency.)

#### `SIOCAIFADDR` (L24–L24)

- **Utility [USED]**: Used in ioctl call to configure interface address
- **Duplication [UNIQUE]**: Platform-specific Darwin ioctl constant, no similar symbols found in RAG
- **Correction [OK]**: _IOW('i', 26, struct ifaliasreq) = 0x80000000|(64<<16)|(0x69<<8)|26 = 0x8040691A. IfAliasReq is 64 bytes (16+16+16+16). Matches.
- **Overengineering [LEAN]**: BSD ioctl for adding an interface address alias; not exposed by libc for Darwin, necessary for configure_address.
- **Tests [GOOD]**: Compile-time ioctl constant with no runtime behavior. Per rule 6, constants are GOOD by default.
- **UNDOCUMENTED [UNDOCUMENTED]**: Private constant with no `///` doc comment. Private-item leniency applies; name is a standard Darwin ioctl identifier. (deliberated: reclassified: documentation: UNDOCUMENTED → DOCUMENTED — Standard BSD ioctl name for adding interface address. Self-descriptive. Reclassified under private-item leniency.)

#### `AF_INET` (L26–L26)

- **Utility [USED]**: Used in SockaddrIn and IPv4 packet version matching
- **Duplication [UNIQUE]**: Standard socket family constant, no similar symbols found in RAG
- **Correction [OK]**: AF_INET = 2 is correct for macOS/Darwin.
- **Overengineering [LEAN]**: Redefined as u8 (not libc's c_int) so it fits directly into struct fields and the AF header byte; justified by Darwin struct layout requirements.
- **Tests [GOOD]**: Compile-time constant with no runtime behavior. Per rule 6, constants are GOOD by default.
- **UNDOCUMENTED [UNDOCUMENTED]**: Private constant shadowing libc::AF_INET with no `///` doc comment. Name is universally understood; private-item leniency applies. (deliberated: reclassified: documentation: UNDOCUMENTED → DOCUMENTED — Universally known socket constant. Private u8 variant justified by Darwin struct layout needs. No doc comment needed.)

#### `AF_INET6` (L27–L27)

- **Utility [USED]**: Used for IPv6 address family in packet write
- **Duplication [UNIQUE]**: Standard socket family constant, no similar symbols found in RAG
- **Correction [OK]**: AF_INET6 = 30 is correct for macOS/Darwin (differs from Linux's 10).
- **Overengineering [LEAN]**: Same rationale as AF_INET — u8 variant needed for the utun write AF header byte.
- **Tests [GOOD]**: Compile-time constant with no runtime behavior. Per rule 6, constants are GOOD by default.
- **UNDOCUMENTED [UNDOCUMENTED]**: Private constant shadowing libc::AF_INET6 with no `///` doc comment. Name is universally understood; private-item leniency applies. (deliberated: reclassified: documentation: UNDOCUMENTED → DOCUMENTED — Universally known socket constant. Same reasoning as AF_INET. Reclassified under private-item leniency.)

#### `CtlInfo` (L32–L35)

- **Utility [USED]**: Instantiated to query utun control ID from kernel
- **Duplication [UNIQUE]**: Platform-specific C-compatible struct matching Darwin kernel layout, no similar symbols found in RAG
- **Correction [OK]**: Matches Darwin struct ctl_info: u32 ctl_id (4) + char ctl_name[96] = 100 bytes total, consistent with CTLIOCGINFO size encoding.
- **Overengineering [LEAN]**: Exact repr(C) mirror of Darwin's ctl_info struct required by CTLIOCGINFO ioctl; no alternative in libc.
- **Tests [GOOD]**: Plain #[repr(C)] struct with no methods — pure data layout for Darwin FFI. Per rule 6, types with no runtime behavior are GOOD by default.
- **UNDOCUMENTED [UNDOCUMENTED]**: Private `#[repr(C)]` struct with no `///` doc comment on the type or its fields. Tolerated under private-item leniency; the section comment uses `//` not `///`. (deliberated: reclassified: documentation: UNDOCUMENTED → DOCUMENTED — Private repr(C) struct that exactly mirrors Darwin's struct ctl_info. Field names match kernel headers. Section comment above provides context. Reclassified under private-item leniency.)

#### `SockaddrCtl` (L38–L45)

- **Utility [USED]**: Instantiated to connect to kernel control socket
- **Duplication [UNIQUE]**: Platform-specific C-compatible struct for Darwin socket control, no similar symbols found in RAG
- **Correction [OK]**: Matches Darwin struct sockaddr_ctl exactly: 1+1+2+4+4+20 = 32 bytes. sc_len is set to 32 at usage site, consistent.
- **Overengineering [LEAN]**: repr(C) mirror of Darwin sockaddr_ctl; not in libc, mandatory for connecting to the kernel control interface.
- **Tests [GOOD]**: Plain #[repr(C)] struct with no methods — pure data layout for Darwin kernel control socket address. Per rule 6, types with no runtime behavior are GOOD by default.
- **UNDOCUMENTED [UNDOCUMENTED]**: Private `#[repr(C)]` struct with no `///` doc comment on type or fields. Layout mirrors Darwin `sockaddr_ctl`; tolerated under private-item leniency. (deliberated: reclassified: documentation: UNDOCUMENTED → DOCUMENTED — Private repr(C) struct mirroring Darwin sockaddr_ctl. Field names are self-documenting and match kernel headers exactly. Reclassified under private-item leniency.)

#### `SockaddrIn` (L48–L54)

- **Utility [USED]**: Used in IfAliasReq fields and make_sockaddr_in
- **Duplication [UNIQUE]**: Platform-specific C-compatible struct for Darwin sockaddr_in, no similar symbols found in RAG
- **Correction [OK]**: Matches Darwin struct sockaddr_in: sin_len(1)+sin_family(1)+sin_port(2)+sin_addr(4)+sin_zero(8) = 16 bytes.
- **Overengineering [LEAN]**: Custom flat layout with u8 arrays for sin_addr/sin_zero allows direct copy_from_slice from Ipv4Addr octets; libc::sockaddr_in wraps sin_addr in in_addr and uses i8 for sin_zero, making direct population awkward.
- **Tests [GOOD]**: Plain #[repr(C)] struct with no methods — mirrors Darwin's sockaddr_in for FFI. Per rule 6, types with no runtime behavior are GOOD by default.
- **UNDOCUMENTED [UNDOCUMENTED]**: Private `#[repr(C)]` struct with no `///` doc comment. Mirrors Darwin `sockaddr_in`; private-item leniency applies. (deliberated: reclassified: documentation: UNDOCUMENTED → DOCUMENTED — Private repr(C) struct mirroring Darwin sockaddr_in. Field names match BSD socket conventions universally. Reclassified under private-item leniency.)

#### `IfAliasReq` (L57–L62)

- **Utility [USED]**: Instantiated in configure_address for IP configuration
- **Duplication [UNIQUE]**: Platform-specific Darwin kernel struct for address configuration, no similar symbols found in RAG
- **Correction [OK]**: Matches Darwin struct ifaliasreq: name(16)+addr(16)+broadaddr(16)+mask(16) = 64 bytes. SIOCAIFADDR encodes size 64. Correct.
- **Overengineering [LEAN]**: repr(C) mirror of Darwin ifaliasreq; necessary for SIOCAIFADDR and not available in libc.
- **Tests [GOOD]**: Plain #[repr(C)] struct with no methods — mirrors Darwin's ifaliasreq for SIOCAIFADDR ioctl. Per rule 6, types with no runtime behavior are GOOD by default.
- **UNDOCUMENTED [UNDOCUMENTED]**: Private `#[repr(C)]` struct with no `///` doc comment. Mirrors Darwin `ifaliasreq`; private-item leniency applies. (deliberated: reclassified: documentation: UNDOCUMENTED → DOCUMENTED — Private repr(C) struct mirroring Darwin ifaliasreq. Field names match kernel headers. Reclassified under private-item leniency.)

#### `IfreqMtu` (L65–L68)

- **Utility [USED]**: Instantiated in set_mtu for interface MTU setting
- **Duplication [UNIQUE]**: Platform-specific C-compatible struct for MTU configuration, no similar symbols found in RAG
- **Correction [NEEDS_FIX]**: Struct is 20 bytes (name[16] + i32) but SIOCSIFMTU = _IOW('i',52,struct ifreq) encodes size=32. The kernel's copyin reads 32 bytes from the user pointer; the final 12 bytes lie past the end of the allocated struct, reading uninitialized stack memory. This is undefined behaviour. The fix is to pad the struct to 32 bytes with a `_pad: [u8; 12]` field.
- **Overengineering [LEAN]**: Minimal repr(C) ifreq variant carrying only the MTU field; appropriate scoping for a single-purpose ioctl.
- **Tests [GOOD]**: Plain #[repr(C)] struct with no methods — mirrors Darwin's ifreq for SIOCSIFMTU ioctl. Per rule 6, types with no runtime behavior are GOOD by default.
- **UNDOCUMENTED [UNDOCUMENTED]**: Private `#[repr(C)]` struct with no `///` doc comment. Mirrors Darwin `ifreq` (MTU variant); private-item leniency applies. (deliberated: reclassified: documentation: UNDOCUMENTED → DOCUMENTED — Correction NEEDS_FIX is valid: struct is 20 bytes but SIOCSIFMTU encodes size 32, causing the kernel's copyin to read 12 bytes past the struct into uninitialized stack memory. This is real UB that should be fixed with padding. Documentation reclassified: private repr(C) struct with field names matching Darwin ifreq; private-item leniency applies.)

#### `last_os_error` (L72–L74)

- **Utility [USED]**: Called in create, configure_address, set_mtu, read, write
- **Duplication [UNIQUE]**: Trivial 2-line wrapper around io::Error::last_os_error(), marked unique by RAG
- **Correction [OK]**: Thin wrapper over io::Error::last_os_error(); no logic errors.
- **Overengineering [LEAN]**: One-line alias for io::Error::last_os_error(); common Rust idiom in dense unsafe syscall code to reduce line noise. Used at least three call sites.
- **Tests [NONE]**: No test file found for macos.rs. This private helper is a one-line wrapper around io::Error::last_os_error(); no dedicated tests exist anywhere in the project for it.
- **UNDOCUMENTED [UNDOCUMENTED]**: Private one-liner helper with no `///` doc comment. Name and body are entirely self-explanatory; private-item leniency applies. (deliberated: reclassified: tests: NONE → GOOD, documentation: UNDOCUMENTED → DOCUMENTED — Tests reclassified NONE→GOOD: this is a single-line delegation to io::Error::last_os_error() with zero logic to test. Testing it would only test the stdlib. Documentation reclassified: private one-liner whose name and body are entirely self-explanatory; no doc comment needed.)

#### `close_and_error` (L79–L83)

- **Utility [USED]**: Called in create to close fd while preserving errno
- **Duplication [DUPLICATE]**: Identical implementation (0.977 similarity) with linux.rs; both capture error before closing fd
- **Correction [OK]**: Correctly captures errno before calling close() to prevent clobbering; pattern is sound.
- **Overengineering [LEAN]**: Encapsulates the subtle pattern of capturing errno before close() can clobber it; used at several error paths in create(), eliminating repeated unsafe boilerplate.
- **Tests [NONE]**: No test file found. This private safety helper (captures errno before close to avoid clobbering) is untested. The error-capture ordering logic is non-trivial but has zero test coverage.
- **PARTIAL [PARTIAL]**: Private function with three `///` lines explaining purpose (errno preservation), the errno-clobbering rationale, and a `Safety:` precondition. Missing `# Examples` and parameter description, but the safety contract and motivation are clearly stated — more complete than typical private helpers. (deliberated: reclassified: duplication: DUPLICATE → UNIQUE — Duplication reclassified DUPLICATE→UNIQUE: linux.rs and macos.rs are platform-specific modules compiled exclusively via cfg. Duplicating a 5-line helper across platform modules is standard practice — extracting to a shared module would create an artificial dependency for trivial code. Tests NONE is retained: the errno-before-close ordering is a subtle correctness property that could benefit from testing. Documentation PARTIAL is correct: has three /// lines covering purpose and safety.)

> **Duplicate of** `rustguard-tun/src/linux.rs:close_and_error` — 97.7% identical — both capture OS error, close fd, and return error

#### `make_sockaddr_in` (L85–L93)

- **Utility [USED]**: Called in configure_address to create IPv4 addresses
- **Duplication [UNIQUE]**: Returns custom SockaddrIn struct specific to macOS; Linux version returns libc::sockaddr_in with different field conversions
- **Correction [OK]**: Correctly initialises SockaddrIn with sin_len=16 (sizeof struct), family, zero port, and the IPv4 octets.
- **Overengineering [LEAN]**: Called three times inside configure_address for addr, broadaddr, and mask; the deduplication is appropriate and the helper is minimal.
- **Tests [NONE]**: No test file found. This private constructor has testable pure logic (field population from an Ipv4Addr) that could be unit-tested without syscalls, but no tests exist.
- **UNDOCUMENTED [UNDOCUMENTED]**: Private constructor helper with no `///` doc comment. Name and signature are self-descriptive; private-item leniency applies. (deliberated: reclassified: documentation: UNDOCUMENTED → DOCUMENTED — Tests NONE retained: this constructor has testable pure logic (field population) that could be verified without syscalls. Documentation reclassified: private helper with a clear name and simple signature; the function body is obvious from the name make_sockaddr_in(addr: Ipv4Addr).)

#### `create` (L95–L192)

- **Utility [DEAD]**: Exported with 0 runtime importers in crate analysis
- **Duplication [UNIQUE]**: macOS utun creation via kernel control socket is fundamentally different from Linux /dev/net/tun approach; different ioctls, device discovery, and configuration flow
- **Correction [NEEDS_FIX]**: Integer overflow at `num + 1`: if a caller supplies a name like "utun4294967295" then num=u32::MAX and num+1 wraps to 0 in release builds (silent auto-assign semantics, wrong interface requested) or panics in debug builds. Use checked_add(1).ok_or(InvalidInput) instead.
- **Overengineering [LEAN]**: Six sequential kernel steps are each necessary (socket, CTLIOCGINFO, connect, getsockopt, configure_address, set_mtu); complexity is inherent to the Darwin utun creation protocol, not artificially layered.
- **Tests [NONE]**: No test file found. The public entry point for creating a utun device involves raw syscalls making direct unit testing hard, but the name-parsing logic (strip_prefix("utun"), parse::<u32>(), unit = num+1) and error paths are pure and fully testable. Zero tests exist.
- **UNDOCUMENTED [UNDOCUMENTED]**: Public function (`pub fn create`) with no `///` doc comment at all. As the primary public entry point for creating a macOS utun device, it should document parameters, return value, error conditions, and include a `# Examples` section. (deliberated: reclassified: utility: DEAD → USED — Utility reclassified DEAD→USED: this is the primary public API entry point for macOS utun creation. It's in a platform-specific module (macos.rs) that is conditionally compiled — the '0 runtime importers' finding reflects analysis on a non-macOS platform or incomplete crate analysis, not actual dead code. Correction NEEDS_FIX retained: num+1 overflow for u32::MAX is technically UB in debug and silently wrong in release; checked_add is the correct fix. Tests NONE retained: name-parsing and error-path logic is testable. Documentation UNDOCUMENTED retained: as the primary public entry point, this function genuinely needs /// documentation covering parameters, return value, and error conditions.)

#### `configure_address` (L194–L218)

- **Utility [USED]**: Called from create to configure interface IP address
- **Duplication [UNIQUE]**: macOS uses single IfAliasReq ioctl (SIOCAIFADDR) for address/mask; Linux configure_interface uses separate ioctls per field
- **Correction [NEEDS_FIX]**: errno is read after close(): `ioctl` sets errno on failure, then `libc::close(sock)` is called unconditionally before the `if ret < 0` check and `last_os_error()` call. If close() itself fails (e.g. EINTR), it overwrites errno with its own error code, masking the real ioctl error. The existing `close_and_error` helper exists precisely to avoid this pattern but is not used here. Fix: capture errno immediately after the ioctl before calling close().
- **Overengineering [LEAN]**: Focused function that opens a temporary DGRAM socket, issues SIOCAIFADDR, and closes it; minimal and correctly scoped.
- **Tests [NONE]**: No test file found. Private function wrapping SIOCAIFADDR ioctl. Fully syscall-dependent; no tests exist anywhere in the project.
- **UNDOCUMENTED [UNDOCUMENTED]**: Private function with no `///` doc comment. Purpose is inferable from name and usage context; private-item leniency applies. (deliberated: reclassified: documentation: UNDOCUMENTED → DOCUMENTED — Correction NEEDS_FIX is valid and confirmed: libc::close(sock) is called before last_os_error(), allowing close() to clobber errno. The existing close_and_error helper in the same file solves exactly this problem but isn't used here. This is a real bug. Tests NONE retained. Documentation reclassified: private function whose name and signature clearly communicate its purpose; private-item leniency applies.)

#### `set_mtu` (L220–L242)

- **Utility [USED]**: Called from create to set interface MTU size
- **Duplication [UNIQUE]**: Similarity score 0.702 indicates different logic; focused MTU-only operation vs Linux's multi-step interface configuration
- **Correction [NEEDS_FIX]**: Same errno-clobbering defect as configure_address: `libc::close(sock)` is called before `last_os_error()`, so a failing close() can replace the ioctl errno with its own. Additionally, the IfreqMtu struct passed here is 20 bytes while the kernel reads 32 bytes, reading uninitialized stack memory (tracked on IfreqMtu). Fix: capture errno before close(), and pad IfreqMtu to 32 bytes.
- **Overengineering [LEAN]**: Mirrors configure_address pattern for a single ioctl; appropriately small and single-purpose.
- **Tests [NONE]**: No test file found. Private function wrapping SIOCSIFMTU ioctl. Fully syscall-dependent; no tests exist anywhere in the project.
- **UNDOCUMENTED [UNDOCUMENTED]**: Private function with no `///` doc comment. Name and signature are self-descriptive; private-item leniency applies. (deliberated: reclassified: documentation: UNDOCUMENTED → DOCUMENTED — Correction NEEDS_FIX is valid: same errno-clobbering defect as configure_address, plus the IfreqMtu struct padding issue means the kernel reads past allocated memory. Both are real bugs. Tests NONE retained. Documentation reclassified: private function with self-descriptive name; private-item leniency applies.)

#### `read` (L245–L263)

- **Utility [DEAD]**: Exported with 0 runtime importers in crate analysis
- **Duplication [UNIQUE]**: Complements write() by stripping macOS utun's 4-byte AF header; different semantic purpose despite similarity score 0.729
- **Correction [OK]**: readbuf is len+4, so after stripping the 4-byte header payload_len <= buf.len() always holds; no out-of-bounds copy is possible. The n<4 guard prevents underflow.
- **Overengineering [LEAN]**: Allocates a temporary Vec per call to accommodate the 4-byte AF header stripping; simpler than scatter-gather and correct. Not overengineered, though the per-call allocation is a minor performance trade-off rather than a design excess.
- **Tests [NONE]**: No test file found. Despite depending on libc::read for I/O, the AF-header-stripping logic (n < 4 error path, payload_len = n - 4, slice copy) is pure and could be tested with a fake fd or by factoring out the parsing logic. Zero tests exist for any of these behaviors.
- **PARTIAL [PARTIAL]**: Public function with a single `///` line describing purpose and the AF-header stripping behaviour. Missing parameter descriptions (`fd`, `buf`), return-value semantics (bytes written into buf), `# Errors` section for the `InvalidData` short-packet case, and `# Examples`. (deliberated: reclassified: utility: DEAD → USED, documentation: UNDOCUMENTED → PARTIAL — Utility reclassified DEAD→USED: same reasoning as create — this is a public API function in a platform-specific module, not dead code. Tests NONE retained: AF-header stripping logic is testable. Documentation reclassified UNDOCUMENTED→PARTIAL: the source clearly shows a `///` doc comment on line 245 describing purpose and AF-header stripping. The evaluator field was inconsistent with its own detail text which correctly identified PARTIAL.)

#### `write` (L266–L296)

- **Utility [DEAD]**: Exported with 0 runtime importers in crate analysis
- **Duplication [UNIQUE]**: macOS-specific: prepends 4-byte AF header inferred from IP version nibble; Linux version writes raw packet without header
- **Correction [OK]**: AF header [0,0,0,af] is big-endian (network order); Darwin utun reads it via ntohl, so AF_INET=2 and AF_INET6=30 are correctly communicated. saturating_sub(4) safely handles the unlikely partial-write edge case.
- **Overengineering [LEAN]**: Prepends AF header, dispatches the write, and returns payload byte count. IP-version detection from the first nibble is the correct minimal approach; no unnecessary abstraction.
- **Tests [NONE]**: No test file found. The pure logic paths — empty-packet rejection, IP version detection via packet[0] >> 4, unknown-version error, and AF-header prepending — are all testable without syscalls. All are completely uncovered.
- **PARTIAL [PARTIAL]**: Public function with a single `///` line describing purpose and the AF-header prepending behaviour. Missing parameter descriptions (`fd`, `packet`), return-value semantics (IP payload bytes, excluding the 4-byte header), `# Errors` for empty-packet and unknown-IP-version cases, and `# Examples`. (deliberated: reclassified: utility: DEAD → USED, documentation: UNDOCUMENTED → PARTIAL — Utility reclassified DEAD→USED: public API in platform-specific module, conditionally compiled for macOS. Tests NONE retained: IP version detection, empty-packet rejection, and AF-header prepending are all testable pure logic. Documentation reclassified UNDOCUMENTED→PARTIAL: line 266 has `/// Write an IP packet to the utun fd, prepending the 4-byte AF header.` The evaluator field contradicted its own detail which correctly identified PARTIAL.)

## Best Practices — 7.5/10

### Rules

| # | Rule | Status | Severity | Detail |
|---|------|--------|----------|--------|
| 2 | No unsafe blocks without clear justification comment | WARN | CRITICAL | Five of the six unsafe blocks lack a // SAFETY: comment: the large block spanning the entire create() body, both blocks in configure_address() and set_mtu(), and the inline unsafe calls in read() and write(). Only close_and_error has an explicit Safety: doc note. The file-level module docstring provides general context but does not substitute for per-block invariant documentation as required by Rust best practices (and enforced by clippy::undocumented_unsafe_blocks). [L96-L196, L201-L222, L226-L245, L256-L263, L281-L281] |
| 3 | Proper error handling with Result/Option (no silent ignores) | WARN | HIGH | The return value of libc::fcntl(fd, libc::F_SETFD, libc::FD_CLOEXEC) is silently discarded. A failure here means the fd will not be marked close-on-exec, allowing it to leak into child processes (e.g., route commands). All other syscall return values are correctly checked. [L108] |
| 9 | Documentation comments on public items | WARN | MEDIUM | pub fn create lacks any /// doc comment. pub fn read and pub fn write are both documented. The create function is the most complex entry point and most in need of documentation covering its parameters, the utunN naming convention, and its error conditions. [L95] |

### Suggestions

- Check the return value of fcntl(F_SETFD, FD_CLOEXEC) instead of silently ignoring it
  ```typescript
  // Before
  // Prevent fd leak into child processes (route commands).
  libc::fcntl(fd, libc::F_SETFD, libc::FD_CLOEXEC);
  // After
  // Prevent fd leak into child processes (route commands).
  if libc::fcntl(fd, libc::F_SETFD, libc::FD_CLOEXEC) < 0 {
      return Err(close_and_error(fd));
  }
  ```
- Add // SAFETY: comments to every unsafe block documenting the invariants being upheld
  ```typescript
  // Before
  unsafe {
      let fd = libc::socket(
          AF_SYSTEM as libc::c_int,
          libc::SOCK_DGRAM,
          SYSPROTO_CONTROL,
      );
  // After
  // SAFETY: AF_SYSTEM, SOCK_DGRAM, and SYSPROTO_CONTROL are valid Darwin
  // constants. The returned fd is immediately checked for < 0, and all
  // subsequent error paths call close_and_error(fd) to prevent leaks.
  unsafe {
      let fd = libc::socket(
          AF_SYSTEM as libc::c_int,
          libc::SOCK_DGRAM,
          SYSPROTO_CONTROL,
      );
  ```
- Add a /// doc comment to the public `create` function covering its parameters and error conditions
  ```typescript
  // Before
  pub fn create(config: &TunConfig) -> io::Result<Tun> {
  // After
  /// Create a macOS utun interface with the given configuration.
  ///
  /// Opens a kernel control socket and connects it to the `utun_control`
  /// kernel extension to create a new TUN device. If `config.name` is
  /// `Some("utunN")` the specific unit is requested; `None` auto-assigns
  /// the next available unit.
  ///
  /// # Errors
  /// Returns an [`io::Error`] if the socket cannot be created, the utun
  /// kernel extension is unavailable, or address/MTU configuration fails.
  pub fn create(config: &TunConfig) -> io::Result<Tun> {
  ```

## Actions

### Quick Wins

- **[correction · medium · small]** Pad IfreqMtu to 32 bytes by adding `_pad: [u8; 12]` (or `_pad: [u8; 12], initialised to zero`) so the kernel's copyin for SIOCSIFMTU reads only allocated memory instead of uninitialized stack bytes. [L65]
- **[correction · medium · small]** In configure_address, capture errno before calling close(): save `let err = io::Error::last_os_error();` immediately after the ioctl call, then close the socket, then return `Err(err)` if ret < 0. Mirrors the close_and_error helper already present in this file. [L210]
- **[correction · medium · small]** In set_mtu, apply the same errno-before-close fix: capture last_os_error() before libc::close(sock), then return that captured error if ret < 0. [L234]
- **[correction · low · small]** In create, replace `num + 1` with `num.checked_add(1).ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "utun unit number too large"))?` to prevent silent wrap-to-zero or debug panic for extreme utun indices. [L131]

### Hygiene

- **[documentation · low · trivial]** Complete JSDoc documentation for: `close_and_error` (`close_and_error`) [L79-L83]
- **[documentation · medium · trivial]** Add JSDoc documentation for exported symbol: `create` (`create`) [L95-L192]
- **[documentation · medium · trivial]** Add JSDoc documentation for exported symbol: `read` (`read`) [L245-L263]
- **[documentation · medium · trivial]** Add JSDoc documentation for exported symbol: `write` (`write`) [L266-L296]
