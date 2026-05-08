---
id: 126h-ix-zig-parity-port
parent: 126-ix-zig-parity-port
type: verification-unit
protocol_version: "2.1"
category: feature
phase: h
status: done
patch_scope: "Verify full Zig parity, record benchmark evidence, and decide whether Zig is only a candidate or promotable."
blast_radius: high
idempotency_contract: conditionally-idempotent
acceptance: "Zig is either rejected with evidence or retained as a candidate through exact Rust parity and neutral-or-better benchmark proof."
exit_criterion: "Changelog entry names source parity, test parity, benchmark artifacts, retained/rejected decision, and next lower-level mechanism."
validation: "npm run test && cargo test -p iex-cli --quiet && cargo test -p iex-core --quiet && cd zig && zig build test --summary all"
expected_exit_code: 0
expected_output_pattern: "tests"
evidence: "Completed with candidate-only decision. Zig source parity, runtime parity, full repo tests, Rust crate tests, ReleaseFast build, dupe audit, and focused parity artifact all passed. Validation captured: npm run test => 52 files and 1841 assertions passed; cargo test -p iex-cli --quiet => 39+39 passed; cargo test -p iex-core --quiet => 86 passed; zig build test --summary all => 7/7; zig build -Doptimize=ReleaseFast --summary all => 3/3; dupe-audit on args/output/search/inspect/regex => 31 segments, 0 candidate pairs, 0 exact duplicates. t3-tape validate could not run because t3-tape is not on PATH. Promotion decision: retain Zig as a candidate lane only; do not replace Rust canonical ix until broad canonical benchmark promotion evidence is captured."
conflict_surface: ""
invariants: ["I1", "I2", "I3", "I4", "I5"]
source_message_anchor: "U1, U3, U5, U6"
source_message_excerpt: "We now need to rewrite everything over into Zig. | It needs to be identical. No shortcuts, no funny business. | create the binary for the Zig version and then test it compared to the Rust version | FinTech level engineering and code standards."
source_message_proof_obligation: "Close the port only with end-to-end functional and performance proof."
entry_state: "126g archived with Rust-vs-Zig parity harness and artifacts."
rollback_surface: "Do not promote Zig; preserve artifacts; leave Rust canonical binary unchanged."
dependencies: "126g-ix-zig-parity-port"
next_todo: NONE
---
# 126h Verification And Promotion Gate

## Execute Now
Run the full parity and benchmark decision gate for the Zig binary.

## Patch Surface

**Modifies:**
- `.docs/todo/changelog/_log.md` - final execution record.
- `.docs/iex-v2-crown-jewel.md` - only if Zig becomes a proven retained lane.

**Must not touch:**
- Native install path or live loop binary unless promotion evidence passes.

## Required Decision Record
- Rust oracle binary path and hash.
- Zig candidate binary path and hash.
- Match-count parity artifact.
- Output-schema parity artifact.
- Benchmark artifact against current Rust and installed predecessor.
- Retained, rejected, or candidate-only status.

## Next todo
`NONE`
