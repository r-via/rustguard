# Review: `rustguard-tun/src/linux.rs`

**Verdict:** NEEDS_REFACTOR

## Symbols

| Symbol | Kind | Exported | Correction | Over-eng. | Utility | Duplication | Tests | Confidence |
|--------|------|----------|------------|-----------|---------|-------------|-------|------------|
| TUNSETIFF | constant | no | OK | LEAN | USED | UNIQUE | GOOD | 90% |
| SIOCSIFADDR | constant | no | OK | LEAN | USED | UNIQUE | GOOD | 90% |
| SIOCSIFDSTADDR | constant | no | OK | LEAN | USED | UNIQUE | GOOD | 90% |
| SIOCSIFNETMASK | constant | no | OK | LEAN | USED | UNIQUE | GOOD | 90% |
| SIOCSIFMTU | constant | no | OK | LEAN | USED | UNIQUE | GOOD | 90% |
| SIOCSIFFLAGS | constant | no | OK | LEAN | USED | UNIQUE | GOOD | 90% |
| SIOCGIFFLAGS | constant | no | OK | LEAN | USED | UNIQUE | GOOD | 90% |
| IFF_TUN | constant | no | OK | LEAN | USED | UNIQUE | GOOD | 90% |
| IFF_NO_PI | constant | no | OK | LEAN | USED | UNIQUE | GOOD | 90% |
| IFF_UP | constant | no | OK | LEAN | USED | UNIQUE | GOOD | 90% |
| IFNAMSIZ | constant | no | OK | LEAN | USED | UNIQUE | GOOD | 90% |
| IfreqFlags | class | no | OK | LEAN | USED | UNIQUE | GOOD | 90% |
| IfreqAddr | class | no | NEEDS_FIX | LEAN | USED | UNIQUE | GOOD | 85% |
| IfreqMtu | class | no | OK | LEAN | USED | UNIQUE | GOOD | 90% |
| last_os_error | function | no | OK | LEAN | USED | UNIQUE | NONE | 88% |
| close_and_error | function | no | OK | LEAN | USED | DUPLICATE | NONE | 85% |
| make_sockaddr_in | function | no | OK | LEAN | USED | DUPLICATE | NONE | 85% |
| set_name | function | no | OK | LEAN | USED | DUPLICATE | NONE | 85% |
| create | function | yes | OK | LEAN | USED | UNIQUE | NONE | 85% |
| configure_interface | function | no | NEEDS_FIX | LEAN | USED | DUPLICATE | NONE | 85% |
| read | function | yes | OK | LEAN | USED | UNIQUE | NONE | 85% |
| write | function | yes | OK | LEAN | USED | UNIQUE | NONE | 85% |

### Details

#### `TUNSETIFF` (L12–L12)

- **Utility [USED]**: Used in libc::ioctl call at L102 to set TUN interface flags
- **Duplication [UNIQUE]**: Linux ioctl constant. No semantic duplicates found in codebase.
- **Correction [OK]**: Value 0x400454ca correctly matches Linux _IOW('T', 202, int): (1<<30)|(0x54<<8)|(202)|(4<<16) = 0x400454ca. Cast from u32 to c_int is safe since 0x400454ca < 2^31.
- **Overengineering [LEAN]**: Required Linux ioctl request code that libc does not export; must be defined manually for kernel ABI.
- **Tests [GOOD]**: Compile-time constant with no runtime behavior. Per rule 6, constants are GOOD by default. No test file exists but none is required.
- **UNDOCUMENTED [UNDOCUMENTED]**: Private constant with a standard Linux ioctl name. No /// doc comment; only a preceding // inline group comment. Tolerated under private-item leniency given the self-descriptive kernel-standard name. (deliberated: reclassified: documentation: UNDOCUMENTED → DOCUMENTED — Private constant with universally recognized Linux kernel ABI name. The evaluator acknowledged 'tolerated under private-item leniency.' Reclassifying to DOCUMENTED: for private kernel ioctl constants, the standard name IS the documentation. No action references this finding.)

#### `SIOCSIFADDR` (L13–L13)

- **Utility [USED]**: Used in libc::ioctl call at L141 to set interface address
- **Duplication [UNIQUE]**: Linux ioctl constant. No semantic duplicates found in codebase.
- **Correction [OK]**: 0x8916 matches the canonical Linux SIOCSIFADDR ioctl constant.
- **Overengineering [LEAN]**: Standard POSIX ioctl constant, necessary for setting the interface address.
- **Tests [GOOD]**: Compile-time constant with no runtime behavior. Per rule 6, constants are GOOD by default.
- **UNDOCUMENTED [UNDOCUMENTED]**: Private constant for a standard Linux ioctl code. No /// doc comment present; tolerated under private-item leniency for well-known kernel constants. (deliberated: reclassified: documentation: UNDOCUMENTED → DOCUMENTED — Same rationale as TUNSETIFF — private constant with canonical POSIX ioctl name that any Linux systems programmer recognizes. Tolerated under private-item leniency per the evaluator's own detail.)

