import { mkdirSync, readdirSync, rmSync, writeFileSync } from "node:fs";
import path from "node:path";

const root = process.cwd();
const outDir = path.join(root, "tests", "materialized");

mkdirSync(outDir, { recursive: true });

for (const name of readdirSync(outDir)) {
  if (/^(parser|metrics|contracts|integration|performance|adversarial-parser|adversarial-metrics|adversarial-contracts|adversarial-abi|adversarial-matrix)-\d{2}\.test\.ts$/.test(name)) {
    rmSync(path.join(outDir, name), { force: true });
  }
}

function writeFile(name, content) {
  writeFileSync(path.join(outDir, name), content, "utf8");
}

function jsString(value) {
  return JSON.stringify(value);
}

function parserExpression(index) {
  const mod = index % 5;
  if (mod === 0) {
    return {
      expr: `lit:ERROR && lit:timeout`,
      mode: "all",
      count: 2,
    };
  }
  if (mod === 1) {
    return {
      expr: `lit:WARN || lit:latency`,
      mode: "any",
      count: 2,
    };
  }
  if (mod === 2) {
    return {
      expr: `prefix:INFO && suffix:trace=iex-v2-${index}-29`,
      mode: "all",
      count: 2,
    };
  }
  if (mod === 3) {
    return {
      expr: `re:\\bmodule=${index % 9}\\b && lit:marker`,
      mode: "all",
      count: 2,
    };
  }
  return {
    expr: `lit:checkpoint`,
    mode: "all",
    count: 1,
  };
}

function createParserFile(fileIdx, startTestIdx, perFile) {
  const lines = [];
  lines.push('import { describe, expect, test } from "vitest";');
  lines.push('import { runExplain } from "../helpers/iex-cli";');
  lines.push("");
  lines.push(`describe("materialized parser suite ${fileIdx}", () => {`);

  for (let i = 0; i < perFile; i += 1) {
    const idx = startTestIdx + i;
    const spec = parserExpression(idx);
    lines.push(`  test("parser contract ${String(idx).padStart(3, "0")}", () => {`);
    lines.push(`    const explained = runExplain(${jsString(spec.expr)});`);
    lines.push(`    expect(explained.source).toBe(${jsString(spec.expr)});`);
    lines.push(`    expect(explained.mode).toBe(${jsString(spec.mode)});`);
    lines.push(`    expect(explained.predicates.length).toBe(${spec.count});`);
    lines.push("  });");
  }

  lines.push("});");
  lines.push("");
  return lines.join("\n");
}

function createMetricsFile(fileIdx, startTestIdx, perFile) {
  const lines = [];
  lines.push('import { describe, expect, test } from "vitest";');
  lines.push('import { classifyHotspot, computeSpeedupPct, mean, percentile, rollingAverage } from "../../tools/scripts/lib/metrics.mjs";');
  lines.push("");
  lines.push(`describe("materialized metrics suite ${fileIdx}", () => {`);

  for (let i = 0; i < perFile; i += 1) {
    const idx = startTestIdx + i;
    const set = [idx + 1, idx + 2, idx + 3, idx + 4, idx + 5];
    const expectedMean = set.reduce((s, v) => s + v, 0) / set.length;
    const p50 = set[Math.floor(0.5 * set.length)];
    const iex = 90 + (idx % 25);
    const rg = 140 + (idx % 35);
    const speed = ((rg - iex) / rg) * 100;
    const discover = 5 + (idx % 9);
    const scan = 30 + (idx % 11);
    const aggregate = 10 + (idx % 7);

    lines.push(`  test("metrics contract ${String(idx).padStart(3, "0")}", () => {`);
    lines.push(`    const values = ${JSON.stringify(set)};`);
    lines.push(`    expect(mean(values)).toBeCloseTo(${expectedMean}, 8);`);
    lines.push(`    expect(percentile(values, 50)).toBe(${p50});`);
    lines.push(`    expect(computeSpeedupPct(${iex}, ${rg})).toBeCloseTo(${speed}, 8);`);
    lines.push(`    const hotspot = classifyHotspot({ phaseMs: { discover: ${discover}, scan: ${scan}, aggregate: ${aggregate} } });`);
    lines.push('    expect(["discover", "scan", "aggregate"]).toContain(hotspot);');
    lines.push(`    expect(rollingAverage(values, 3).length).toBe(values.length);`);
    lines.push("  });");
  }

  lines.push("});");
  lines.push("");
  return lines.join("\n");
}

