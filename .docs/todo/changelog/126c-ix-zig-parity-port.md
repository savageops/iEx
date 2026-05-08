---
id: 126c-ix-zig-parity-port
parent: 126-ix-zig-parity-port
type: execution-unit
protocol_version: "2.1"
category: feature
phase: c
status: done
patch_scope: "Port the command taxonomy, argv lowering, and sentinel renderers into Zig."
blast_radius: medium
idempotency_contract: idempotent
acceptance: "Zig CLI help and output renderer golden tests match Rust for search, matches, inspect, explain, and guided errors."
exit_criterion: "Zig CLI tests pass and Rust-vs-Zig help snapshots match approved fixtures."
validation: "cd zig && zig build test --summary all"
expected_exit_code: 0
expected_output_pattern: "Build Summary"
evidence: "Completed. Zig CLI ABI parity now covers exact top-level help, exact subcommand help for search/matches/inspect/explain, guided top-level compat errors, canonical dispatch, text/json/report renderers, matches JSON, --emit-report, and inspect CLI flag grammar. Validation passed: zig build test --summary all => 7/7; npm run test -- tests/zig/zig-runtime-parity.test.ts tests/zig/zig-port-contract.test.ts => 27/27; npm run test => 52 files and 1841 assertions passed."
conflict_surface: ""
invariants: ["I1", "I2", "I3", "I4"]
source_message_anchor: "U1, U2, U3"
source_message_excerpt: "We now need to rewrite everything over into Zig. | We will do one file at a time, convert it entirely into Zig. | It needs to be identical. No shortcuts, no funny business."
source_message_proof_obligation: "Freeze user-visible ABI before engine behavior is implemented."
entry_state: "126b completed with buildable Zig root and no Rust mutation."
rollback_surface: "Revert zig/src/cli/** and affected zig/src/main.zig hunks."
dependencies: "126b-ix-zig-parity-port"
next_todo: .docs/todo/pending/126d-ix-zig-parity-port.md
---
# 126c CLI ABI Port

## Execute Now
Convert the Rust CLI command surface into Zig without implementing search internals.

## Patch Surface

**Adds/Modifies:**
- `zig/src/main.zig` - command dispatch.
- `zig/src/cli/args.zig` - canonical command grammar and compat lowering.
- `zig/src/cli/output.zig` - `ix.result.v1`, `ix.next.v1`, and `ix.error.v1` renderers.

**Must not touch:**
- `zig/src/core/search.zig` - belongs to 126e.

## Exit State
- Zig knows the same command taxonomy as Rust.
- Unsupported behavior fails as explicit missing parity, not fallback execution.

## Next todo
`.docs/todo/pending/126d-ix-zig-parity-port.md`