#### `SIOCSIFDSTADDR` (L14–L14)

- **Utility [USED]**: Used in libc::ioctl call at L147 to set destination address
- **Duplication [UNIQUE]**: Linux ioctl constant. No semantic duplicates found in codebase.
- **Correction [OK]**: 0x8918 matches the canonical Linux SIOCSIFDSTADDR ioctl constant.
- **Overengineering [LEAN]**: Standard POSIX ioctl constant for setting the point-to-point destination address.
- **Tests [GOOD]**: Compile-time constant with no runtime behavior. Per rule 6, constants are GOOD by default.
- **UNDOCUMENTED [UNDOCUMENTED]**: Private constant for the SIOCSIFDSTADDR ioctl. No /// doc comment; tolerated under private-item leniency. (deliberated: reclassified: documentation: UNDOCUMENTED → DOCUMENTED — Private constant, standard Linux kernel name. Self-documenting by convention. Reclassified under private-item leniency.)

#### `SIOCSIFNETMASK` (L15–L15)

- **Utility [USED]**: Used in libc::ioctl call at L154 to set interface netmask
- **Duplication [UNIQUE]**: Linux ioctl constant. No semantic duplicates found in codebase.
- **Correction [OK]**: 0x891c matches the canonical Linux SIOCSIFNETMASK ioctl constant.
- **Overengineering [LEAN]**: Standard POSIX ioctl constant for setting the network mask.
- **Tests [GOOD]**: Compile-time constant with no runtime behavior. Per rule 6, constants are GOOD by default.
- **UNDOCUMENTED [UNDOCUMENTED]**: Private constant for the SIOCSIFNETMASK ioctl. No /// doc comment; tolerated under private-item leniency. (deliberated: reclassified: documentation: UNDOCUMENTED → DOCUMENTED — Private constant, standard Linux kernel name. Self-documenting. Reclassified under private-item leniency.)

#### `SIOCSIFMTU` (L16–L16)

- **Utility [USED]**: Used in libc::ioctl call at L169 to set MTU
- **Duplication [UNIQUE]**: Linux ioctl constant. No semantic duplicates found in codebase.
- **Correction [OK]**: 0x8922 matches the canonical Linux SIOCSIFMTU ioctl constant.
- **Overengineering [LEAN]**: Standard POSIX ioctl constant for setting MTU; minimal and necessary.
- **Tests [GOOD]**: Compile-time constant with no runtime behavior. Per rule 6, constants are GOOD by default.
- **UNDOCUMENTED [UNDOCUMENTED]**: Private constant for the SIOCSIFMTU ioctl. No /// doc comment; tolerated under private-item leniency. (deliberated: reclassified: documentation: UNDOCUMENTED → DOCUMENTED — Private constant, standard Linux kernel name. Self-documenting. Reclassified under private-item leniency.)

#### `SIOCSIFFLAGS` (L17–L17)

- **Utility [USED]**: Used in libc::ioctl call at L181 to set interface flags
- **Duplication [UNIQUE]**: Linux ioctl constant. No semantic duplicates found in codebase.
- **Correction [OK]**: 0x8914 matches the canonical Linux SIOCSIFFLAGS ioctl constant.
- **Overengineering [LEAN]**: Standard POSIX ioctl constant for setting interface flags.
- **Tests [GOOD]**: Compile-time constant with no runtime behavior. Per rule 6, constants are GOOD by default.
- **UNDOCUMENTED [UNDOCUMENTED]**: Private constant for the SIOCSIFFLAGS ioctl. No /// doc comment; tolerated under private-item leniency. (deliberated: reclassified: documentation: UNDOCUMENTED → DOCUMENTED — Private constant, standard Linux kernel name. Self-documenting. Reclassified under private-item leniency.)

#### `SIOCGIFFLAGS` (L18–L18)

