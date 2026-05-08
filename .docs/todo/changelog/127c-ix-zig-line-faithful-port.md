---
id: 127c-ix-zig-line-faithful-port
parent: 127-ix-zig-line-faithful-port
type: execution-unit
protocol_version: "2.1"
category: feature
phase: c
status: done
patch_scope: "Port explain output from Rust plan metadata to Zig without private fallback formatting."
blast_radius: medium
blast_radius_justification: "Explain is an operator-visible benchmark prerequisite and validates expression-planner identity."
idempotency_contract: idempotent
idempotency_notes: "Renderer changes are deterministic and fixture-driven."
acceptance: "Zig explain emits Rust-compatible predicate and mode metadata for tested expression classes."
exit_criterion: "npm test -- --run tests/zig/zig-runtime-parity.test.ts exits 0 with explain parity assertions."
validation: "npm test -- --run tests/zig/zig-runtime-parity.test.ts"
expected_exit_code: 0
expected_output_pattern: "passed"
evidence: "Fixed Zig explain JSON escaping by routing source and predicate values through writeJsonString. Validated Rust and Zig explain parity for lit:timeout && re:\\bERROR\\b; outputs matched exactly. C:\\Users\\Savage\\.local\\zig\\zig-x86_64-windows-0.16.0\\zig.exe build test --summary all passed 9/9; ReleaseFast build passed 3/3; npm test -- --run tests/zig/zig-runtime-parity.test.ts passed 25/25."
conflict_surface: ""
invariants: ["I1", "I2", "I3"]
source_message_anchor: "U1, U2, U3"
source_message_excerpt: "port to Zig needs to be on the level where it's addressing every single line | the detail, how it's done, why it's done that way, and convert it | We can start with the search, because obviously the search and the explain is needed for the benchmarking."
source_message_proof_obligation: "Make explain prove the same plan the search engine executes."
entry_state: "127b archived with expression metadata in Zig."
rollback_surface: "Revert explain-related edits in zig/src/main.zig and zig/src/cli/output.zig."
dependencies: "127b-ix-zig-line-faithful-port"
next_todo: .docs/todo/pending/127d-ix-zig-line-faithful-port.md
---
# 127c Explain Renderer

## Execute Now
Render Zig explain from the canonical expression plan and assert Rust-oracle parity.

## Patch Surface

**Modifies:**
- `zig/src/main.zig` - explain dispatch.
- `zig/src/cli/output.zig` - explain text/JSON renderer.
- `tests/zig/zig-runtime-parity.test.ts` - explain parity cases.

## Next todo
`.docs/todo/pending/127d-ix-zig-line-faithful-port.md`