function createContractFile(fileIdx, startTestIdx, perFile) {
  const lines = [];
  lines.push('import { describe, expect, test } from "vitest";');
  lines.push('import { parseJsonl, validateRun } from "../../tools/scripts/lib/metrics.mjs";');
  lines.push("");
  lines.push(`describe("materialized report contract suite ${fileIdx}", () => {`);

  for (let i = 0; i < perFile; i += 1) {
    const idx = startTestIdx + i;
    const run = {
      runId: `run-${idx}`,
      timestamp: `2026-04-08T10:${String(idx % 60).padStart(2, "0")}:00.000Z`,
      iexMs: 100 + idx,
      rgMs: 150 + idx,
      speedupPct: 20,
      phaseMs: { discover: 10, scan: 20, aggregate: 5, total: 35 },
      slowestFiles: [{ path: "x", duration_ms: 2, bytes: 10 }],
      linuxDominantFile: {
        target_class: "linux_amd_asic_reg_giant_header",
        min_bytes: 8388608,
        targeted_files_scanned: 0,
        targeted_bytes_scanned: 0,
        targeted_slowest_files: 0,
        targeted_slowest_bytes: 0,
        eligible_files: 0,
        activated_files: 0,
        bailout_files: 0,
        max_shard_threads: 0,
        max_range_count: 0,
        max_chunk_bytes: 0,
      },
      matchCount: idx,
      corpus: "tools/data/corpus",
    };

    lines.push(`  test("report schema ${String(idx).padStart(3, "0")}", () => {`);
    lines.push(`    const run = ${JSON.stringify(run)};`);
    lines.push("    expect(() => validateRun(run)).not.toThrow();");
    lines.push("    const parsed = parseJsonl(`${JSON.stringify(run)}\\n`);");
    lines.push("    expect(parsed).toHaveLength(1);");
    lines.push("    expect(parsed[0].runId).toBe(run.runId);");
    lines.push("  });");
  }

  lines.push("});");
  lines.push("");
  return lines.join("\n");
}

