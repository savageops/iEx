# IX Performance Case Matrix

## Contract

`tools/scripts/perf-case-matrix.mjs` owns the high-cardinality diagnostic benchmark matrix. It is not a product search path and it is not a replacement for the canonical suite loop. Its job is to generate many deterministic, challenging probes that can be sampled quickly, timed per case, and ranked by wall time, engine time, scan time, match count, scanned bytes, and slowest-file tail.

## Current Default

- Case count: `888`.
- Generator owner: `tools/scripts/lib/perf-case-matrix.mjs`.
- CLI owner: `tools/scripts/perf-case-matrix.mjs`.
- Static contract tests: `tests/perf/perf-case-matrix.test.ts`.
- Search/explain pressure diagnostics: `tests/perf/search-explain-pressure.test.ts`.
- Materialized test generator: `tools/scripts/materialize-tests.mjs` now generates `1776` TypeScript contract tests: the original `888` matrix/contract tranche plus an additional `888` adversarial tranche.

## Case Families

- `suite-en-literal`
- `suite-en-literal-casei`
- `suite-en-word`
- `suite-en-alternates`
- `suite-en-surrounding-words`
- `suite-en-no-literal`
- `suite-ru-literal`
- `suite-ru-literal-casei`
- `suite-linux-literal`
- `suite-linux-word`
- `suite-linux-alternates`
- `suite-linux-no-literal`
- `synthetic-and`
- `synthetic-or`
- `synthetic-prefix-suffix`
- `synthetic-absent-literal`

The matrix deliberately over-represents scan-dominant lanes because the active trajectory gaps are no-literal regex and Linux tree/header-tail workloads. These cases are designed to expose cost surfaces; they do not authorize promotion by themselves.

## Adversarial Test Tranche

The second `888` generated tests are not extra happy-path coverage. They are designed as high-pressure invariant probes around surfaces likely to hide performance and quality regressions:

- `adversarial-parser-*`: ambiguous boolean grammar, mixed literal/regex/prefix/suffix expressions, and predicate cardinality preservation through `ix explain`.
- `adversarial-metrics-*`: tail-risk statistics, non-finite sample rejection, regression slope stability, semivariance, absolute delta, and hotspot classification under adversarial phase distributions.
- `adversarial-contracts-*`: benchmark-ledger validation, malformed run rejection, sparse JSONL handling, and canonical Linux dominant-file telemetry shape.
- `adversarial-abi-*`: single terminal `ix.result.v1` sentinel parsing with surrounding stdout noise and strict terminal-state payload checks.
- `adversarial-matrix-*`: stats-only CLI argument preservation, retention-rule authority metadata, thread-argument lowering, slowest-case ranking, and family summary aggregation.

## Commands

Generate and inspect the matrix without running probes:

```powershell
npm run bench:matrix:dry -- --count 888 --limit 32
```

Measure a bounded slice against the current release binary:

```powershell
npm run bench:matrix -- --count 888 --limit 64 --samples 3 --warmup 1 --ix-binary target\release\ix.exe
```

Run the full matrix when the machine is thermally stable:

```powershell
npm run bench:matrix -- --count 888 --limit 888 --samples 2 --warmup 1 --ix-binary target\release\ix.exe
```

Run the bounded `ix explain` plus `ix search` pressure lane:

```powershell
npm run test:pressure
```

This writes `tools/reports/search-explain-pressure-latest.json` with ranked parser and scan-cost tails. The report classifies `ix explain` cases above `75 ms` as slow, `ix search` cases above `2 s` as slow, and `ix search` cases above `10 s` as severe. The search test keeps a `60 s` runaway ceiling so the lane records slow cases instead of aborting before the full tail is materialized.

## Interpretation Rules

- Treat `suite-linux-no-literal` as the first regression target because it was slower than the installed predecessor in the last status audit.
- Treat `suite-en-no-literal` as the pure no-required-literal regex probe.
- Treat Linux literal/word/alternates as tail-tax probes; inspect slowest files and scanned-byte distribution before editing scheduler policy.
- Promote no code from this matrix without the normal immutable current-binary snapshot and current-vs-installed/predecessor proof gate.
- Keep canonical CLI suite reports separate from this diagnostic matrix. Matrix reports are exploratory evidence, not headline competitor claims.
