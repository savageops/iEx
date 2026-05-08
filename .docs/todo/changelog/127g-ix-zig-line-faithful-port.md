---
id: 127g-ix-zig-line-faithful-port
parent: 127-ix-zig-line-faithful-port
type: execution-unit
protocol_version: "2.1"
category: feature
phase: g
status: done
patch_scope: "Port Rust file strategy: binary sniffing, tiny/small read policy, mmap-equivalent ownership, streaming fallback, and line accounting."
blast_radius: high
blast_radius_justification: "File loading controls correctness on binary/large files and dominates performance on benchmark corpora."
idempotency_contract: idempotent
idempotency_notes: "File strategy is deterministic for fixture files and platform-gated where required."
acceptance: "Zig handles binary files, empty files, CRLF/LF, giant files, and line numbering with Rust-oracle parity."
exit_criterion: "zig build test --summary all and npm parity tests exit 0."
validation: "cd zig; zig build test --summary all; cd ..; npm test -- --run tests/zig/zig-runtime-parity.test.ts"
expected_exit_code: 0
expected_output_pattern: "passed"
evidence: "Ported Rust binary-sniff skip semantics for NUL bytes in the first 1024 bytes before files_scanned/bytes_scanned accounting. Fixed JSON slowest_files rendering for skipped binary files. Added Rust-oracle parity test for likely binary files. Validated with Zig build test 10/10, ReleaseFast build 3/3, and npm runtime parity 27/27."
conflict_surface: ""
invariants: ["I1", "I2", "I3", "I4"]
source_message_anchor: "U1, U2, U3"
source_message_excerpt: "port to Zig needs to be on the level where it's addressing every single line | the detail, how it's done, why it's done that way, and convert it | We can start with the search, because obviously the search and the explain is needed for the benchmarking."
source_message_proof_obligation: "Port Rust file-ownership decisions that affect correctness and timing."
entry_state: "127f archived with Rust-compatible discovery file sets."
rollback_surface: "Revert file loading edits in zig/src/core/search.zig."
dependencies: "127f-ix-zig-line-faithful-port"
next_todo: .docs/todo/pending/127h-ix-zig-line-faithful-port.md
---
# 127g File Strategy

## Execute Now
Port Rust file loading, binary sniffing, and line accounting into Zig.

## Patch Surface

**Modifies:**
- `zig/src/core/search.zig` - file strategy and line scanning.

## Next todo
`.docs/todo/pending/127h-ix-zig-line-faithful-port.md`
