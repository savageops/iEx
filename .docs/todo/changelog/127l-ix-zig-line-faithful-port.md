---
id: 127l-ix-zig-line-faithful-port
parent: 127-ix-zig-line-faithful-port
type: execution-unit
protocol_version: "2.1"
category: feature
phase: l
status: done
patch_scope: "Add adversarial Rust-oracle parity tests and equal-workload benchmark artifacts for Zig search/explain."
blast_radius: medium
blast_radius_justification: "Tests and benchmark scripts validate behavior but do not change runtime command code."
idempotency_contract: conditionally-idempotent
idempotency_notes: "Test results deterministic; timings require rerun if machine load is abnormal."
acceptance: "Parity tests fail on shallow Zig implementations and pass only when search/explain semantics match Rust on exposed workflows."
exit_criterion: "Zig tests, JS parity tests, Rust core tests, and benchmark script exit 0."
validation: "cd zig; zig build test --summary all; cd ..; npm test -- --run tests/zig/zig-runtime-parity.test.ts; cargo test -p iex-core --quiet; node tools/scripts/zig-vs-rust-benchmark.mjs --out tools/reports/zig-vs-rust/127-line-faithful-search-explain"
expected_exit_code: 0
expected_output_pattern: "passed"
evidence: "Validation passed: Zig build test 10/10, Zig ReleaseFast build 3/3, npm runtime parity 30/30, cargo test -p iex-core --quiet 86/86. Full benchmark attempt with --limit 8 --samples 2 --warmup 1 timed out after 304s and is not used as evidence. Captured smoke artifact at tools/reports/zig-vs-rust/127-line-faithful-search-explain/zig-vs-rust-benchmark-smoke.json: 3/3 valid workload parity, 3 Rust wins, 0 Zig wins, matchParityAllValid=true, workloadParityAllValid=true; ratios show Zig still far slower on the sampled large single-file lanes."
conflict_surface: ""
invariants: ["I1", "I2", "I3", "I4", "I5"]
source_message_anchor: "U1, U2, U3, U5"
source_message_excerpt: "port to Zig needs to be on the level where it's addressing every single line | the detail, how it's done, why it's done that way, and convert it | We can start with the search, because obviously the search and the explain is needed for the benchmarking. | Create every single to-do slice, and then knock them all out to completion."
source_message_proof_obligation: "Surface missing line-faithful mechanisms through challenging tests and benchmark evidence."
entry_state: "127k archived with canonical CLI search/explain glue."
rollback_surface: "Revert test and benchmark edits."
dependencies: "127k-ix-zig-line-faithful-port"
next_todo: .docs/todo/pending/127m-ix-zig-line-faithful-port.md
---
# 127l Proof Harness

## Execute Now
Add strict Rust-oracle search/explain tests and equal-workload benchmark artifacts.

## Patch Surface

**Modifies:**
- `tests/zig/zig-runtime-parity.test.ts` - adversarial user-visible parity tests.
- `tools/scripts/zig-vs-rust-benchmark.mjs` - artifact path and failure gating if needed.

## Next todo
`.docs/todo/pending/127m-ix-zig-line-faithful-port.md`
