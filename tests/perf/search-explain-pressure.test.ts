import { spawnSync } from "node:child_process";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import path from "node:path";
import { describe, expect, test } from "vitest";

import { buildIxSearchArgs, buildPerformanceCaseMatrix, filterAvailablePerfCases } from "../../tools/scripts/lib/perf-case-matrix.mjs";
import { ensureBinary } from "../helpers/iex-cli";

const ROOT = process.cwd();
const REPORT_PATH = path.join(ROOT, "tools", "reports", "search-explain-pressure-latest.json");
const SEARCH_SAMPLE_COUNT = 32;
const SLOW_EXPLAIN_MS = 75;
const RUNAWAY_EXPLAIN_MS = 5_000;
const SLOW_SEARCH_MS = 2_000;
const SEVERE_SEARCH_MS = 10_000;
const RUNAWAY_SEARCH_MS = 60_000;

type Severity = "normal" | "slow" | "severe";

type TimedResult = {
  id: string;
  family: string;
  expression: string;
  wallMs: number;
  severity?: Severity;
  engineMs?: number;
  scanMs?: number;
  scanSharePct?: number;
  filesScanned?: number;
  bytesScanned?: number;
  matchCount?: number;
  predicateCount?: number;
  mode?: string;
  slowestFiles?: Array<{
    path: string;
    durationMs: number;
    bytes: number;
  }>;
};

function expectMonotonicRanking(entries: TimedResult[]) {
  for (let index = 1; index < entries.length; index += 1) {
    expect(entries[index - 1].wallMs).toBeGreaterThanOrEqual(entries[index].wallMs);
  }
}

function expectSearchTelemetryShape(searchReport: any) {
  const stats = searchReport.stats;
  const timings = stats.timings;

  expect(stats.files_discovered).toBeGreaterThan(0);
  expect(stats.files_scanned).toBeGreaterThan(0);
  expect(stats.files_discovered).toBeGreaterThanOrEqual(stats.files_scanned);
  expect(stats.bytes_scanned).toBeGreaterThan(0);
  expect(stats.matches_found).toBeGreaterThanOrEqual(0);
  expect(Number.isFinite(timings.discover_ms)).toBe(true);
  expect(Number.isFinite(timings.scan_ms)).toBe(true);
  expect(Number.isFinite(timings.aggregate_ms)).toBe(true);
  expect(Number.isFinite(timings.total_ms)).toBe(true);
  expect(timings.total_ms).toBeGreaterThanOrEqual(timings.scan_ms);
  expect(timings.total_ms).toBeGreaterThanOrEqual(timings.aggregate_ms);
  expect(stats.concurrency.available_threads).toBeGreaterThan(0);
  expect(stats.concurrency.outer_scan_threads).toBeGreaterThan(0);
  expect(["materialized", "materialized_prepared", "streaming_stats_only"]).toContain(stats.concurrency.execution_mode);
  expect(Array.isArray(stats.slowest_files)).toBe(true);
  expect(stats.slowest_files.length).toBeGreaterThan(0);
  for (const slowestFile of stats.slowest_files.slice(0, 5)) {
    expect(typeof slowestFile.path).toBe("string");
    expect(slowestFile.path.length).toBeGreaterThan(0);
    expect(Number.isFinite(slowestFile.duration_ms)).toBe(true);
    expect(slowestFile.duration_ms).toBeGreaterThanOrEqual(0);
    expect(Number.isFinite(slowestFile.bytes)).toBe(true);
    expect(slowestFile.bytes).toBeGreaterThan(0);
  }
}

const pressureFamilies = new Set([
  "suite-en-no-literal",
  "suite-ru-literal",
  "suite-ru-literal-casei",
  "suite-linux-literal",
  "suite-linux-word",
  "suite-linux-alternates",
  "suite-linux-no-literal",
  "synthetic-absent-literal",
]);

const pressureCases = filterAvailablePerfCases(buildPerformanceCaseMatrix({ count: 160 }))
  .filter((caseSpec) => pressureFamilies.has(caseSpec.family))
  .slice(0, 48);

const report: {
  generatedAt: string;
  explain: TimedResult[];
  search: TimedResult[];
} = {
  generatedAt: new Date().toISOString(),
  explain: [],
  search: [],
};

function timedIx(args: string[]) {
  const started = process.hrtime.bigint();
  const result = spawnSync(ensureBinary(), args, {
    cwd: ROOT,
    encoding: "utf8",
    maxBuffer: 128 * 1024 * 1024,
    windowsHide: true,
  });
  const ended = process.hrtime.bigint();
  const wallMs = Number(ended - started) / 1_000_000;

  if (result.status !== 0) {
    throw new Error(`ix command failed code=${result.status}\nargs=${args.join(" ")}\n${result.stderr}`);
  }

  return {
    wallMs,
    stdout: result.stdout,
  };
}

function explainSeverity(wallMs: number): Severity {
  return wallMs >= SLOW_EXPLAIN_MS ? "slow" : "normal";
}

function searchSeverity(wallMs: number): Severity {
  if (wallMs >= SEVERE_SEARCH_MS) {
    return "severe";
  }
  if (wallMs >= SLOW_SEARCH_MS) {
    return "slow";
  }
  return "normal";
}