- **Utility [USED]**: Used in libc::ioctl call at L177 to get current flags
- **Duplication [UNIQUE]**: Linux ioctl constant. No semantic duplicates found in codebase.
- **Correction [OK]**: 0x8913 matches the canonical Linux SIOCGIFFLAGS ioctl constant.
- **Overengineering [LEAN]**: Standard POSIX ioctl constant for getting interface flags; needed for the get-then-set pattern.
- **Tests [GOOD]**: Compile-time constant with no runtime behavior. Per rule 6, constants are GOOD by default.
- **UNDOCUMENTED [UNDOCUMENTED]**: Private constant for the SIOCGIFFLAGS ioctl. No /// doc comment; tolerated under private-item leniency. (deliberated: reclassified: documentation: UNDOCUMENTED → DOCUMENTED — Private constant, standard Linux kernel name. Self-documenting. Reclassified under private-item leniency.)

#### `IFF_TUN` (L21–L21)

- **Utility [USED]**: Used in bitwise OR at L95 to set TUN mode in ifr_flags
- **Duplication [UNIQUE]**: TUN interface flag constant. No semantic duplicates found in codebase.
- **Correction [OK]**: 0x0001 matches the Linux kernel IFF_TUN TUN device flag.
- **Overengineering [LEAN]**: Linux TUN flag constant not exported by libc; correctly defined inline.
- **Tests [GOOD]**: Compile-time constant with no runtime behavior. Per rule 6, constants are GOOD by default.
- **UNDOCUMENTED [UNDOCUMENTED]**: Private constant for the Linux IFF_TUN flag. No /// doc comment; only a preceding // group comment. Tolerated under private-item leniency for a well-known kernel flag. (deliberated: reclassified: documentation: UNDOCUMENTED → DOCUMENTED — Private constant, well-known Linux TUN flag. The preceding group comment ('// TUN flags.') provides category context. Self-documenting name. Reclassified under private-item leniency.)

#### `IFF_NO_PI` (L22–L22)

- **Utility [USED]**: Used in bitwise OR at L95 to set no-protocol-info flag
- **Duplication [UNIQUE]**: TUN interface flag constant. No semantic duplicates found in codebase.
- **Correction [OK]**: 0x1000 matches the Linux kernel IFF_NO_PI flag value.
- **Overengineering [LEAN]**: Linux TUN flag constant to suppress the 4-byte packet-info header; minimal and necessary.
- **Tests [GOOD]**: Compile-time constant with no runtime behavior. Per rule 6, constants are GOOD by default.
- **UNDOCUMENTED [UNDOCUMENTED]**: Private constant for the IFF_NO_PI flag. No /// doc comment; tolerated under private-item leniency. (deliberated: reclassified: documentation: UNDOCUMENTED → DOCUMENTED — Private constant, well-known Linux TUN flag alongside IFF_TUN. Self-documenting name with group comment. Reclassified under private-item leniency.)

#### `IFF_UP` (L25–L25)

- **Utility [USED]**: Used in bitwise OR at L180 to bring interface up
- **Duplication [UNIQUE]**: Interface flag constant. No semantic duplicates found in codebase.
- **Correction [OK]**: 0x1 matches the standard Linux IFF_UP net_device flag.
- **Overengineering [LEAN]**: Standard interface-up flag; single bit constant, no abstraction overhead.
- **Tests [GOOD]**: Compile-time constant with no runtime behavior. Per rule 6, constants are GOOD by default.
- **UNDOCUMENTED [UNDOCUMENTED]**: Private constant for the IFF_UP interface flag. No /// doc comment; tolerated under private-item leniency. (deliberated: reclassified: documentation: UNDOCUMENTED → DOCUMENTED — Private constant, universally known interface flag. Self-documenting name with group comment. Reclassified under private-item leniency.)

#### `IFNAMSIZ` (L27–L27)

- **Utility [USED]**: Used extensively: array size in structs (L33, L40, L46), buffer operations (L74, L96, L115, L133)
- **Duplication [UNIQUE]**: Interface name size constant. No semantic duplicates found in codebase.
- **Correction [OK]**: 16 matches the Linux kernel IFNAMSIZ definition.
- **Overengineering [LEAN]**: Kernel-defined interface name buffer size; must be matched exactly.
- **Tests [GOOD]**: Compile-time constant with no runtime behavior. Per rule 6, constants are GOOD by default.
- **UNDOCUMENTED [UNDOCUMENTED]**: Private constant matching the Linux IFNAMSIZ value (16). No /// doc comment; tolerated under private-item leniency for a well-known kernel constant. (deliberated: reclassified: documentation: UNDOCUMENTED → DOCUMENTED — Private constant matching the canonical Linux kernel IFNAMSIZ (16). Any systems programmer recognizes this immediately. Reclassified under private-item leniency.)

#### `IfreqFlags` (L32–L36)

