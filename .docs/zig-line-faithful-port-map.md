# IX Zig Line-Faithful Port Map

Created UTC: 2026-05-08

## Boundary

Rust remains the oracle. Zig remains the sibling implementation lane under `zig/`. The corrective migration target is not "a Zig searcher"; it is the IX search/explain system translated mechanism-by-mechanism so benchmark results compare equal work.

## Immediate Frontier: Search And Explain

The first executable frontier is search and explain because those commands define the benchmarkable surface:

```text
argv -> canonical expression plan -> explainable matcher strategy -> discovery/file strategy -> scan outcome -> SearchStats -> output sentinel/report
```

This is the highest-risk gap in the current Zig lane. The archived `126` chain proved command existence and selected parity fixtures, while `.docs/zig-port-architecture.md` records that the Rust oracle still owns the actual cost model and search semantics.

## Rust Source Owners

| Rust owner | Current role | Zig target owner | Required translation |
|------------|--------------|------------------|----------------------|
| `crates/iex-core/src/expr.rs:17-45` | `LogicMode`, predicate variants, regex predicate storage, fast-path/decomposition metadata | `zig/src/core/expr.zig` | Preserve predicate taxonomy and add matcher metadata instead of bare `{ kind, value }`. |
| `crates/iex-core/src/expr.rs:47-76` | `RegexFastPath` variants: plain literal, ASCII casefold, fixed-width bytes regex, word-boundary literal, alternates | `zig/src/core/regex.zig` plus `expr.zig` | Port strategy classification and expose strategy identity for explain/stats. |
| `crates/iex-core/src/expr.rs:78-128` | Regex decomposition plan and candidate-line telemetry | `zig/src/core/regex.zig` | Port literal-gated decomposition and telemetry, including bailout counters. |
| `crates/iex-core/src/expr.rs:147-174` | ASCII casefold searcher, anchor plan, reject-fast constants/gate | `zig/src/core/regex.zig` | Port casefold search and reject-fast prefix gates as explicit mechanisms. |
| `crates/iex-core/src/expr.rs:373-740` | Parser, byte-mode support predicates, first-column, fast count, range fast count | `zig/src/core/expr.zig`, `regex.zig`, `search.zig` | Implement API-equivalent plan methods; explain and scan must consume the same plan object. |
| `crates/iex-core/src/stats.rs:6-200` | Search timing, slow file, concurrency, Linux strategy, decomposition, byte-shard, fallback stats, `SearchStats` | `zig/src/core/stats.zig`, `zig/src/core/search.zig`, `zig/src/cli/output.zig` | Add Zig stats owner with Rust-compatible field semantics before benchmark claims. |
| `crates/iex-core/src/engine.rs:30-58` | File-size, shard, queue, Linux dominant-file constants | `zig/src/core/search.zig` | Preserve thresholds or record platform-gated deviations. |
| `crates/iex-core/src/engine.rs:60-102` | `SearchConfig`, prepared options/targets, hit/report shapes | `zig/src/core/search.zig` | Preserve config/report ABI and prepared boundary identity. |
| `crates/iex-core/src/engine.rs:104-148` | Internal outcome/discovery/streaming outcome structs | `zig/src/core/search.zig` | Port scan outcome and discovery aggregation; do not collapse stats into global counters only. |
| `crates/iex-core/src/engine.rs:156-211` | Execution mode, concurrency planner, Linux dominant-file execution state | `zig/src/core/search.zig` | Port execution-mode state machine and strategy telemetry. |
| `crates/iex-core/src/engine.rs:213-304` | Linux strategy selectors and thread planner | `zig/src/core/search.zig` | Port selector math and thread caps; timing proof must expose chosen strategy. |
| `crates/iex-core/src/engine.rs:308-423` | Prepared target API and `run_search` entrypoints | `zig/src/core/search.zig` | Keep canonical CLI and prepared replay boundaries separate. |
| `crates/iex-core/src/engine.rs:436-615` | Materialized and materialized-prepared execution | `zig/src/core/search.zig` | Port materialization, aggregation, and prepared path validation. |
| `crates/iex-core/src/engine.rs:616-904` | Discovery, walk builder, root partition, finalization, normalization, duplicate/overlap pruning | `zig/src/core/search.zig` | Preserve file-set equality, hidden-root behavior, root dedupe, and overlap pruning. |
| `crates/iex-core/src/engine.rs:904-1004` | Linux dominant-file target and available-thread utilities | `zig/src/core/search.zig` | Port platform-specific eligibility and keep telemetry honest when inactive. |
| `crates/iex-core/src/engine.rs:1006-1226` | Streaming stats-only discovery/scan worker topology | `zig/src/core/search.zig` | Port streaming stats-only mode; do not use row-scan as a hidden substitute when Rust would stream. |
| `crates/iex-core/src/engine.rs:1227-1374` | Parallel scan, outcome merge, hit ordering, hit retention | `zig/src/core/search.zig` | Preserve count-vs-retained-hit semantics and global hit order. |
| `crates/iex-core/src/engine.rs:1375-1494` | File scan strategy, read policy, binary sniff, file outcomes | `zig/src/core/search.zig` | Preserve binary skip, empty file handling, tiny/small/large policies, and failure outcomes. |
| `crates/iex-core/src/engine.rs:1495-1784` | Loaded-byte line scan and match collection | `zig/src/core/search.zig` | Preserve LF/CRLF trimming, column semantics, byte/string matcher dispatch, and per-line preview behavior. |
| `crates/iex-core/src/engine.rs:1785-2077` | Fast-count, fallback byte-line profile, shard merge, unicode prefilter merge | `zig/src/core/search.zig`, `regex.zig` | Port count-only kernels and stats merge geometry. |
| `crates/iex-core/src/engine.rs:2108-2331` | Regex decomposition and dominant-file parallel plan builders | `zig/src/core/search.zig` | Port shard planning formulas and bailout semantics. |
| `crates/iex-core/src/engine.rs:2331-2388` | Chunk ranges, CR trimming, completed outcomes | `zig/src/core/search.zig` | Preserve exact range and outcome terminal states. |
| `crates/iex-core/src/engine.rs:2443+` | Rust adversarial tests for planner/discovery/search semantics | `zig/tests/**`, `tests/zig/**` | Mirror the invariant set in Zig-native and Rust-oracle runtime tests. |
| `crates/iex-cli/src/explain.rs` | Operator-facing explain command | `zig/src/main.zig`, `zig/src/cli/output.zig` | Explain must render the same plan used by search. |
| `crates/iex-cli/src/search.rs` | Operator-facing search/matches command | `zig/src/main.zig`, `zig/src/cli/args.zig`, `zig/src/cli/output.zig` | Preserve search/matches sentinel and JSON/report ABI. |

