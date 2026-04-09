# iEx Engine v2

iEx (`intelligent expressions`) is a Rust-first search framework designed to evolve beyond linear regex workflows.

## Layout

- `crates/iex-core`: expression planner + high-throughput scanner
- `crates/iex-cli`: CLI binary surface
- `crates/iex-bench`: benchmark helper binary
- `tests`: Vitest suites (300+ materialized tests)
- `tools/scripts`: benchmark loop + report tooling
- `dashboard`: live HTML dashboard for loop telemetry
- `.refs/ripgrep`: upstream baseline reference clone
- `todo/pending`: planning-spec todo chain

## Quickstart

```powershell
cargo build -p iex-cli
npm install
npm run fixtures
npm run test
npm run bench:loop
npm run dashboard
```

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
  - loop runner (suite-profile stream): `npm run bench:loop -- --loops 1 --build-profile release --warmup 1`
  - legacy synthetic loop runner: `npm run bench:loop:diag -- --loops 1 --build-profile release --warmup 1`
  - live dashboard: `node tools/scripts/dashboard-server.mjs`
  - optional external contenders are configured in `tools/scripts/competitors.json`; `ripgrep` is the baseline and `ugrep` is the current best-contender lane when available locally
- Reports:
  - `tools/reports/live-metrics.jsonl` (append-only history)
  - `tools/reports/latest.json` (latest snapshot)
  - `tools/reports/bench/v2-vs-rg-*.json` (full range report artifact)
  - `tools/reports/bench/latest.json` (latest range report)
  - `tools/reports/bench/ripgrep-benchsuite-*.csv` (canonical ripgrep benchsuite raw data)
  - `.docs/bench/metrics-index.md` (human-readable metric interpretation guide)
- Canonical performance measurement is ripgrep suite output; diagnostic harness remains available for instrumentation and tuning.
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
