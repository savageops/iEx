---
id: 127f-ix-zig-line-faithful-port
parent: 127-ix-zig-line-faithful-port
type: execution-unit
protocol_version: "2.1"
category: feature
phase: f
status: done
patch_scope: "Port Rust root discovery, overlap pruning, hidden traversal, symlink handling, and discovery telemetry."
blast_radius: high
blast_radius_justification: "Discovery changes the file set, match counts, and benchmark denominator."
idempotency_contract: idempotent
idempotency_notes: "Traversal decisions are deterministic for fixed filesystem fixtures."
acceptance: "Zig discovers the same tested file set as Rust for roots, files, hidden roots, duplicate roots, overlap roots, and symlink options."
exit_criterion: "npm test -- --run tests/zig/zig-runtime-parity.test.ts exits 0 with discovery parity cases."
validation: "npm test -- --run tests/zig/zig-runtime-parity.test.ts"
expected_exit_code: 0
expected_output_pattern: "passed"
evidence: "Zig search now prepares roots before scanning: normalized comparable keys, duplicate root pruning, ancestor/descendant overlap pruning, and telemetry for pruned_roots/overlap_pruned_roots/discovered_duplicate_paths. Added Rust-oracle parity test for duplicate file roots and parent/child root overlap. Validated with Zig build test 10/10, ReleaseFast build 3/3, and npm runtime parity 26/26."
conflict_surface: ""
invariants: ["I1", "I2", "I4"]
source_message_anchor: "U1, U2, U3"
source_message_excerpt: "port to Zig needs to be on the level where it's addressing every single line | the detail, how it's done, why it's done that way, and convert it | We can start with the search, because obviously the search and the explain is needed for the benchmarking."
source_message_proof_obligation: "Make benchmark file-set equality true before timing comparisons."
entry_state: "127e archived with stats schema capable of exposing discovery telemetry."
rollback_surface: "Revert discovery edits in zig/src/core/search.zig."
dependencies: "127e-ix-zig-line-faithful-port"
next_todo: .docs/todo/pending/127g-ix-zig-line-faithful-port.md
---
# 127f Discovery

## Execute Now
Port Rust discovery and pruning semantics into Zig.

## Patch Surface

**Modifies:**
- `zig/src/core/search.zig` - discovery, pruning, duplicate root handling, symlink/hidden traversal.

## Next todo
`.docs/todo/pending/127g-ix-zig-line-faithful-port.md`
