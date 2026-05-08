---
id: 127e-ix-zig-line-faithful-port
parent: 127-ix-zig-line-faithful-port
type: execution-unit
protocol_version: "2.1"
category: feature
phase: e
status: done
patch_scope: "Port Rust SearchStats schema, aggregation fields, and report serialization into Zig."
blast_radius: medium
blast_radius_justification: "Stats are user-visible, benchmark-visible, and used to verify the actual execution path."
idempotency_contract: idempotent
idempotency_notes: "Schema fields and renderers are deterministic."
acceptance: "Zig search JSON/report output includes Rust-compatible stats owners for discovery, execution mode, slowest file, matcher telemetry, and truncation."
exit_criterion: "npm test -- --run tests/zig/zig-runtime-parity.test.ts exits 0 with report schema assertions."
validation: "npm test -- --run tests/zig/zig-runtime-parity.test.ts"
expected_exit_code: 0
expected_output_pattern: "passed"
evidence: "Added zig/src/core/stats.zig as the typed Rust-compatible stats schema owner and wired SearchReport.stats from execution state before JSON/report rendering. Validated with Zig build test 10/10, ReleaseFast build 3/3, and npm runtime parity 25/25."
conflict_surface: ""
invariants: ["I2", "I3", "I4", "I5"]
source_message_anchor: "U1, U2, U3"
source_message_excerpt: "port to Zig needs to be on the level where it's addressing every single line | the detail, how it's done, why it's done that way, and convert it | We can start with the search, because obviously the search and the explain is needed for the benchmarking."
source_message_proof_obligation: "Expose enough telemetry to prove equal execution work before benchmarking."
entry_state: "127d archived with matcher strategy metadata."
rollback_surface: "Revert stats/report edits in zig/src/core and zig/src/cli/output.zig."
dependencies: "127d-ix-zig-line-faithful-port"
next_todo: .docs/todo/pending/127f-ix-zig-line-faithful-port.md
---
# 127e Stats Schema

## Execute Now
Port Rust search stats and report aggregation into Zig as benchmark evidence, not decoration.

## Patch Surface

**Adds/Modifies:**
- `zig/src/core/stats.zig` - stats schema if not already present.
- `zig/src/core/search.zig` - aggregation population.
- `zig/src/cli/output.zig` - JSON/report renderers.

## Next todo
`.docs/todo/pending/127f-ix-zig-line-faithful-port.md`
