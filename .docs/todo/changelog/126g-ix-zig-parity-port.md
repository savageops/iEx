---
id: 126g-ix-zig-parity-port
parent: 126-ix-zig-parity-port
type: execution-unit
protocol_version: "2.1"
category: feature
phase: g
status: done
patch_scope: "Convert and wire the test and benchmark parity surface for the Zig binary."
blast_radius: high
idempotency_contract: conditionally-idempotent
acceptance: "Shared tests can run Rust and Zig binaries against the same fixtures, then emit parity and timing artifacts."
exit_criterion: "A Rust-vs-Zig parity artifact is written with zero schema or match-count divergences on the mandatory matrix."
validation: "npm run test && cd zig && zig build test --summary all"
expected_exit_code: 0
expected_output_pattern: "tests"
evidence: "Completed. The shared Rust-vs-Zig parity harness is wired under tests/zig and validates the operator-visible pipeline against target/release/ix.exe. Focused parity/performance artifact written to tools/reports/zig-parity/20260507-172055/zig-parity-artifact.json with parity_all_functional_equal=true, Rust hash 209AC14C3D6F453661D883FF7C814B50E96545E692847E232FA3CF719CD0681E, Zig hash F7A8954AAC2DDB78A081B783E4199D87703316BEAE5B46CDCC77EAB630A62F9E, and 7-sample focused timings. Validation passed: npm run test => 52 files and 1841 assertions passed; zig build -Doptimize=ReleaseFast --summary all => 3/3 succeeded."
conflict_surface: ""
invariants: ["I1", "I4", "I5"]
source_message_anchor: "U2, U3, U5, U6"
source_message_excerpt: "We will do one file at a time, convert it entirely into Zig. | It needs to be identical. No shortcuts, no funny business. | create the binary for the Zig version and then test it compared to the Rust version | FinTech level engineering and code standards."
source_message_proof_obligation: "Convert proof infrastructure only after source parity exists."
entry_state: "126f archived with all command capabilities ported."
rollback_surface: "Revert shared parity scripts, generated fixtures, and Zig test files added by this unit."
dependencies: "126f-ix-zig-parity-port"
next_todo: .docs/todo/pending/126h-ix-zig-parity-port.md
---
# 126g Test And Benchmark Parity

## Execute Now
Convert the test and benchmark proof surface so Rust and Zig binaries can be measured against identical fixtures.

## Patch Surface

**Adds/Modifies:**
- `zig/tests/**` - Zig-native unit and integration tests.
- `tests/**` - shared Rust-vs-Zig parity harness only where necessary.
- `tools/reports/**` - generated parity artifacts.

## Exit State
- The repo can prove functional parity before speed evaluation.
- Benchmark artifacts identify Rust and Zig binary paths explicitly.

## Next todo
`.docs/todo/pending/126h-ix-zig-parity-port.md`
