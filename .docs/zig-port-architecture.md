# IX Zig Port Architecture

Created UTC: 2026-05-06 23:04:53

## Contract

The Zig port is a sibling implementation lane rooted at `zig/`, adjacent to `crates/`. Rust remains the behavioral oracle until Zig proves command, output, match-count, test, and benchmark parity. No Rust module is replaced by Zig until the Zig binary is measured against the current canonical `target/release/ix.exe` and the installed predecessor binary with immutable snapshots.

## Current Boundary

- The pinned compiler is available at `C:\Users\Savage\.local\zig\zig-x86_64-windows-0.16.0\zig.exe`; `zig build`, `zig build test`, and `zig build -Doptimize=ReleaseFast` are validated through that path.
- The current Zig binary is `zig/zig-out/bin/ix-zig.exe`. Rust `target/release/ix.exe` remains the behavioral oracle and canonical binary.
- The operator ABI is ported for `ix search`, `ix matches`, `ix inspect`, `ix explain`, top-level help, and subcommand help.
- Default `ix search` emits Rust-compatible literal hit rows plus exactly one `ix.result.v1` sentinel in text mode.
- `ix search --json`, `ix matches --json`, and `--emit-report` emit Rust-compatible report envelopes for the tested schema surface.
- `ix matches` uses the same Zig search execution path and suppresses the terminal sentinel unless JSON/report output is explicitly requested.
- `ix inspect` is read-only and now covers bounded file windows, `--skip`, `--limit`, `--start-line`, `--end-line`, `--range`, `--total-count`, `--head`, `--all`, grouped/records/json formats, multipath file windows, asymmetric `-B/-A` match context, and exact subcommand help.
- The rg-shaped top-level translator lowers into canonical search arguments and must not become a second search engine.
- Regex execution is owned in Zig for the currently tested IX operator set: literal regex, top-level alternation, grouped alternation with suffix consumption, character classes/ranges, digit classes, word boundaries, line anchors, exact `{N}` repetition, `?`, `*`, `+`, `\w`, `\s`, inline `(?i)`, escaped literals, `\xNN`, and ASCII ignore-case through the rg-shaped translator.
- StringZilla v4.6.0 SIMD search kernels are linked via C FFI (`zig/src/sz_shim.c` → `zig/src/core/sz.zig`). The shim compiles with `SZ_DYNAMIC_DISPATCH=0` and `-mavx2`, selecting the Haswell AVX2 backend at compile time (32 bytes/cycle). Three hot paths in `search.zig` use `sz.indexOf` and `sz.indexOfByte` instead of `std.mem.indexOf`/`indexOfScalar` (1 byte/cycle). This closes the "no memmem kernels" gap for literal substring search. Benchmark evidence: first Zig win over Rust on real-codebase-literal (0.93x ratio, Zig 7% faster). The `.refs/stringzilla/include` directory is the header source; no StringZilla source is vendored into `zig/src/`.
- Exact trigram acceleration is now represented by a pure admission layer at `zig/src/core/trigram.zig`. The current slice extracts mandatory byte trigrams for literals, prefix/suffix predicates, and conservative regex prefixes such as `re:auth_\d+`; it rejects regexes without mandatory exact bytes and casefold regexes until folded-byte semantics are explicit. This is not runtime pruning yet and does not create a separate query class.
- Current proof is candidate-lane proof, not promotion proof. Focused parity and microbench evidence lives at `tools/reports/zig-parity/20260507-172055/zig-parity-artifact.json`; Rust-vs-Zig equal-field benchmark probes live under `tools/reports/zig-vs-rust/`. Broad canonical benchmark promotion remains blocked until search/explain are line-faithful to the Rust engine.
- Deep benchmark review found the earlier Zig search lane was a scaffold, not a full Rust search port. Fixed parity defects: explicit hidden roots are scanned like Rust, files larger than 1 GiB no longer fail with `StreamTooLong`, `--max-hits` limits retained rows without truncating match counts, and stats-only regex counting now distinguishes row-output semantics from Rust fast-count semantics for tested occurrence and surrounding-word decomposition cases.
- Corrective chain `127` added a line-faithful source map and narrowed the first executable frontier to search/explain. Closed parity gaps in expression tokenization/plan metadata, explain JSON escaping, typed stats ownership, duplicate/overlap root pruning telemetry, binary sniff skip semantics, literal occurrence count tests, requested-thread telemetry, and rg-shaped JSON ingress. Remaining search/explain parity debt is still architectural: Zig does not yet implement Rust's mmap/tiny-file policy, prepared targets, streaming stats-only worker topology, Rayon/crossbeam parallelism, memmem/Aho-Corasick kernels, full regex HIR classification, reject-fast gates, Unicode casefold prefilters, byte-shard fast-count kernels, Linux dominant-file execution, or Rust regex crate semantics.
- Current canonical smoke evidence is `tools/reports/zig-vs-rust/127-line-faithful-search-explain/zig-vs-rust-benchmark-smoke.json`: 3/3 valid workload parity, 3 Rust wins, 0 Zig wins. The broad eight-case benchmark attempt timed out after 304 seconds and left child `ix-zig.exe` searches that were terminated before final ReleaseFast validation.
- Equal-ground search benchmark refresh on 2026-05-08 lives at `tools/reports/zig-vs-rust/20260508-search-equal-ground/zig-vs-rust-limit5-samples2.json`. It ran canonical CLI stats-only search with 5 English suite rows, 2 measured samples, and 1 warmup. All 5 rows had match parity and workload parity; Rust won 5/5. Median Zig/Rust wall-time ratio was 34.2145x, with the worst sampled row at 261.6703x on English alternates. This is valid as a parity-gated search comparison for those rows only, not a full language-vs-language promotion artifact.