- **Utility [USED]**: Instantiated at L95 and L176 for ioctl calls managing interface flags
- **Duplication [UNIQUE]**: Struct for interface flags ioctl request. No semantic duplicates found.
- **Correction [OK]**: Layout: 16 (ifr_name) + 2 (c_short) + 22 (_pad) = 40 bytes, correctly matching sizeof(struct ifreq) on Linux x86_64. The comment 'union padding to 32 bytes' is misleading (the union portion is 24 bytes, total is 40), but the actual byte layout is correct.
- **Overengineering [LEAN]**: Idiomatic Rust strategy for the C ifreq union: a separate repr(C) struct per usage variant avoids unsafe union field access. Three structs for three distinct ioctl payloads is minimal.
- **Tests [GOOD]**: #[repr(C)] struct with no methods used purely for FFI layout. No runtime behavior beyond field layout. Per rule 6, types with no runtime behavior are GOOD by default.
- **UNDOCUMENTED [UNDOCUMENTED]**: Private #[repr(C)] struct mirroring the Linux ifreq union layout for flag-based ioctl calls. No /// doc comment on the type or any field; the _pad field carries only an inline comment. Tolerated under private-item leniency. (deliberated: reclassified: documentation: UNDOCUMENTED → DOCUMENTED — Private repr(C) FFI struct with self-descriptive field names (ifr_name, ifr_flags, _pad). Section header comment ('Structs matching Linux kernel layout') provides context. Private-item leniency applies; the struct's purpose is clear from its name and fields.)

#### `IfreqAddr` (L39–L42)

- **Utility [USED]**: Instantiated at L138 and reused for address, destination, and netmask ioctl operations
- **Duplication [UNIQUE]**: Struct for interface address ioctl request. No semantic duplicates found.
- **Correction [NEEDS_FIX]**: IfreqAddr is 16 (ifr_name) + 16 (sockaddr_in) = 32 bytes, but Linux struct ifreq is 40 bytes. The kernel's copy_from_user reads sizeof(ifreq) = 40 bytes from the user pointer, reading 8 bytes beyond this struct's allocation. In practice the three ioctls used (SIOCSIFADDR, SIOCSIFDSTADDR, SIOCSIFNETMASK) only consume the first 32 bytes so no crash occurs, but the struct violates the ifreq ABI and relies on valid stack memory existing past the allocation. An 8-byte trailing `_pad` field should be added.
- **Overengineering [LEAN]**: Minimal repr(C) struct matching the kernel ifreq layout for address-related ioctls.
- **Tests [GOOD]**: #[repr(C)] struct with no methods used purely for FFI layout. No runtime behavior beyond field layout. Per rule 6, types with no runtime behavior are GOOD by default.
- **UNDOCUMENTED [UNDOCUMENTED]**: Private #[repr(C)] struct for address-related ioctl operations. No /// doc comment on the type or its fields; tolerated under private-item leniency. (deliberated: reclassified: documentation: UNDOCUMENTED → DOCUMENTED — Correction NEEDS_FIX confirmed: struct is 32 bytes but Linux ifreq is 40. The kernel's copy_from_user reads sizeof(ifreq)=40 bytes, reading 8 bytes past the struct allocation. While functionally harmless for SIOCSIFADDR/SIOCSIFDSTADDR/SIOCSIFNETMASK (only first 32 bytes consumed), this is a real ABI violation reading potentially uninitialized stack memory. Adding _pad: [u8; 8] is the correct fix. Documentation reclassified to DOCUMENTED: private repr(C) struct with self-descriptive fields under private-item leniency.)

#### `IfreqMtu` (L45–L49)

- **Utility [USED]**: Instantiated at L160 for ioctl call to set MTU
- **Duplication [UNIQUE]**: Struct for interface MTU ioctl request. No semantic duplicates found.
- **Correction [OK]**: Layout: 16 (ifr_name) + 4 (c_int) + 20 (_pad) = 40 bytes, correctly matching sizeof(struct ifreq) on Linux x86_64.
- **Overengineering [LEAN]**: Minimal repr(C) struct for the MTU ioctl payload; explicit padding is required to match the 40-byte ifreq union size.
- **Tests [GOOD]**: #[repr(C)] struct with no methods used purely for FFI layout. No runtime behavior beyond field layout. Per rule 6, types with no runtime behavior are GOOD by default.
- **UNDOCUMENTED [UNDOCUMENTED]**: Private #[repr(C)] struct for MTU ioctl operations. No /// doc comment on the type or fields; _pad carries only an inline comment. Tolerated under private-item leniency. (deliberated: reclassified: documentation: UNDOCUMENTED → DOCUMENTED — Private repr(C) FFI struct with clear field names and inline padding comment. Section header provides context. Reclassified under private-item leniency.)

