---
id: 126f-ix-zig-parity-port
parent: 126-ix-zig-parity-port
type: execution-unit
protocol_version: "2.1"
category: feature
phase: f
status: done
patch_scope: "Port read-only inspection windows, match context, and explain command rendering into Zig."
blast_radius: medium
idempotency_contract: idempotent
acceptance: "Zig `inspect` and `explain` outputs match Rust grouped and JSON contracts on file-window and context fixtures."
exit_criterion: "Rust-vs-Zig inspect/explain golden tests pass."
validation: "cd zig && zig build test --summary all && npm run test -- tests/perf/search-explain-pressure.test.ts"
expected_exit_code: 0
expected_output_pattern: "tests"
evidence: "Completed. zig/src/core/inspect.zig owns read-only file windows, --skip/--limit/--start-line/--end-line/--range/--total-count/--head/--all bounds, multipath file windows, grouped/records/json output, ix.next.v1 continuation hints, and asymmetric -B/-A match context. Explain rendering remains in zig/src/cli/output.zig and is parity-tested. Validation passed: npm run test -- tests/zig/zig-runtime-parity.test.ts tests/zig/zig-port-contract.test.ts => 27/27; cargo test -p iex-cli --quiet => 39+39 passed."
conflict_surface: ""
invariants: ["I1", "I3", "I4"]
source_message_anchor: "U1, U2, U3"
source_message_excerpt: "We now need to rewrite everything over into Zig. | We will do one file at a time, convert it entirely into Zig. | It needs to be identical. No shortcuts, no funny business."
source_message_proof_obligation: "Port the remaining command capabilities before converting the full test suite."
entry_state: "126e archived with search and matches parity."
rollback_surface: "Revert zig/src/core/inspect.zig and associated CLI renderer changes."
dependencies: "126e-ix-zig-parity-port"
next_todo: .docs/todo/pending/126g-ix-zig-parity-port.md
---
# 126f Inspect And Explain Port

## Execute Now
Convert Rust inspection and explain-output behavior into Zig.

## Patch Surface

**Adds/Modifies:**
- `zig/src/core/inspect.zig` - bounded windows and match context.
- `zig/src/cli/explain.zig` - explain rendering if split from CLI output.
- `zig/tests/inspect_golden.zig` - grouped and JSON parity fixtures.

## Exit State
- Zig owns all four public IX commands without hidden Rust delegation.
- Read-only inspection remains read-only.

## Next todo
`.docs/todo/pending/126g-ix-zig-parity-port.md`
