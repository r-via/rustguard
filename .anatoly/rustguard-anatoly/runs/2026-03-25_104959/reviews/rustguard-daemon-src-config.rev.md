# Review: `rustguard-daemon/src/config.rs`

**Verdict:** NEEDS_REFACTOR

## Symbols

| Symbol | Kind | Exported | Correction | Over-eng. | Utility | Duplication | Tests | Confidence |
|--------|------|----------|------------|-----------|---------|-------------|-------|------------|
| Config | class | yes | OK | LEAN | USED | UNIQUE | WEAK | 88% |
| InterfaceConfig | class | yes | OK | LEAN | USED | UNIQUE | GOOD | 90% |
| PeerConfig | class | yes | OK | LEAN | USED | UNIQUE | GOOD | 90% |
| CidrAddr | class | yes | OK | LEAN | USED | UNIQUE | WEAK | 88% |
| decode_key | function | no | OK | LEAN | USED | UNIQUE | WEAK | 60% |
| parse_interface | function | no | NEEDS_FIX | LEAN | USED | UNIQUE | WEAK | 85% |
| parse_peer | function | no | OK | LEAN | USED | UNIQUE | WEAK | 60% |
| parse_cidr | function | no | OK | LEAN | USED | UNIQUE | WEAK | 60% |
| prefix_to_netmask | function | yes | OK | LEAN | USED | UNIQUE | WEAK | 85% |

### Details

#### `Config` (L13–L16)

- **Utility [USED]**: Returned by public parsing functions from_file and parse, used in tests
- **Duplication [UNIQUE]**: Struct definition with no similar types found in RAG results
- **Correction [OK]**: Plain data struct; field types are correct and consistent with the rest of the module.
- **Overengineering [LEAN]**: Two-field struct that maps directly to WireGuard config sections. No unnecessary indirection or generics.
- **Tests [WEAK]**: The `parse` method is exercised by `parse_full_config` (happy path with two peers, port, address, keepalive) and `parse_v6_allowed_ips`. However, all error paths (key outside section, bad line without '=', no [Interface] section) are untested, and `from_file` has zero test coverage.
- **PARTIAL [PARTIAL]**: Has a struct-level `/// Parsed WireGuard configuration.` doc comment, but neither public field (`interface`, `peers`) carries a `///` comment explaining its role. No `# Examples` section for a public API type. (deliberated: confirmed — Tests WEAK is accurate: error paths (bad line, key outside section, no [Interface]) and from_file are untested. Documentation PARTIAL is fair: struct has a doc comment but the two public fields lack individual doc comments. For a two-field struct with self-descriptive names (interface, peers) missing field docs is minor, but PARTIAL is the correct classification.)

#### `InterfaceConfig` (L19–L25)

- **Utility [USED]**: Returned by parse_interface, stored in Config.interface, essential data type
- **Duplication [UNIQUE]**: Struct definition with no similar types found in RAG results
- **Correction [OK]**: Field types are appropriate. Storing the derived netmask as Ipv4Addr alongside the address is a design choice, not a bug.
- **Overengineering [LEAN]**: Five fields that map one-to-one to WireGuard INI keys. Using Option<(Ipv6Addr, u8)> for optional dual-stack is appropriately minimal.
- **Tests [GOOD]**: Pure data struct with no runtime behavior. Rule 6 applies: types/interfaces with no runtime logic are GOOD by default. Fields are indirectly populated and asserted in `parse_full_config`.
- **UNDOCUMENTED [UNDOCUMENTED]**: No `///` doc comment on the struct definition, and none of the five public fields (`private_key`, `listen_port`, `address`, `netmask`, `address_v6`) are documented. Completely absent for a public API type. (deliberated: confirmed — UNDOCUMENTED is correct: no doc comment at all on a public struct with five fields including non-obvious ones like netmask (stored separately from address) and address_v6 as Option<(Ipv6Addr, u8)>. A consumer needs to understand the tuple semantics. Tests are GOOD per rule 6 (pure data struct), no change needed there.)

#### `PeerConfig` (L28–L34)

- **Utility [USED]**: Returned by parse_peer, stored in Config.peers vector, represents peer data
- **Duplication [UNIQUE]**: Struct definition with no similar types found in RAG results
- **Correction [OK]**: All fields have correct types; Optional wrapping of endpoint, preshared_key, and keepalive is appropriate.
- **Overengineering [LEAN]**: Five optional/required fields mirroring the WireGuard [Peer] spec precisely. No spurious abstraction.
- **Tests [GOOD]**: Pure data struct with no runtime behavior. Rule 6 applies. Fields are exercised indirectly in `parse_full_config` (endpoint, allowed_ips, persistent_keepalive) and the optional-absent cases (endpoint=None, keepalive=None).
- **UNDOCUMENTED [UNDOCUMENTED]**: No `///` doc comment on the struct definition, and none of the five public fields (`public_key`, `preshared_key`, `endpoint`, `allowed_ips`, `persistent_keepalive`) carry any documentation. (deliberated: confirmed — UNDOCUMENTED is correct: public struct with five fields, no doc comments. Fields like allowed_ips (Vec<CidrAddr>) and the Option wrapping on several fields deserve explanation for API consumers.)

