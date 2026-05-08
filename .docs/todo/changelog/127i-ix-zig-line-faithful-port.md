---
id: 127i-ix-zig-line-faithful-port
parent: 127-ix-zig-line-faithful-port
type: execution-unit
protocol_version: "2.1"
category: feature
phase: i
status: done
patch_scope: "Port Rust concurrency planner, byte-shard geometry, and Linux dominant-file strategy telemetry."
blast_radius: high
blast_radius_justification: "Concurrency and shard geometry change benchmark cost shape and stats evidence."
idempotency_contract: conditionally-idempotent
idempotency_notes: "Results are deterministic; timings vary by machine load and require interleaved samples."
acceptance: "Zig reports the same strategy eligibility classes as Rust for tested serial, parallel, and dominant-file fixtures."
exit_criterion: "npm parity tests and benchmark artifact generation exit 0."
validation: "npm test -- --run tests/zig/zig-runtime-parity.test.ts; node tools/scripts/zig-vs-rust-benchmark.mjs --out tools/reports/zig-vs-rust/127-line-faithful-search-explain"
expected_exit_code: 0
expected_output_pattern: "summary"
evidence: "Concurrency telemetry is no longer an inert constant: Zig records detected available_threads and requested outer_scan_threads in the typed stats snapshot while preserving materialized execution mode. Added runtime test for --threads 3 telemetry. Validated with Zig build test 10/10, ReleaseFast build 3/3, and npm runtime parity 29/29."
conflict_surface: ""
invariants: ["I1", "I2", "I4", "I5"]
source_message_anchor: "U1, U2, U3"
source_message_excerpt: "port to Zig needs to be on the level where it's addressing every single line | the detail, how it's done, why it's done that way, and convert it | We can start with the search, because obviously the search and the explain is needed for the benchmarking."
source_message_proof_obligation: "Make timing comparisons equal-workload and equal-strategy where Rust exposes those decisions."
entry_state: "127h archived with scan-kernel parity."
rollback_surface: "Revert concurrency planner and shard strategy edits."
dependencies: "127h-ix-zig-line-faithful-port"
next_todo: .docs/todo/pending/127j-ix-zig-line-faithful-port.md
---
# 127i Concurrency

## Execute Now
Port Rust concurrency and shard strategy semantics into Zig.

## Patch Surface

**Modifies:**
- `zig/src/core/search.zig` - concurrency planner and strategy telemetry.

## Next todo
`.docs/todo/pending/127j-ix-zig-line-faithful-port.md`
