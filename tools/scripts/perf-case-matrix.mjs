import path from "node:path";
import {
  DEFAULT_PERF_CASE_COUNT,
  buildPerformanceCaseMatrix,
  filterAvailablePerfCases,
  measurePerformanceCase,
  summarizePerformanceMatrix,
  writePerformanceMatrixReport,
} from "./lib/perf-case-matrix.mjs";
import { resolveIexBinaryPath } from "./lib/benchmark-runner.mjs";
import { argValue, timestampSlug, toPositiveInt } from "./lib/script-helpers.mjs";

const args = process.argv.slice(2);
const count = toPositiveInt(argValue(args, "--count", String(DEFAULT_PERF_CASE_COUNT)), DEFAULT_PERF_CASE_COUNT);
const limit = toPositiveInt(argValue(args, "--limit", String(count)), count);
const samples = toPositiveInt(argValue(args, "--samples", "1"), 1);
const warmup = Math.max(0, Number(argValue(args, "--warmup", "0")));
const threadsArg = argValue(args, "--threads", undefined);
const threads = threadsArg ? Number(threadsArg) : undefined;
const buildProfile = argValue(args, "--build-profile", undefined);
const ixBinary = argValue(args, "--ix-binary", undefined);
const outPath = argValue(
  args,
  "--out",
  path.join(process.cwd(), "tools", "reports", `perf-case-matrix-${timestampSlug()}.json`),
);
const dryRun = args.includes("--dry-run");

function familyCounts(cases) {
  const counts = new Map();
  for (const caseSpec of cases) {
    counts.set(caseSpec.family, (counts.get(caseSpec.family) ?? 0) + 1);
  }
  return Object.fromEntries([...counts.entries()].sort(([left], [right]) => left.localeCompare(right)));
}

const matrix = buildPerformanceCaseMatrix({ count });
const available = filterAvailablePerfCases(matrix);
const selected = available.slice(0, Math.min(limit, available.length));

if (dryRun) {
  const report = {
    mode: "dry-run",
    generatedCount: matrix.length,
    availableCount: available.length,
    selectedCount: selected.length,
    familyCounts: familyCounts(matrix),
    unavailable: matrix
      .filter((caseSpec) => !available.includes(caseSpec))
      .slice(0, 25)
      .map((caseSpec) => ({ id: caseSpec.id, family: caseSpec.family, corpus: caseSpec.corpus })),
    firstCases: selected.slice(0, 25),
  };
  console.log(JSON.stringify(report, null, 2));
  process.exit(0);
}

const binaryPath = resolveIexBinaryPath({ buildProfile, iexBinaryPath: ixBinary });
const results = [];

for (const caseSpec of selected) {
  const measured = measurePerformanceCase(binaryPath, caseSpec, {
    warmup,
    samples,
    threads,
  });
  results.push(measured);
  console.log(
    `[perf-matrix] ${caseSpec.id} ${caseSpec.family} wall=${measured.selected.wallMs.toFixed(2)}ms engine=${measured.selected.engineMs.toFixed(2)}ms scan=${measured.selected.scanMs.toFixed(2)}ms scanShare=${measured.scanSharePct.toFixed(1)}% matches=${measured.selected.matchCount}`,
  );
}

const report = {
  mode: "measured",
  generatedCount: matrix.length,
  availableCount: available.length,
  selectedCount: selected.length,
  samples,
  warmup,
  threads: Number.isFinite(threads) ? threads : null,
  binaryPath,
  outputPath: outPath,
  summary: summarizePerformanceMatrix(results),
  results,
};

writePerformanceMatrixReport(report, outPath);
console.log(`[perf-matrix] wrote ${outPath}`);