#### `CidrAddr` (L38–L41)

- **Utility [USED]**: Contains IPv4/IPv6 CIDR logic via contains_v4, contains_v6, contains methods
- **Duplication [UNIQUE]**: Struct definition with no similar types found in RAG results
- **Correction [OK]**: Struct fields addr: IpAddr and prefix_len: u8 are the correct types to represent a CIDR block for both v4 and v6.
- **Overengineering [LEAN]**: Minimal two-field struct. The impl block exposes contains_v4/contains_v6 typed variants alongside the unified contains; the typed methods are genuinely useful when the caller already has a typed IP. The CIDR math is hand-rolled but no CIDR crate is listed as a dependency, so this is not a NIH violation.
- **Tests [WEAK]**: `contains_v4` is well tested (normal /24, host /32, default /0, cross-family v4-vs-v6). `contains_v6` is only tested for /64 normal range; the /0 default-route and /128 host edge cases are missing. The public `contains` dispatcher method (which routes to contains_v4 or contains_v6) is never called directly in any test.
- **PARTIAL [PARTIAL]**: Has a meaningful struct-level doc comment (`/// An IP address with prefix length (CIDR notation). Supports v4 and v6.`), but neither public field (`addr`, `prefix_len`) has a `///` comment, and no `# Examples` section is present on this public type. (deliberated: confirmed — Tests WEAK is accurate: contains_v6 is missing /0 and /128 edge cases, and the public contains() dispatcher is never directly tested. Documentation PARTIAL is correct: struct has a meaningful doc comment but public fields lack individual docs and no examples are provided.)

#### `decode_key` (L167–L174)

- **Utility [USED]**: Called in parse_interface (line 187) and parse_peer (lines 245, 247) for key decoding
- **Duplication [UNIQUE]**: Decodes base64 string to 32-byte key. RAG found base64_key (0.706-0.708 score) which is the inverse operation (encodes bytes to base64 string). Different parameters, return types, and semantics despite high similarity score indicate complementary functions, not duplicates.
- **Correction [OK]**: BASE64_STANDARD requires padding; WireGuard 32-byte keys produce exactly 44-char base64 with one trailing '='. Because Config::parse uses split_once('=') on the assignment separator (the first '='), everything after it—including trailing base64 padding '='—is preserved in the value, so decoding is correct.
- **Overengineering [LEAN]**: Eight-line helper: base64 decode then fixed-size array coercion. Both steps are needed; nothing extraneous.
- **Tests [WEAK]**: Exercised indirectly on the happy path whenever `Config::parse` is called with valid base64 keys. Neither of the two error branches (invalid base64 encoding, decoded length != 32) is covered by any test.
- **UNDOCUMENTED [UNDOCUMENTED]**: Private function (`fn`, no `pub`) with no `///` doc comment. The name is reasonably self-descriptive, so lack of documentation is tolerable for a private helper, hence reduced confidence. (deliberated: confirmed — Tests WEAK confirmed: both error branches (invalid base64, wrong length) have no coverage. UNDOCUMENTED on a private function with a self-descriptive name warrants the already-reduced confidence of 60. The function name and signature make its purpose clear, so this is a very minor issue.)

#### `parse_interface` (L176–L226)

