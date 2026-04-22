# Regex whole-buffer context-gate proof - 2026-04-22

## Objective
Repair the whole-buffer regex decomposition candidate without reverting immediately by removing false-positive candidate lines before line-boundary recovery.

## Runtime change
- kept the whole-buffer candidate-discovery shape live
- added decomposition telemetry to `SearchStats` and benchmark artifacts:
  - `eligible_files`
  - `counted_files`
  - `bailout_files`
  - `candidate_lines_checked`
  - `duplicate_candidate_hits_skipped`
  - `candidate_lines_matched`
- added a planner-owned local context gate for decomposed literals when the HIR proves `word + whitespace + literal + whitespace + word`
- aligned HIR analysis with byte-regex semantics via `unicode(false)` so decomposition analysis sees the same ASCII `\w` / `\s` classes the runtime executes

## Validation
- `cargo test -p iex-core`
- `cargo test`
- paused active `tools/scripts/bench-loop-suite.mjs` proof contention before benchmarking
- verified the pinned live loop binary hash matched the immutable baseline snapshot:
  - live: `tools/reports/candidate-compare/iex-cli-candidate-clean-20260422-000826.exe`
  - baseline: `tools/reports/candidate-compare/iex-cli-baseline-20260422-143320.exe`
  - sha256: `088DB52726BDF2E4C2DD8F8218740E171CB211B5EE666419D735B6ABE412CF1A`

## Immutable pair
- baseline: `tools/reports/candidate-compare/iex-cli-baseline-20260422-143320.exe`
- candidate: `tools/reports/candidate-compare/iex-cli-candidate-whole-buffer-context-gate-20260422-144932.exe`

## Proof runs
- target lane `re:\w+\s+Holmes\s+\w+` on `.refs/ripgrep/benchsuite/subtitles/en.sample.txt`
  - `runId=1776862194340-e2847140`: candidate `86.4008 ms`, baseline `89.0156 ms`, ratio `0.9706`
  - `runId=1776862197764-7afc8392`: candidate `88.4464 ms`, baseline `81.8208 ms`, ratio `1.0810`
  - `runId=1776862201228-8e1d5632`: candidate `102.3794 ms`, baseline `106.9780 ms`, ratio `0.9570`
- target lane telemetry
  - every proof run: `eligible_files=1`, `counted_files=1`, `bailout_files=0`
  - every proof run: `candidate_lines_checked=39`, `candidate_lines_matched=39`, `duplicate_candidate_hits_skipped=0`
- stress lane `re:\w{5}\s+\w{5}\s+\w{5}\s+\w{5}\s+\w{5}\s+\w{5}\s+\w{5}` on `.refs/ripgrep/benchsuite/subtitles/en.sample.txt`
  - `runId=1776862210163-9026c4fe`: candidate `262.8973 ms`, baseline `294.8113 ms`, ratio `0.8917`
  - `runId=1776862216979-a4d057ec`: candidate `274.6286 ms`, baseline `225.5590 ms`, ratio `1.2175`
  - `runId=1776862223492-77b37068`: candidate `227.1057 ms`, baseline `225.0191 ms`, ratio `1.0093`
- stress lane telemetry
  - every proof run: `eligible_files=0`, `counted_files=0`, `bailout_files=0`
  - every proof run: `candidate_lines_checked=0`, `candidate_lines_matched=0`

## Median-of-three read
- target lane:
  - candidate median `88.4464 ms`
  - baseline median `89.0156 ms`
  - net `0.9936x` = `0.64% faster`
- stress lane:
  - candidate median `262.8973 ms`
  - baseline median `225.5590 ms`
  - net `1.1655x` = `16.55% slower`

## Takeaway
- the local context gate removed the previously dominant false-positive work on the motivating lane:
  - prior telemetry: `candidate_lines_checked=169`
  - repaired telemetry: `candidate_lines_checked=39`
- the decomposition path now engages only where intended and stays fully inert on the unsupported no-literal comparator
- the motivating lane is no longer obviously wrong; it is slightly ahead on median-of-three with the loop paused
- the remaining blocker is not candidate-line explosion anymore; it is broader binary-level or proof-level instability outside the active decomposition lane

## Verdict
- keep the candidate live in workspace
- do not promote it yet
- next repair target:
  - isolate inactive-lane variance from decomposition logic with one more binary-layout-aware proof slice before deciding on landing or backout