## Architecture Shape

```text
iEx/
|- crates/                  // Rust oracle until parity is proven
|- zig/                     // Zig implementation lane
|  |- build.zig             // pinned build graph and test steps
|  |- build.zig.zon         // dependency lock surface when introduced
|  |- src/
|  |  |- main.zig           // CLI entry and command dispatch
|  |  |- cli/               // argv grammar, compat lowering, renderers
|  |  |- core/              // expression, search, regex byte semantics, trigram admission, inspect, stats
|  |  `- bench/             // Rust/Zig parity probes only when needed
|  `- tests/                // Zig-native unit and golden-output tests
|- tests/                   // shared JS parity and benchmark harness
`- tools/                   // immutable binary snapshots and reports
```

## User-Exposed Pipeline

The Zig port must preserve the workflow that operators, dashboards, transcripts, and agents see before it optimizes private internals:

```text
<argv> -> <canonical command> -> <expression plan> -> <execution report> -> <versioned sentinel> -> <next action>
```

- Search owns `ix.result.v1` with expression, files scanned, match count, truncation state, and schema.
- Matches uses the same search execution path but does not print a terminal result sentinel.
- Inspect owns the Rust-compatible `ix.inspect.file` header shape and `ix.next.v1` continuation metadata so file navigation remains scriptable.
- Errors own `ix.error.v1` and must expose code-visible failure state, not hidden reasoning text.
- Tests must assert these observable rows because this is the pipeline the user actually experiences.

## Port Order

1. Toolchain and scaffold: pin Zig 0.16.x, create `zig/build.zig`, build one no-op executable only after toolchain verification. The repo-side scaffold exists; compiler validation is blocked until `zig version` returns `0.16.0`.
2. CLI ABI: port argument taxonomy and sentinel rendering before engine internals so user-visible contracts freeze early.
3. Expression core: port parser and planner with golden `ix explain` JSON parity against Rust.
4. Search engine: port discovery, scan strategy, literal/prefix/suffix execution, regex execution, report emission, and measured serial telemetry.
5. Inspection: port bounded file windows, match context, JSON output, continuation hints, and multipath file-window output.
6. Test and benchmark parity: maintain Rust-vs-Zig parity tests for the operator-visible pipeline before any speed claim.
7. Promotion gate: Zig can become a replacement candidate only after broad canonical benchmark evidence shows match-count parity and neutral-or-better results against the current Rust snapshot.

## Design Invariants

- I1: Zig does not import, shell out to, or wrap the Rust binary for shipped behavior.
- I2: Rust and Zig use the same command grammar and the same observable output schema.
- I3: Any unsupported Rust capability in Zig is reported as an explicit missing parity gate, never hidden by fallback behavior.
- I4: Performance claims require match-count parity and immutable binary evidence.
- I5: Prepared replay, canonical CLI benchmark, and live-loop reports remain separate ledgers.

## External Source Anchors

- Zig 0.16.0 is the latest stable release listed by the official Zig download page as of this planning pass.
- The official Zig build-system documentation models projects through `build.zig`, build DAG steps, `b.addExecutable`, `b.installArtifact`, standard target/optimization options, tests, and generated artifacts.