- **Utility [USED]**: Helper called by Config::parse on line 215 to parse interface configuration section
- **Duplication [UNIQUE]**: Parses [Interface] config section. RAG found parse_peer (0.706 score) with similar structure (both iterate kvs, match keys, return Result<Config>). However, they parse different semantic domains: parse_interface extracts interface-specific fields (privatekey, listenport, address) returning InterfaceConfig, while parse_peer handles peer fields (publickey, endpoint, allowedips) returning PeerConfig. Different required fields, validation logic, and return types make them semantically distinct despite structural similarity.
- **Correction [NEEDS_FIX]**: When an Address entry has no '/' prefix notation, the code unconditionally defaults to prefix '24' via unwrap_or((part, "24")). For an IPv6 address without an explicit prefix (e.g., 'Address = fd15::1'), prefix_len 24 is stored instead of the correct default of 128. By contrast, parse_cidr correctly branches on addr.is_ipv4() to pick 32 vs 128. In practice WireGuard configs always carry the CIDR prefix, but the inconsistency is a latent correctness bug.
- **Overengineering [LEAN]**: Handles all mandatory and optional WireGuard Interface keys including dual-stack Address parsing. Length (~50 lines) is proportional to the number of real fields and their distinct parsing rules. The two-pass design (collect kvs then parse) is a clean scanning/validation separation, not an unnecessary layer.
- **Tests [WEAK]**: `parse_full_config` checks listen_port, address, and netmask for the happy path. Missing coverage: dual-stack Address line (v4 + v6), missing PrivateKey error, missing Address error, malformed port, and the dual-stack v6 address field on the interface struct itself. Default ListenPort behavior is also never verified.
- **UNDOCUMENTED [UNDOCUMENTED]**: Private function with no `///` doc comment. The function has non-trivial behavior (default port of 51820, comma-separated address parsing, optional IPv6). A private item, so tolerable, but notable given its complexity. (deliberated: confirmed — Correction NEEDS_FIX is verified: line 196 uses unwrap_or((part, "24")) which assigns prefix 24 to IPv6 addresses missing a slash, while parse_cidr correctly branches on is_ipv4() to pick 32 vs 128. This is directly visible in the source code and is a genuine latent bug. Raising confidence to 85 since the inconsistency is unambiguous. Tests WEAK confirmed: missing coverage for dual-stack, missing PrivateKey, malformed port, default ListenPort. UNDOCUMENTED on a private function with non-trivial behavior (default port, comma-separated dual-stack parsing) is fair at reduced confidence.)

#### `parse_peer` (L228–L269)

- **Utility [USED]**: Helper called by Config::parse on lines 130 and 137 for each peer configuration
- **Duplication [UNIQUE]**: Parses [Peer] config section. RAG found parse_interface (0.706 score) with similar structure. However, different fields parsed (publickey, presharedkey, endpoint, allowedips, persistentkeepalive vs privatekey, listenport, address), different return type (PeerConfig vs InterfaceConfig), and different helper calls (parse_cidr vs prefix_to_netmask) indicate distinct semantic contracts despite structural parallelism.
- **Correction [OK]**: All fields are parsed and validated correctly. The standard-library SocketAddr parser handles both IPv4 and bracketed-IPv6 endpoint formats. AllowedIPs delegates correctly to parse_cidr.
- **Overengineering [LEAN]**: Parallel structure to parse_interface, appropriate for the distinct Peer key set. No shared logic that could be unified without losing clarity; each key arm is simple.
- **Tests [WEAK]**: `parse_full_config` covers endpoint, allowed_ips, and persistent_keepalive (present and absent). The `preshared_key` field is never exercised by any test. Error paths (missing PublicKey, malformed endpoint, malformed keepalive value) are not tested.
- **UNDOCUMENTED [UNDOCUMENTED]**: Private function with no `///` doc comment. Parses multiple peer fields with error handling; undocumented but private, so lower confidence applied per policy. (deliberated: confirmed — Tests WEAK confirmed: preshared_key is never exercised, error paths (missing PublicKey, malformed endpoint/keepalive) are untested. UNDOCUMENTED on a private function is fair at reduced confidence. The function has parallel structure to parse_interface and its behavior is reasonably inferable from context.)

#### `parse_cidr` (L271–L282)

- **Utility [USED]**: Helper called by parse_peer on line 261 to parse CIDR addresses in allowed_ips
- **Duplication [UNIQUE]**: Parses CIDR notation string to IpAddr and prefix. No similar functions found in RAG results
- **Correction [OK]**: Correctly infers the default prefix from the address family (32 for v4, 128 for v6) when no '/' is present. Parsing and error handling are sound.
- **Overengineering [LEAN]**: Twelve-line function: split on '/', parse addr, pick protocol-appropriate default prefix. Concise and does exactly one job.
- **Tests [WEAK]**: Exercised indirectly through `parse_full_config` (IPv4 CIDRs) and `parse_v6_allowed_ips` (IPv6 CIDR). The no-slash default-prefix path (bare IP address without '/') is never tested. Error paths (unparseable address, unparseable prefix) have no coverage.
- **UNDOCUMENTED [UNDOCUMENTED]**: Private function with no `///` doc comment. Contains notable implicit behavior (default prefix 32 for IPv4, 128 for IPv6 when omitted) that would benefit from documentation even internally. (deliberated: confirmed — Tests WEAK confirmed: the no-slash default-prefix path is never tested, nor are error paths (unparseable address/prefix). UNDOCUMENTED fair at reduced confidence for a private helper, though the inline comment documents the default-prefix behavior which partially mitigates.)

#### `prefix_to_netmask` (L284–L292)

