# Documentation

- **Files with findings:** 32
- **Actions:** 135

## Shards

- [ ] [shard.1.md](./shard.1.md) (10 files — 5 CRITICAL, 5 NEEDS_REFACTOR)
- [ ] [shard.2.md](./shard.2.md) (10 files — 10 NEEDS_REFACTOR)
- [ ] [shard.3.md](./shard.3.md) (10 files — 10 NEEDS_REFACTOR)
- [ ] [shard.4.md](./shard.4.md) (2 files — 2 NEEDS_REFACTOR)

## Verdict Distribution

| Verdict | Count | % |
|---------|-------|---|
| DOCUMENTED | 90 | 26% |
| PARTIAL | 123 | 36% |
| UNDOCUMENTED | 129 | 38% |

---

## Methodology

**Model:** haiku

Evaluates JSDoc coverage on exported symbols and optional /docs/ concept coverage.

### Rating Criteria

- **DOCUMENTED**: Symbol has a complete JSDoc comment covering description, params, and return type.
- **PARTIAL**: JSDoc exists but is incomplete (missing params, outdated description, or lacking return type).
- **UNDOCUMENTED**: No JSDoc documentation found for an exported symbol. Types and interfaces default to DOCUMENTED.

*Generated: 2026-03-25T14:30:01.979Z*
