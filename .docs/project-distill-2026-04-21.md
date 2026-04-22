# iEx Project Distill - 2026-04-21

Superseded by [project-distill-2026-04-22.md](./project-distill-2026-04-22.md) for the current cold-start state.

## Mission Lock
- iEx v2 is a clean Rust search engine and benchmark platform whose crown-jewel target is sustained, transparent proof that it is at least `50%` faster than ripgrep on agreed workloads.
- The repo doctrine is strict: no fallback hacks, no parallel systems, no vague performance claims, and no candidate promotion without direct proof against the current canonical or live binary.
- The project now has two coupled products: the engine itself and the benchmark-governance system that proves whether the engine is actually improving.

## What Changed In This Stretch
- The project stopped treating matcher speed alone as the main story and started reading the suite as a systems diagnosis surface: execution-path arbitration, literal fast paths, tail debt, hotspot dominance, p95 drift, and live loop behavior.
- The benchmark dashboard became an operator surface instead of a pile of one-off output. `npm run bench:loop` feeds live telemetry, `npm run bench:report` anchors raw provenance, and pinned snapshots keep those two surfaces tied to exact binaries.
- Environment handling was made explicit. "Local" now has separate proof meanings for native Windows, WSL-installed Linux `iex`, and the repo-built candidate.

## Highest-Value Retained Wins
- Directory-root `--stats-only` streaming dispatch was restored for any directory-root stats-only search, not just multipath runs. That change flipped the heavy Linux directory lanes back onto the cheaper streaming path and improved the installed native `iex` comparison by `17.74%` overall (`overallRatio=0.8225935725710628`).
- `expr.rs` gained the narrow `FixedWidthBytesRegex` fast path for literal-equivalent non-ASCII `re:(?i)...` patterns whose case variants keep stable byte width. That let `suite-ru-literal-casei` use shard-safe range counting instead of the generic unshardable regex fallback.
- The resulting Russian-lane repair was decisive: targeted replay improved `suite-ru-literal-casei` by `51.89%`, and the refreshed full active-suite compare versus the installed WSL `iex` improved overall direct time by `22.05%` with `11/12` profile wins.
- The live dashboard now measures current iEx against both ripgrep and a previous pinned iEx snapshot via `competitors.iex_previous`, which makes self-regression visible without replacing the external challenger model.
- `ugrep` was intentionally removed from live tracking, and `tools/scripts/dashboard-server.mjs` now emits explicit `no-store` cache headers so stale dashboard HTML does not mask current live wiring.
- A WSL-native install at `~/.local/bin/iex` now exists as a stable installed baseline for direct repo-candidate versus installed-binary proof.

## Canonical User-Facing Surface
- `iex search` and `iex explain` remain the canonical CLI contract.
- The rg-style ingress layer is intentionally narrow: it lowers a safe subset of ripgrep-shaped invocations into the native iEx search path so old `rg`-shaped calls stop failing immediately without rebuilding ripgrep semantics.
- Unsupported rg-only flags remain guided non-zero errors, not fake success or hidden fallback behavior.

## Rejected Slice
- Re-enabling outer-parallel inner sharding for word-boundary fast counts was tested, then rejected. It produced multi-second giant-file tail spikes on AMD header workloads and did not survive the proof gate.

## Current State
- iEx is materially stronger than the older installed native and WSL baselines on the active suite, and the benchmark governance model is much more trustworthy than it was at the start of this wave.
- The repo now has immutable promotion discipline: snapshot the current canonical or live binary, compare the candidate on the exact workload, and only repoint the loop to a pinned snapshot after suite-level proof.
- The project is healthier, but the headline target is still open. The remaining strategic question is how to push from broad parity wins toward sustained `>=50%` faster than ripgrep across the active suite, including p95 and giant-tail behavior.

## Cold-Start Structural Snapshot
### U1: Benchmark Governance Flow
<Candidate edit or rebuild>
  └─ snapshot current canonical or live binary
  └─ run candidate-vs-current proof on the exact workload
        │ proof passes
        ▼
  <Pinned promoted snapshot>
  ├─ restart live loop → `--iex-binary <snapshot>`
  ├─ optional self-history lane → `--previous-iex-binary <snapshot>`
  ├─ live artifacts → `tools/reports/live-metrics.jsonl`, `tools/reports/latest.json`
  └─ observable state → exact active binary path is recoverable from repo artifacts

### U4: Promotion State Machine
<Current snapshot pinned>
  │ candidate built and compared
  ▼
<Candidate under proof>
  ├─ invariant: immutable baseline, same workload, reproducible artifact
  │ proof fails
  ▼
<Rejected candidate>
  └─ effect: no live-loop promotion, changelog records the failure

<Current snapshot pinned>
  │ proof passes
  ▼
<Promoted pinned snapshot>
  └─ effect: loop repointed and telemetry confirms the new path

## Fresh Chat Anchors
- Mission: beat ripgrep with proof, not vibes.
- Big wins kept: directory-root stats-only streaming dispatch; fixed-width non-ASCII case-insensitive fast path.
- Big experiment rejected: outer-parallel inner sharding for word-boundary fast counts.
- Important commits: `b3c96c1` for the performance slice, `0c6435c` for rg-style CLI compatibility.
- Live dashboard model: compare the current candidate against ripgrep and previous-build iEx.
- Promotion rule: snapshot, compare candidate versus current canonical or live binary, then promote a pinned snapshot only if the proof wins.

## Sources
- `AGENTS.md`
- `.docs/todo/changelog/_log.md`
- `git log --oneline --decorate -n 12`
- 2026-04-21 project distill handoff turn
- memory registry entries for the iEx benchmark, install, and compat slices
