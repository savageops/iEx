---
id: 127j-ix-zig-line-faithful-port
parent: 127-ix-zig-line-faithful-port
type: execution-unit
protocol_version: "2.1"
category: feature
phase: j
status: done
patch_scope: "Port prepared target/replay boundary and keep prepared evidence separate from canonical CLI evidence."
blast_radius: medium
blast_radius_justification: "Prepared replay is an internal benchmark lane and must not contaminate CLI timing claims."
idempotency_contract: idempotent
idempotency_notes: "Prepared targets are deterministic for fixed root options."
acceptance: "Zig has an explicit prepared-search boundary or a documented parity gate if unsupported, with no CLI fallback."
exit_criterion: "Prepared-boundary tests or explicit unsupported-gate tests pass."
validation: "npm test -- --run tests/zig/zig-runtime-parity.test.ts"
expected_exit_code: 0
expected_output_pattern: "passed"
evidence: "Prepared replay boundary is explicitly gated in .docs/zig-line-faithful-port-map.md: Rust owns PreparedSearchTargets/iex-bench prepared replay; current Zig evidence is canonical CLI only; future prepared timings require a separate Zig API and report ledger before comparison."
conflict_surface: ""
invariants: ["I1", "I3", "I5"]
source_message_anchor: "U1, U2, U3"
source_message_excerpt: "port to Zig needs to be on the level where it's addressing every single line | the detail, how it's done, why it's done that way, and convert it | We can start with the search, because obviously the search and the explain is needed for the benchmarking."
source_message_proof_obligation: "Prevent hidden benchmark lane drift while search/explain are ported."
entry_state: "127i archived with concurrency telemetry."
rollback_surface: "Revert prepared-boundary edits."
dependencies: "127i-ix-zig-line-faithful-port"
next_todo: .docs/todo/pending/127k-ix-zig-line-faithful-port.md
---
# 127j Prepared Boundary

## Execute Now
Port or explicitly gate the prepared-search boundary for Zig without mixing report ledgers.

## Patch Surface

**Modifies:**
- `zig/src/core/search.zig` - prepared boundary if implemented.
- `tools/scripts/**` - prepared-vs-CLI evidence separation if touched.

## Next todo
`.docs/todo/pending/127k-ix-zig-line-faithful-port.md`
