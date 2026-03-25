# Documentation — Shard 2

## Findings

| File | Verdict | Documentation | Conf. | Details |
|------|---------|---------------|-------|---------|
| `rustguard-core/src/messages.rs` | NEEDS_REFACTOR | 12 | 90% | [details](../reviews/rustguard-core-src-messages.rev.md) |
| `rustguard-core/src/timers.rs` | NEEDS_REFACTOR | 1 | 90% | [details](../reviews/rustguard-core-src-timers.rev.md) |
| `rustguard-enroll/src/control.rs` | NEEDS_REFACTOR | 9 | 90% | [details](../reviews/rustguard-enroll-src-control.rev.md) |
| `rustguard-crypto/src/aead.rs` | NEEDS_REFACTOR | 8 | 93% | [details](../reviews/rustguard-crypto-src-aead.rev.md) |
| `rustguard-daemon/src/config.rs` | NEEDS_REFACTOR | 5 | 90% | [details](../reviews/rustguard-daemon-src-config.rev.md) |
| `rustguard-tun/src/linux.rs` | NEEDS_REFACTOR | 3 | 90% | [details](../reviews/rustguard-tun-src-linux.rev.md) |
| `rustguard-enroll/src/protocol.rs` | NEEDS_REFACTOR | 8 | 88% | [details](../reviews/rustguard-enroll-src-protocol.rev.md) |
| `rustguard-tun/src/bpf_loader.rs` | NEEDS_REFACTOR | 1 | 88% | [details](../reviews/rustguard-tun-src-bpf_loader.rev.md) |
| `rustguard-enroll/src/fast_udp.rs` | NEEDS_REFACTOR | 7 | 92% | [details](../reviews/rustguard-enroll-src-fast_udp.rev.md) |
| `rustguard-tun/src/macos.rs` | NEEDS_REFACTOR | 4 | 90% | [details](../reviews/rustguard-tun-src-macos.rev.md) |

## Symbol Details

### `rustguard-core/src/messages.rs`

