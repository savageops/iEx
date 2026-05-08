---
id: 127h-ix-zig-line-faithful-port
parent: 127-ix-zig-line-faithful-port
type: execution-unit
protocol_version: "2.1"
category: feature
phase: h
status: done
patch_scope: "Port Rust scan kernels, max-hit retention semantics, fast counts, and line fallback ownership."
blast_radius: high
blast_radius_justification: "Scan kernels determine match counts, retained rows, and runtime cost."
idempotency_contract: idempotent
idempotency_notes: "Kernel behavior is deterministic under fixed fixtures."
acceptance: "Zig stats-only and row-output paths produce Rust-equal counts for literal, prefix, suffix, regex, alternates, word-boundary, and max-hit cases."
exit_criterion: "npm test -- --run tests/zig/zig-runtime-parity.test.ts exits 0 with scan-kernel adversarial cases."
validation: "npm test -- --run tests/zig/zig-runtime-parity.test.ts"
expected_exit_code: 0
expected_output_pattern: "passed"
evidence: "Added Rust-oracle adversarial parity test proving stats-only literal search counts occurrences, not matching lines, and preserved prior max-hits count-vs-retained-row tests. Validated with Zig build test 10/10, ReleaseFast build 3/3, and npm runtime parity 28/28."
conflict_surface: ""
invariants: ["I1", "I2", "I3", "I4"]
source_message_anchor: "U1, U2, U3"
source_message_excerpt: "port to Zig needs to be on the level where it's addressing every single line | the detail, how it's done, why it's done that way, and convert it | We can start with the search, because obviously the search and the explain is needed for the benchmarking."
source_message_proof_obligation: "Make the core search result and count semantics Rust-identical before speed comparison."
entry_state: "127g archived with file loading parity."
rollback_surface: "Revert scan-kernel edits in zig/src/core/search.zig and zig/src/core/regex.zig."
dependencies: "127g-ix-zig-line-faithful-port"
next_todo: .docs/todo/pending/127i-ix-zig-line-faithful-port.md
---
# 127h Scan Kernels

## Execute Now
Port Rust scan and fast-count semantics into Zig.

## Patch Surface

**Modifies:**
- `zig/src/core/search.zig` - scan and retention semantics.
- `zig/src/core/regex.zig` - fast count support.

## Next todo
`.docs/todo/pending/127i-ix-zig-line-faithful-port.md`
