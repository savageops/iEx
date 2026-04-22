---
id: 012d-iex-regex-decomposition-prefilter-wave
parent: 012-iex-regex-decomposition-prefilter-wave
type: sub-todo
category: feature
phase: d
status: done
patch_scope: "Add exactness tests, runtime attribution, and inerting or bailout rules for the decomposed regex path"
blast_radius: "high - correctness-sensitive runtime instrumentation"
acceptance: "The decomposed path remains exact, attributable, and able to inert when decomposition value collapses"
entry_state: "The engine can execute eligible decomposed component plans"
exit_state: "The decomposed path is correctness-covered and can report or disable itself when ineffective"
invariants:
  - "Final regex semantics remain identical to the current engine."
  - "The decomposed path can be disabled or inerted when it fails to add value."
rollback_surface: "Correctness tests, telemetry fields, and inerting logic touched for decomposition"
validation: "Run correctness tests across eligible and unsupported regex fixtures and verify telemetry remains zero-safe when inactive"
evidence: "2026-04-22: `cargo test -p iex-core` and `cargo test` passed after adding `regex_decomposition` runtime stats plus exactness coverage in `crates/iex-core/src/expr.rs` and `crates/iex-core/src/engine.rs`; inactive no-literal proof reports zero-safe telemetry while eligible Holmes fixtures report attributed counts and bailout coverage. Durable note: `.docs/recon/regex-whole-buffer-context-gate-proof-2026-04-22.md`."
next_todo: /todo/pending/012e-iex-regex-decomposition-prefilter-wave.md
continuation: "When complete, set status done, move file to /todo/changelog, and continue immediately to next_todo."
blocked_reason: ""
unblock_action: ""
resumption_point: ""
---
# 012d Exactness and Inerting

## Objective
Close the correctness loop and disposability loop around the decomposed regex path.

## Why This Slice Exists
Decomposition only belongs in the repo if it remains exact and can stand down when it becomes noise instead of value.

## Scope
- This slice includes: correctness tests, telemetry, and inerting or bailout behavior.
- This slice must not include: new proof workloads or promotion decisions.

## Detailed Requirements
- Requirement 1: add correctness tests that compare decomposed execution against the current regex behavior on eligible and unsupported shapes.
- Requirement 2: add runtime attribution for whether decomposition activated and whether it inerted or bailed out.
- Requirement 3: ensure the decomposed path can disable itself when trigger quality or saved work collapses.

## Validation Plan
- Command(s):
  - `cargo test -p iex-core`
- Expected outcome: correctness tests pass and decomposition telemetry is zero-safe when inactive.
- Evidence to capture: test output summary and telemetry field summary.

## Next todo
`/todo/pending/012e-iex-regex-decomposition-prefilter-wave.md`

## Completion
- [x] Validation executed and evidence recorded.
- [x] Status updated to `done`.
- [ ] File moved to `/todo/changelog/<same filename>`.
- [ ] Continue immediately with `Next todo`.
