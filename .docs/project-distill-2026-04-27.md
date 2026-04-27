# iEx Project Distill - 2026-04-27

## Mission Lock
- iEx remains the Intelligent Expressions file-search engine and benchmark platform.
- The operator command is `ix`; the Cargo package is still `iex-cli`; compatibility binaries and historical `iex-cli-*.exe` proof snapshots are lineage artifacts, not the preferred terminal surface.
- Performance claims require exact binary proof against both the current canonical build and the installed predecessor.

## Current Operator Surface
- Current workspace binary: `target/release/ix.exe`.
- Native Windows installer source: `tools/scripts/install-native.ps1`, which installs `ix.exe` under `%LOCALAPPDATA%\Programs\iEx\bin` and adds a PowerShell `ix` profile function.
- Current predecessor comparator: `C:\Users\Savage\AppData\Local\Programs\iEx\bin\iex.exe`.
- CLI search syntax: `ix search <expr> [PATH]...`.
- CLI explain syntax: `ix explain <expr>`.

## Current Benchmark Truth
- Authoritative refresh artifact: `tools/reports/candidate-compare/110-ix-current-vs-installed-20260427-233905/summary.json`.
- Dashboard-suite samples: `3`.
- Suite result versus ripgrep: `12` wins, `0` losses.
- Suite result versus installed predecessor: `9` wins, `3` losses.
- Focused exact recheck samples: `9`.
- Superseded noisy lane: `suite-en-alternates` is green on exact recheck at `0.9678677341x` versus the installed predecessor.

## Confirmed Loss Frontier
- `suite-linux-no-literal`: `777.8426 ms` current, `708.8126 ms` predecessor, `1.0973882236x` ratio, `623.9763 ms` scan phase.
- `suite-linux-word`: `649.1551 ms` current, `631.1250 ms` predecessor, `1.0285681917x` ratio, `492.5571 ms` scan phase.
- Both losses are scan-dominant Linux tree lanes with `29` outer scan threads, `144,017,913` dominant targeted bytes, `0` dominant activated files, and inactive byte sharding.
- The top observed tail files are AMD ASIC register headers under `.refs/ripgrep/benchsuite/linux/drivers/gpu/drm/amd/include/asic_reg`.

## Current Architecture Truth
- Canonical external benchmark lane is the CLI lane through `ix search ... --json --stats-only`.
- Prepared replay remains an internal steady-state metric and must not be blended into headline competitor proof.
- Runtime telemetry already exposes the necessary attribution blocks for the current frontier: `phaseTimings`, `concurrency`, `linuxStrategy`, `linuxDominantFile`, `regexDecomposition`, and `slowestFiles`.
- The current active gap is not a parser or branding problem. It is a Linux scan-kernel and giant-header tail-economics problem where the existing dominant-file targeting observes the byte mass but does not activate a cheaper execution geometry.
- Historical docs and changelog entries may contain `iex search`, `iex-cli.exe`, or `iex-cli-live-*.exe` because those artifacts describe older proof snapshots. Active docs should prefer `ix` and timestamped `ix-*.exe` snapshots.

## Next Proof Blueprint
- Start from the Linux loss frontier, not broad suite averages.
- Measure scan-kernel costs separately from discovery, aggregation, and process overhead.
- Target one removable probe at a time around dominant-header byte sharding, no-literal rejection cost, word-boundary verification cost, or per-file tail reduction geometry.
- Retain only candidates that beat the pre-change `ix.exe` snapshot, stay neutral-or-better against `C:\Users\Savage\AppData\Local\Programs\iEx\bin\iex.exe`, preserve match counts, and keep adjacent Linux lanes stable.
- Rejected variants must leave a hypothesis, code seam, proof artifact, revert evidence, and next lower-level mechanism.

## Documentation Drift Map
- `README.md` is the public operator contract and should describe `ix`, current benchmark governance, and the latest loss frontier without rewriting historical proof.
- `.docs/iex-v2-crown-jewel.md` is the long-lived architecture and doctrine reference; its current handoff should point here.
- `.docs/project-distill-2026-04-22.md` is superseded historical proof lineage.
- `.docs/recon/*` and old `.docs/todo/changelog/_log.md` entries are immutable evidence by default; do not mass-rewrite them to chase terminology.