#### `last_os_error` (L51–L53)

- **Utility [USED]**: Called at L84 and L130 to capture OS error state
- **Duplication [UNIQUE]**: Trivial wrapper function (single line). No semantic duplicates found.
- **Correction [OK]**: Correct thin wrapper around io::Error::last_os_error().
- **Overengineering [LEAN]**: One-line alias that shortens repeated error capture sites. Trivial enough that it is not worth flagging; avoids the verbose qualified path in unsafe blocks.
- **Tests [NONE]**: No test file exists for this module. Private helper wrapping io::Error::last_os_error() has no unit tests whatsoever.
- **UNDOCUMENTED [UNDOCUMENTED]**: Private one-line wrapper around io::Error::last_os_error(). No /// doc comment; self-descriptive name and trivial body tolerated under private-item leniency. (deliberated: reclassified: documentation: UNDOCUMENTED → DOCUMENTED — Tests NONE confirmed — no tests exist, factually correct. However, this is a one-line private wrapper around io::Error::last_os_error() in a privileged FFI module; testing burden is minimal but absence is not critical. Documentation reclassified to DOCUMENTED: single-line private function whose name perfectly describes its behavior.)

#### `close_and_error` (L55–L59)

- **Utility [USED]**: Called at L105, L142, L149, L156, L171, L184 to clean up and propagate errors
- **Duplication [DUPLICATE]**: Identical 4-line function at score 0.987 — gets last OS error, closes fd, returns error. Code is byte-for-byte identical.
- **Correction [OK]**: Correctly captures the OS error before calling libc::close, preventing close() from overwriting errno before the error is read.
- **Overengineering [LEAN]**: Captures errno before close() can clobber it, then closes the fd. This two-step idiom is a well-known requirement in C/FFI error paths and is used in both create() and configure_interface().
- **Tests [NONE]**: No test file exists for this module. Private helper combining fd close and last OS error capture has no unit tests. The close-then-error ordering is untested.
- **UNDOCUMENTED [UNDOCUMENTED]**: Private helper that closes a file descriptor and captures the OS error. No /// doc comment; name is sufficiently descriptive for an internal helper. (deliberated: reclassified: documentation: UNDOCUMENTED → DOCUMENTED — Duplication DUPLICATE confirmed: byte-for-byte identical implementation exists in linux_mq.rs. Both modules are in the same crate and could share a common linux_util module. Tests NONE confirmed factually. Documentation reclassified to DOCUMENTED: private 4-line helper with perfectly descriptive name (closes fd, returns the error).)

> **Duplicate of** `rustguard-tun/src/linux_mq.rs:close_and_error` — 99% identical — identical implementation of error capture and fd cleanup

#### `make_sockaddr_in` (L61–L70)

- **Utility [USED]**: Called at L139, L146, L153 to construct sockaddr_in structures from IPv4 addresses
- **Duplication [DUPLICATE]**: Identical sockaddr_in constructor at score 0.974. Both fill AF_INET address structure with same logic.
- **Correction [OK]**: u32::from_ne_bytes(addr.octets()) is a bitwise reinterpretation of the byte array, so the bytes stored in memory for s_addr are identical to addr.octets() on all endiannesses. Since the kernel reads s_addr as network-byte-order bytes in memory, this is correct.
- **Overengineering [LEAN]**: Builds a sockaddr_in from an Ipv4Addr. Called three times in configure_interface for address, destination, and netmask; deduplication is appropriate.
- **Tests [NONE]**: No test file exists. Private helper constructing sockaddr_in from Ipv4Addr has no unit tests. Address byte ordering (ne_bytes vs be_bytes) is a correctness-critical detail that is completely untested.
- **UNDOCUMENTED [UNDOCUMENTED]**: Private helper constructing a libc::sockaddr_in from an Ipv4Addr. No /// doc comment; tolerated under private-item leniency given the descriptive name. (deliberated: reclassified: documentation: UNDOCUMENTED → DOCUMENTED — Duplication DUPLICATE confirmed: identical sockaddr_in constructor in linux_mq.rs, both in same crate. Reasonable refactoring target. Tests NONE confirmed. Documentation reclassified to DOCUMENTED: private helper with self-descriptive name and clear parameter type (Ipv4Addr → sockaddr_in).)

> **Duplicate of** `rustguard-tun/src/linux_mq.rs:make_sockaddr_in` — 99% identical — both create libc::sockaddr_in with same field initialization

#### `set_name` (L72–L76)