| Symbol | Lines | Documentation | Conf. | Detail |
|--------|-------|---------------|-------|--------|
| `MSG_INITIATION` | L5–L5 | UNDOCUMENTED | 80% | [USED] Used in Initiation::to_bytes (line 103) to encode message type field |... |
| `MSG_RESPONSE` | L6–L6 | UNDOCUMENTED | 80% | [USED] Used in Response::to_bytes (line 121) to encode message type field | [... |
| `MSG_COOKIE_REPLY` | L7–L7 | UNDOCUMENTED | 90% | [USED] Used in CookieReply::to_bytes (line 148) to encode message type field ... |
| `MSG_TRANSPORT` | L8–L8 | UNDOCUMENTED | 90% | [USED] Used in Transport::to_bytes (line 162) to encode message type field | ... |
| `Initiation` | L22–L29 | PARTIAL | 88% | [USED] Struct with impl methods (to_bytes, from_bytes) and used in test_initi... |
| `INITIATION_SIZE` | L31–L31 | UNDOCUMENTED | 78% | [USED] Used in Initiation::to_bytes (line 101) and from_bytes signature (line... |
| `Response` | L45–L52 | PARTIAL | 88% | [USED] Struct with impl methods (to_bytes, from_bytes) and used in test_respo... |
| `RESPONSE_SIZE` | L54–L54 | UNDOCUMENTED | 78% | [USED] Used in Response::to_bytes (line 120) and from_bytes signature (line 1... |
| `Transport` | L64–L68 | PARTIAL | 85% | [USED] Struct with impl methods (to_bytes, from_bytes) and used in test_trans... |
| `CookieReply` | L79–L83 | PARTIAL | 88% | [USED] Struct with impl methods (to_bytes, from_bytes) for cookie reply messa... |
| `COOKIE_REPLY_SIZE` | L85–L85 | UNDOCUMENTED | 90% | [USED] Used in CookieReply::to_bytes (line 146) and from_bytes signature (lin... |
| `TRANSPORT_HEADER_SIZE` | L87–L87 | UNDOCUMENTED | 80% | [USED] Used in Transport::to_bytes (line 161) and from_bytes (line 167) for b... |

### `rustguard-core/src/timers.rs`

| Symbol | Lines | Documentation | Conf. | Detail |
|--------|-------|---------------|-------|--------|
| `REKEY_TIMEOUT` | L39–L39 | PARTIAL | 85% | [USED] Exported constant used locally in should_retry_handshake method (L150)... |

### `rustguard-enroll/src/control.rs`

| Symbol | Lines | Documentation | Conf. | Detail |
|--------|-------|---------------|-------|--------|
| `socket_path` | L18–L20 | PARTIAL | 90% | [USED] Called internally by start_listener (L79). Centralizes socket path def... |
| `new_window` | L26–L28 | UNDOCUMENTED | 85% | [USED] Exported factory function to create enrollment window. Part of public ... |
| `is_open` | L31–L41 | PARTIAL | 88% | [DEAD] Exported utility function. Not called within this file and no imports ... |
| `open_window` | L44–L51 | PARTIAL | 72% | [USED] Called internally by handle_client at L128. Implements OPEN protocol c... |
| `close_window` | L54–L56 | PARTIAL | 90% | [USED] Called internally by handle_client at L131. Implements CLOSE protocol ... |
| `remaining` | L59–L73 | PARTIAL | 88% | [USED] Called internally by handle_client at L137. Implements STATUS response... |
| `start_listener` | L77–L104 | PARTIAL | 85% | [USED] Exported entry point to start control socket listener. Core public API... |
| `send_command` | L147–L166 | PARTIAL | 85% | [USED] Exported public API for CLI/client-side usage. Pre-computed analysis s... |
| `cleanup` | L169–L171 | PARTIAL | 85% | [USED] Exported cleanup function to remove socket file. Part of public API fo... |

### `rustguard-crypto/src/aead.rs`

| Symbol | Lines | Documentation | Conf. | Detail |
|--------|-------|---------------|-------|--------|
| `AEAD_TAG_LEN` | L7–L7 | UNDOCUMENTED | 93% | [DEAD] Exported constant with 0 runtime/type-only importers. Public API const... |
| `MAX_PACKET_SIZE` | L10–L10 | PARTIAL | 88% | [DEAD] Exported constant with 0 runtime/type-only importers. Public API const... |
| `seal` | L16–L28 | PARTIAL | 90% | [DEAD] Exported core AEAD encryption function with 0 runtime/type-only import... |
| `open` | L33–L45 | PARTIAL | 92% | [DEAD] Exported core AEAD decryption function with 0 runtime/type-only import... |
| `xseal` | L48–L54 | PARTIAL | 90% | [DEAD] Exported extended-nonce encryption function with 0 runtime/type-only i... |
| `xopen` | L57–L63 | PARTIAL | 90% | [DEAD] Exported extended-nonce decryption function with 0 runtime/type-only i... |
| `seal_to` | L68–L78 | PARTIAL | 90% | [DEAD] Exported in-place encryption function with 0 runtime/type-only importe... |
| `open_to` | L83–L96 | PARTIAL | 88% | [DEAD] Exported in-place decryption function with 0 runtime/type-only importe... |

### `rustguard-daemon/src/config.rs`

| Symbol | Lines | Documentation | Conf. | Detail |
|--------|-------|---------------|-------|--------|
| `Config` | L13–L16 | PARTIAL | 88% | [USED] Returned by public parsing functions from_file and parse, used in test... |
| `InterfaceConfig` | L19–L25 | UNDOCUMENTED | 90% | [USED] Returned by parse_interface, stored in Config.interface, essential dat... |
| `PeerConfig` | L28–L34 | UNDOCUMENTED | 90% | [USED] Returned by parse_peer, stored in Config.peers vector, represents peer... |
| `CidrAddr` | L38–L41 | PARTIAL | 88% | [USED] Contains IPv4/IPv6 CIDR logic via contains_v4, contains_v6, contains m... |
| `prefix_to_netmask` | L284–L292 | UNDOCUMENTED | 85% | [USED] Public function called in parse_interface on line 200 to convert CIDR ... |

### `rustguard-tun/src/linux.rs`

| Symbol | Lines | Documentation | Conf. | Detail |
|--------|-------|---------------|-------|--------|
| `create` | L78–L122 | UNDOCUMENTED | 85% | [USED] Exported public API function for Linux TUN device creation. Zero in-fi... |
| `read` | L193–L199 | PARTIAL | 85% | [USED] Exported public API function for reading packets from TUN device. Zero... |
| `write` | L203–L212 | PARTIAL | 85% | [USED] Exported public API function for writing packets to TUN device. Zero i... |

### `rustguard-enroll/src/protocol.rs`

| Symbol | Lines | Documentation | Conf. | Detail |
|--------|-------|---------------|-------|--------|
| `ENROLL_REQUEST_SIZE` | L17–L17 | UNDOCUMENTED | 88% | [USED] Exported const in library enrollment protocol API. Pre-computed shows ... |
| `ENROLL_RESPONSE_SIZE` | L18–L18 | UNDOCUMENTED | 88% | [USED] Exported const in library enrollment protocol API. Matches known false... |
| `derive_token_key` | L22–L24 | PARTIAL | 85% | [USED] Exported function in library crate public API. Pre-computed 0 importer... |
| `build_request` | L27–L36 | PARTIAL | 80% | [USED] Exported protocol function in library crate. Matches known false-posit... |
| `parse_request` | L40–L51 | PARTIAL | 80% | [USED] Exported protocol function in library crate. Matches known FP pattern ... |
| `EnrollmentOffer` | L54–L58 | PARTIAL | 85% | [USED] Exported struct in library crate public API. Follows known false-posit... |
| `build_response` | L61–L75 | PARTIAL | 80% | [USED] Exported protocol function in library crate. Matches known pattern for... |
| `parse_response` | L78–L102 | PARTIAL | 80% | [USED] Exported protocol function in library crate. Pre-computed 0 importers ... |

### `rustguard-tun/src/bpf_loader.rs`

| Symbol | Lines | Documentation | Conf. | Detail |
|--------|-------|---------------|-------|--------|
| `XdpProgram` | L37–L41 | PARTIAL | 88% | [DEAD] Exported pub struct with 0 runtime importers per pre-computed analysis... |

### `rustguard-enroll/src/fast_udp.rs`

| Symbol | Lines | Documentation | Conf. | Detail |
|--------|-------|---------------|-------|--------|
| `BATCH_SIZE` | L14–L14 | PARTIAL | 92% | [DEAD] Exported constant with zero external importers per pre-computed analys... |
| `PKT_BUF_SIZE` | L16–L16 | PARTIAL | 92% | [DEAD] Exported constant with zero external importers per pre-computed analys... |
| `RecvBatch` | L19–L24 | PARTIAL | 90% | [DEAD] Exported struct with zero external importers. Serves as the core data ... |
| `recv_batch` | L39–L83 | PARTIAL | 88% | [DEAD] Exported function (macOS fallback version) with zero external importer... |
| `send_packet` | L106–L108 | UNDOCUMENTED | 85% | [DEAD] Exported function (macOS fallback version) with zero external importer... |
| `recv_batch` | L112–L126 | PARTIAL | 88% | [DEAD] Exported function (macOS fallback version) with zero external importer... |
| `send_packet` | L129–L131 | UNDOCUMENTED | 85% | [DEAD] Exported function (macOS fallback version) with zero external importer... |

### `rustguard-tun/src/macos.rs`

| Symbol | Lines | Documentation | Conf. | Detail |
|--------|-------|---------------|-------|--------|
| `close_and_error` | L79–L83 | PARTIAL | 88% | [USED] Called in create to close fd while preserving errno | [DUPLICATE] Iden... |
| `create` | L95–L192 | UNDOCUMENTED | 90% | [DEAD] Exported with 0 runtime importers in crate analysis | [UNIQUE] macOS u... |
| `read` | L245–L263 | PARTIAL | 90% | [DEAD] Exported with 0 runtime importers in crate analysis | [UNIQUE] Complem... |
| `write` | L266–L296 | PARTIAL | 90% | [DEAD] Exported with 0 runtime importers in crate analysis | [UNIQUE] macOS-s... |

## Hygiene

- [ ] <!-- ACT-3c9a32-1 --> **[documentation · medium · trivial]** `rustguard-core/src/messages.rs`: Add JSDoc documentation for exported symbol: `MSG_INITIATION` (`MSG_INITIATION`) [L5-L5]
- [ ] <!-- ACT-3c9a32-2 --> **[documentation · medium · trivial]** `rustguard-core/src/messages.rs`: Add JSDoc documentation for exported symbol: `MSG_RESPONSE` (`MSG_RESPONSE`) [L6-L6]
- [ ] <!-- ACT-3c9a32-3 --> **[documentation · medium · trivial]** `rustguard-core/src/messages.rs`: Add JSDoc documentation for exported symbol: `MSG_COOKIE_REPLY` (`MSG_COOKIE_REPLY`) [L7-L7]
- [ ] <!-- ACT-3c9a32-4 --> **[documentation · medium · trivial]** `rustguard-core/src/messages.rs`: Add JSDoc documentation for exported symbol: `MSG_TRANSPORT` (`MSG_TRANSPORT`) [L8-L8]
- [ ] <!-- ACT-3c9a32-6 --> **[documentation · medium · trivial]** `rustguard-core/src/messages.rs`: Add JSDoc documentation for exported symbol: `INITIATION_SIZE` (`INITIATION_SIZE`) [L31-L31]
- [ ] <!-- ACT-3c9a32-8 --> **[documentation · medium · trivial]** `rustguard-core/src/messages.rs`: Add JSDoc documentation for exported symbol: `RESPONSE_SIZE` (`RESPONSE_SIZE`) [L54-L54]
- [ ] <!-- ACT-3c9a32-11 --> **[documentation · medium · trivial]** `rustguard-core/src/messages.rs`: Add JSDoc documentation for exported symbol: `COOKIE_REPLY_SIZE` (`COOKIE_REPLY_SIZE`) [L85-L85]
- [ ] <!-- ACT-3c9a32-12 --> **[documentation · medium · trivial]** `rustguard-core/src/messages.rs`: Add JSDoc documentation for exported symbol: `TRANSPORT_HEADER_SIZE` (`TRANSPORT_HEADER_SIZE`) [L87-L87]
- [ ] <!-- ACT-efa3ec-4 --> **[documentation · medium · trivial]** `rustguard-crypto/src/aead.rs`: Add JSDoc documentation for exported symbol: `AEAD_TAG_LEN` (`AEAD_TAG_LEN`) [L7-L7]
- [ ] <!-- ACT-dbf131-3 --> **[documentation · medium · trivial]** `rustguard-daemon/src/config.rs`: Add JSDoc documentation for exported symbol: `InterfaceConfig` (`InterfaceConfig`) [L19-L25]
- [ ] <!-- ACT-dbf131-4 --> **[documentation · medium · trivial]** `rustguard-daemon/src/config.rs`: Add JSDoc documentation for exported symbol: `PeerConfig` (`PeerConfig`) [L28-L34]
- [ ] <!-- ACT-dbf131-6 --> **[documentation · medium · trivial]** `rustguard-daemon/src/config.rs`: Add JSDoc documentation for exported symbol: `prefix_to_netmask` (`prefix_to_netmask`) [L284-L292]
- [ ] <!-- ACT-18d76d-3 --> **[documentation · medium · trivial]** `rustguard-enroll/src/control.rs`: Add JSDoc documentation for exported symbol: `new_window` (`new_window`) [L26-L28]
- [ ] <!-- ACT-18d76d-5 --> **[documentation · medium · trivial]** `rustguard-enroll/src/control.rs`: Add JSDoc documentation for exported symbol: `is_open` (`is_open`) [L31-L41]
- [ ] <!-- ACT-c2ad01-2 --> **[documentation · medium · trivial]** `rustguard-enroll/src/fast_udp.rs`: Add JSDoc documentation for exported symbol: `BATCH_SIZE` (`BATCH_SIZE`) [L14-L14]
- [ ] <!-- ACT-c2ad01-4 --> **[documentation · medium · trivial]** `rustguard-enroll/src/fast_udp.rs`: Add JSDoc documentation for exported symbol: `PKT_BUF_SIZE` (`PKT_BUF_SIZE`) [L16-L16]
- [ ] <!-- ACT-c2ad01-6 --> **[documentation · medium · trivial]** `rustguard-enroll/src/fast_udp.rs`: Add JSDoc documentation for exported symbol: `RecvBatch` (`RecvBatch`) [L19-L24]
- [ ] <!-- ACT-c2ad01-8 --> **[documentation · medium · trivial]** `rustguard-enroll/src/fast_udp.rs`: Add JSDoc documentation for exported symbol: `recv_batch` (`recv_batch`) [L39-L83]
- [ ] <!-- ACT-c2ad01-10 --> **[documentation · medium · trivial]** `rustguard-enroll/src/fast_udp.rs`: Add JSDoc documentation for exported symbol: `send_packet` (`send_packet`) [L106-L108]
- [ ] <!-- ACT-c2ad01-12 --> **[documentation · medium · trivial]** `rustguard-enroll/src/fast_udp.rs`: Add JSDoc documentation for exported symbol: `recv_batch` (`recv_batch`) [L112-L126]
- [ ] <!-- ACT-c2ad01-14 --> **[documentation · medium · trivial]** `rustguard-enroll/src/fast_udp.rs`: Add JSDoc documentation for exported symbol: `send_packet` (`send_packet`) [L129-L131]
- [ ] <!-- ACT-d84ed3-1 --> **[documentation · medium · trivial]** `rustguard-enroll/src/protocol.rs`: Add JSDoc documentation for exported symbol: `ENROLL_REQUEST_SIZE` (`ENROLL_REQUEST_SIZE`) [L17-L17]
- [ ] <!-- ACT-d84ed3-2 --> **[documentation · medium · trivial]** `rustguard-enroll/src/protocol.rs`: Add JSDoc documentation for exported symbol: `ENROLL_RESPONSE_SIZE` (`ENROLL_RESPONSE_SIZE`) [L18-L18]
- [ ] <!-- ACT-62f2c7-5 --> **[documentation · medium · trivial]** `rustguard-tun/src/bpf_loader.rs`: Add JSDoc documentation for exported symbol: `XdpProgram` (`XdpProgram`) [L37-L41]
- [ ] <!-- ACT-c62ee4-5 --> **[documentation · medium · trivial]** `rustguard-tun/src/linux.rs`: Add JSDoc documentation for exported symbol: `create` (`create`) [L78-L122]
- [ ] <!-- ACT-b59607-8 --> **[documentation · medium · trivial]** `rustguard-tun/src/macos.rs`: Add JSDoc documentation for exported symbol: `create` (`create`) [L95-L192]
- [ ] <!-- ACT-b59607-10 --> **[documentation · medium · trivial]** `rustguard-tun/src/macos.rs`: Add JSDoc documentation for exported symbol: `read` (`read`) [L245-L263]
- [ ] <!-- ACT-b59607-12 --> **[documentation · medium · trivial]** `rustguard-tun/src/macos.rs`: Add JSDoc documentation for exported symbol: `write` (`write`) [L266-L296]
- [ ] <!-- ACT-3c9a32-5 --> **[documentation · low · trivial]** `rustguard-core/src/messages.rs`: Complete JSDoc documentation for: `Initiation` (`Initiation`) [L22-L29]
- [ ] <!-- ACT-3c9a32-7 --> **[documentation · low · trivial]** `rustguard-core/src/messages.rs`: Complete JSDoc documentation for: `Response` (`Response`) [L45-L52]
- [ ] <!-- ACT-3c9a32-9 --> **[documentation · low · trivial]** `rustguard-core/src/messages.rs`: Complete JSDoc documentation for: `Transport` (`Transport`) [L64-L68]
- [ ] <!-- ACT-3c9a32-10 --> **[documentation · low · trivial]** `rustguard-core/src/messages.rs`: Complete JSDoc documentation for: `CookieReply` (`CookieReply`) [L79-L83]
- [ ] <!-- ACT-fb2f2d-6 --> **[documentation · low · trivial]** `rustguard-core/src/timers.rs`: Complete JSDoc documentation for: `REKEY_TIMEOUT` (`REKEY_TIMEOUT`) [L39-L39]
- [ ] <!-- ACT-dbf131-2 --> **[documentation · low · trivial]** `rustguard-daemon/src/config.rs`: Complete JSDoc documentation for: `Config` (`Config`) [L13-L16]
- [ ] <!-- ACT-dbf131-5 --> **[documentation · low · trivial]** `rustguard-daemon/src/config.rs`: Complete JSDoc documentation for: `CidrAddr` (`CidrAddr`) [L38-L41]
- [ ] <!-- ACT-18d76d-2 --> **[documentation · low · trivial]** `rustguard-enroll/src/control.rs`: Complete JSDoc documentation for: `socket_path` (`socket_path`) [L18-L20]
- [ ] <!-- ACT-18d76d-6 --> **[documentation · low · trivial]** `rustguard-enroll/src/control.rs`: Complete JSDoc documentation for: `open_window` (`open_window`) [L44-L51]
- [ ] <!-- ACT-18d76d-7 --> **[documentation · low · trivial]** `rustguard-enroll/src/control.rs`: Complete JSDoc documentation for: `close_window` (`close_window`) [L54-L56]
- [ ] <!-- ACT-18d76d-8 --> **[documentation · low · trivial]** `rustguard-enroll/src/control.rs`: Complete JSDoc documentation for: `remaining` (`remaining`) [L59-L73]
- [ ] <!-- ACT-18d76d-9 --> **[documentation · low · trivial]** `rustguard-enroll/src/control.rs`: Complete JSDoc documentation for: `start_listener` (`start_listener`) [L77-L104]
- [ ] <!-- ACT-18d76d-10 --> **[documentation · low · trivial]** `rustguard-enroll/src/control.rs`: Complete JSDoc documentation for: `send_command` (`send_command`) [L147-L166]
- [ ] <!-- ACT-18d76d-11 --> **[documentation · low · trivial]** `rustguard-enroll/src/control.rs`: Complete JSDoc documentation for: `cleanup` (`cleanup`) [L169-L171]
- [ ] <!-- ACT-d84ed3-3 --> **[documentation · low · trivial]** `rustguard-enroll/src/protocol.rs`: Complete JSDoc documentation for: `derive_token_key` (`derive_token_key`) [L22-L24]
- [ ] <!-- ACT-d84ed3-4 --> **[documentation · low · trivial]** `rustguard-enroll/src/protocol.rs`: Complete JSDoc documentation for: `build_request` (`build_request`) [L27-L36]
- [ ] <!-- ACT-d84ed3-5 --> **[documentation · low · trivial]** `rustguard-enroll/src/protocol.rs`: Complete JSDoc documentation for: `parse_request` (`parse_request`) [L40-L51]
- [ ] <!-- ACT-d84ed3-6 --> **[documentation · low · trivial]** `rustguard-enroll/src/protocol.rs`: Complete JSDoc documentation for: `EnrollmentOffer` (`EnrollmentOffer`) [L54-L58]
- [ ] <!-- ACT-d84ed3-7 --> **[documentation · low · trivial]** `rustguard-enroll/src/protocol.rs`: Complete JSDoc documentation for: `build_response` (`build_response`) [L61-L75]
- [ ] <!-- ACT-d84ed3-8 --> **[documentation · low · trivial]** `rustguard-enroll/src/protocol.rs`: Complete JSDoc documentation for: `parse_response` (`parse_response`) [L78-L102]
- [ ] <!-- ACT-c62ee4-7 --> **[documentation · low · trivial]** `rustguard-tun/src/linux.rs`: Complete JSDoc documentation for: `read` (`read`) [L193-L199]
- [ ] <!-- ACT-c62ee4-8 --> **[documentation · low · trivial]** `rustguard-tun/src/linux.rs`: Complete JSDoc documentation for: `write` (`write`) [L203-L212]
- [ ] <!-- ACT-b59607-6 --> **[documentation · low · trivial]** `rustguard-tun/src/macos.rs`: Complete JSDoc documentation for: `close_and_error` (`close_and_error`) [L79-L83]

## Documentation Coverage

### `rustguard-enroll/src/control.rs` — 30% covered

- [ ] **socket_path** — PARTIAL → `rustguard-enroll/src/control.rs`: Brief one-liner present; missing return description and examples.
- [ ] **new_window** — MISSING → `rustguard-enroll/src/control.rs`: No doc comment on constructor; initial state semantics are undocumented.
- [ ] **is_open** — PARTIAL → `rustguard-enroll/src/control.rs`: Purpose stated; parameter and examples absent.
- [ ] **open_window** — PARTIAL → `rustguard-enroll/src/control.rs`: Duration parameter mentioned in prose; overwrite behavior and examples missing.
- [ ] **close_window** — PARTIAL → `rustguard-enroll/src/control.rs`: One-liner present; no parameter or examples.
- [ ] **remaining** — PARTIAL → `rustguard-enroll/src/control.rs`: Return semantics noted; parameter and examples absent.
- [ ] **start_listener** — PARTIAL → `rustguard-enroll/src/control.rs`: Thread and return noted; parameters, Errors section, and examples absent.
- [ ] **handle_client** — MISSING → `rustguard-enroll/src/control.rs`: Private function with no doc comment; protocol dispatch logic undocumented.
- [ ] **send_command** — PARTIAL → `rustguard-enroll/src/control.rs`: Purpose stated; command format, Errors section, and examples absent.
- [ ] **cleanup** — PARTIAL → `rustguard-enroll/src/control.rs`: Purpose stated; parameter description and silent-error behavior undocumented.

### `rustguard-enroll/src/protocol.rs` — 48% covered

- [ ] **ENROLL_REQUEST_SIZE** — MISSING: Exported size constant has no `///` doc comment; consumers cannot discover its meaning without reading the module-level prose.
- [ ] **ENROLL_RESPONSE_SIZE** — MISSING: Exported size constant has no `///` doc comment; same issue as ENROLL_REQUEST_SIZE.
- [ ] **EnrollmentOffer fields** — PARTIAL: Struct-level doc summarises all three fields but individual `pub` fields carry no `///` annotations.
- [ ] **Public API examples** — MISSING: No public function includes a `# Examples` section. The test module provides roundtrip coverage but is not surfaced as rustdoc examples.
