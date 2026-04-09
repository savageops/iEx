# iEx Engine v2

iEx (`intelligent expressions`) is a Rust-first search framework designed to evolve beyond linear regex workflows.

Canonical repo: [github.com/savageops/iEx](https://github.com/savageops/iEx)  
Canonical site: [iex.run](https://iex.run)

## Layout

- `crates/iex-core`: expression planner + high-throughput scanner
- `crates/iex-cli`: CLI binary surface
- `crates/iex-bench`: benchmark helper binary
- `tests`: Vitest suites (300+ materialized tests)
- `tools/scripts`: benchmark loop + report tooling
- `dashboard`: live HTML dashboard for loop telemetry
- `.refs/ripgrep`: upstream baseline reference clone
- `todo/pending`: planning-spec todo chain

## Search Execution Model

- Direct file roots scan as a single target without walking through ignore traversal first.
- Very large direct-file `--stats-only` workloads can shard safe fast-count byte ranges across cores inside that same single-file ownership path.
- Directory roots use one discovered file list, then auto-select scan parallelism from corpus size.
- Benchmark telemetry should reflect this single ownership path; avoid separate hybrid auto walkers or hidden fallback scan modes.

## Quickstart

```powershell
cargo build -p iex-cli
npm install
npm run fixtures
npm run test
npm run bench:loop
npm run dashboard
```

## Native install

Install iEx as a native shell command so operators and agents use the same system-visible binary.

Windows:

```powershell
npm run install:native:windows
```

Cross-platform dispatcher:

```powershell
npm run install:native
```

Windows installer contract:
- builds `target/release/iex-cli.exe` when needed
- snapshots the current release binary before any rebuild that would replace it
- installs the native command to `%LOCALAPPDATA%\Programs\iEx\bin\iex.exe`
- adds that directory to the user `PATH`
- writes a PowerShell profile block that removes the built-in `iex` alias (`Invoke-Expression`) and remaps `iex` to the installed iEx binary

macOS / Linux:

```bash
npm run install:native:unix
```

Cross-platform dispatcher:

```bash
npm run install:native
```

Unix installer contract:
- builds `target/release/iex-cli` when needed
- snapshots the current release binary before any rebuild that would replace it
- installs the native command to `~/.local/bin/iex`
- appends `~/.local/bin` PATH export blocks to the common shell profiles when missing

Friend-safe macOS binary from CI:
- run the GitHub Actions workflow in [build-native-binaries.yml](/E:/Workspaces/01_Projects/01_Github/iEx-Engine-v2/.github/workflows/build-native-binaries.yml)
- download `iex-macos-x64` for Intel Macs or `iex-macos-arm64` for Apple Silicon Macs
- unpack the archive and then run `bash tools/scripts/install-native.sh --source-binary /path/to/iex`

Verification:

```powershell
iex --help
iex search "lit:Sherlock Holmes" . --stats-only
```

PowerShell note: `iex` normally resolves to `Invoke-Expression`. The Windows installer takes ownership of the `iex` command name in the current user's PowerShell profile so new sessions resolve to the native iEx binary. `iex.exe` remains directly callable at any time.

Cross-build note: a macOS binary cannot be linked from this Windows machine by Rust target installation alone. It still needs a real macOS runner or Apple SDK/toolchain. The workflow exists to keep that build canonical and reproducible.

## Benchmark telemetry

- Canonical benchmark suite (ripgrep benchsuite):
  - list suite cases: `npm run bench:suite:list`
  - download suite corpora (default `subtitles-en`): `npm run bench:suite:download`
  - download specific corpus set: `npm run bench:suite:download -- subtitles-en subtitles-ru`
  - Windows-safe full data bootstrap (subtitles + linux fallback path): `npm run bench:suite:bootstrap-data`
  - run suite: `npm run bench:report`
  - iEx diagnostic harness (non-canonical, for local hotspot triage):
  - one-shot run: `node tools/scripts/run-once-benchmark.mjs --build-profile release --warmup 1`
  - range report run: `npm run bench:report:diag -- --reps 5 --build-profile release --warmup 1 --samples 1`
  - direct contender rerun on codedb: `npm run bench:contenders:direct -- --reps 25 --warmup 3`
  - archive the current live loop and clear it for a fresh run while preserving self-improvement baseline knowledge: `npm run bench:reset`
  - loop runner (suite-profile stream): `npm run bench:loop -- --loops 1 --build-profile release --warmup 1`
  - legacy synthetic loop runner: `npm run bench:loop:diag -- --loops 1 --build-profile release --warmup 1`
  - pinned-binary replay without rebuild: `npm run bench:loop -- --loops 1 --iex-binary tools/reports/candidate-compare/iex-cli-baseline-YYYYMMDD-HHMMSS.exe`
  - live dashboard: `node tools/scripts/dashboard-server.mjs`
  - optional external contenders are configured in `tools/scripts/competitors.json`; `ripgrep` is the baseline and `ugrep` is the current best-contender lane when available locally
- Reports:
  - `tools/reports/live-metrics.jsonl` (append-only live diagnostics history written by `npm run bench:loop`)
  - `tools/reports/latest.json` (latest live diagnostics snapshot)
  - `tools/reports/self-improvement-baseline.json` (preserved profile-normalized baseline used after live-window resets)
  - `tools/reports/bench/v2-vs-rg-*.json` (full range report artifact)
  - `tools/reports/bench/latest.json` (latest range report)
  - `tools/reports/bench/ugrep-vs-iex-rg-direct-*.json` (direct contender rerun artifact on codedb)
  - `tools/reports/bench/contenders-direct-latest.json` (latest direct contender rerun)
  - `tools/reports/bench/ripgrep-benchsuite-*.csv` (latest raw ripgrep benchsuite artifact written by `npm run bench:report`)
  - `.docs/bench/metrics-index.md` (human-readable metric interpretation guide)
- Dashboard provenance: the live monitor summarizes `tools/reports/live-metrics.jsonl` and separately surfaces the latest `ripgrep-benchsuite-*.csv` artifact so benchsuite refresh status stays visible beside the live diagnostics feed.
- Canonical performance measurement is the ripgrep benchsuite raw artifact plus the live iEx diagnostics window; the live diagnostics harness remains available for instrumentation and tuning.
- Benchmark scripts accept `--iex-binary <path>` for explicit baseline/candidate replays; when supplied, the runner uses that immutable binary path instead of rebuilding `target/release/iex-cli.exe`.
- Metric model:
  - `iexMs`: iEx core engine time (`report.stats.timings.total_ms`)
  - `iexCliMs`: full CLI wall-clock time (startup + parse + engine + output)
  - `iexProcessOverheadMs`: `iexCliMs - iexMs`
  - `rgMs`: ripgrep CLI wall-clock time
  - `iexToRgRatio`: `iexMs / rgMs` (above 1.0x means iEx slower, below 1.0x means iEx faster)
  - `competitors.<name>.durationMs`: optional direct-invocation timings for external contenders such as `ugrep`
  - dashboard summary includes a `Competitor Summary` lane and primary challenger selection when contender data is present
  - Range report includes min/median/p95/max by profile and phase slowdown attribution.

## Ripgrep harvest

- Reference clone: `.refs/ripgrep`
- Harvest report: `.docs/recon/ripgrep-harvest-2026-04-08.md`
- Key directive: reuse proven fast-path ideas (literal acceleration, adaptive search strategy, parallel traversal) while implementing iEx with clean, independent structure and measurable contracts.

## Performance goal

The target is to iteratively optimize iEx until it beats ripgrep with a tracked goal of >= 50% faster than ripgrep on agreed benchmark suites.

Current benchmark truth must always come from the latest generated report artifact in `tools/reports/bench/` and live dashboard summary instead of static README claims.

## Agent and operator usage rule

Once native install is present, prefer `iex` for local search, search validation, and operator workflows in this repo. Use `rg` for repo archaeology when the task is not validating iEx behavior itself or when the native install has not been activated in the current shell yet.