- **Utility [USED]**: Called at L97, L114, L135, L166, L174 to populate interface name buffers safely
- **Duplication [DUPLICATE]**: Identical 4-line function at score 0.979 — copies interface name bytes with length-clamped slicing.
- **Correction [OK]**: Correctly truncates name to IFNAMSIZ-1 bytes. All callers pass a freshly zeroed [0u8; IFNAMSIZ] buffer, so the null terminator is already present at buf[len] and beyond.
- **Overengineering [LEAN]**: Safely null-terminates a name into a fixed [u8; 16] buffer. Called five times across create() and configure_interface(); centralising the bounds-safe copy is correct.
- **Tests [NONE]**: No test file exists. Private helper copying name bytes into fixed-length buffer has no unit tests. Truncation behavior for names at or beyond IFNAMSIZ-1 bytes and null-termination are untested.
- **UNDOCUMENTED [UNDOCUMENTED]**: Private helper that copies an interface name into a fixed-size IFNAMSIZ buffer with null termination. No /// doc comment; tolerated under private-item leniency. (deliberated: reclassified: documentation: UNDOCUMENTED → DOCUMENTED — Duplication DUPLICATE confirmed: identical implementation in linux_mq.rs. Tests NONE confirmed. Documentation reclassified to DOCUMENTED: private helper with clear name and obvious semantics (copies name into fixed-size buffer).)

> **Duplicate of** `rustguard-tun/src/linux_mq.rs:set_name` — 99% identical — byte-for-byte same implementation of clamped name copying

#### `create` (L78–L122)

- **Utility [USED]**: Exported public API function for Linux TUN device creation. Zero in-file importers is expected for library module public API; known false positive pattern from cross-crate usage not visible in same-crate analysis
- **Duplication [UNIQUE]**: Platform-specific TUN device creation. Score 0.834 with macos.rs but different semantic contract: Linux uses TUNSETIFF ioctl on /dev/net/tun, macOS uses kernel control socket with CTLIOCGINFO. Different callee chains (configure_interface vs configure_address+set_mtu). Different invariants mandated by platform APIs.
- **Correction [OK]**: Uses correctly-sized IfreqFlags (40 bytes) for TUNSETIFF. All error paths close fd before returning. Name extraction handles both null-terminated and fully-filled IFNAMSIZ cases. configure_interface cleanup on error is correct.
- **Overengineering [LEAN]**: Linear sequence of open → TUNSETIFF → name extraction → configure_interface. No unnecessary abstraction; delegates address/MTU/flags work to configure_interface cleanly.
- **Tests [NONE]**: No test file exists for this module. Public entry point performing unsafe ioctl calls to open /dev/net/tun and configure the interface has zero test coverage. Requires root/CAP_NET_ADMIN but integration tests under those conditions are still absent.
- **UNDOCUMENTED [UNDOCUMENTED]**: Public function — the primary entry point for opening and configuring a Linux TUN device. Has zero /// doc comments. Needs at minimum: purpose description, parameter docs for TunConfig fields, io::Result error conditions, and an # Examples section. (deliberated: confirmed — Tests NONE confirmed: no integration tests exist even though this is the public entry point. Requires CAP_NET_ADMIN but integration test infrastructure is still absent. Documentation UNDOCUMENTED confirmed and kept: this is a PUBLIC function — the primary entry point for Linux TUN creation — with zero doc comments. Unlike private helpers, public API surface must have Rustdoc for users. Action 5 is valid (though its description incorrectly says 'JSDoc' instead of 'Rustdoc').)

#### `configure_interface` (L124–L189)

