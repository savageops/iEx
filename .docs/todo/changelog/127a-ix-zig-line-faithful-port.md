---
id: 127a-ix-zig-line-faithful-port
parent: 127-ix-zig-line-faithful-port
type: execution-unit
protocol_version: "2.1"
category: feature
phase: a
status: done
patch_scope: "Create the Rust-line ownership map and complete Zig migration manifest without changing runtime behavior."
blast_radius: low
blast_radius_justification: "Documentation-only contract lock; runtime binaries and tests are not modified."
idempotency_contract: idempotent
idempotency_notes: "Re-running replaces the same architecture map with the current source inventory."
acceptance: ".docs/zig-line-faithful-port-map.md maps Rust search/explain owners to Zig target owners and names the complete remaining port slices."
exit_criterion: "ix inspect .docs/zig-line-faithful-port-map.md --all exits 0 and contains 'Immediate Frontier: Search And Explain'."
validation: ".\\target\\release\\ix.exe inspect .docs\\zig-line-faithful-port-map.md --all"
expected_exit_code: 0
expected_output_pattern: "Immediate Frontier: Search And Explain"
evidence: "Validated with .\\target\\release\\ix.exe inspect .docs\\zig-line-faithful-port-map.md --all. Output contains 'Immediate Frontier: Search And Explain', maps Rust expr/engine/stats/search/explain source owners to Zig targets, and names the complete remaining port manifest."
conflict_surface: ""
invariants: ["I1", "I2", "I3", "I4", "I5"]
source_message_anchor: "U1, U2, U4, U5"
source_message_excerpt: "port to Zig needs to be on the level where it's addressing every single line | the detail, how it's done, why it's done that way, and convert it | Use the planning spec skill to map this in entirety, the complete port. | Create every single to-do slice, and then knock them all out to completion."
source_message_proof_obligation: "Freeze the source-line interpretation and the total migration map before touching the engine."
entry_state: "Prior chain 126 is archived; .docs/zig-port-architecture.md records current search/explain parity debt."
rollback_surface: "Delete .docs/zig-line-faithful-port-map.md and this 127 chain if the corrective map is rejected."
dependencies: ""
next_todo: .docs/todo/pending/127b-ix-zig-line-faithful-port.md
---
# 127a Baseline Line Map

## Execute Now
Create the source-owned Rust-to-Zig migration map that governs the remaining port.

## Original User Message Proof

| Source Anchor | Verbatim Original Snippet | Slice Proof Obligation | Evidence Capture |
|---------------|---------------------------|------------------------|------------------|
| U1 | "port to Zig needs to be on the level where it's addressing every single line" | Map Rust source owners to Zig target owners instead of broad feature labels. | Architecture map inspection |
| U4 | "Use the planning spec skill to map this in entirety, the complete port." | Name the complete port topology while executing search/explain first. | Architecture map inspection |

## Patch Surface

**Adds:**
- `.docs/zig-line-faithful-port-map.md` - line-faithful Rust ownership map and migration manifest.

**Must not touch:**
- `crates/**` - Rust remains oracle.
- `zig/src/**` - runtime behavior starts in 127b.

## Exit State
- `.docs/zig-line-faithful-port-map.md` exists and names source owners, Zig targets, validation gates, and residual full-port slices.
- Validation command exited 0 and emitted the expected `Immediate Frontier: Search And Explain` section.

## Next todo
`.docs/todo/pending/127b-ix-zig-line-faithful-port.md`
