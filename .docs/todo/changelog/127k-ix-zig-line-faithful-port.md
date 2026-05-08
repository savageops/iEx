---
id: 127k-ix-zig-line-faithful-port
parent: 127-ix-zig-line-faithful-port
type: execution-unit
protocol_version: "2.1"
category: feature
phase: k
status: done
patch_scope: "Wire CLI search and explain through the canonical Zig engine and eliminate duplicate matcher/report paths."
blast_radius: medium
blast_radius_justification: "CLI glue is the operator surface; dispatch drift creates visible contract divergence."
idempotency_contract: idempotent
idempotency_notes: "Dispatch and renderer ownership are deterministic."
acceptance: "All Zig search/explain commands use the same expression and engine contracts, including rg-shaped translator ingress."
exit_criterion: "npm parity tests pass with canonical and rg-shaped invocations."
validation: "npm test -- --run tests/zig/zig-runtime-parity.test.ts"
expected_exit_code: 0
expected_output_pattern: "passed"
evidence: "Added Rust-oracle runtime test proving rg-shaped top-level -e --json ingress lowers into the canonical search report path. Validated with Zig build test 10/10, ReleaseFast build 3/3, and npm runtime parity 30/30."
conflict_surface: ""
invariants: ["I2", "I3", "I4"]
source_message_anchor: "U1, U2, U3"
source_message_excerpt: "port to Zig needs to be on the level where it's addressing every single line | the detail, how it's done, why it's done that way, and convert it | We can start with the search, because obviously the search and the explain is needed for the benchmarking."
source_message_proof_obligation: "Make the operator-visible command path prove the same engine used by tests and benchmarks."
entry_state: "127j archived with prepared boundary settled."
rollback_surface: "Revert CLI glue edits."
dependencies: "127j-ix-zig-line-faithful-port"
next_todo: .docs/todo/pending/127l-ix-zig-line-faithful-port.md
---
# 127k CLI Integration

## Execute Now
Wire CLI search and explain through one Zig engine path.

## Patch Surface

**Modifies:**
- `zig/src/main.zig` - command dispatch.
- `zig/src/cli/args.zig` - translator ingress if parity gaps remain.
- `zig/src/cli/output.zig` - sentinel/report renderer.

## Next todo
`.docs/todo/pending/127l-ix-zig-line-faithful-port.md`
