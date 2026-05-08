---
id: 127b-ix-zig-line-faithful-port
parent: 127-ix-zig-line-faithful-port
type: execution-unit
protocol_version: "2.1"
category: feature
phase: b
status: done
patch_scope: "Port Rust expression plan data shape and parser semantics into Zig."
blast_radius: medium
blast_radius_justification: "Expression plans feed explain, search matching, stats, and CLI reports."
idempotency_contract: idempotent
idempotency_notes: "Source edits are deterministic and covered by Zig unit tests plus Rust-oracle fixtures."
acceptance: "Zig expression plans expose predicate type, raw text, parsed bytes, logic mode, and matcher metadata required by Rust explain/search parity."
exit_criterion: "zig build test --summary all exits 0 with expression tests covering boolean splitting, prefix/suffix/literal/re, and invalid mixed operators."
validation: "cd zig; zig build test --summary all"
expected_exit_code: 0
expected_output_pattern: "Build Summary: .* success"
evidence: "Validated with C:\\Users\\Savage\\.local\\zig\\zig-x86_64-windows-0.16.0\\zig.exe build test --summary all from zig/; Build Summary: 3/3 steps succeeded; 9/9 tests passed. Cross-binary runtime guard also passed: npm test -- --run tests/zig/zig-runtime-parity.test.ts; 25 tests passed."
conflict_surface: ""
invariants: ["I1", "I2", "I3"]
source_message_anchor: "U1, U2, U3"
source_message_excerpt: "port to Zig needs to be on the level where it's addressing every single line | the detail, how it's done, why it's done that way, and convert it | We can start with the search, because obviously the search and the explain is needed for the benchmarking."
source_message_proof_obligation: "Make the expression plan rich enough for explain and search benchmarking parity."
entry_state: "127a archived with .docs/zig-line-faithful-port-map.md."
rollback_surface: "Revert zig/src/core/expr.zig and expression-test edits."
dependencies: "127a-ix-zig-line-faithful-port"
next_todo: .docs/todo/pending/127c-ix-zig-line-faithful-port.md
---
# 127b Expression Core

## Execute Now
Port the Rust expression plan shape into Zig as the canonical search/explain contract.

## Original User Message Proof

| Source Anchor | Verbatim Original Snippet | Slice Proof Obligation | Evidence Capture |
|---------------|---------------------------|------------------------|------------------|
| U3 | "We can start with the search, because obviously the search and the explain is needed for the benchmarking." | Expression planning must become the shared input to both search and explain. | Zig tests and Rust-oracle parity fixtures |

## Patch Surface

**Modifies:**
- `zig/src/core/expr.zig` - expression model, parser, and metadata.

**Must not touch:**
- `crates/**` - oracle only.

## Next todo
`.docs/todo/pending/127c-ix-zig-line-faithful-port.md`
