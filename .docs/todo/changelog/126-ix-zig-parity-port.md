---
id: 126-ix-zig-parity-port
type: parent
protocol_version: "2.1"
spec_status: approved
category: feature
status: done
epic_boundary: "Add a sibling Zig implementation of IX that preserves the Rust command, output, test, and benchmark contracts before any promotion decision."
subtodo_start: .docs/todo/changelog/126a-ix-zig-parity-port.md
subtodo_final: .docs/todo/changelog/126h-ix-zig-parity-port.md
continuation: "After each completed execution unit: record evidence, set status done, move to .docs/todo/changelog/, continue immediately to next_todo. Never batch-archive."
source_message_policy: "Every lettered unit carries verbatim source-message proof."
---
# 126 IX Zig Parity Port

## Objective
Create the execution chain for a full Rust-to-Zig IX port rooted at `zig/`, adjacent to `crates/`. The Rust implementation remains the oracle until the Zig binary proves identical behavior and neutral-or-better performance through captured evidence.

## Scope

**In scope:**
- Add a Zig implementation lane under `zig/`.
- Port command ABI, expression planning, search, inspect, stats, tests, and benchmark comparison.
- Preserve Rust as parity oracle until all gates pass.

**Out of scope:**
- No Rust replacement before parity proof.
- No shell wrapper around Rust as Zig behavior.
- No speed claim without match-count parity and immutable binary evidence.

## Original User Message Capture

| Anchor ID | Information Piece | Verbatim Original Snippet | Required Coverage |
|-----------|-------------------|---------------------------|-------------------|
| U1 | objective | "We now need to rewrite everything over into Zig." | all units |
| U2 | execution order | "We will do one file at a time, convert it entirely into Zig." | 126b-126f |
| U3 | parity | "It needs to be identical. No shortcuts, no funny business." | 126a-126h |
| U4 | structure | "Just make sure it's next to crates." | 126b |
| U5 | proof | "create the binary for the Zig version and then test it compared to the Rust version" | 126g-126h |
| U6 | quality | "FinTech level engineering and code standards." | 126a-126h |

## Invariants
- I1: `crates/` remains the Rust oracle until Zig parity is proven.
- I2: `zig/` is the only Zig implementation root.
- I3: No shipped Zig path shells out to the Rust binary.
- I4: CLI text and JSON output schemas stay byte-contract compatible where order is specified.
- I5: Benchmark evidence separates canonical CLI, prepared replay, and live-loop ledgers.

## Chain Manifest

| File | Phase | Role | Status |
|------|-------|------|--------|
| `.docs/todo/changelog/126-ix-zig-parity-port.md` | parent | Chain root | archived |
| `.docs/todo/changelog/126a-ix-zig-parity-port.md` | a | Baseline and contract lock | archived |
| `.docs/todo/changelog/126b-ix-zig-parity-port.md` | b | Zig toolchain and root scaffold | archived |
| `.docs/todo/changelog/126c-ix-zig-parity-port.md` | c | CLI ABI port | archived |
| `.docs/todo/changelog/126d-ix-zig-parity-port.md` | d | Expression core port | archived |
| `.docs/todo/changelog/126e-ix-zig-parity-port.md` | e | Search engine port | archived |
| `.docs/todo/changelog/126f-ix-zig-parity-port.md` | f | Inspect and explain port | archived |
| `.docs/todo/changelog/126g-ix-zig-parity-port.md` | g | Test and benchmark parity | archived |
| `.docs/todo/changelog/126h-ix-zig-parity-port.md` | h | Verification and promotion gate | archived |

## Phase Plan

| Letter | Role | Patch Surface | Depends On |
|--------|------|---------------|------------|
| `a` | Baseline lock | Docs and oracle inventory only | none |
| `b` | Scaffold | `zig/build.zig`, `zig/src/main.zig`, toolchain docs | `a` |
| `c` | CLI ABI | `zig/src/cli/**` | `b` |
| `d` | Expression core | `zig/src/core/expr.zig` | `c` |
| `e` | Search engine | `zig/src/core/search.zig`, `stats.zig` | `d` |
| `f` | Inspect/explain | `zig/src/core/inspect.zig`, CLI explain rendering | `e` |
| `g` | Tests/bench | `zig/tests/**`, shared parity scripts | `f` |
| `h` | Closeout | docs, changelog, immutable artifacts | all prior |

## Closeout Evidence

- Zig build/test: `zig build test --summary all` passed 7/7; `zig build -Doptimize=ReleaseFast --summary all` passed 3/3.
- Runtime parity: `npm run test -- tests/zig/zig-runtime-parity.test.ts tests/zig/zig-port-contract.test.ts` passed 27/27.
- Full repo validation: `npm run test` passed 52 files and 1841 assertions.
- Rust oracle validation: `cargo test -p iex-cli --quiet` passed 39+39 tests; `cargo test -p iex-core --quiet` passed 86 tests.
- Focused parity artifact: `tools/reports/zig-parity/20260507-172055/zig-parity-artifact.json` records `parity_all_functional_equal=true`.
- Duplicate-risk review: dupe-audit over Zig args/output/search/inspect/regex reported 31 segments, 0 candidate pairs, 0 exact duplicates.
- Blocker recorded: `t3-tape validate` could not run because `t3-tape` is not on PATH.

## Next todo
`NONE`
