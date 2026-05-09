import { mkdtempSync, rmSync, writeFileSync } from "node:fs";
import os from "node:os";
import path from "node:path";
import { describe, expect, test } from "vitest";

import {
  assessPreviousIexComparatorAuthority,
  describeBenchmarkScenario,
  resolveBenchmarkReportPaths,
  resolveExplicitBinaryPath,
  resolveIexBenchBinaryPath,
  resolveIexBinaryPath,
  validatePreparedReplayBenchmarkOptions,
  validateRunOneBenchmarkOptions,
} from "../../tools/scripts/lib/benchmark-runner.mjs";

describe("benchmark runner explicit binary selection", () => {
  test("uses an explicit binary path without falling back to target/release", () => {
    const tempDir = mkdtempSync(path.join(os.tmpdir(), "iex-binary-path-"));
    const fakeBinary = path.join(tempDir, process.platform === "win32" ? "iex-cli.exe" : "iex-cli");

    try {
      writeFileSync(fakeBinary, "stub", "utf8");

      expect(resolveExplicitBinaryPath(fakeBinary)).toBe(fakeBinary);
      expect(resolveIexBinaryPath({ buildProfile: "release", iexBinaryPath: fakeBinary })).toBe(fakeBinary);
    } finally {
      rmSync(tempDir, { recursive: true, force: true });
    }
  });

  test("fails fast when the explicit binary path does not exist", () => {
    const missingBinary = path.join(os.tmpdir(), "iex-missing-binary", "iex-cli.exe");

    expect(() => resolveExplicitBinaryPath(missingBinary)).toThrow(/explicit iex binary not found/i);
    expect(() => resolveIexBinaryPath({ iexBinaryPath: missingBinary })).toThrow(/explicit iex binary not found/i);
  });

  test("describes the benchmark scenario contract explicitly", () => {
    const scenario = describeBenchmarkScenario({
      corpus: "E:/tmp/corpus",
      expression: "lit:needle",
    });

    expect(scenario.id).toBe("cli_stats_only_search");
    expect(scenario.label).toMatch(/stats-only/i);
    expect(scenario.contract).toMatch(/discovery plus scan/i);
    expect(scenario.preparedTargets).toBe(false);
    expect(scenario.statsOnly).toBe(true);
    expect(scenario.corpus).toBe("E:/tmp/corpus");
    expect(scenario.expression).toBe("lit:needle");
  });

  test("describes the prepared replay scenario separately", () => {
    const scenario = describeBenchmarkScenario({
      scenarioId: "prepared_corpus_replay",
      corpus: "E:/tmp/linux",
      expression: "lit:PM_RESUME",
      samples: 5,
    });

    expect(scenario.id).toBe("prepared_corpus_replay");
    expect(scenario.label).toMatch(/prepared/i);
    expect(scenario.contract).toMatch(/Discovery is paid once/i);
    expect(scenario.preparedTargets).toBe(true);
    expect(scenario.replayLoops).toBe(5);
  });

  test("marks non-ascii case-insensitive regex lanes as correctness-sensitive", () => {
    const scenario = describeBenchmarkScenario({
      corpus: "E:/tmp/ru.txt",
      expression: "re:(?i)Шерлок Холмс",
    });

    expect(scenario.correctnessSensitive).toBe(true);
    expect(scenario.comparisonDiscipline).toBe("match_count_parity_required");
    expect(scenario.correctnessNotes).toMatch(/match-count parity/i);
  });

  test("downgrades previous iEx authority when non-ascii casei match counts diverge", () => {
    const assessment = assessPreviousIexComparatorAuthority({
      expression: "re:(?i)Шерлок Холмс",
      previousAvailable: true,
      candidateMatchCount: 604,
      previousMatchCount: 583,
    });

    expect(assessment.authority).toBe("timing_only");
    expect(assessment.matchCountParity).toBe(false);
    expect(assessment.caveat).toMatch(/timing-only/i);
  });

  test("keeps previous iEx authority when non-ascii casei match counts align", () => {
    const assessment = assessPreviousIexComparatorAuthority({
      expression: "re:(?i)Шерлок Холмс",
      previousAvailable: true,
      candidateMatchCount: 604,
      previousMatchCount: 604,
    });

    expect(assessment.authority).toBe("authoritative");
    expect(assessment.matchCountParity).toBe(true);
    expect(assessment.caveat).toBeNull();
  });

  test("separates prepared report files from the canonical cli lane", () => {
    const cli = resolveBenchmarkReportPaths({ scenarioId: "cli_stats_only_search" });
    const prepared = resolveBenchmarkReportPaths({ scenarioId: "prepared_corpus_replay" });

    expect(cli.liveJsonl).toMatch(/live-metrics\.jsonl$/);
    expect(cli.latestJson).toMatch(/latest\.json$/);
    expect(prepared.liveJsonl).toMatch(/prepared-live-metrics\.jsonl$/);
    expect(prepared.latestJson).toMatch(/prepared-latest\.json$/);
    expect(prepared.liveJsonl).not.toBe(cli.liveJsonl);
    expect(prepared.latestJson).not.toBe(cli.latestJson);
  });

  test("uses an explicit iex-bench binary path without falling back to target/release", () => {
    const tempDir = mkdtempSync(path.join(os.tmpdir(), "iex-bench-binary-path-"));
    const fakeBinary = path.join(tempDir, process.platform === "win32" ? "iex-bench.exe" : "iex-bench");

    try {
      writeFileSync(fakeBinary, "stub", "utf8");
      expect(resolveIexBenchBinaryPath({ buildProfile: "release", iexBenchBinaryPath: fakeBinary })).toBe(fakeBinary);
    } finally {
      rmSync(tempDir, { recursive: true, force: true });
    }
  });

  test("rejects unknown options on both benchmark lane APIs", () => {
    expect(() => validateRunOneBenchmarkOptions({ scenarioId: "prepared_corpus_replay" })).toThrow(
      /runOneBenchmark received unknown option/,
    );
    expect(() => validatePreparedReplayBenchmarkOptions({ previousIexBinaryPath: "ix-old.exe" })).toThrow(
      /runPreparedReplayBenchmark received unknown option/,
    );
  });
});
