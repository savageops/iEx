---
id: 127d-ix-zig-line-faithful-port
parent: 127-ix-zig-line-faithful-port
type: execution-unit
protocol_version: "2.1"
category: feature
phase: d
status: done
patch_scope: "Port Rust regex/literal fast-path classification, alternates, word-boundary, and reject-fast gates."
blast_radius: high
blast_radius_justification: "Matcher classification changes search correctness, stats-only counts, and benchmark cost shape."
idempotency_contract: idempotent
idempotency_notes: "Matcher planner changes are deterministic and guarded by parity tests."
acceptance: "Zig records the same tested matcher strategy classes as Rust and rejects unsupported regex semantics explicitly."
exit_criterion: "zig build test --summary all and npm test -- --run tests/zig/zig-runtime-parity.test.ts exit 0."
validation: "cd zig; zig build test --summary all; cd ..; npm test -- --run tests/zig/zig-runtime-parity.test.ts"
expected_exit_code: 0
expected_output_pattern: "passed"
evidence: "Expression plans now classify matcher strategies as internal metadata, including literal, alternates, word-boundary, ASCII casefold, fixed-width, and decomposition candidate classes. Search reports now derive linux_strategy.matcher_strategy_supported and outer_parallel_shard_safe from the plan instead of hardcoded true values. Validated with Zig build test 9/9, ReleaseFast build 3/3, and npm runtime parity 25/25."
conflict_surface: ""
invariants: ["I1", "I2", "I3", "I4"]
source_message_anchor: "U1, U2, U3"
source_message_excerpt: "port to Zig needs to be on the level where it's addressing every single line | the detail, how it's done, why it's done that way, and convert it | We can start with the search, because obviously the search and the explain is needed for the benchmarking."
source_message_proof_obligation: "Port the matcher mechanism instead of preserving a smaller line-regex shim."
entry_state: "127c archived with explain metadata wired to expression plans."
rollback_surface: "Revert zig/src/core/regex.zig and matcher metadata changes."
dependencies: "127c-ix-zig-line-faithful-port"
next_todo: .docs/todo/pending/127e-ix-zig-line-faithful-port.md
---
# 127d Matcher Planner

## Execute Now
Port the Rust matcher classification layer into Zig and surface unsupported gaps as explicit parity failures.

## Patch Surface

**Modifies:**
- `zig/src/core/regex.zig` - fast-path planner, count paths, regex support boundary.
- `zig/src/core/expr.zig` - matcher metadata if needed.

## Next todo
`.docs/todo/pending/127e-ix-zig-line-faithful-port.md`
