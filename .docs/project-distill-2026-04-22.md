# iEx Project Distill - 2026-04-22

> Superseded current snapshot: see [project-distill-2026-04-27.md](./project-distill-2026-04-27.md). This file is preserved as historical 2026-04-22 proof lineage and should not be treated as the current operator state.

## Mission Lock
- iEx v2 remains a Rust search engine plus benchmark platform whose headline target is transparent proof that it can become at least `50%` faster than ripgrep on agreed workloads.
- The repo doctrine is unchanged: no fake compatibility, no parallel ownership, no performance claims without exact binary-vs-binary proof, and no live-loop promotion without immutable evidence.
- The current project has two equally canonical products: the search engine and the benchmark-governance system that proves whether the engine is actually better.

## What Changed In This Stretch
- The regex-next lane was re-priced around proof instead of aspiration. The naive line-candidate loop was proved, regressed, and rejected. The next slice moved to whole-buffer candidate discovery plus line-boundary recovery.
- The workspace candidate stayed live for repair instead of being reverted immediately. That produced a second repair pass, then a planner-owned local context gate for the `word + whitespace + literal + whitespace + word` family.
- Runtime stats and benchmark artifacts now expose `regex_decomposition` / `regexDecomposition` attribution, so proof can distinguish active wins, bailouts, and fully inert unsupported lanes.
- The docs are now aligned to the actual engine contract: Rust `regex` plus `regex::bytes`, not a PCRE2 surface, and a live-loop binary can differ from the current workspace candidate.

## Highest-Value Retained Wins
- The current decomposition shape is directionally correct: whole-buffer literal discovery comes first, line-boundary recovery happens only around surviving candidates, and the full bytes regex confirms only those lines.
- The new local context gate removed the main false-positive cost on the motivating Holmes lane. Candidate-line work dropped from `169` to `39`, which matches the true line-hit count in the proof corpus.
- `SearchStats.regex_decomposition` is append-only and zero-safe. It records `eligible_files`, `counted_files`, `bailout_files`, `candidate_lines_checked`, `duplicate_candidate_hits_skipped`, and `candidate_lines_matched`.
- `tools/scripts/lib/benchmark-runner.mjs` now propagates that runtime block into benchmark artifacts as `regexDecomposition`, so dashboard and proof artifacts can attribute decomposition behavior without a second telemetry channel.

## Canonical User-Facing Surface
- `iex search <expr> [PATH]...` and `iex explain <expr>` remain the canonical CLI contract.
- The rg-style ingress layer is still intentionally narrow. It accepts a safe subset of ripgrep-shaped invocations and lowers them into the native search path instead of pretending to implement full ripgrep semantics.
- Unsupported rg-only flags remain guided non-zero errors.
- Regex syntax currently follows Rust `regex` semantics. Look-around and backreferences are not part of the shipped engine contract.

## Current Architecture Truth
- The canonical regex engines are `regex::Regex` and `regex::bytes::Regex`.
- On stats-only byte-mode searches, the loaded-byte path is:
  1. shard-safe fast count when the current plan is range-safe
  2. regex decomposition when one strong required literal exists and no narrower fast path already owns the pattern
  3. the normal byte-mode line loop otherwise
- Decomposition is planner-owned and bounded. It can stay inert, count exactly, or bail out when candidate-line volume reaches the ceiling.
- Current benchmark proof artifacts can therefore show three distinct states for decomposition:
  - active and counted
  - active but bailed out
  - fully inactive

## Current State
- The active live loop is still pinned to immutable snapshot `tools/reports/candidate-compare/iex-cli-candidate-clean-20260422-000826.exe`, as confirmed by `tools/reports/latest.json` and the running `bench-loop-suite.mjs` process.
- The repaired decomposition candidate is live in the workspace and archived as immutable proof snapshot `tools/reports/candidate-compare/iex-cli-candidate-whole-buffer-context-gate-20260422-144932.exe`, but it has not been promoted into the active loop.
- The motivating proof lane `re:\w+\s+Holmes\s+\w+` is slightly ahead on median-of-three, but the unsupported no-literal comparator remains mixed. That means the regex-parent is promising, not promotable.
- The next blocker is no longer candidate selectivity on the motivating lane. It is isolation of inactive-lane binary variance before any promotion decision.

## Rejected Slice
- The per-line regex line-candidate prefilter is rejected. It added line-loop tax up front, regressed the exact subtitle workload it was supposed to help, and is not the correct parent for this repo.

## Cold-Start Structural Snapshot
### U1: Regex Decomposition Stats-Only Flow
<re: expression>
  |- guard: no narrower fast path + strongest required literal + stats-only byte mode
  |- planner: optional local context gate from HIR
  v
<Whole-buffer candidate discovery>
  |- `memmem` finder over the full byte slice
  |- duplicate candidate hits on the same line are skipped
  |- bailout when candidate-line ceiling is reached
  v
<Line-boundary recovery>
  |- recover `line_start` / `line_end` around the candidate offset
  |- trim trailing carriage return
  v
<Full bytes-regex confirm>
  |- exact `regex::bytes` match on the recovered line only
  |- mutate `regex_decomposition` stats
  v
<Terminal state>
  |- counted
  |- bailout
  |- inert fallback to canonical byte-mode loop

### U4: Promotion State Machine
<Pinned live snapshot>
  | candidate built and compared
  v
<Candidate under proof>
  |- invariant: immutable baseline, same workload, attributed telemetry
  | proof fails or stays mixed
  v
<Workspace candidate only>
  |- effect: live loop remains on the pinned snapshot

<Pinned live snapshot>
  | full-suite proof clears
  v
<Promoted immutable snapshot>
  |- effect: loop repointed and `latest.json` confirms the new binary path

## Fresh Chat Anchors
- No PCRE2 claims. The current regex contract is Rust regex plus byte regex.
- `regexDecomposition` is now a first-class benchmark artifact field.
- The live-loop binary and the workspace candidate are separate operator concepts and must stay separate in docs.
- The line-candidate parent is dead. The byte-offset-first decomposition parent is the live lane.

## Sources
- `README.md`
- `.docs/iex-v2-crown-jewel.md`
- `.docs/recon/regex-line-candidate-proof-2026-04-22.md`
- `.docs/recon/regex-whole-buffer-line-recovery-proof-2026-04-22.md`
- `.docs/recon/regex-whole-buffer-line-recovery-repair-pass-2026-04-22.md`
- `.docs/recon/regex-whole-buffer-context-gate-proof-2026-04-22.md`
- `crates/iex-core/src/expr.rs`
- `crates/iex-core/src/engine.rs`
- `crates/iex-core/src/stats.rs`
- `tools/scripts/lib/benchmark-runner.mjs`
- `tools/reports/latest.json`