function createIntegrationFile(fileIdx, startTestIdx, perFile) {
  const lines = [];
  lines.push('import { describe, expect, test } from "vitest";');
  lines.push('import { runSearchJson } from "../helpers/iex-cli";');
  lines.push("");
  lines.push('const corpus = "tools/data/corpus";');
  lines.push("");
  lines.push(`describe("materialized integration suite ${fileIdx}", () => {`);

  for (let i = 0; i < perFile; i += 1) {
    const idx = startTestIdx + i;
    const variant = idx % 4;
    let expr = "lit:ERROR && lit:timeout";
    if (variant === 1) expr = "prefix:WARN";
    if (variant === 2) expr = `re:\\bmodule=${idx % 9}\\b`;
    if (variant === 3) expr = "lit:INFO || lit:ERROR";

    lines.push(`  test("integration search ${String(idx).padStart(3, "0")}", () => {`);
    lines.push(`    const report = runSearchJson(${jsString(expr)}, corpus, ["--max-hits", "1000"]);`);
    lines.push("    const stats = report.stats;");
    lines.push("    const timings = stats.timings;");
    lines.push("    expect(report.expression).toBeDefined();");
    lines.push("    expect(stats.files_discovered).toBeGreaterThan(0);");
    lines.push("    expect(stats.files_scanned).toBeGreaterThan(0);");
    lines.push("    expect(stats.files_discovered).toBeGreaterThanOrEqual(stats.files_scanned);");
    lines.push("    expect(stats.bytes_scanned).toBeGreaterThan(0);");
    lines.push("    expect(stats.matches_found).toBeGreaterThanOrEqual(0);");
    lines.push("    expect(Number.isFinite(timings.discover_ms)).toBe(true);");
    lines.push("    expect(Number.isFinite(timings.scan_ms)).toBe(true);");
    lines.push("    expect(Number.isFinite(timings.aggregate_ms)).toBe(true);");
    lines.push("    expect(Number.isFinite(timings.total_ms)).toBe(true);");
    lines.push("    expect(timings.total_ms).toBeGreaterThanOrEqual(timings.scan_ms);");
    lines.push("    expect(timings.total_ms).toBeGreaterThanOrEqual(timings.aggregate_ms);");
    lines.push("    expect(stats.concurrency.available_threads).toBeGreaterThan(0);");
    lines.push("    expect(stats.concurrency.outer_scan_threads).toBeGreaterThan(0);");
    lines.push("    expect([\"materialized\", \"materialized_prepared\", \"streaming_stats_only\"]).toContain(stats.concurrency.execution_mode);");
    lines.push("    expect(Array.isArray(stats.slowest_files)).toBe(true);");
    lines.push("    expect(stats.slowest_files.length).toBeGreaterThan(0);");
    lines.push("    for (const slowestFile of stats.slowest_files.slice(0, 3)) {");
    lines.push("      expect(typeof slowestFile.path).toBe(\"string\");");
    lines.push("      expect(slowestFile.path.length).toBeGreaterThan(0);");
    lines.push("      expect(Number.isFinite(slowestFile.duration_ms)).toBe(true);");
    lines.push("      expect(slowestFile.duration_ms).toBeGreaterThanOrEqual(0);");
    lines.push("      expect(Number.isFinite(slowestFile.bytes)).toBe(true);");
    lines.push("      expect(slowestFile.bytes).toBeGreaterThan(0);");
    lines.push("    }");
    lines.push("  });");
  }

  lines.push("});");
  lines.push("");
  return lines.join("\n");
}

function createPerfFile(fileIdx, startTestIdx, perFile) {
  const lines = [];
  lines.push('import { describe, expect, test } from "vitest";');
  lines.push('import { buildIxSearchArgs, buildPerformanceCaseMatrix } from "../../tools/scripts/lib/perf-case-matrix.mjs";');
  lines.push("");
  lines.push("const matrix = buildPerformanceCaseMatrix({ count: 888 });");
  lines.push("");
  lines.push(`describe("materialized benchmark suite ${fileIdx}", () => {`);

  for (let i = 0; i < perFile; i += 1) {
    const idx = startTestIdx + i;
    lines.push(`  test("benchmark contract ${String(idx).padStart(3, "0")}", () => {`);
    lines.push(`    const caseSpec = matrix[${idx - 1}];`);
    lines.push("    expect(caseSpec).toBeDefined();");
    lines.push("    expect(caseSpec.id).toMatch(/^ix-perf-\\d{4}$/);");
    lines.push(`    expect(caseSpec.ordinal).toBe(${idx});`);
    lines.push("    expect(caseSpec.expression.length).toBeGreaterThan(8);");
    lines.push("    expect(caseSpec.retentionRule.length).toBeGreaterThan(20);");
    lines.push("    expect(caseSpec.retentionRule).toMatch(/aggregate timing|parity|promotion|match-count|current canonical|neutral-or-better/);");
    lines.push("    expect(caseSpec.expectedHotspot).toBe(\"scan\");");
    lines.push("    expect(caseSpec.statsOnly).toBe(true);");
    lines.push("    expect(caseSpec.tags.length).toBe(3);");
    lines.push("    const args = buildIxSearchArgs(caseSpec);");
    lines.push("    expect(args).toEqual([\"search\", caseSpec.expression, caseSpec.corpus, \"--json\", \"--stats-only\"]);");
    lines.push("  });");
  }

  lines.push("});");
  lines.push("");
  return lines.join("\n");
}