- **Utility [USED]**: Called at L117 from create() to configure interface addresses, netmask, MTU, and bring it up
- **Duplication [DUPLICATE]**: Identical interface configuration logic at score 0.970 with linux_mq.rs. Both perform same sequence: set address (SIOCSIFADDR), destination (SIOCSIFDSTADDR), netmask (SIOCSIFNETMASK), MTU (SIOCSIFMTU), then get+set flags (SIOCGIFFLAGS/SIOCSIFFLAGS). Same ioctl codes, same struct patterns, interchangeable.
- **Correction [NEEDS_FIX]**: Uses IfreqAddr which is 32 bytes rather than the 40-byte ifreq the kernel expects. Passing &req to SIOCSIFADDR, SIOCSIFDSTADDR, and SIOCSIFNETMASK causes the kernel to read 8 bytes beyond the struct boundary during copy_from_user. Functionally harmless for these specific ioctls since only the first 32 bytes are consumed, but IfreqAddr must be padded to 40 bytes for strict ABI correctness. All socket fd close-on-error paths are otherwise correct.
- **Overengineering [LEAN]**: Sequential ioctl calls to set address, destination, netmask, MTU, and bring the interface up. Each step is mandatory for a usable TUN device; no superfluous layers.
- **Tests [NONE]**: No test file exists. Private function performing multiple sequential ioctl calls (address, destination, netmask, MTU, flags) has no tests. Error handling on each ioctl path and the IFF_UP bring-up sequence are entirely untested.
- **UNDOCUMENTED [UNDOCUMENTED]**: Private function orchestrating all ioctl calls (address, destination, netmask, MTU, IFF_UP). No /// doc comment; tolerated under private-item leniency. (deliberated: reclassified: documentation: UNDOCUMENTED → DOCUMENTED — Correction NEEDS_FIX confirmed: this function uses IfreqAddr which is undersized (32 vs 40 bytes). The root cause is in IfreqAddr's definition (action 1 addresses it), but the usage site is where the ABI violation manifests. Duplication DUPLICATE confirmed: 98% identical to linux_mq.rs version — same ioctl sequence, same structs, same error handling. Tests NONE confirmed. Documentation reclassified to DOCUMENTED: private function with descriptive name; private-item leniency applies.)

> **Duplicate of** `rustguard-tun/src/linux_mq.rs:configure_interface` — 98% identical — same ioctl sequence and struct manipulation for Linux TUN configuration

#### `read` (L193–L199)

- **Utility [USED]**: Exported public API function for reading packets from TUN device. Zero in-file importers expected for library module public API; pattern matches known false positives for library crate exports
- **Duplication [UNIQUE]**: Read from TUN device with error handling. No semantic duplicates found in RAG results.
- **Correction [OK]**: Correctly calls libc::read, checks for negative ssize_t return, and casts to usize only after confirming n >= 0.
- **Overengineering [LEAN]**: Minimal safe wrapper over libc::read with error propagation. IFF_NO_PI removes any need for header stripping, keeping this to 5 lines.
- **Tests [NONE]**: No test file exists. Public function wrapping libc::read on the TUN fd has no unit or integration tests. Error path (negative return) and buffer-size edge cases are untested.
- **PARTIAL [PARTIAL]**: Public function with two /// doc lines describing purpose and the IFF_NO_PI clean-frame property. Missing parameter descriptions for fd and buf, return value semantics, # Errors section, and # Examples. (deliberated: confirmed — Tests NONE confirmed: no tests for this public FFI wrapper. Documentation PARTIAL confirmed and kept: has two /// doc lines describing purpose and IFF_NO_PI behavior, but missing parameter docs, return value semantics, and # Errors section. For a public function, PARTIAL is the correct assessment.)

#### `write` (L203–L212)

- **Utility [USED]**: Exported public API function for writing packets to TUN device. Zero in-file importers expected for library module public API; consistent with false positive pattern for library crate public functions
- **Duplication [UNIQUE]**: Direct IP packet write at score 0.751 with macos.rs but different logic: macOS adds 4-byte family header and subtracts 4 from written count; Linux writes raw packet directly. Score below 0.82 threshold and code semantics differ.
- **Correction [OK]**: Empty-packet guard is correct. libc::write return value is checked for negative before casting ssize_t to usize.
- **Overengineering [LEAN]**: Minimal safe wrapper over libc::write with an empty-packet guard and error propagation. No unnecessary abstractions.
- **Tests [NONE]**: No test file exists. Public function wrapping libc::write on the TUN fd has no tests. The empty-packet guard (InvalidInput early return) and error path on negative libc::write return are both untested.
- **PARTIAL [PARTIAL]**: Public function with two /// doc lines describing purpose and IFF_NO_PI direct-write behavior. Missing parameter descriptions for fd and packet, # Errors section (notably the empty-packet InvalidInput case), and # Examples. (deliberated: confirmed — Tests NONE confirmed: no tests for this public FFI wrapper. Documentation PARTIAL confirmed and kept: has two /// doc lines but omits parameter descriptions, the empty-packet InvalidInput error case in # Errors, and # Examples. For a public function, PARTIAL is correct.)

## Best Practices — 6.75/10

### Rules

