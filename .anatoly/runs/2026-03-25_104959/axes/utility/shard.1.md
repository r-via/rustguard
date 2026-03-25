# Utility — Shard 1

## Findings

| File | Verdict | Utility | Conf. | Details |
|------|---------|---------|-------|---------|
| `rustguard-kmod/src/noise.rs` | NEEDS_REFACTOR | 1 | 92% | [details](../reviews/rustguard-kmod-src-noise.rev.md) |
| `rustguard-core/src/timers.rs` | NEEDS_REFACTOR | 1 | 90% | [details](../reviews/rustguard-core-src-timers.rev.md) |
| `rustguard-kmod/src/timers.rs` | NEEDS_REFACTOR | 1 | 90% | [details](../reviews/rustguard-kmod-src-timers.rev.md) |

## Symbol Details

### `rustguard-kmod/src/noise.rs`

| Symbol | Lines | Utility | Conf. | Detail |
|--------|-------|---------|-------|--------|
| `random_bytes` | L173–L177 | DEAD | 75% | [DEAD] Non-exported generic function for random byte generation. Defined but ... |

### `rustguard-core/src/timers.rs`

| Symbol | Lines | Utility | Conf. | Detail |
|--------|-------|---------|-------|--------|
| `KEEPALIVE_TIMEOUT` | L51–L51 | LOW_VALUE | 88% | [DEAD] Exported constant with zero local or external usage. Not referenced in... |

### `rustguard-kmod/src/timers.rs`

| Symbol | Lines | Utility | Conf. | Detail |
|--------|-------|---------|-------|--------|
| `KEEPALIVE_TIMEOUT_NS` | L20–L20 | DEAD | 88% | [DEAD] Constant is defined but never referenced in the file. Not exported, ze... |

## Quick Wins

- [ ] <!-- ACT-050448-4 --> **[utility · high · trivial]** `rustguard-kmod/src/timers.rs`: Remove dead code: `KEEPALIVE_TIMEOUT_NS` is exported but unused (`KEEPALIVE_TIMEOUT_NS`) [L20-L20]

## Refactors

- [ ] <!-- ACT-58e5ff-16 --> **[utility · medium · trivial]** `rustguard-kmod/src/noise.rs`: Remove dead code: `random_bytes` is exported but unused (`random_bytes`) [L173-L177]
