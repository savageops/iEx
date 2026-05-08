---
id: 127-ix-zig-line-faithful-port
type: parent
protocol_version: "2.1"
spec_status: approved
category: feature
status: done
epic_boundary: "Convert IX search and explain to a line-faithful Zig implementation while preserving a complete migration map for the remaining Rust surfaces."
subtodo_start: .docs/todo/changelog/127a-ix-zig-line-faithful-port.md
subtodo_final: .docs/todo/changelog/127m-ix-zig-line-faithful-port.md
continuation: "After each completed execution unit: record evidence, set status done, move to /todo/changelog/, continue immediately to next_todo. Never batch-archive. Never pause between units."
source_message_policy: "Every lettered unit MUST include source_message_anchor, source_message_excerpt, source_message_proof_obligation, and an Original User Message Proof section with verbatim snippets from the original user message."
---
# 127 IX Zig Line-Faithful Port

## Objective
Replace the current Zig search/explain scaffold with a mechanism-faithful port of the Rust IX search and explain pipeline. Rust remains the oracle until Zig can explain the same expression plans, execute the same search strategy classes, expose the same stats contracts, and pass adversarial parity tests on the operator-visible pipeline.

## Rationale
The prior `126` chain proved a working Zig lane and basic command/output parity, but `.docs/zig-port-architecture.md` records that Rust `engine.rs` and `expr.rs` still own the true search architecture. Benchmarking the current Zig lane as a language comparison would be invalid because it omits mmap/tiny-file strategy, ignore-style discovery, prepared targets, byte fast paths, regex decomposition, concurrency policy, and telemetry that affect cost and semantics.

## Scope

**In scope:**
- Produce a complete Rust-to-Zig migration map for the full IX port.
- Execute the immediate search/explain frontier with line-faithful Rust oracle coverage.
- Port expression representation, explain output, discovery, scan strategy, matcher fast paths, stats, concurrency planning, and prepared-search boundaries required for benchmark parity.
- Add strict Rust-oracle tests and benchmark proof gates for search/explain.

**Out of scope:**
- No promotion of Zig over Rust until full proof gates pass.
- No shelling out from Zig to Rust.
- No performance claim while search/explain mechanisms remain smaller than the Rust oracle.

## Source Language Anchors
- "port to Zig needs to be on the level where it's addressing every single line"
- "the detail, how it's done, why it's done that way, and convert it"
- "We can start with the search, because obviously the search and the explain is needed for the benchmarking."
- "Use the planning spec skill to map this in entirety, the complete port."
- "Create every single to-do slice, and then knock them all out to completion."

## Original User Message Capture

| Anchor ID | Information Piece | Verbatim Original Snippet | Required Coverage |
|-----------|-------------------|---------------------------|-------------------|
| U1 | line-faithful port | "port to Zig needs to be on the level where it's addressing every single line" | 127a-127m |
| U2 | mechanism rationale | "the detail, how it's done, why it's done that way, and convert it" | 127a-127k |
| U3 | first executable frontier | "We can start with the search, because obviously the search and the explain is needed for the benchmarking." | 127b-127l |
| U4 | complete migration map | "Use the planning spec skill to map this in entirety, the complete port." | 127a |
| U5 | autonomous execution | "Create every single to-do slice, and then knock them all out to completion." | 127a-127m |

## Source Message Coverage

| Unit | Source Anchor(s) | Slice Proof Obligation |
|------|------------------|------------------------|
| 127a | U1, U2, U4, U5 | Create the Rust-line ownership map and full port manifest before further implementation. |
| 127b | U1, U2, U3 | Port expression plan shape, parser contracts, and explainable predicate metadata. |
| 127c | U1, U2, U3 | Port explain JSON/text ownership so benchmarks describe the same plan classes. |
| 127d | U1, U2, U3 | Port Rust regex/literal fast-path classification and reject gates. |
| 127e | U1, U2, U3 | Port SearchStats schema and aggregation surfaces that users and reports observe. |
| 127f | U1, U2, U3 | Port root normalization, overlap pruning, hidden traversal, and discovery telemetry. |
| 127g | U1, U2, U3 | Port file strategy: binary sniff, tiny/small/full read, streaming, and line ownership. |
| 127h | U1, U2, U3 | Port scan kernels, max-hit retention semantics, and stats-only counting. |
| 127i | U1, U2, U3 | Port concurrency planner, shard geometry, and Linux dominant-file gate contracts. |
| 127j | U1, U2, U3 | Port prepared target/replay boundary without conflating it with CLI benchmarks. |
| 127k | U1, U2, U3 | Wire CLI search/explain through the canonical Zig engine only. |
| 127l | U1, U2, U3, U5 | Add adversarial Rust-oracle tests and equal-workload benchmark artifacts. |
| 127m | U1, U2, U4, U5 | Close the chain with docs, changelog, duplicate-risk review, and explicit residual gates. |

## Constraints