function adversarialParserExpression(index) {
  const mod = index % 6;
  if (mod === 0) {
    return {
      expr: `lit:value-${index} && re:\\b(session|handshake|checkpoint)\\b && suffix:trace=${index % 17}`,
      mode: "all",
      count: 3,
    };
  }
  if (mod === 1) {
    return {
      expr: `prefix:WARN || prefix:ERROR || lit:latency-${index}`,
      mode: "any",
      count: 3,
    };
  }
  if (mod === 2) {
    return {
      expr: `re:\\w{${3 + (index % 5)}}\\s+\\w{${4 + (index % 4)}}\\s+\\w{${5 + (index % 3)}}`,
      mode: "all",
      count: 1,
    };
  }
  if (mod === 3) {
    return {
      expr: `lit:alpha-${index} && prefix:module-${index % 13} && suffix:stage-${index % 11}`,
      mode: "all",
      count: 3,
    };
  }
  if (mod === 4) {
    return {
      expr: `lit:unicode-${index} || re:\\b[A-Z]{${2 + (index % 4)}}-[0-9]{${2 + (index % 3)}}\\b`,
      mode: "any",
      count: 2,
    };
  }
  return {
    expr: `lit:ampersand-${index} && lit:pipe-${index}`,
    mode: "all",
    count: 2,
  };
}

function createAdversarialParserFile(fileIdx, startTestIdx, perFile) {
  const lines = [];
  lines.push('import { describe, expect, test } from "vitest";');
  lines.push('import { runExplain } from "../helpers/iex-cli";');
  lines.push("");
  lines.push(`describe("adversarial parser suite ${fileIdx}", () => {`);

  for (let i = 0; i < perFile; i += 1) {
    const idx = startTestIdx + i;
    const spec = adversarialParserExpression(idx);
    lines.push(`  test("ambiguous grammar pressure ${String(idx).padStart(4, "0")}", () => {`);
    lines.push(`    const explained = runExplain(${jsString(spec.expr)});`);
    lines.push(`    expect(explained.source).toBe(${jsString(spec.expr)});`);
    lines.push(`    expect(explained.mode).toBe(${jsString(spec.mode)});`);
    lines.push(`    expect(explained.predicates).toHaveLength(${spec.count});`);
    lines.push('    for (const predicate of explained.predicates) {');
    lines.push('      expect(predicate.type.length).toBeGreaterThan(0);');
    lines.push('      expect(predicate.value.length).toBeGreaterThan(0);');
    lines.push("    }");
    lines.push("  });");
  }

  lines.push("});");
  lines.push("");
  return lines.join("\n");
}

