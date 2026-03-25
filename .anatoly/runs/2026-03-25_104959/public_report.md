<p align="center">
  <img src="https://raw.githubusercontent.com/r-via/anatoly/main/assets/imgs/logo.jpg" width="400" alt="Anatoly" />
</p>

# Anatoly Audit Report

> **41 files** reviewed in **79 min** — **$48.76** in AI analysis so you don't have to.
> Verdict: **CRITICAL** · 5 critical bugs found · 37 files with findings

## Findings Summary

| Category | High | Medium | Low | Total |
|----------|------|--------|-----|-------|
| Correction errors | 47 | 8 | 0 | 55 |
| Dead code | 1 | 1 | 0 | 2 |
| Duplicates | 10 | 27 | 0 | 37 |
| Test coverage gaps | 51 | 18 | 161 | 230 |
| Best practices | 20 | 3 | 0 | 23 |
| Documentation gaps | 28 | 4 | 117 | 149 |

## Critical Findings

- 🔴 **rustguard-core/src/replay.rs** `ReplayWindow` — The struct definition is correct, but shift_window (lines 109-117) contains a critical direction inversion in its sub...
- 🔴 **rustguard-enroll/src/server.rs** `run` — Two distinct correctness bugs: (1) In the MSG_TRANSPORT inbound handler, decrypt_buf is a fixed [0u8; 2048] stack arr...
- 🔴 **rustguard-kmod/src/replay.rs** `ReplayWindow` — Critical bug in shift_window (line 91): the bit-shift carry loop iterates `(0..BITMAP_LEN).rev()` — i.e., from index ...
- 🔴 **rustguard-tun/src/linux_mq.rs** `MultiQueueTun` — Two bugs in create(): (1) Line 107: libc::close(fd0) is called inside a map_err closure body without an inner unsafe{...
- 🔴 **rustguard-tun/src/xdp.rs** `XdpSocket` — Four correctness bugs across the impl block: (1) tx_send copies data.len() bytes into a UMEM frame without verifying ...
- 🟡 **rustguard-cli/src/main.rs** `cmd_serve` — Three flags that expect a following value use `args.get(i).cloned().unwrap_or_default()` at lines 82, 86, and 93. Whe...
- 🟡 **rustguard-cli/src/main.rs** `cmd_join` — The --token flag at line 163 uses the same `args.get(i).cloned().unwrap_or_default()` pattern. When --token is the la...
- 🟡 **rustguard-core/src/cookie.rs** `CookieState` — In process_reply, self.received is assigned only inside #[cfg(feature = "std")]. On no_std, after a successful cookie...
- 🟡 **rustguard-core/src/cookie.rs** `fill_random` — let _ = buf discards the mutable reference without writing any bytes; the buffer remains all zeros. Every call to ran...
- 🟡 **rustguard-core/src/cookie.rs** `fill_random` — let _ = buf discards the mutable reference without writing any bytes; the buffer remains all zeros. Every call to ran...

## Axes

| Axis | Health | Findings | Details |
|------|--------|----------|---------|
| Correction | `█████████░` 85% OK | 47 high · 8 med | [View →](./axes/correction/index.md) |
| Utility | `██████████` 99% used | 1 high · 1 med | [View →](./axes/utility/index.md) |
| Duplication | `█████████░` 90% unique | 10 high · 27 med | [View →](./axes/duplication/index.md) |
| Tests | `████░░░░░░` 39% covered | 51 high · 18 med · 161 low | [View →](./axes/tests/index.md) |
| Documentation | `███░░░░░░░` 30% documented | 28 high · 4 med · 117 low | [View →](./axes/documentation/index.md) |
| Best Practices | `███████░░░` avg 7.3 / 10 | 20 high · 3 med | [View →](./axes/best-practices/index.md) |

---

<details>
<summary><strong>Run Details</strong></summary>

Run `2026-03-25_104959` · 79.4 min · $48.76

| Axis | Calls | Duration | Cost | Tokens (in/out) |
|------|-------|----------|------|-----------------|
| utility | 38 | 38.9m | $2.05 | 349 / 319883 |
| duplication | 38 | 26.3m | $1.69 | 350 / 232156 |
| correction | 38 | 129.1m | $14.58 | 87 / 530237 |
| overengineering | 38 | 24.7m | $3.72 | 107 / 92575 |
| tests | 38 | 26.7m | $4.62 | 83 / 104317 |
| best_practices | 38 | 76.7m | $8.22 | 117 / 268783 |
| documentation | 38 | 40.4m | $6.61 | 83 / 173248 |

**Phase durations:**

| Phase | Duration |
|-------|----------|
| scan | 598ms |
| estimate | 212ms |
| triage | 3ms |
| rag-index | 801.3s |
| review | 3476.5s |
| internal-docs | 172.5s |
| report | 69ms |

</details>

<details>
<summary><strong>Methodology</strong></summary>

Each file is evaluated through 7 independent axis evaluators running in parallel.
Every symbol is analysed individually with a confidence score (0–100).
Findings below 30% confidence are discarded; those below 60% are excluded from verdicts.

**Verdicts:** CLEAN (no findings) · NEEDS_REFACTOR (confirmed findings) · CRITICAL (ERROR-level bugs)

**Severity:** High = ERROR or high-confidence NEEDS_FIX/DEAD/DUPLICATE · Medium = lower confidence or OVER · Low = minor

See each axis folder for detailed rating criteria.

</details>

*Generated: 2026-03-25T14:30:01.972Z*