| Dimension | Constraint |
|-----------|------------|
| Category boundary | Only feature-port work for IX Zig search/explain plus complete migration planning. |
| Blast radius ceiling | high — search/explain are central operator and benchmark surfaces. |
| Structural boundary | `zig/` is the Zig implementation root; `crates/` remains the Rust oracle. |
| Dependency boundary | No Zig code may import, wrap, or shell to Rust for shipped behavior. |
| Rollback surface | Revert files listed in each unit; Rust artifacts remain untouched except oracle tests or docs. |
| Parallelism | Units are sequential because each search mechanism depends on the preceding expression and stats contracts. |

## Invariants
- I1: Rust `crates/iex-core/src/engine.rs`, `expr.rs`, `stats.rs`, and CLI explain/search files remain the behavioral oracle until final proof.
- I2: Zig search/explain must expose the same user-visible command pipeline: argv -> canonical expression plan -> execution report -> versioned sentinel.
- I3: Any missing Rust mechanism is recorded as a failing parity gate, not hidden behind fallback output.
- I4: Benchmark evidence is invalid unless match counts and report semantics are equal on the same workload.
- I5: Prepared replay evidence remains separate from canonical CLI evidence.

## Chain Manifest

| File | Phase | Role | Status |
|------|-------|------|--------|
| `.docs/todo/changelog/127-ix-zig-line-faithful-port.md` | parent | Chain root | archived |
| `.docs/todo/changelog/127a-ix-zig-line-faithful-port.md` | a | Baseline line map and full migration manifest | archived |
| `.docs/todo/changelog/127b-ix-zig-line-faithful-port.md` | b | Expression model and parser parity | archived |
| `.docs/todo/changelog/127c-ix-zig-line-faithful-port.md` | c | Explain output parity | archived |
| `.docs/todo/changelog/127d-ix-zig-line-faithful-port.md` | d | Regex and literal fast-path planner | archived |
| `.docs/todo/changelog/127e-ix-zig-line-faithful-port.md` | e | Stats schema and aggregation | archived |
| `.docs/todo/changelog/127f-ix-zig-line-faithful-port.md` | f | Root discovery and pruning | archived |
| `.docs/todo/changelog/127g-ix-zig-line-faithful-port.md` | g | File loading and line ownership | archived |
| `.docs/todo/changelog/127h-ix-zig-line-faithful-port.md` | h | Scan kernels and count semantics | archived |
| `.docs/todo/changelog/127i-ix-zig-line-faithful-port.md` | i | Concurrency and shard strategy | archived |
| `.docs/todo/changelog/127j-ix-zig-line-faithful-port.md` | j | Prepared target boundary | archived |
| `.docs/todo/changelog/127k-ix-zig-line-faithful-port.md` | k | CLI search/explain glue | archived |
| `.docs/todo/changelog/127l-ix-zig-line-faithful-port.md` | l | Rust-oracle tests and benchmarks | archived |
| `.docs/todo/changelog/127m-ix-zig-line-faithful-port.md` | m | Verification and closeout | archived |

## Phase Plan

| Letter | Role | Patch Surface | Depends On | Parallelizable |
|--------|------|---------------|------------|----------------|
| `a` | Baseline / contract lock | `.docs/zig-line-faithful-port-map.md` | none | No |
| `b` | Expression core | `zig/src/core/expr.zig`, expression tests | `a` | No |
| `c` | Explain renderer | `zig/src/main.zig`, `zig/src/cli/output.zig`, explain tests | `b` | No |
| `d` | Matcher planner | `zig/src/core/regex.zig`, new matcher metadata | `c` | No |
| `e` | Stats schema | `zig/src/core/stats.zig`, output/report renderers | `d` | No |
| `f` | Discovery | `zig/src/core/search.zig` root/discovery ownership | `e` | No |
| `g` | File strategy | `zig/src/core/search.zig` file loading and binary sniffing | `f` | No |
| `h` | Scan kernels | `zig/src/core/search.zig`, `zig/src/core/regex.zig` fast counts | `g` | No |
| `i` | Concurrency | Zig execution planner and shard telemetry | `h` | No |
| `j` | Prepared boundary | Zig prepared target/replay boundary and report ledger separation | `i` | No |
| `k` | CLI integration | command dispatch and report/sentinel consistency | `j` | No |
| `l` | Test/benchmark proof | `tests/zig/**`, `tools/scripts/**`, `tools/reports/**` | `k` | No |
| `m` | Closeout | `.docs/**`, dupe-risk review, final status | `l` | No |

## Validation Expectations
- `zig build test --summary all` exits `0`.
- `zig build -Doptimize=ReleaseFast --summary all` exits `0`.
- Rust-oracle JS parity tests cover explain JSON, search rows, stats reports, hidden/discovery behavior, max-hit truncation, and benchmark fixture counts.
- `cargo test -p iex-core --quiet` and focused CLI Rust tests remain green.
- A new `tools/reports/zig-vs-rust/127-line-faithful-search-explain/` artifact records equal match counts before any speed claim.

## Next todo
`NONE`