function writeReport() {
  const explain = [...report.explain].sort((left, right) => right.wallMs - left.wallMs);
  const search = [...report.search].sort((left, right) => right.wallMs - left.wallMs);
  const ranked = {
    generatedAt: report.generatedAt,
    thresholds: {
      slowExplainMs: SLOW_EXPLAIN_MS,
      runawayExplainMs: RUNAWAY_EXPLAIN_MS,
      slowSearchMs: SLOW_SEARCH_MS,
      severeSearchMs: SEVERE_SEARCH_MS,
      runawaySearchMs: RUNAWAY_SEARCH_MS,
    },
    summary: {
      explainCount: explain.length,
      searchCount: search.length,
      slowExplainCount: explain.filter((entry) => entry.severity === "slow").length,
      slowSearchCount: search.filter((entry) => entry.severity === "slow").length,
      severeSearchCount: search.filter((entry) => entry.severity === "severe").length,
      slowestExplain: explain[0] ?? null,
      slowestSearch: search[0] ?? null,
    },
    explain,
    search,
  };
  mkdirSync(path.dirname(REPORT_PATH), { recursive: true });
  writeFileSync(REPORT_PATH, `${JSON.stringify(ranked, null, 2)}\n`, "utf8");
  return ranked;
}

describe("search and explain pressure diagnostics", () => {
  test("pressure corpus covers current slow-lane families", () => {
    const families = new Set(pressureCases.map((caseSpec) => caseSpec.family));

    expect(pressureCases.length).toBeGreaterThanOrEqual(32);
    for (const family of pressureFamilies) {
      expect(families).toContain(family);
    }
  });

  test("explain pressure ranks parser planning cost across adversarial expressions", () => {
    for (const caseSpec of pressureCases) {
      const measured = timedIx(["explain", caseSpec.expression]);
      const explained = JSON.parse(measured.stdout);

      expect(explained.source).toBe(caseSpec.expression);
      expect(["all", "any"]).toContain(explained.mode);
      expect(explained.predicates.length).toBeGreaterThan(0);

      report.explain.push({
        id: caseSpec.id,
        family: caseSpec.family,
        expression: caseSpec.expression,
        wallMs: measured.wallMs,
        severity: explainSeverity(measured.wallMs),
        predicateCount: explained.predicates.length,
        mode: explained.mode,
      });

      expect(measured.wallMs).toBeLessThan(RUNAWAY_EXPLAIN_MS);
    }
  });

  test("search pressure ranks scan cost across predecessor-gap families", () => {
    for (const caseSpec of pressureCases.slice(0, SEARCH_SAMPLE_COUNT)) {
      const measured = timedIx(buildIxSearchArgs(caseSpec));
      const searchReport = JSON.parse(measured.stdout);
      const timings = searchReport.stats.timings;

      expectSearchTelemetryShape(searchReport);

      report.search.push({
        id: caseSpec.id,
        family: caseSpec.family,
        expression: caseSpec.expression,
        wallMs: measured.wallMs,
        severity: searchSeverity(measured.wallMs),
        engineMs: timings.total_ms,
        scanMs: timings.scan_ms,
        scanSharePct: timings.total_ms > 0 ? (timings.scan_ms / timings.total_ms) * 100 : 0,
        filesScanned: searchReport.stats.files_scanned,
        bytesScanned: searchReport.stats.bytes_scanned,
        matchCount: searchReport.stats.matches_found,
        slowestFiles: searchReport.stats.slowest_files.slice(0, 3).map((entry: any) => ({
          path: entry.path,
          durationMs: entry.duration_ms,
          bytes: entry.bytes,
        })),
      });

      expect(measured.wallMs).toBeLessThan(RUNAWAY_SEARCH_MS);
    }
  }, 180_000);

  test("writes a ranked pressure report for slow-lane triage", () => {
    const ranked = writeReport();

    expect(ranked.explain).toHaveLength(pressureCases.length);
    expect(ranked.search).toHaveLength(SEARCH_SAMPLE_COUNT);
    expect(ranked.summary.explainCount).toBe(pressureCases.length);
    expect(ranked.summary.searchCount).toBe(SEARCH_SAMPLE_COUNT);
    expect(ranked.summary.slowestSearch?.wallMs).toBe(ranked.search[0].wallMs);
    expect(ranked.summary.slowestExplain?.wallMs).toBe(ranked.explain[0].wallMs);
    expectMonotonicRanking(ranked.search);
    expectMonotonicRanking(ranked.explain);
    expect(ranked.summary.slowestSearch?.slowestFiles?.length).toBeGreaterThan(0);
    expect(ranked.search.every((entry) => (entry.slowestFiles?.length ?? 0) > 0)).toBe(true);

    const persisted = JSON.parse(readFileSync(REPORT_PATH, "utf8"));
    expect(persisted.summary).toEqual(ranked.summary);
    expect(persisted.search[0].id).toBe(ranked.search[0].id);
    expect(persisted.explain[0].id).toBe(ranked.explain[0].id);
  });
});
