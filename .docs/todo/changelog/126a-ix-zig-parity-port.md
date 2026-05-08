---
id: 126a-ix-zig-parity-port
parent: 126-ix-zig-parity-port
type: execution-unit
protocol_version: "2.1"
category: feature
phase: a
status: done
patch_scope: "Freeze Rust oracle, Zig root, command ABI, and missing-toolchain boundary before source conversion."
blast_radius: low
idempotency_contract: idempotent
acceptance: "The baseline records current Rust ABI, current dirty tree, missing Zig toolchain, and exact first Zig root contract."
exit_criterion: ".docs/zig-port-architecture.md exists and names the same invariants as this unit."
validation: "ix inspect .docs/zig-port-architecture.md && ix --help && git status --short"
expected_exit_code: 0
expected_output_pattern: "IX Zig Port Architecture"
evidence: "Validated with ix inspect .docs/zig-port-architecture.md, ix --help, and git status --short. Architecture doc exists, names zig/ adjacent to crates/, records Rust as oracle, and records missing Zig toolchain. ix --help reports canonical commands search, matches, inspect, explain plus the rg-shaped translator and agent sentinels."
conflict_surface: ""
invariants: ["I1", "I2", "I3", "I4", "I5"]
source_message_anchor: "U1, U3, U4, U6"
source_message_excerpt: "We now need to rewrite everything over into Zig. | It needs to be identical. No shortcuts, no funny business. | Just make sure it's next to crates. | FinTech level engineering and code standards."
source_message_proof_obligation: "Reject ambiguous port semantics before any Zig source exists."
entry_state: "Current Rust CLI responds to ix --help; zig command is unavailable on PATH."
rollback_surface: "Delete .docs/zig-port-architecture.md and this 126 chain if baseline scope is rejected."
dependencies: ""
next_todo: .docs/todo/pending/126b-ix-zig-parity-port.md
---
# 126a Baseline And Contract Lock

## Execute Now
Record the non-negotiable Rust-oracle and Zig-root boundary before code conversion.

## Original User Message Proof

| Source Anchor | Verbatim Original Snippet | Slice Proof Obligation | Evidence Capture |
|---------------|---------------------------|------------------------|------------------|
| U3 | "It needs to be identical. No shortcuts, no funny business." | Baseline must state no fallback or wrapper behavior is allowed. | Architecture doc and `ix --help` output |
| U4 | "Just make sure it's next to crates." | Baseline must name `zig/` adjacent to `crates/`. | Architecture doc |

## Patch Surface

**Modifies:**
- `.docs/zig-port-architecture.md` - canonical migration blueprint.

**Must not touch:**
- `crates/**` - Rust oracle remains unchanged.
- `zig/**` - scaffold belongs to 126b.

## Exit State
- `.docs/zig-port-architecture.md` defines `zig/` as the sibling implementation lane.
- Current `ix --help` output is captured as ABI oracle evidence.
- Missing Zig toolchain is recorded as a capability boundary.

## Completion
- [x] Pre-flight passed.
- [x] Validation commands executed.
- [x] Evidence captured.
- [x] Status set to `done`.

## Next todo
`.docs/todo/pending/126b-ix-zig-parity-port.md`
