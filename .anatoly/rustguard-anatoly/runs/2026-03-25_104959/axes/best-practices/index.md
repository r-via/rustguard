# Best Practices

- **Files with findings:** 15
- **Actions:** 0

## Shards

- [ ] [shard.1.md](./shard.1.md) (10 files — 4 CRITICAL, 6 NEEDS_REFACTOR)
- [ ] [shard.2.md](./shard.2.md) (5 files — 5 NEEDS_REFACTOR)

## Verdict Distribution

| Metric | Value |
|--------|-------|
| Average score | 5.4/10 |
| Min / Max | 1.5 / 8.5 |

---

## Methodology

**Model:** sonnet

File-level evaluation against 17 TypeGuard v2 rules. Starts at 10/10, penalties subtracted per violation:

| # | Rule | Severity | Penalty |
|---|------|----------|---------|
| 1 | Strict mode (tsconfig strict: true) | HIGH | -1 pt |
| 2 | No `any` (explicit or implicit) | CRITICAL | -3 pts |
| 3 | Discriminated unions over type assertions | MEDIUM | -0.5 pt |
| 4 | Utility types (Pick, Omit, Partial, Record) | MEDIUM | -0.5 pt |
| 5 | Immutability (readonly, as const) | MEDIUM | -0.5 pt |
| 6 | Interface vs Type consistency | MEDIUM | -0.5 pt |
| 7 | File size < 300 lines | HIGH | -1 pt |
| 8 | ESLint compliance | HIGH | -1 pt |
| 9 | JSDoc on public exports | MEDIUM | -0.5 pt |
| 10 | Modern 2026 practices | MEDIUM | -0.5 pt |
| 11 | Import organization | MEDIUM | -0.5 pt |
| 12 | Async/Promises/Error handling | HIGH | -1 pt |
| 13 | Security (no secrets, eval, injection) | CRITICAL | -4 pts |
| 14 | Performance (no N+1, sync I/O) | MEDIUM | -0.5 pt |
| 15 | Testability (DI, low coupling) | MEDIUM | -0.5 pt |
| 16 | TypeScript 5.5+ features | MEDIUM | -0.5 pt |
| 17 | Context-adapted rules | MEDIUM | -0.5 pt |

*Generated: 2026-03-25T14:30:01.981Z*