function createAdversarialMetricsFile(fileIdx, startTestIdx, perFile) {
  const lines = [];
  lines.push('import { describe, expect, test } from "vitest";');
  lines.push('import { classifyHotspot, computeRatio, linearRegressionSlope, meanAbsoluteDelta, semivarianceAbove, summarizeSeries } from "../../tools/scripts/lib/metrics.mjs";');
  lines.push("");
  lines.push(`describe("adversarial metrics suite ${fileIdx}", () => {`);

  for (let i = 0; i < perFile; i += 1) {
    const idx = startTestIdx + i;
    const base = 20 + (idx % 31);
    const slope = (idx % 7) - 3;
    const spike = idx % 5 === 0 ? base * 9 : base + 4;
    const values = Array.from({ length: 17 }, (_, n) => base + slope * n + (n % 4));
    values[values.length - 1] = spike;
    const threshold = base + 10;
    const phase = idx % 3 === 0
      ? { discover: 91, scan: 90, aggregate: 1 }
      : idx % 3 === 1
        ? { discover: 3, scan: 144, aggregate: 12 }
        : { discover: 8, scan: 7, aggregate: 77 };
    const expectedHotspot = phase.scan >= phase.discover && phase.scan >= phase.aggregate
      ? "scan"
      : phase.discover >= phase.scan && phase.discover >= phase.aggregate
        ? "discover"
        : "aggregate";

    lines.push(`  test("tail-risk telemetry invariant ${String(idx).padStart(4, "0")}", () => {`);
    lines.push(`    const values = ${JSON.stringify(values)};`);
    lines.push("    const summary = summarizeSeries([...values, Number.NaN, Number.POSITIVE_INFINITY]);");
    lines.push("    expect(summary.count).toBe(values.length);");
    lines.push("    expect(summary.min).toBeLessThanOrEqual(summary.median);");
    lines.push("    expect(summary.median).toBeLessThanOrEqual(summary.max);");
    lines.push("    expect(summary.p90).toBeGreaterThanOrEqual(summary.median);");
    lines.push(`    expect(semivarianceAbove(values, ${threshold})).toBeGreaterThanOrEqual(0);`);
    lines.push("    expect(meanAbsoluteDelta(values)).toBeGreaterThanOrEqual(0);");
    lines.push("    expect(Number.isFinite(linearRegressionSlope(values))).toBe(true);");
    lines.push(`    expect(classifyHotspot({ phaseMs: ${JSON.stringify(phase)} })).toBe(${jsString(expectedHotspot)});`);
    lines.push(`    expect(computeRatio(${idx + 1}, ${idx + 2})).toBeGreaterThan(0);`);
    lines.push("  });");
  }

  lines.push("});");
  lines.push("");
  return lines.join("\n");
}

function createAdversarialContractFile(fileIdx, startTestIdx, perFile) {
  const lines = [];
  lines.push('import { describe, expect, test } from "vitest";');
  lines.push('import { parseJsonl, validateRun } from "../../tools/scripts/lib/metrics.mjs";');
  lines.push("");
  lines.push(`describe("adversarial report contract suite ${fileIdx}", () => {`);

  for (let i = 0; i < perFile; i += 1) {
    const idx = startTestIdx + i;
    const validRun = {
      runId: `adversarial-${idx}`,
      timestamp: `2026-05-06T00:${String(idx % 60).padStart(2, "0")}:00.000Z`,
      iexMs: 40 + idx,
      rgMs: 80 + idx,
      speedupPct: 50,
      phaseMs: { discover: idx % 9, scan: 20 + (idx % 13), aggregate: idx % 7, total: 40 + idx },
      slowestFiles: [{ path: `tail-${idx}.txt`, duration_ms: idx % 100, bytes: 1024 + idx }],
      linuxDominantFile: {
        target_class: "linux_amd_asic_reg_giant_header",
        min_bytes: 8388608,
        targeted_files_scanned: idx % 3,
        targeted_bytes_scanned: idx * 17,
        targeted_slowest_files: idx % 2,
        targeted_slowest_bytes: idx * 13,
        eligible_files: idx % 5,
        activated_files: idx % 4,
        bailout_files: idx % 2,
        max_shard_threads: 0,
        max_range_count: 0,
        max_chunk_bytes: 0,
      },
      matchCount: idx,
      corpus: "tools/data/corpus",
    };
    const invalidRun = idx % 2 === 0
      ? { ...validRun, runId: "" }
      : { ...validRun, phaseMs: { discover: 1, scan: 2, aggregate: 3, total: Number.NaN } };

    lines.push(`  test("malformed benchmark ledger rejection ${String(idx).padStart(4, "0")}", () => {`);
    lines.push(`    const validRun = ${JSON.stringify(validRun)};`);
    lines.push("    expect(() => validateRun(validRun)).not.toThrow();");
    lines.push(`    const invalidRun = ${JSON.stringify(invalidRun)};`);
    lines.push("    expect(() => validateRun(invalidRun)).toThrow();");
    lines.push("    const parsed = parseJsonl(`\\n${JSON.stringify(validRun)}\\n\\n`);");
    lines.push("    expect(parsed).toHaveLength(1);");
    lines.push("    expect(parsed[0].runId).toBe(validRun.runId);");
    lines.push("  });");
  }

  lines.push("});");
  lines.push("");
  return lines.join("\n");
}

