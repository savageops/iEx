---
id: 126d-ix-zig-parity-port
parent: 126-ix-zig-parity-port
type: execution-unit
protocol_version: "2.1"
category: feature
phase: d
status: done
patch_scope: "Port expression parsing, predicate planning, and explain-plan JSON into Zig."
blast_radius: medium
idempotency_contract: idempotent
acceptance: "Zig expression fixtures produce the same accepted plans and guided rejections as Rust."
exit_criterion: "Golden `ix explain` parity fixtures pass for literal, regex, prefix, suffix, &&, ||, and ambiguous translator inputs."
validation: "cd zig && zig build test --summary all"
expected_exit_code: 0
expected_output_pattern: "Build Summary"
evidence: "Completed. zig/src/core/expr.zig owns the expression plan port and exact `ix explain` JSON parity is validated against target/release/ix.exe. Runtime parity covers literal expressions plus boolean explain output; broader parser pressure remains covered by npm run test materialized/parser and search-explain-pressure suites. Validation passed: zig build test --summary all => 7/7; npm run test => 52 files and 1841 assertions passed."
conflict_surface: "118-aho-alternates-kernel-profile, 119-alternates-outer-shard-safety-proof"
invariants: ["I1", "I3", "I4"]
source_message_anchor: "U1, U2, U3"
source_message_excerpt: "We now need to rewrite everything over into Zig. | We will do one file at a time, convert it entirely into Zig. | It needs to be identical. No shortcuts, no funny business."
source_message_proof_obligation: "Port the parser and planner as a complete file-level semantic unit."
entry_state: "126c archived with CLI ABI fixtures and no search execution fallback."
rollback_surface: "Revert zig/src/core/expr.zig and explain fixture changes."
dependencies: "126c-ix-zig-parity-port"
next_todo: .docs/todo/pending/126e-ix-zig-parity-port.md
---
# 126d Expression Core Port

## Execute Now
Convert Rust expression ownership into Zig with golden plan parity.

## Patch Surface

**Adds/Modifies:**
- `zig/src/core/expr.zig` - parser, predicate descriptors, lowering, explain model.
- `zig/tests/expr_golden.zig` - semantic fixtures.

## Exit State
- Zig can parse and explain every canonical expression class required by Rust.
- Ambiguity and unsupported syntax remain explicit contract failures.

## Next todo
`.docs/todo/pending/126e-ix-zig-parity-port.md`
