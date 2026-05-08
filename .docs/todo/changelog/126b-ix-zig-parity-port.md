---
id: 126b-ix-zig-parity-port
parent: 126-ix-zig-parity-port
type: execution-unit
protocol_version: "2.1"
category: feature
phase: b
status: done
patch_scope: "Create the Zig root and build graph without porting engine behavior."
blast_radius: low
idempotency_contract: conditionally-idempotent
acceptance: "`zig build --summary all` can build a minimal `ix-zig` executable after Zig is installed and pinned."
exit_criterion: "zig/build.zig, zig/src/main.zig, and toolchain pin notes exist; build succeeds on pinned Zig."
validation: "zig version && cd zig && zig build --summary all"
expected_exit_code: 0
expected_output_pattern: "Build Summary"
evidence: "Completed. Pinned Zig 0.16.0 verified at C:\\Users\\Savage\\.local\\zig\\zig-x86_64-windows-0.16.0\\zig.exe. Validation passed: zig.exe version => 0.16.0; zig build --summary all => 3/3 steps succeeded; zig build test --summary all => 5/5 tests passed; npm run test -- tests/zig/zig-port-contract.test.ts tests/zig/zig-runtime-parity.test.ts => 9/9 passed; T3 Tape validate => OK. Structural evidence: sibling zig/ root exists and does not delegate to Rust/Cargo/installed IX."
conflict_surface: ""
invariants: ["I1", "I2", "I3"]
source_message_anchor: "U1, U2, U4"
source_message_excerpt: "We now need to rewrite everything over into Zig. | We will do one file at a time, convert it entirely into Zig. | Just make sure it's next to crates."
source_message_proof_obligation: "Create only the sibling Zig root and first buildable file boundary."
entry_state: "126a archived with Rust oracle, Zig root, and missing toolchain boundary recorded."
rollback_surface: "Delete zig/ files added by this unit; do not touch crates/."
dependencies: "126a-ix-zig-parity-port"
next_todo: .docs/todo/pending/126c-ix-zig-parity-port.md
blocked_reason: ""
unblock_action: ""
resumption_point: "Continue from 126c/126d with runtime parity tests as the ABI gate."
---
# 126b Zig Toolchain And Root Scaffold

## Execute Now
Pin a Zig toolchain and create the minimal `zig/` build root adjacent to `crates/`.

## Original User Message Proof

| Source Anchor | Verbatim Original Snippet | Slice Proof Obligation | Evidence Capture |
|---------------|---------------------------|------------------------|------------------|
| U4 | "Just make sure it's next to crates." | `zig/` must be created at repo root, not under a Rust crate. | Directory listing and build output |

## Patch Surface

**Adds:**
- `zig/build.zig` - Zig build graph.
- `zig/src/main.zig` - first executable entrypoint.
- `zig/README.md` - toolchain and lane contract.

**Must not touch:**
- `crates/**` - no Rust rewrite in the scaffold unit.

## Exit State
- `zig/` exists as the only Zig implementation root.
- Zig build graph can produce a binary once the pinned toolchain is present.
- The executable is contract-labeled incomplete until later parity units pass.

## Completed State
- Structural scaffold exists.
- Static anti-delegation and runtime parity contract tests pass.
- Formal Zig 0.16.0 build and test validation passes through the absolute compiler path.

## Next todo
`.docs/todo/pending/126c-ix-zig-parity-port.md`