| # | Rule | Status | Severity | Detail |
|---|------|--------|----------|--------|
| 2 | No unsafe blocks without clear justification comment | FAIL | CRITICAL | Five distinct unsafe sites (close_and_error L52, the entire body of create() L70, configure_interface() L116, read() L188, write() L197) carry no `// SAFETY:` comment explaining the invariants that make each call sound. The module-level doc comment and operational comments like `// 1. Open /dev/net/tun.` explain *what* is done, not *why* the unsafety is justified. Additionally, wrapping the entire body of `create()` and `configure_interface()` in a single large `unsafe` block violates the Rust idiom of minimising unsafe surface area. [L52, L70-L111, L116-L175, L188, L197] |
| 9 | Documentation comments on public items | WARN | MEDIUM | `pub fn read` (L183) and `pub fn write` (L192) both have `///` doc comments. However, `pub fn create` (L70) — the primary entry point of this module — has no documentation comment at all. Users of the crate cannot learn its contract, error conditions, or platform requirements from `rustdoc`. [L70] |

### Suggestions

- Add `// SAFETY:` justification comments to every unsafe block, explaining the invariants that make each FFI call sound.
  ```typescript
  // Before
  pub fn read(fd: i32, buf: &mut [u8]) -> io::Result<usize> {
      let n = unsafe { libc::read(fd, buf.as_mut_ptr() as *mut _, buf.len()) };
      ...
  }
  // After
  pub fn read(fd: i32, buf: &mut [u8]) -> io::Result<usize> {
      // SAFETY: `fd` is a valid open file descriptor owned by the caller.
      // `buf` is valid for writes of exactly `buf.len()` bytes for the
      // duration of the call. The return value is checked immediately.
      let n = unsafe { libc::read(fd, buf.as_mut_ptr() as *mut _, buf.len()) };
      ...
  }
  ```
- Minimise unsafe scope in `create()` and `configure_interface()` by extracting only the FFI calls into targeted unsafe blocks rather than wrapping the entire function body.
  ```typescript
  // Before
  pub fn create(config: &TunConfig) -> io::Result<Tun> {
      unsafe {
          let fd = libc::open(...);
          // ... all safe Rust logic also inside unsafe
      }
  }
  // After
  pub fn create(config: &TunConfig) -> io::Result<Tun> {
      // SAFETY: path is a valid NUL-terminated C string literal.
      let fd = unsafe {
          libc::open(b"/dev/net/tun\0".as_ptr() as *const libc::c_char,
                     libc::O_RDWR | libc::O_CLOEXEC)
      };
      if fd < 0 { return Err(last_os_error()); }
      // ... safe Rust logic outside unsafe ...
  }
  ```
- Add a `///` doc comment to `pub fn create` describing its purpose, the config fields it consumes, and possible error conditions.
  ```typescript
  // Before
  pub fn create(config: &TunConfig) -> io::Result<Tun> {
  // After
  /// Create and configure a Linux TUN device.
  ///
  /// Opens `/dev/net/tun`, registers an `IFF_TUN | IFF_NO_PI` interface,
  /// and configures the address, destination, netmask, MTU, and `IFF_UP`
  /// flag via a temporary `AF_INET` socket.
  ///
  /// # Errors
  /// Returns `io::Error` if the device cannot be opened, any ioctl fails,
  /// or the kernel-assigned interface name is not valid UTF-8.
  pub fn create(config: &TunConfig) -> io::Result<Tun> {
  ```

## Actions

### Quick Wins

- **[correction · low · small]** Add an 8-byte trailing padding field to IfreqAddr so its size equals sizeof(struct ifreq) = 40 bytes: add `_pad: [u8; 8]` after ifr_addr. Without it the kernel's copy_from_user reads 8 bytes beyond the struct allocation for every SIOCSIFADDR/SIOCSIFDSTADDR/SIOCSIFNETMASK ioctl call. [L39]

### Refactors

- **[duplication · medium · small]** Deduplicate: `close_and_error` duplicates `close_and_error` in `rustguard-tun/src/linux_mq.rs` (`close_and_error`) [L55-L59]
- **[duplication · medium · small]** Deduplicate: `make_sockaddr_in` duplicates `make_sockaddr_in` in `rustguard-tun/src/linux_mq.rs` (`make_sockaddr_in`) [L61-L70]
- **[duplication · medium · small]** Deduplicate: `set_name` duplicates `set_name` in `rustguard-tun/src/linux_mq.rs` (`set_name`) [L72-L76]
- **[duplication · medium · small]** Deduplicate: `configure_interface` duplicates `configure_interface` in `rustguard-tun/src/linux_mq.rs` (`configure_interface`) [L124-L189]

### Hygiene

- **[documentation · medium · trivial]** Add JSDoc documentation for exported symbol: `create` (`create`) [L78-L122]
- **[documentation · low · trivial]** Complete JSDoc documentation for: `read` (`read`) [L193-L199]
- **[documentation · low · trivial]** Complete JSDoc documentation for: `write` (`write`) [L203-L212]