- **Utility [USED]**: Public function called in parse_interface on line 200 to convert CIDR prefix to netmask
- **Duplication [UNIQUE]**: Converts IPv4 prefix length to netmask representation. No similar functions found in RAG results
- **Correction [OK]**: The three-way branch eliminates all overflow/underflow risk: prefix==0 returns 0.0.0.0 directly; prefix>=32 returns 255.255.255.255 directly; remaining 1-31 shift u32::MAX left by (32-prefix), which is always 1-31 bits—safe for u32. Ipv4Addr::from(u32) uses network (big-endian) byte order consistently with the arithmetic, producing correct netmasks (e.g., /24 → 0xFFFFFF00 → 255.255.255.0).
- **Overengineering [LEAN]**: Nine-line direct bit-shift computation with correct edge-case guards for 0 and ≥32. Minimal and correct.
- **Tests [WEAK]**: Only indirectly validated for prefix=24 via the `netmask` assertion in `parse_full_config`. The two explicit edge-case branches — prefix=0 (returns 0.0.0.0) and prefix>=32 (returns 255.255.255.255) — have no test coverage whatsoever.
- **UNDOCUMENTED [UNDOCUMENTED]**: Public function (`pub fn`) with no `///` doc comment at all. Converts a prefix length to an `Ipv4Addr` netmask with specific edge-case handling (prefix=0 → all zeros, prefix≥32 → all ones). Missing parameter description, return semantics, and `# Examples` for a public API. (deliberated: confirmed — Tests WEAK confirmed: only /24 is indirectly tested via parse_full_config; the explicit edge-case branches for prefix=0 and prefix>=32 have zero coverage. UNDOCUMENTED is a more significant finding here since this is a public function with specific edge-case behavior that callers need to understand. Confidence 85 is appropriate for a public API item.)

## Best Practices — 9/10

### Rules

| # | Rule | Status | Severity | Detail |
|---|------|--------|----------|--------|
| 4 | Derive common traits on public types | WARN | MEDIUM | `Config` and `InterfaceConfig` and `PeerConfig` only derive `Debug`, missing `Clone` and `PartialEq`. `CidrAddr` derives `Debug` and `Clone` but is missing `PartialEq`. As public config types used across crate boundaries, these traits are commonly expected for ergonomic usage (e.g., cloning a config for testing, comparing configs in integration tests). [L13, L19, L27, L36] |
| 9 | Documentation comments on public items | WARN | MEDIUM | The module has a `//!` doc comment. `CidrAddr` and its methods are documented. However, `Config`, `InterfaceConfig`, `PeerConfig`, `Config::from_file`, `Config::parse`, and `prefix_to_netmask` all lack `///` doc comments. These are key public-facing types and functions that consumers of the crate need to understand. [L13, L19, L27, L101, L107, L293] |

### Suggestions

- Add missing derived traits to public config structs for ergonomic cross-crate usage
  ```typescript
  // Before
  #[derive(Debug)]
  pub struct Config {
      pub interface: InterfaceConfig,
      pub peers: Vec<PeerConfig>,
  }
  // After
  #[derive(Debug, Clone, PartialEq)]
  pub struct Config {
      pub interface: InterfaceConfig,
      pub peers: Vec<PeerConfig>,
  }
  ```
- Add documentation comments to public API entry points
  ```typescript
  // Before
  pub fn from_file(path: &Path) -> io::Result<Self> {
      let contents = fs::read_to_string(path)?;
      Self::parse(&contents)
  }
  // After
  /// Load and parse a WireGuard configuration from a file on disk.
  ///
  /// # Errors
  /// Returns an `io::Error` if the file cannot be read or if the
  /// configuration is malformed.
  pub fn from_file(path: &Path) -> io::Result<Self> {
      let contents = fs::read_to_string(path)?;
      Self::parse(&contents)
  }
  ```
- Add PartialEq to CidrAddr to enable equality comparisons without manual field checks
  ```typescript
  // Before
  #[derive(Debug, Clone)]
  pub struct CidrAddr {
  // After
  #[derive(Debug, Clone, PartialEq, Eq)]
  pub struct CidrAddr {
  ```

## Actions

### Quick Wins

- **[correction · low · small]** Replace the hardcoded default prefix '24' with a type-aware default: parse the address first, then choose '32' for IPv4 and '128' for IPv6 when no prefix notation is present—mirroring the logic already used in parse_cidr. [L196]

### Hygiene

- **[documentation · low · trivial]** Complete JSDoc documentation for: `Config` (`Config`) [L13-L16]
- **[documentation · medium · trivial]** Add JSDoc documentation for exported symbol: `InterfaceConfig` (`InterfaceConfig`) [L19-L25]
- **[documentation · medium · trivial]** Add JSDoc documentation for exported symbol: `PeerConfig` (`PeerConfig`) [L28-L34]
- **[documentation · low · trivial]** Complete JSDoc documentation for: `CidrAddr` (`CidrAddr`) [L38-L41]
- **[documentation · medium · trivial]** Add JSDoc documentation for exported symbol: `prefix_to_netmask` (`prefix_to_netmask`) [L284-L292]
