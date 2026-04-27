# iEx v2 Crown Jewel Guide

## 1) Vision Lock

iEx v2 (`intelligent expressions`) is a Rust-first, CLI-first search platform that must be both tidy and capability-complete.

Canonical repo: [github.com/savageops/iEx](https://github.com/savageops/iEx)  
Canonical site: [iex.run](https://iex.run)

Primary objective:
- become the fastest search engine in this project scope,
- beat ripgrep on transparent benchmarks,
- track progress to a hard target of at least 50% faster than ripgrep on agreed workloads.

This document is the durable execution map for architecture, benchmarking, optimization, and quality gates.

Current operational handoff:
- Read [project-distill-2026-04-27.md](./project-distill-2026-04-27.md) first for the current benchmark-governance state, retained wins, rejected slices, and fresh-chat anchors.
- Treat the long historical status notes later in this document as archival context; the dated distill is the current cold-start snapshot.

## 2) Canonical Architecture

### Rust source of truth
- `crates/iex-core`
  - expression planning (`literal`, `prefix`, `suffix`, `regex`)
  - byte-native regex and literal matching paths
  - planner-owned regex decomposition for eligible `--stats-only` byte-mode regex lanes
  - file discovery across one or more roots
  - adaptive scan strategy (inline read for small files, mmap for larger files)
  - direct-file `--stats-only` chunked fast-count path for safe large-file literal workloads
  - shard-aware `--stats-only` fast-count planning with outer-parallel safety guards
  - fixed-width non-ASCII case-insensitive regex fast-count sharding for literal-equivalent byte regexes
  - directory-root `--stats-only` streaming dispatch for balanced heavy-corpus traversal
  - adaptive scan thread heuristic
  - concurrency telemetry (`execution_mode`, outer scan threads, sharding maxima)
  - regex decomposition telemetry (`eligible_files`, `counted_files`, `bailout_files`, candidate-line attribution)
  - per-phase timings
  - slow-file telemetry
- `crates/iex-cli`
  - `search` command
  - `explain` command
  - JSON report output and report-file emit hooks
- `crates/iex-bench`
  - loop-style benchmark helper binary for Rust-side timing

### JavaScript quality and telemetry
- `tests/materialized/*.test.ts`
  - fully explicit test cases
  - parser, contracts, integration, benchmark checks
- `tools/scripts`
  - fixture generation
  - one-shot benchmark
  - multi-profile loop runner
  - SSE dashboard server
- `dashboard/index.html`
  - live chart and status cards for run telemetry

### Reference-only competitor intelligence
- `.refs/ripgrep`
  - cloned upstream reference
- `.docs/recon/ripgrep-harvest-2026-04-08.md`
  - harvested strengths, cliffs, and iEx action directives

## 3) Ripgrep Harvest: Reuse and Outperform

### What to harvest
- literal-aware acceleration as primary fast path
- adaptive strategy between mmap and buffered/reader paths
- highly optimized traversal and ignore behavior
- benchmark discipline with multiple corpora and fair comparisons

### What to improve in iEx
- reduce fixed strategy ceilings with adaptive controls
- avoid hidden performance cliffs by exposing planner/strategy diagnostics
- preserve deterministic outputs without collapsing all parallelism
- keep aggregation lightweight under high worker pressure

## 4) Benchmark Contract

Artifacts:
- `tools/reports/bench/ripgrep-benchsuite-*.csv` (canonical external raw baseline from `npm run bench:report`)
- `tools/reports/live-metrics.jsonl` (append-only live suite-loop diagnostics from `npm run bench:loop`)
- `tools/reports/latest.json` (latest live run snapshot)
- `tools/reports/candidate-compare/ix-*.exe` (timestamped immutable baseline/candidate snapshots for exact binary replays; older `iex-cli-*.exe` snapshots remain historical proof lineage)
- `tools/reports/candidate-compare/*.json` (candidate-vs-current proof artifacts, including active-loop promotion proofs)

Run schema highlights:
- iEx core duration (`iexMs`)
- iEx CLI duration (`iexCliMs`)
- iEx process overhead (`iexProcessOverheadMs`)
- ripgrep duration (`rgMs`) as baseline target
- previous-build iEx path (`previousIexBinaryPath`) and ratio (`iexToPreviousRatio`) for self-regression tracking
- comparator map (`competitors`) centered on ripgrep (`ripgrep`) and optional previous-build iEx (`iex_previous`)
- phase timings (`discover`, `scan`, `aggregate`, `total`)
- slowest files (`slowestFiles`)
- regex decomposition attribution (`regexDecomposition`)
- goal tracking (`speedupPct`, `goalGapPct`)

Goal interpretation:
- `speedupPct >= 50` means target achieved for that run profile.
- `iexMs` is engine-focused to isolate true search-core speed from process startup overhead.
- `iexCliMs` keeps startup and argument parsing overhead visible for operator decisions.
- Dashboard summaries must stay explicit that they are derived from live loop telemetry, not from the benchsuite CSV itself.
- Benchmark scripts accept `--iex-binary <path>` so live loops, direct contender reruns, and one-shot diagnostics can replay an immutable binary snapshot without rebuilding `target/release/ix.exe`.
- Benchmark scripts also accept `--previous-iex-binary <path>` so the dashboard can render current-vs-previous iEx ratios in the same live competitor lane model used for ripgrep, without letting the previous build replace the external challenger heuristics.

Live comparator flow snapshot:
- current loop binary
  └─ `runOneBenchmark(...)`
    ├─ active iEx measurement → `{ iexMs, iexCliMs, iexProcessOverheadMs }`
    ├─ external competitor → `competitors.ripgrep`
    ├─ previous iEx snapshot → `competitors.iex_previous`
    └─ live artifacts → `tools/reports/live-metrics.jsonl`, `tools/reports/latest.json`
- dashboard server
  └─ `summarizeHistory(...)`
    ├─ keeps `iex_previous` inside `competitorSummary`
    └─ excludes `iex_previous` from `primaryChallenger`
- dashboard UI
  └─ trend + recent-run tables render `iEx/previous` only when a previous snapshot was configured and measured

### Active loop promotion gate
1. Identify the exact binary path currently driving the live suite loop.
2. Compare the candidate against that exact binary on the full suite with interleaved multi-pair samples, not a one-profile spot check.
3. Promote only when the suite-level proof shows a real gain signal rather than a one-off lane win.
4. Copy the promoted binary to a timestamped immutable `ix-live-*.exe` snapshot and restart the loop with `--iex-binary <snapshot>`.
5. Verify `tools/reports/latest.json` reports that promoted snapshot path so the dashboard is reading the new loop, not stale history.

## 5) Quality System

### Materialized tests
- current suite count: 400 explicit tests
- no runtime-generated hidden test factories
- deterministic generation script: `tools/scripts/materialize-tests.mjs`

### Core validation loop
1. `cargo build -p iex-cli`
2. `npm run fixtures`
3. `npm run test`
4. `node tools/scripts/run-once-benchmark.mjs`
5. `node tools/scripts/bench-loop.mjs --loops 1 --build-profile release --warmup 1`
6. `node tools/scripts/dashboard-server.mjs`
7. for baseline-vs-candidate proof, snapshot the current canonical binary first and use `--iex-binary` during replay instead of overwriting the canonical runner

## 6) Native operator adoption

Native command ownership matters because iEx should be reachable as a first-class system search tool, not only as a repo-local build artifact.

Windows contract:
- native install path: `%LOCALAPPDATA%\Programs\iEx\bin\ix.exe`
- profile contract: installer must expose `ix` as the first-class operator command without colliding with the built-in PowerShell `iex` alias
- path contract: installer must add the install directory to the user `PATH`

Latest Windows proof refresh:
- authoritative artifact: `tools/reports/candidate-compare/110-ix-current-vs-installed-20260427-233905/summary.json`
- current build: `target/release/ix.exe`
- installed predecessor comparator: `C:\Users\Savage\AppData\Local\Programs\iEx\bin\iex.exe`
- suite shape: `12/12` wins versus ripgrep and `9/12` wins versus the installed predecessor on the three-sample dashboard suite
- exact focused recheck: `suite-en-alternates` is green at `0.9678677341x` versus installed; confirmed predecessor-loss frontier is `suite-linux-no-literal` at `1.0973882236x` and `suite-linux-word` at `1.0285681917x`
- cost center: Linux scan lanes with `144,017,913` dominant targeted bytes, `0` dominant activated files, inactive sharding, and tail-heavy AMD ASIC register headers

macOS / Linux contract:
- native install path: `~/.local/bin/ix`
- shell contract: installer must ensure `~/.local/bin` is exported in the common login shell profiles without duplicate blocks

Script ownership:
- `tools/scripts/install-native.ps1`
- `tools/scripts/install-native.sh`
- `tools/scripts/install-native.mjs`
- `.github/workflows/build-native-binaries.yml`

Adoption rule:
- once native install is present, agents and operators should prefer `iex` for local search and search-validation workflows
- do not use `rg` for local repo search in this workspace unless `iex` is unavailable and that blocker is explicitly recorded

Release artifact rule:
- macOS friend-share binaries should be produced on native GitHub macOS runners, not by ad-hoc Windows cross-link attempts
- workflow artifacts must stay architecture-explicit: `iex-macos-x64` for Intel and `iex-macos-arm64` for Apple Silicon
- unpacked macOS artifacts feed directly into `tools/scripts/install-native.sh --source-binary ...`

## 7) Optimization Strategy (Next)

### Priority order
1. reduce scan-phase dominance for literal-heavy and mixed profiles
2. optimize traversal/discovery overhead for suffix/sparse-match profiles
3. tighten expression planner to increase selectivity before line-level checks
4. limit CLI startup overhead so `iexCliMs` tracks closer to `iexMs`

### Current focused closure lane (2026-04-09)
1. add ASCII case-insensitive literal and word-boundary fast-path coverage so `suite-en-literal-casei` stops falling onto the generic regex path
2. sharpen the tiny ASCII literal hot kernel so recent wins versus `ugrep` on `suite-en-literal`, `suite-en-word`, and `suite-en-alternates` become durable margin, not fragile parity
3. install contender-aware dashboard and benchmark ratchets so `iEx/rg` and `iEx/ugrep` stay visible across filtered windows and cannot silently regress
4. do not close the wave until canonical benchsuite, direct contender reruns, CLI-truth checks, and Windows/Linux release smoke all agree that `iEx` is the fastest measured tool in the canonical contract

### Current focused closure lane (2026-04-20)
1. keep literal-equivalent regex detection narrow and exact so only fixed-width non-ASCII `(?i)` forms graduate into the shard fast-count path
2. preserve the existing ASCII-specialized literal and word-boundary searchers instead of collapsing them into a generic regex branch
3. prove every benchmark-affecting slice directly against the installed WSL `iex` bin before widening back out to the full active suite
4. treat cold-file `/mnt/*` access behavior as its own diagnostic surface so first-touch variance does not get mistaken for a matcher-kernel regression

### Current focused closure lane (2026-04-22)
1. keep regex decomposition narrow: one strong required literal, no narrower fast path already owning the pattern, and byte-mode stats-only activation only
2. preserve append-only `SearchStats.regex_decomposition` attribution so proof can distinguish active wins, bailouts, and inert unsupported lanes
3. use planner-owned local context gates and candidate-line bailouts to reduce false-positive work before line-boundary recovery
4. do not promote the workspace candidate into the live loop until full-suite proof versus the exact pinned loop binary is neutral or better
5. keep `linux_strategy` / `linuxStrategy` as proof context only for now; the 2026-04-22 single-root selector candidate was rejected after immutable Linux proof, so the next active runtime parent is `025` giant-file Linux tail closure
6. treat `linux_dominant_file` / `linuxDominantFile` as proof context only for now; the 2026-04-22 Linux giant-file parent ended non-promotable after full four-lane proof
7. the direct `regex-automata` recovered-line anchored one-pass verifier parent is also now closed as non-promotable; the `024` proof stayed materially slower than the pinned clean binary even on a supported counted corpus, so the next parent must be re-priced from fresh weak-lane evidence instead of inheriting that verifier seam

### Rules of engagement
- no workaround paths
- no parallel systems
- no speed claims without telemetry evidence
- no architecture drift from canonical ownership

## 8) Planning-Spec Todo Chain

Parent:
- `todo/pending/001-iex-v2-rust-search-platform.md`

Sub-chain:
- `001a` baseline lock
- `001b` monorepo ownership
- `001c` core engine
- `001d` benchmark foundation
- `001e` live observability
- `001f` materialized test matrix
- `001g` optimization wave
- `001h` docs/handoff hardening
- `001i` regression closeout

Focused contender-gap chain:
- parent: `todo/pending/002-iex-ugrep-hot-lane-closure.md`
- `002a` contender-gap contract lock
- `002b` ASCII casefold literal family
- `002c` tiny ASCII hot-kernel uplift
- `002d` contender ratchet and filtered visibility
- `002e` canonical proof gate
- `002f` verification and closeout

Harvested follow-on chains:
- parent: `todo/changelog/006-iex-upstream-harvest-expansion.md`
- `006a` frontier candidate lock
- `006b` local mirror contract
- `006c` keyword-pack harvest runs
- `006d` harvest ledger
- `006e` adoption funnel
- `006f` frontier closeout
- parent: `todo/pending/003-iex-pma-reject-fast-gate.md`
- `003a` reject-fast contract lock
- `003b` planner reject-fast contract
- `003c` canonical reject-fast integration
- `003d` reject-fast proof gate
- `003e` reject-fast closeout
- parent: `todo/changelog/004-iex-rare-byte-anchor-ranking.md`
- `004a` rare-byte anchor contract lock
- `004b` anchor analysis contract
- `004c` canonical anchor integration
- `004d` anchor-ranking proof gate
- `004e` rare-byte anchor closeout
- parent: `todo/pending/005-iex-pin-count-alternates-kernel.md`
- `005a` tiny-alternates contract lock
- `005b` tiny-alternates dispatch contract
- `005c` canonical tiny-alternates kernel
- `005d` tiny-alternates proof gate
- `005e` tiny-alternates closeout

Operator contract chain:
- parent: `todo/changelog/007-iex-multi-root-search-paths.md`
- `007a` multi-root contract lock
- `007b` CLI and config widening
- `007c` canonical multi-root discovery
- `007d` CLI and proof gate
- `007e` multi-root closeout
- parent: `todo/pending/017-iex-tree-command-parity.md`
- `017a` tree contract lock
- `017b` CLI slash-flag contract
- `017c` canonical tree traversal report
- `017d` renderer, tests, and docs
- `017e` verification and closeout
- parent: `todo/pending/020-iex-semantic-similarity-search.md`
- `020a` semantic contract lock
- `020b` shared discovery and chunk schema
- `020c` semantic runtime and model bakeoff
- `020d` local semantic cache and index
- `020e` semantic CLI and report surface
- `020f` packaging boundary and optional reranking
- `020g` semantic validation and closeout
- parent: `todo/pending/019-iex-repo-audit-ergonomics-surface.md`
- `019a` ergonomics contract lock
- `019b` canonical path filters and named scopes
- `019c` expression files and working rewrites
- `019d` help, docs, and test parity
- `019e` verification and closeout
- parent: `todo/pending/008-iex-multipath-turbo-wave.md`
- `008a` multipath turbo contract lock
- `008b` root normalization and overlap pruning
- `008c` multipath discovery scheduling and telemetry
- `008d` medium and long literal regime dispatch
- `008e` adaptive bailout and proof gate
- `008f` verification and closeout

Historical benchmark status snapshots (archival context):
- Current live truth now lives in `project-distill-2026-04-27.md`, `tools/reports/latest.json`, and `tools/reports/live-metrics.jsonl`.
- The bullets below are preserved as implementation history and proof lineage, not as the current dashboard headline.
- latest default profile loop is consistently above 50% speedup for `iexMs` (engine metric).
- CLI wall-time overhead remains tracked and is an active optimization target.
- the live loop promotion gate is now exercised, not theoretical: extended proof artifact `tools/reports/candidate-compare/live-loop-compare-extended-20260409-232531.json` beat the previous active-loop binary on 8 of 12 suite profiles with median candidate/baseline engine ratio `0.9816`, and the loop was repointed to immutable snapshot `../iEx-Engine-v2/tools/reports/candidate-compare/iex-cli-live-20260409-232626.exe` with `tools/reports/latest.json` confirming the new path after restart.
- current contender-gap work is narrowed to the hot-lane English literal family; recent live wins are already present on `suite-en-literal`, `suite-en-word`, and `suite-en-alternates`, while `suite-en-literal-casei` remains the primary remaining `ugrep` loss lane.
- the contender-gap chain is now evidence-gated to a strongest measurable outcome: it does not close on parity alone; it closes only when the proof package supports fastest-measured language.
- the upstream frontier harvest surface is now widened to nine mirrored repos under `.refs`: `ripgrep`, `ugrep`, `re2`, `hyperscan`, `vectorscan`, `regex`, `aho-corasick`, `memchr`, and `stringzilla`.
- the widened frontier is not abstract research: it already produced concrete adoption-grade mechanisms for poison-literal gating, PMA/hash-window rejection, least-common anchor ranking, pin-count tiny-pattern kernels, RE2-style atom/prefilter trees, regex-automata start-state-specialized prefilters, memchr packed-pair heuristics, and StringZilla anomaly-ranked character selection.
- the second-pass `.refs` scan sharpened the frontier again: the next huge leap is now best framed as adaptive acceleration control, not one more fixed heuristic. The highest-signal mechanisms are memchr-style self-disabling prefilters, RE2 prefix-accel bail rules, StringZilla medium/long-needle anomaly verification, aho-corasick Teddy-style tiny-set confirming paths, and ripgrep file-size and matcher-shape strategy arbitration.
- the strongest immediate `006` doctrine is now explicit: planner-owned eligibility plus engine-owned bailout should land before another ambitious reject-fast retry, so `003` can reopen with a cheaper consumer and better tail protection instead of repeating broad whole-file consumption.
- `005` also has a clearer end-state now: the best alternates leap is likely not only `pin2` through `pin8`, but a tiny-set SIMD-confirming path that stays inside canonical alternates ownership and borrows Teddy-style selectivity without importing a second engine.
- the frontier also has clearer defer lines now: ugrep-style indexed Bloom/HFA pruning is real but would be a second operating mode, regex reverse-inner/start-state tricks strengthen the case for strict bailout rules before reuse, and Hyperscan FDR bucket packing should stay behind planner-owned dispatch instead of becoming an early compile-heavy detour.
- `004` is now archived as a promotable runtime wave: the rare-byte anchor path improved `re:(?i)Sherlock Holmes` from `99.5026 ms` to `71.5135 ms` median engine time against the pinned pre-rebuild snapshot (`+28.13%`, ratio `0.7187`) while keeping `re:\bSherlock Holmes\b` neutral-to-better (`+0.47%`) and improving diagnostic `re:(?i)\bSherlock Holmes\b` by `+10.62%`.
- the strongest next blended move for `006` runtime work is also explicit: use RE2 atom/prefilter-tree ideas and regex-automata prefilter/start-state discipline to choose when acceleration is worth it, then let ripgrep-style file/searcher arbitration decide how to execute it.
- the next harvested optimization wave is split into three non-overlapping parents instead of one blurred mega-todo: PMA-style reject-fast gating, rare-byte anchor ranking, and pin-count tiny-alternates specialization.
- each harvested parent carries its own immutable current-versus-candidate binary proof gate and explicit `>=20%` motivating improvement target against the pre-rebuild canonical binary on the exact motivating workload.
- `003a` and `003b` are now archived: the reject-fast wave has a locked immutable baseline snapshot plus planner-owned `RejectFastGate` metadata on long ASCII literal-family fast paths.
- the first `003c` runtime-consumption attempt was reverted after pinned candidate-vs-current proof showed worse `iexMs` on both motivating workloads; the next move stays inside `003c` and must find a cheaper canonical consumer before any promotion claim.
- with `004` landed, the next clean runtime move is no longer anchor selection itself; it is either a more selective `003c` consumer that benefits from the stronger literal-quality discipline, or `005` tiny-alternates specialization if the immediate target is contender-facing alternates pressure.
- the operator contract gap that rejected multi-root invocations is now closed: `iex search` and `iex-bench` both expose `[PATH]...`, and `SearchConfig` now carries one canonical roots-list contract instead of a split singular-root path.
- the multi-root implementation stayed inside canonical ownership: direct files remain direct scan targets, directory roots share one walker surface, overlapping roots are deduped before scan, and pinned live-binary proof on `lit:Sherlock Holmes` stayed neutral-to-better (`39.7138 ms` baseline vs `38.6666 ms` candidate, `+2.64%`).
- the next runtime wave is now locked as its own planning-spec parent instead of an implied follow-up: `008` combines multipath tax removal with one adaptive medium/long literal acceleration slice so traversal cost and planner/runtime selectivity improve under one proof gate.
- the current equal-workload baseline is explicit and should anchor every `008` proof replay: `tools/reports/candidate-compare/multipath-equivalent-baseline-20260410-010846.json` shows single-root benchsuite at `1501.6144 ms`, equivalent split roots at `1775.7652 ms` (`+18.26%`), and overlap root-plus-child at `1911.7175 ms` (`+27.31%`) despite identical files, matches, and bytes scanned.
- that same baseline clarifies the first optimization order: fix root normalization and discovery scheduling first, then let the harvested `memchr` plus `RE2` plus `StringZilla` doctrine add planner-owned regime selection and engine-owned bailout without hiding the traversal tax behind matcher wins.
- the first durable `008` runtime slice is now proven, not hypothetical: root normalization moved overlap cleanup ahead of traversal, `SearchStats` now exposes retained versus pruned root telemetry, and immutable proof `tools/reports/candidate-compare/multipath-root-normalization-compare-20260410-014913.json` shows overlap root-plus-child improving from `2400.2522 ms` to `1786.9055 ms` median total (`-25.55%`) while single-root stayed within gate at `-1.30%` and equivalent split roots also improved `-5.01%`.
- that proof also keeps the acceleration doctrine honest: no experimental medium/long literal accelerator landed in this slice, so the measured gain is attributable to canonical multipath ownership cleanup instead of a blended matcher change that would be harder to trust.
- the next `008c` leap is now also proven with runtime code, not just planning: stats-only directory workloads no longer have to fully materialize discovery before scan starts, because `engine.rs` now uses one bounded producer-consumer walk-to-scan path while the collecting-hits path stays on the existing materialized ownership.
- immutable snapshot `tools/reports/candidate-compare/iex-cli-baseline-20260410-043347.exe` is the proof anchor for this slice, and the candidate beat it on the motivating heavy literal workload in every checked topology: single-root proof `008c-streaming-single-root-proof-20260410-023553.json` improved median engine time from `2093.0069 ms` to `1805.7132 ms` (`+13.73%`), equivalent split-root proof `008c-streaming-multipath-proof-20260410-023706.json` improved from `1553.9253 ms` to `1306.6116 ms` (`+15.92%`), and overlap-root proof `008c-streaming-overlap-proof-20260410-023808.json` improved from `2043.2110 ms` to `1818.3792 ms` (`+11.00%`), all with identical `1233` matches and identical discovered and scanned file counts.
- the telemetry contract is intentionally more truthful now: on the streaming stats-only path, `discover_ms` and `scan_ms` are overlapping wall-clock phases instead of additive staging buckets, so `total_ms` can be materially lower than either phase sum without implying an accounting bug.
- the next recovery on top of `008c` also changed the diagnosis, not just the numbers: the lingering bad lanes were giant-file-tail dominated, not merely root-count mistakes. Artifact `tools/reports/candidate-compare/008c-dominant-file-override-20260410-142706.json` shows that keeping the stronger local candidate and allowing stats-only dominant files (`>= 512 MiB`) to use the existing chunk-parallel fast-count kernel under a hard `4`-thread cap recovered all five checked lanes versus the pinned stable `008c` baseline: `sherlock_single_root` `+7.72%`, `linux_literal` `+19.18%`, `linux_word` `+13.14%`, `multipath_split` `+0.55%`, and `multipath_overlap` `+19.02%`.
- the next follow-through is now measured too: the repo-local candidate built from `7b60de1` did not clear the active-loop promotion gate against the exact running loop snapshot `../iEx-Engine-v2/tools/reports/candidate-compare/iex-cli-live-20260409-232626.exe`; full-suite artifact `../iEx-Engine-v2/tools/reports/candidate-compare/live-loop-compare-extended-2026-04-10T03-00-34-340Z.json` recorded `3` wins, `9` losses, and median engine regression `-3.99%`, so the dashboard loop correctly stayed pinned to the older immutable snapshot instead of restarting on a weaker suite result.
- the first `008d` medium/long literal regime experiment also failed its proof gate and was reverted: artifact `tools/reports/candidate-compare/008d-literal-regime-proof-20260410-050724.json` improved the motivating ASCII literal lane `+4.71%` but regressed the plain regex-literal lane `-14.97%` and the UTF-8 literal lane `-9.51%`, which means the next acceleration move needs narrower planner eligibility and a cleaner bailout doctrine rather than a broad anchor-led widening.
- the heavy repeatability surface is now materially stronger than the previous live snapshot: three-run aggregate proof `tools/reports/candidate-compare/008c-heavy-compare-candidate-live-native-aggregate3-exec-20260410-130158.json` shows `sherlock_single_root` improving `+12.00%` versus dashboard live and `+11.53%` versus native installed, while `multipath_split` and `multipath_overlap` improve `+25.36%` and `+26.13%` versus native installed; `ru_file_single_path` remains a native-owned tail and `linux_word` still needs skepticism because its arithmetic mean edges positive without posting the fastest individual run.
- the active loop has now been repointed to immutable snapshot `../iEx-Engine-v2/tools/reports/candidate-compare/iex-cli-live-20260410-161022.exe`: the old pinned process `bench-loop-suite.mjs --iex-binary ...iex-cli-live-20260409-232626.exe` was replaced by the same loop command targeting the new snapshot, and `../iEx-Engine-v2/tools/reports/latest.json` plus `../iEx-Engine-v2/tools/reports/live-metrics.jsonl` now confirm the new path on fresh runs.
- the dashboard monitor header now surfaces that same active binary explicitly instead of forcing operators to infer it from provenance files: `dashboard/index.html` renders an `Active binary` field and `tools/scripts/dashboard-server.mjs` resolves it from `tools/reports/latest.json` with a fallback to the newest live run, so the page header and the live-loop pin now speak the same contract.
- deep best-of-3 replay against four distinct historical binaries now sharpens the truth about that live pin: immutable artifact `tools/reports/candidate-compare/current-live-vs-history-deep-bestof3-20260410-163651.json` shows the active snapshot is the newest distinct build and still the best broad single-root operator choice, winning `4` of `7` shared heavy single-root lanes and posting the best single-root geomean versus every compared predecessor (`-10.67%` versus `iex-cli-prerebuild-20260410-142531.exe`, `-17.92%` versus `iex-cli-baseline-20260410-054005.exe`, `-11.64%` versus the previous loop live snapshot, and `-10.04%` versus the installed native binary).
- that same replay also makes the remaining gap explicit instead of hiding it behind the live pin: on best-of-3 multipath proof the current live snapshot lost both checked lanes by small margins, with `multipath_split` going to `iex-cli-prerebuild-20260410-142531.exe` (`1117.4093 ms` versus `1134.0904 ms`) and `multipath_overlap` going to `iex-cli-baseline-20260410-054005.exe` (`1106.6040 ms` versus `1161.7312 ms`), while the previous live loop snapshot `iex-cli-live-20260409-232626.exe` still failed multipath preflight with exit `2`.
- fresh Insect research plus `engine.rs` review now points at the most credible next multipath leap: the streaming stats-only path is likely paying a double-pool scheduler tax because `ignore::WalkBuilder::build_parallel()` still chooses its own traversal threads while iEx simultaneously spawns scan workers and can still trigger inner dominant-file chunk pools. The next clean cut should therefore stay inside `008c` and start with explicit traversal-thread budgeting, then move to `WalkParallel::visit(...)` plus per-thread local batching if the lighter fix does not clear the gap.
- the deeper full-picture review keeps that recommendation in bounds instead of letting the remaining multipath gap hijack the roadmap: the live loop and dashboard still benchmark `--stats-only` only, `tools/reports/latest.json` still records the mutable `target/release/iex-cli.exe` path unless an explicit snapshot path is injected, the deep history replay is scored by best-of-3 instead of median or p95, and the current multipath proof surface is only two Sherlock-derived workloads. Recomputed medians still keep the active live snapshot ahead on broad single-root by a modest `~3-5%` geomean versus the recent prerebuild and baseline binaries, but leave multipath behind the prerebuild by `~6%` median geomean, which means the next high-value move should stay focused on versus-ripgrep scan-kernel and tail-cost wins first, with only a narrow `008c` scheduler follow-through instead of another broad traversal rewrite.
- the fast-count repair wave closed a real correctness hole in the large-direct-file hot path instead of adding another heuristic: `engine.rs` now caps shard size with `.min(...)` so files above the `64 MiB` threshold can actually split into multiple owned ranges again, and both file-level scanning and inner fast counts now share one `OnceLock`-owned global Rayon pool with bounded task fan-out instead of rebuilding pools inline.
- the same repair also widened the truthful fast path for compound `All` plans without inventing a second matcher: `expr.rs` now chooses a cheapest reject-first predicate, counts owned lines exactly across shard boundaries, returns `None` for unsupported `Any` plans, and keeps range ownership by line start so chunked direct-file counts stay exact instead of double-counting boundary lines.
- immutable proof artifact `tools/reports/candidate-compare/013-compound-fast-count-proof-rebuilt-20260410-202521.json` shows the repaired release binary (`SHA256 FF1315659329271EAA91A23FFEAC3F52A599F3ABA2AB5473628A17DAB7F1D360`) beating the snapshotted pre-rebuild canonical binary (`SHA256 B20CF693DB20489B2934010E710DF4F35D1ED8557CD4C86B95F620AADA92FC03`) on `.refs/ripgrep/benchsuite/subtitles/en.sample.txt` with `lit:Sherlock && lit:Holmes` and `--threads 1`: best wall time fell from `204.401 ms` to `44.774 ms` (`+78.10%`) while engine time fell from `194.636 ms` to `35.806 ms` (`+81.60%`) with identical `matches_found = 30`.
- the live-loop promotion gate rejects that same `013` candidate on the full suite: immutable artifact `tools/reports/candidate-compare/013-live-vs-new-update-suite-compare-20260410-185144z.json` compared the pinned live snapshot (`SHA256 B20CF693DB20489B2934010E710DF4F35D1ED8557CD4C86B95F620AADA92FC03`) against the rebuilt `013` candidate (`SHA256 FF1315659329271EAA91A23FFEAC3F52A599F3ABA2AB5473628A17DAB7F1D360`) across all `12` `bench-loop-suite` workloads with three interleaved measured rounds per profile; match counts stayed identical, but the candidate lost `10/12` engine medians and `11/12` wall medians, ended `88.23%` slower on engine geomean and `79.41%` slower on wall geomean, and regressed hardest on the Linux literal, word, alternates, and no-literal lanes, so the compound direct-file win is real but not safe to promote into the active loop.
- repeated proof now removes most of the remaining noise from that verdict: aggregate artifact `tools/reports/candidate-compare/013-live-vs-new-update-suite-compare-aggregate3-20260410-190923z.json` combines three fresh full-suite interleaved compare passes (`36` profile-pass outcomes total) against the same live snapshot and candidate snapshot. The candidate won only `2/36` engine profile-pass medians and `2/36` wall profile-pass medians, stayed `80.90%` slower on engine geomean and `78.91%` slower on wall geomean after aggregating the three reruns, and only `suite-ru-literal-casei` remained net-positive at all (`+3.10%` engine median-of-pass-medians), while every Linux lane stayed deeply negative in every pass. The promotion gate is therefore blocked by stable suite evidence, not a one-run anomaly.
- the rollback of that losing `013` slice is now measured too, not assumed: `expr.rs` was restored to the pre-013 compound-count contract, `engine.rs` dropped the byte-size auto-thread heuristic, HashSet dedupe, `.min(...)` shard helper, and global-pool changes while preserving the earlier dominant-file override plus streaming-gate work, and `cargo test -p iex-core` passed `41` tests before proof.
- fresh interleaved suite reruns show the catastrophic `013` regression is gone after the rollback, but the reverted branch is still only parity-grade rather than promotion-grade: aggregate artifact `tools/reports/candidate-compare/015-live-vs-rollback-suite-compare-aggregate3-20260410-192727z.json` combines three new full-suite passes against the same pinned live snapshot (`SHA256 B20CF693DB20489B2934010E710DF4F35D1ED8557CD4C86B95F620AADA92FC03`) using rollback candidate snapshot `tools/reports/candidate-compare/iex-cli-candidate-015-rollback-suite-20260410-212220.exe` (`SHA256 D9360D16166084BCDAF345C65DBEEC8390FE17D63108337108CB0575FE9E0B7B`).
- that rollback aggregate keeps correctness exact on all `12` workloads and materially changes the suite shape versus `013`: profile-pass wins moved to rollback `21` versus live `15` on engine and rollback `23` versus live `13` on wall, Linux regressions were replaced by real wins (`suite-linux-alternates` `+10.85%` engine median-of-pass-medians, `suite-linux-word` `+4.16%`, `suite-linux-literal` `+0.72%`), and the worst remaining lane is now modest (`suite-en-literal` `-8.22%` engine).
- the top-line promotion doctrine still holds though: rollback aggregate geomean is effectively flat but not better, with engine ratio `1.0032x` (`-0.32%`) and wall ratio `1.0000x` (`-0.0003%`) versus the pinned live candidate. That means the rollback successfully removed the net-negative wave and restored near-parity, but it does not yet justify repointing the active loop away from `iex-cli-live-20260410-161022.exe`.
- the next operator proof changed that verdict from "near parity" to "promote": fresh three-way artifact `tools/reports/candidate-compare/current-working-native-dashboard-live-suite-compare-20260410-194704z.json` compared the current working snapshot `tools/reports/candidate-compare/iex-cli-current-working-compare-20260410-214403.exe` (`SHA256 D9360D16166084BCDAF345C65DBEEC8390FE17D63108337108CB0575FE9E0B7B`) against both the native installed binary and the active dashboard live snapshot `../iEx-Engine-v2/tools/reports/candidate-compare/iex-cli-live-20260410-161022.exe` (`SHA256 B20CF693DB20489B2934010E710DF4F35D1ED8557CD4C86B95F620AADA92FC03`) across the full `12`-profile suite with one warmup and three measured rounds per profile under rotating order.
- that three-way result is promotion-grade rather than a one-lane tease: `current_working` posted the best aggregate engine geomean at `297.4984 ms` and the best wall geomean at `419.6631 ms`, versus `native_installed` at `305.0308 ms` / `420.7754 ms` and `dashboard_live` at `307.3671 ms` / `458.8894 ms`; it also led workload wins with `7/12` engine and `6/12` wall, while correctness stayed exact on all `12` workloads.
- the pairwise proof is explicit enough to justify the loop repoint: `current_working` beat `native_installed` by `+2.47%` on engine geomean and `+0.26%` on wall geomean, and beat the active dashboard live snapshot by `+3.21%` on engine geomean and `+8.55%` on wall geomean.
- the active loop has now been promoted to immutable snapshot `../iEx-Engine-v2/tools/reports/candidate-compare/iex-cli-live-20260410-220309.exe` with the same winning SHA256 `D9360D16166084BCDAF345C65DBEEC8390FE17D63108337108CB0575FE9E0B7B`: old live-loop PID `158140` targeting `iex-cli-live-20260410-161022.exe` was replaced by PID `6116` running `bench-loop-suite.mjs --loops 0 --warmup 1 --samples 3 --iex-binary ...iex-cli-live-20260410-220309.exe`.
- post-restart telemetry now closes the operator contract cleanly: `../iEx-Engine-v2/tools/reports/latest.json` and fresh trailing entries in `../iEx-Engine-v2/tools/reports/live-metrics.jsonl` both report `iex-cli-live-20260410-220309.exe`, so the dashboard header, live-loop runner, and provenance files are all back on one canonical binary path.
- the new concurrency-planner phase-1 slice is now implemented in runtime code instead of living only as a diagram: `engine.rs` resolves a run-level execution budget first, models byte-range sharding as a file-local plan with `shard_threads`, `chunk_bytes`, and `range_count`, and writes append-only `stats.concurrency` telemetry through `SearchStats` so proof artifacts can see the actual plan that ran.
- the same slice also closes the specific EN regression class that exposed the weakness in the earlier 90%-threads rollout: the fixed `64 MiB` shard geometry is gone, mid-size single-file fast-count work now co-designs shard count and chunk size so `range_count` can actually feed the chosen workers, and deterministic test `parallel_fast_count_plan_feeds_threads_with_multiple_ranges_on_mid_size_files` now pins that contract in `cargo test -p iex-core`.
- the first full-suite proof keeps the doctrine honest instead of overpromoting the slice: immutable artifact `tools/reports/candidate-compare/planner-phase1-vs-threads90-suite-compare-2026-04-11t21-42-38-019z.json` shows dramatic EN wins on the single-file shard-safe lanes (`suite-en-alternates` `+28.66%`, `suite-en-literal-casei` `+24.68%`, `suite-en-literal` `+16.73%`) and clean telemetry for the chosen plans, but the full 12-profile median is still slightly behind the current `threads90` candidate (`271.7086 ms` to `272.7053 ms`, `-0.37%`) because three Linux tree lanes regressed. That makes the planner architecture real and valuable, but not yet promotion-grade for the active suite.
- the April 12 re-check updates the strategic read again: the earlier "008c multipath-first" suspicion is now secondary rather than primary. `tools/reports/latest.json` is pinned to immutable candidate `tools/reports/candidate-compare/iex-cli-candidate-threads90-20260411-201130.exe`, the current suite-style loop feed shows that binary ahead of ripgrep across the sampled Linux and subtitle lanes, and the strongest current code delta lives in materialized-path concurrency plus file-local sharding rather than in streaming discovery alone. The remaining streaming gate issue is still real because `should_stream_stats_only(...)` keys off `input_roots > 1`, but the higher-value frontier is now Linux tree stability and collect-hits parity on top of the `threads90` floor, not another broad scheduler rewrite.
- that same re-check also keeps the proof contract honest: `tools/scripts/lib/benchmark-runner.mjs` still benchmarks `iex search ... --json --stats-only`, so live-loop wins remain stats-only wins until they are mirrored on collect-hits workloads. The correct next move is therefore narrow and measurable: keep planner work constrained to lanes that beat `threads90`, widen proof beyond best-of-3 when promotion claims matter, and avoid letting the older Sherlock-centric multipath gap drag the roadmap away from the now-winning materialized path.
- the capped collect-hits lane now has its own proof-grade win instead of inheriting stats-only optimism: `engine.rs` threads `SearchConfig::max_hits` into materialized and prepared scans, keeps counting every matching line, and stops constructing retained `SearchHit` objects after each file has enough local hits to satisfy the final global cap. Proof artifact `tools/reports/candidate-compare/051-max-hit-candidate-proof-2026-04-24T01-58-09-074Z.json` shows the candidate at `337.0749 ms` median total versus pre-051 snapshot `876.7887 ms` and installed native `923.1046 ms` on `lit:the || lit:and || lit:Holmes || lit:Watson --max-hits 10`, with identical `1,099,790` matches, `10` hits, first hit column `15`, and last retained hit line `68`.
- the follow-on multi-file capped-hit proof now closes the aggregation side of the same contract: `merge_outcome` receives `max_hits`, `merge_hits` keeps only the globally earliest retained hits under the cap, and final output ordering remains path/line/column. Proof artifact `tools/reports/candidate-compare/052-global-max-hit-candidate-proof-2026-04-24T02-12-04-059Z.json` shows `lit:if` on the Linux corpus improving from pre-052 `1133.2783 ms` to candidate `959.4368 ms` and installed native `2341.235 ms`, while aggregate median fell from `344.3711 ms` to `150.1793 ms` against the snapshot with identical `2,219,280` matches and `10` retained hits.
- the retained `078` replay trims a real double-scan tax from case-sensitive plain regex literals without opening a new engine path: `RegexFastPath::PlainLiteral` now omits `RejectFastGate`, while case-insensitive literals and alternates keep their gates. Proof artifact `tools/reports/candidate-compare/078-reject-fast-strong-replay-proof-2026-04-24T18-41-38-774Z.json` shows `re:PM_RESUME` improving versus installed native at ratio `0.9773` with `39/39/39` match parity; the initially suspicious direct-literal guard was rechecked in `tools/reports/candidate-compare/078-direct-literal-guard-recheck-2026-04-24T18-44-44-583Z.json` and passed at candidate/snapshot `0.9734`, candidate/installed `0.9671`.

## 9) Operating Doctrine

- simple by design, never capability-reduced
- measurable progress over speculative complexity
- independent iEx implementation inspired by harvested patterns, not copy-paste architecture drift
- objective truth lives in command outputs, telemetry artifacts, and explicit tests
- performance candidates must pass the full retention lifecycle before promotion: snapshot baseline, exact current-vs-installed proof, smallest removable probe, focused tests, repeated interleaved benchmarks, telemetry activation, adjacent-lane guards, and durable retain/reject evidence
- deep research is hypothesis generation only; worse-than-current candidates are reverted even if they beat installed native once, and rejected variants must record the failed mechanism plus the next lower-level mechanism instead of becoming dead-end noise