function createAdversarialAbiFile(fileIdx, startTestIdx, perFile) {
  const lines = [];
  lines.push('import { describe, expect, test } from "vitest";');
  lines.push("");
  lines.push("function terminalSentinels(stdout) {");
  lines.push('  return stdout.split(/\\r?\\n/).filter((line) => line.includes("ix.result.v1"));');
  lines.push("}");
  lines.push("");
  lines.push("function parseTerminal(line) {");
  lines.push('  const jsonStart = line.indexOf("{");');
  lines.push("  if (jsonStart < 0) throw new Error('missing terminal json');");
  lines.push("  const jsonEnd = line.lastIndexOf('}');");
  lines.push("  if (jsonEnd < jsonStart) throw new Error('unterminated terminal json');");
  lines.push("  return JSON.parse(line.slice(jsonStart, jsonEnd + 1));");
  lines.push("}");
  lines.push("");
  lines.push(`describe("adversarial agent ABI suite ${fileIdx}", () => {`);

  for (let i = 0; i < perFile; i += 1) {
    const idx = startTestIdx + i;
    const terminal = {
      bytes: 1024 + idx,
      cmd: "search",
      expr: `lit:abi-${idx}`,
      files: { discovered: 3 + (idx % 5), scanned: 3 + (idx % 5), skipped: 0 },
      matches: idx % 11,
      ms: { discover: idx % 3, scan: 10 + (idx % 9), aggregate: idx % 2, total: 20 + (idx % 13) },
      status: "ok",
    };
    lines.push(`  test("single terminal sentinel invariant ${String(idx).padStart(4, "0")}", () => {`);
    lines.push(`    const terminal = ${JSON.stringify(terminal)};`);
    lines.push('    const stdout = `hit one\\nhit two\\n-- ix.result.v1 ${JSON.stringify(terminal)} --\\n`;');
    lines.push("    const sentinels = terminalSentinels(stdout);");
    lines.push("    expect(sentinels).toHaveLength(1);");
    lines.push("    const parsed = parseTerminal(sentinels[0]);");
    lines.push("    expect(parsed.status).toBe('ok');");
    lines.push("    expect(parsed.matches).toBe(terminal.matches);");
    lines.push("    expect(parsed.files.scanned).toBeGreaterThanOrEqual(parsed.files.skipped);");
    lines.push("  });");
  }

  lines.push("});");
  lines.push("");
  return lines.join("\n");
}

