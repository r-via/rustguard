# Documentation — Shard 4

## Findings

| File | Verdict | Documentation | Conf. | Details |
|------|---------|---------------|-------|---------|
| `rustguard-daemon/src/peer.rs` | NEEDS_REFACTOR | 1 | 90% | [details](../reviews/rustguard-daemon-src-peer.rev.md) |
| `rustguard-enroll/src/pool.rs` | NEEDS_REFACTOR | 1 | 90% | [details](../reviews/rustguard-enroll-src-pool.rev.md) |

## Symbol Details

### `rustguard-daemon/src/peer.rs`

| Symbol | Lines | Documentation | Conf. | Detail |
|--------|-------|---------------|-------|--------|
| `Peer` | L16–L26 | PARTIAL | 90% | [DEAD] Exported struct with zero documented importers per pre-computed analys... |

### `rustguard-enroll/src/pool.rs`

| Symbol | Lines | Documentation | Conf. | Detail |
|--------|-------|---------------|-------|--------|
| `IpPool` | L9–L20 | PARTIAL | 90% | [USED] Exported pub struct in library crate rustguard-enroll. Zero in-crate i... |

## Hygiene

- [ ] <!-- ACT-eacdc0-2 --> **[documentation · medium · trivial]** `rustguard-daemon/src/peer.rs`: Add JSDoc documentation for exported symbol: `Peer` (`Peer`) [L16-L26]
- [ ] <!-- ACT-92d12d-2 --> **[documentation · low · trivial]** `rustguard-enroll/src/pool.rs`: Complete JSDoc documentation for: `IpPool` (`IpPool`) [L9-L20]
