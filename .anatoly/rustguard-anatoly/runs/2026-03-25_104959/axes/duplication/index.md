# Duplication

- **Files with findings:** 7
- **Actions:** 37

## Shards

- [ ] [shard.1.md](./shard.1.md) (7 files — 2 CRITICAL, 5 NEEDS_REFACTOR)

## Verdict Distribution

| Verdict | Count | % |
|---------|-------|---|
| UNIQUE | 77 | 68% |
| DUPLICATE | 37 | 32% |

---

## Methodology

**Model:** haiku

Identifies code clones via RAG semantic vector search against the codebase index.

### Rating Criteria

- **UNIQUE**: No semantically similar function found, or similarity score < 0.75.
- **DUPLICATE**: Similarity score >= 0.85 with matching logic/behavior. The duplicate target file and symbol are reported.

*Generated: 2026-03-25T14:30:01.976Z*