function createAdversarialMatrixFile(fileIdx, startTestIdx, perFile) {
  const lines = [];
  lines.push('import { describe, expect, test } from "vitest";');
  lines.push('import { buildIxSearchArgs, buildPerformanceCaseMatrix, summarizePerformanceMatrix } from "../../tools/scripts/lib/perf-case-matrix.mjs";');
  lines.push("");
  lines.push("const matrix = buildPerformanceCaseMatrix({ count: 1776 });");
  lines.push("");
  lines.push(`describe("adversarial matrix suite ${fileIdx}", () => {`);

  for (let i = 0; i < perFile; i += 1) {
    const idx = startTestIdx + i;
    const matrixIndex = (idx - 1) % 1776;
    const wallA = 10 + (idx % 37);
    const wallB = wallA + 70 + (idx % 19);
    const wallC = wallA + 140 + (idx % 23);
    lines.push(`  test("regression ranking pressure ${String(idx).padStart(4, "0")}", () => {`);
    lines.push(`    const caseSpec = matrix[${matrixIndex}];`);
    lines.push("    expect(caseSpec).toBeDefined();");
    lines.push("    expect(caseSpec.statsOnly).toBe(true);");
    lines.push("    expect(caseSpec.retentionRule).toMatch(/parity|neutral|canonical|scan|contract|authority/i);");
    lines.push("    const args = buildIxSearchArgs(caseSpec, { threads: 7 });");
    lines.push('    expect(args).toContain("--threads");');
    lines.push('    expect(args).toContain("7");');
    lines.push(`    const summary = summarizePerformanceMatrix([
      { case: { ...caseSpec, id: caseSpec.id + "-a" }, selected: { wallMs: ${wallA}, engineMs: ${wallA}, scanMs: ${wallA - 1}, filesScanned: 1, bytesScanned: 10, matchCount: 1, slowestFiles: [] }, scanSharePct: 90 },
      { case: { ...caseSpec, id: caseSpec.id + "-b" }, selected: { wallMs: ${wallB}, engineMs: ${wallB}, scanMs: ${wallB - 1}, filesScanned: 1, bytesScanned: 10, matchCount: 1, slowestFiles: [] }, scanSharePct: 90 },
      { case: { ...caseSpec, id: caseSpec.id + "-c" }, selected: { wallMs: ${wallC}, engineMs: ${wallC}, scanMs: ${wallC - 1}, filesScanned: 1, bytesScanned: 10, matchCount: 1, slowestFiles: [] }, scanSharePct: 90 },
    ]);`);
    lines.push(`    expect(summary.slowest[0].wallMs).toBe(${wallC});`);
    lines.push("    expect(summary.families[0].count).toBe(3);");
    lines.push("  });");
  }

  lines.push("});");
  lines.push("");
  return lines.join("\n");
}

let counter = 1;

for (let i = 0; i < 6; i += 1) {
  writeFile(`parser-${String(i + 1).padStart(2, "0")}.test.ts`, createParserFile(i + 1, counter, 40));
  counter += 40;
}

for (let i = 0; i < 5; i += 1) {
  writeFile(`metrics-${String(i + 1).padStart(2, "0")}.test.ts`, createMetricsFile(i + 1, counter, 40));
  counter += 40;
}

for (let i = 0; i < 4; i += 1) {
  writeFile(`contracts-${String(i + 1).padStart(2, "0")}.test.ts`, createContractFile(i + 1, counter, 32));
  counter += 32;
}

for (let i = 0; i < 4; i += 1) {
  writeFile(`integration-${String(i + 1).padStart(2, "0")}.test.ts`, createIntegrationFile(i + 1, counter, 40));
  counter += 40;
}

for (let i = 0; i < 4; i += 1) {
  writeFile(`performance-${String(i + 1).padStart(2, "0")}.test.ts`, createPerfFile(i + 1, counter, 40));
  counter += 40;
}

for (let i = 0; i < 4; i += 1) {
  writeFile(`adversarial-parser-${String(i + 1).padStart(2, "0")}.test.ts`, createAdversarialParserFile(i + 1, counter, 36));
  counter += 36;
}

for (let i = 0; i < 6; i += 1) {
  writeFile(`adversarial-metrics-${String(i + 1).padStart(2, "0")}.test.ts`, createAdversarialMetricsFile(i + 1, counter, 48));
  counter += 48;
}

for (let i = 0; i < 4; i += 1) {
  writeFile(`adversarial-contracts-${String(i + 1).padStart(2, "0")}.test.ts`, createAdversarialContractFile(i + 1, counter, 48));
  counter += 48;
}

for (let i = 0; i < 3; i += 1) {
  writeFile(`adversarial-abi-${String(i + 1).padStart(2, "0")}.test.ts`, createAdversarialAbiFile(i + 1, counter, 48));
  counter += 48;
}

for (let i = 0; i < 3; i += 1) {
  writeFile(`adversarial-matrix-${String(i + 1).padStart(2, "0")}.test.ts`, createAdversarialMatrixFile(i + 1, counter, 40));
  counter += 40;
}

console.log(`materialized tests generated: ${counter - 1}`);
