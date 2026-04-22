---
id: 012e-iex-regex-decomposition-prefilter-wave
parent: 012-iex-regex-decomposition-prefilter-wave
type: sub-todo
category: feature
phase: e
status: done
patch_scope: "Run immutable candidate-versus-current proof for the decomposed regex path on motivating workloads"
blast_radius: "medium - benchmark proof only"
acceptance: "The decomposed path is benchmarked against the current canonical binary on exact regex-heavy workloads with immutable artifacts"
entry_state: "The decomposed path is exact, attributable, and inertable"
exit_state: "The parent has proof artifacts showing whether decomposition materially reduces duplicate regex work"
invariants:
  - "Proof uses exact current-versus-candidate comparison."
  - "Unsupported shapes remain part of the proof set to guard against regressions."
rollback_surface: "Proof artifacts and benchmark metadata created during this slice"
validation: "Run the exact benchmark workflow and confirm artifact completeness"
evidence: "2026-04-22: immutable proof compared `tools/reports/candidate-compare/iex-cli-candidate-whole-buffer-context-gate-20260422-144932.exe` against `tools/reports/candidate-compare/iex-cli-baseline-20260422-143320.exe` after verifying the pinned live binary hash. Motivating Holmes lane median: `88.4464 ms` vs `89.0156 ms` (`0.9936x`, 0.64% faster) with `candidate_lines_checked=39`; unsupported no-literal lane remained decomposition-inert but mixed at median `262.8973 ms` vs `225.5590 ms` (`1.1655x`). Durable note: `.docs/recon/regex-whole-buffer-context-gate-proof-2026-04-22.md`."
next_todo: /todo/pending/012f-iex-regex-decomposition-prefilter-wave.md
continuation: "When complete, set status done, move file to /todo/changelog, and continue immediately to next_todo."
blocked_reason: ""
unblock_action: ""
resumption_point: ""
---
# 012e Decomposition Proof

## Objective
Benchmark the decomposed regex path on the workloads it is meant to improve and preserve immutable proof artifacts.

## Why This Slice Exists
This parent is only worth shipping if decomposition saves real work on real regex cases while keeping exact answers.

## Scope
- This slice includes: regex-heavy proof workloads and artifact capture.
- This slice must not include: new runtime features.

## Detailed Requirements
- Requirement 1: snapshot the current canonical binary before benchmarking the candidate.
- Requirement 2: benchmark at least one regex-heavy workload where literal decomposition should reduce duplicate work.
- Requirement 3: include at least one unsupported or marginal shape in the proof set to catch regressions.
- Requirement 4: preserve exact hashes, workload ids, and median deltas in a machine-readable artifact.

## Validation Plan
- Command(s):
  - `node tools/scripts/run-once-benchmark.mjs --expression "re:(ERR_SYS|PME_TURN_OFF|LINK_REQ_RST|CFG_BME_EVT)" --corpus ".refs/ripgrep/benchsuite/linux" --samples 5 --warmup 2 --build-profile release`
- Expected outcome: one immutable proof artifact exists for the decomposition lane.
- Evidence to capture: artifact path and measured delta summary.

## Next todo
`/todo/pending/012f-iex-regex-decomposition-prefilter-wave.md`

## Completion
- [x] Validation executed and evidence recorded.
- [x] Status updated to `done`.
- [ ] File moved to `/todo/changelog/<same filename>`.
- [ ] Continue immediately with `Next todo`.
