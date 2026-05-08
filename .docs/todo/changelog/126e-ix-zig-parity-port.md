---
id: 126e-ix-zig-parity-port
parent: 126-ix-zig-parity-port
type: execution-unit
protocol_version: "2.1"
category: feature
phase: e
status: done
patch_scope: "Port discovery, scan execution, stats, and search reports into Zig."
blast_radius: high
idempotency_contract: idempotent
acceptance: "Zig `search` and `matches` produce Rust-identical match counts and schemas on the shared fixture corpus."
exit_criterion: "Rust-vs-Zig parity runner reports identical results for literal, prefix, suffix, regex, boolean, stats-only, and zero-match cases."
validation: "npm run test -- tests/perf/search-explain-pressure.test.ts && cd zig && zig build test --summary all"
expected_exit_code: 0
expected_output_pattern: "tests"
evidence: "Completed as a candidate Zig search lane. zig/src/core/search.zig and zig/src/core/regex.zig own discovery, file scanning, hit rows, JSON report envelopes, measured serial telemetry, regex/literal/prefix/suffix predicates, compat lowering, matches --json, and --emit-report without Rust delegation. Runtime parity covers literal, boolean, regex alternation, anchors, repetition, classes, ASCII ignore-case, repeated -e, --regexp, -j/--threads, stats-only, zero/default telemetry envelope, and hit payloads against target/release/ix.exe. Validation passed: cargo test -p iex-core --quiet => 86 passed; npm run test => 52 files and 1841 assertions passed; dupe-audit => 31 segments, 0 candidate pairs, 0 exact duplicates."
conflict_surface: "118-aho-alternates-kernel-profile, 119-alternates-outer-shard-safety-proof"
invariants: ["I1", "I3", "I4", "I5"]
source_message_anchor: "U1, U2, U3, U6"
source_message_excerpt: "We now need to rewrite everything over into Zig. | We will do one file at a time, convert it entirely into Zig. | It needs to be identical. No shortcuts, no funny business. | FinTech level engineering and code standards."
source_message_proof_obligation: "Port the behavioral core with parity pressure, not superficial compilation."
entry_state: "126d archived with expression-plan parity and explicit unsupported syntax boundaries."
rollback_surface: "Revert zig/src/core/search.zig, zig/src/core/stats.zig, and parity harness changes."
dependencies: "126d-ix-zig-parity-port"
next_todo: .docs/todo/pending/126f-ix-zig-parity-port.md
---
# 126e Search Engine Port

## Execute Now
Convert Rust search execution into Zig with Rust as the match-count oracle.

## Patch Surface

**Adds/Modifies:**
- `zig/src/core/search.zig` - discovery, scan, match aggregation, report production.
- `zig/src/core/stats.zig` - telemetry schema.
- `tests/perf/*` or helper scripts only where needed for Rust-vs-Zig parity.

## Exit State
- Zig search can execute the shared fixture matrix without shelling to Rust.
- Match-count and schema differences are recorded as failures, not accepted drift.

## Next todo
`.docs/todo/pending/126f-ix-zig-parity-port.md`
