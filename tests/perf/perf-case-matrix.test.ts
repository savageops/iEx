import { describe, expect, test } from "vitest";

import {
  DEFAULT_PERF_CASE_COUNT,
  buildIxSearchArgs,
  buildPerformanceCaseMatrix,
  filterAvailablePerfCases,
  summarizePerformanceMatrix,
} from "../../tools/scripts/lib/perf-case-matrix.mjs";

describe("performance case matrix contract", () => {
  test("materializes the default 888-case diagnostic corpus", () => {
    const matrix = buildPerformanceCaseMatrix();
    const ids = new Set(matrix.map((caseSpec) => caseSpec.id));

    expect(matrix).toHaveLength(DEFAULT_PERF_CASE_COUNT);
    expect(ids.size).toBe(DEFAULT_PERF_CASE_COUNT);
    expect(matrix[0].id).toBe("ix-perf-0001");
    expect(matrix.at(-1)?.id).toBe("ix-perf-0888");

    for (let index = 0; index < matrix.length; index += 1) {
      const ordinal = index + 1;
      const expectedId = `ix-perf-${String(ordinal).padStart(4, "0")}`;
      expect(matrix[index].id).toBe(expectedId);
      expect(matrix[index].ordinal).toBe(ordinal);
    }
  });

  test("covers every current miss and near-miss family", () => {
    const families = new Set(buildPerformanceCaseMatrix().map((caseSpec) => caseSpec.family));

    expect(families).toContain("suite-en-no-literal");
    expect(families).toContain("suite-linux-literal");
    expect(families).toContain("suite-linux-word");
    expect(families).toContain("suite-linux-alternates");
    expect(families).toContain("suite-linux-no-literal");
    expect(families).toContain("suite-ru-literal-casei");
    expect(families).toContain("synthetic-absent-literal");
  });

  test("attaches retention discipline to each case", () => {
    const matrix = buildPerformanceCaseMatrix({ count: 64 });

    for (const caseSpec of matrix) {
      expect(caseSpec.expression.length).toBeGreaterThan(8);
      expect(caseSpec.corpus.length).toBeGreaterThan(0);
      expect(caseSpec.retentionRule.length).toBeGreaterThan(20);
      expect(caseSpec.retentionRule).toMatch(/aggregate timing|parity|promotion|match-count|current canonical|neutral-or-better/);
      expect(caseSpec.expectedHotspot).toBe("scan");
      expect(caseSpec.statsOnly).toBe(true);
      expect(caseSpec.tags).toHaveLength(3);
      expect(caseSpec.tags).toContain(caseSpec.family.includes("linux") ? "linux-suite" : "portable");
      expect(caseSpec.tags).toContain(caseSpec.family.includes("casei") ? "case-insensitive" : "case-sensitive");
    }
  });

  test("keeps family morphology aligned to benchmark pressure", () => {
    const matrix = buildPerformanceCaseMatrix({ count: 160 });

    for (const caseSpec of matrix) {
      if (caseSpec.family.endsWith("literal") && !caseSpec.family.includes("casei") && !caseSpec.family.includes("no-literal")) {
        expect(caseSpec.expression.startsWith("lit:")).toBe(true);
      }
      if (caseSpec.family.includes("casei")) {
        expect(caseSpec.expression).toContain("(?i)");
        expect(caseSpec.pressure).toMatch(/casefold|unicode/);
      }
      if (caseSpec.family.endsWith("word") || caseSpec.family.endsWith("no-literal")) {
        expect(caseSpec.expression).toContain("re:");
        expect(caseSpec.expression).toContain("\\");
      }
      if (caseSpec.family.endsWith("alternates")) {
        expect(caseSpec.expression).toContain("|");
        expect(caseSpec.pressure).toMatch(/multi_literal|linux_tree/);
      }
      if (caseSpec.family === "synthetic-absent-literal") {
        expect(caseSpec.expression).toMatch(/^lit:__ix_absent_probe_\d{4}__$/);
      }
    }
  });

  test("builds canonical ix stats-only argv without invoking a benchmark loop", () => {
    const [caseSpec] = buildPerformanceCaseMatrix({ count: 1 });
    const args = buildIxSearchArgs(caseSpec, { threads: 7 });

    expect(args).toEqual(["search", caseSpec.expression, caseSpec.corpus, "--json", "--stats-only", "--threads", "7"]);
  });

  test("filters unavailable suite corpora without deleting synthetic probes", () => {
    const matrix = buildPerformanceCaseMatrix({ count: 32, root: "Z:/definitely/missing/ix/root" });
    const available = filterAvailablePerfCases(matrix);

    expect(available).toHaveLength(0);
  });

  test("summarizes measured case results by family and slowest tail", () => {
    const [first, second] = buildPerformanceCaseMatrix({ count: 2 });
    const summary = summarizePerformanceMatrix([
      {
        case: first,
        selected: {
          wallMs: 12,
          engineMs: 10,
          scanMs: 8,
          filesScanned: 1,
          bytesScanned: 100,
          matchCount: 2,
          slowestFiles: [],
        },
        scanSharePct: 80,
      },
      {
        case: second,
        selected: {
          wallMs: 20,
          engineMs: 16,
          scanMs: 12,
          filesScanned: 1,
          bytesScanned: 200,
          matchCount: 3,
          slowestFiles: [],
        },
        scanSharePct: 75,
      },
    ]);

    expect(summary.count).toBe(2);
    expect(summary.families).toHaveLength(2);
    expect(summary.families.map((entry) => entry.count).reduce((sum, count) => sum + count, 0)).toBe(summary.count);
    expect(summary.slowest[0].wallMs).toBe(20);
    expect(summary.slowest[0]).toMatchObject({
      id: second.id,
      family: second.family,
      expression: second.expression,
      corpus: second.corpus,
      engineMs: 16,
      scanMs: 12,
      filesScanned: 1,
      bytesScanned: 200,
      matchCount: 3,
    });
  });
});