## Current Zig Gaps

| Zig file | Current state | Required correction |
|----------|---------------|---------------------|
| `zig/src/core/expr.zig` | 102-line parser with fixed predicate array and no matcher strategy metadata | Expand into Rust-equivalent plan shape with support predicates and plan methods. |
| `zig/src/core/regex.zig` | Hand-rolled subset matcher/count owner | Split support boundary from fast-path planner; add explicit strategy metadata and parity failures for unsupported Rust semantics. |
| `zig/src/core/search.zig` | Linear recursive walker plus streaming line scan | Port discovery normalization, outcome structs, stats schema, file strategies, execution modes, concurrency, and prepared boundary. |
| `zig/src/cli/output.zig` | Basic renderer for current report shape | Render Rust-compatible stats/report/explain fields once schema lands. |
| `tests/zig/zig-runtime-parity.test.ts` | Useful but still selected parity coverage | Add Rust-oracle adversarial cases that fail against shallow implementations. |
| `tools/scripts/zig-vs-rust-benchmark.mjs` | Equal binary harness exists | Gate benchmark interpretation on line-faithful strategy/count parity. |

## Complete Port Manifest

The complete Rust-to-Zig migration decomposes into these slices:

1. Search/explain line-faithful frontier: this `127` chain.
2. Inspect command full parity audit: verify all `crates/iex-cli/src/inspect.rs` and `crates/iex-core/src/inspect.rs` behavior against Zig after search/explain settles.
3. CLI compatibility grammar: ensure `crates/iex-cli/src/compat.rs` translator behavior has one Zig canonical lowering path and no second matcher semantics.
4. Agent/output sentinel hardening: port `crates/iex-cli/src/agent_output.rs` fields, error states, and report envelopes beyond search/explain.
5. Benchmark platform parity: port `crates/iex-bench/src/main.rs` concepts or keep Rust-only benchmark oracle with a Zig lane that reports exact prepared-vs-CLI ownership.
6. Test migration: mirror Rust unit and integration invariants into Zig-native tests where feasible, and keep JS runtime tests as cross-binary oracle tests.
7. Promotion gate: snapshot Rust `target/release/ix.exe`, compare Zig ReleaseFast against the snapshot and installed predecessor, require match-count parity and neutral-or-better performance before any active-loop change.

## Verification Doctrine

- A shallow pass is not a pass: tests must fail if Zig scans a smaller file set, truncates counts with `--max-hits`, ignores binary sniffing, omits stats owners, or renders explain from a different plan than search.
- Rust source is the proof oracle, not a style reference. Every Zig mechanism must name the Rust owner it translates.
- Unsupported capabilities are explicit parity blockers. They are never hidden by fallback text, silent row-scan substitution, or benchmark caveats.

## Prepared Replay Boundary

Rust owns prepared target discovery through `PreparedSearchTargets` and the `iex-bench` prepared replay lane. Zig does not yet expose a prepared target API; therefore Zig evidence is canonical CLI evidence only. Any future prepared benchmark must land as a separate Zig API and report ledger before its timings are compared with Rust prepared replay.
