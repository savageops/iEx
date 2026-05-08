import { existsSync, mkdirSync, writeFileSync, readFileSync } from "node:fs";
import os from "node:os";
import path from "node:path";
import { spawnSync } from "node:child_process";
import { describe, beforeAll, expect, test } from "vitest";

const ROOT = process.cwd();
const ZIG_EXE =
  process.env.ZIG_EXE ??
  "C:\\Users\\Savage\\.local\\zig\\zig-x86_64-windows-0.16.0\\zig.exe";
const ZIG_IX = path.join(ROOT, "zig", "zig-out", "bin", "ix-zig.exe");
const RUST_IX = path.join(ROOT, "target", "release", "ix.exe");
const REPORT_PATH = path.join(ROOT, "tools", "reports", "zig-vs-rust-latest.json");
const WARMUP_RUNS = 1;
const SAMPLE_RUNS = 5;

type BinaryTag = "rust" | "zig";

type TimedRun = {
  binary: BinaryTag;
  wallMs: number;
  engineMs: number;
  scanMs: number;
  matchCount: number;
  filesScanned: number;
  bytesScanned: number;
};

type BenchmarkCase = {
  id: string;
  label: string;
  expression: string;
  corpus: string;
  extraArgs: string[];
};

type BenchmarkResult = {
  case: BenchmarkCase;
  rust: { median: TimedRun; runs: TimedRun[] };
  zig: { median: TimedRun; runs: TimedRun[] };
  matchCountParity: boolean;
  bytesScannedParity: boolean;
  filesScannedParity: boolean;
  speedup: number;
};

function timedSearch(binary: string, tag: BinaryTag, args: string[]): TimedRun {
  const started = process.hrtime.bigint();
  const result = spawnSync(binary, args, {
    cwd: ROOT,
    encoding: "utf8",
    maxBuffer: 128 * 1024 * 1024,
    windowsHide: true,
    timeout: 120_000,
  });
  const ended = process.hrtime.bigint();
  const wallMs = Number(ended - started) / 1_000_000;

  if (result.status !== 0) {
    throw new Error(
      `${tag} search failed (code=${result.status})\nargs: ${args.join(" ")}\nstderr: ${result.stderr}`,
    );
  }

  const report = JSON.parse(result.stdout);
  return {
    binary: tag,
    wallMs,
    engineMs: report.stats.timings.total_ms,
    scanMs: report.stats.timings.scan_ms,
    matchCount: report.stats.matches_found,
    filesScanned: report.stats.files_scanned,
    bytesScanned: report.stats.bytes_scanned,
  };
}

function median(runs: TimedRun[]): TimedRun {
  const sorted = [...runs].sort((a, b) => a.wallMs - b.wallMs);
  return sorted[Math.floor(sorted.length / 2)];
}

function runBenchmark(caseSpec: BenchmarkCase): BenchmarkResult {
  const searchArgs = [
    "search",
    caseSpec.expression,
    caseSpec.corpus,
    "--json",
    "--stats-only",
    ...caseSpec.extraArgs,
  ];

  // warmup
  for (let i = 0; i < WARMUP_RUNS; i++) {
    timedSearch(RUST_IX, "rust", searchArgs);
    timedSearch(ZIG_IX, "zig", searchArgs);
  }

  // measured runs
  const rustRuns: TimedRun[] = [];
  const zigRuns: TimedRun[] = [];
  for (let i = 0; i < SAMPLE_RUNS; i++) {
    rustRuns.push(timedSearch(RUST_IX, "rust", searchArgs));
    zigRuns.push(timedSearch(ZIG_IX, "zig", searchArgs));
  }

  const rustMedian = median(rustRuns);
  const zigMedian = median(zigRuns);

  return {
    case: caseSpec,
    rust: { median: rustMedian, runs: rustRuns },
    zig: { median: zigMedian, runs: zigRuns },
    matchCountParity: rustMedian.matchCount === zigMedian.matchCount,
    bytesScannedParity: rustMedian.bytesScanned === zigMedian.bytesScanned,
    filesScannedParity: rustMedian.filesScanned === zigMedian.filesScanned,
    speedup: zigMedian.wallMs > 0 ? zigMedian.wallMs / rustMedian.wallMs : 0,
  };
}

// --- corpus generation ---

const FIXTURE_DIR = path.join(os.tmpdir(), "ix-zig-vs-rust-bench");
const SMALL_FIXTURE = path.join(FIXTURE_DIR, "small.txt");
const MEDIUM_DIR = path.join(FIXTURE_DIR, "medium");
const LARGE_DIR = path.join(FIXTURE_DIR, "large");

function ensureFixtures() {
  mkdirSync(FIXTURE_DIR, { recursive: true });

  // small: single file, 1000 lines
  if (!existsSync(SMALL_FIXTURE)) {
    const lines: string[] = [];
    for (let i = 0; i < 1000; i++) {
      if (i % 10 === 0) {
        lines.push(`line ${i}: needle in a haystack of words and tokens`);
      } else {
        lines.push(
          `line ${i}: the quick brown fox jumps over the lazy dog ${i}`,
        );
      }
    }
    writeFileSync(SMALL_FIXTURE, lines.join("\n") + "\n", "utf8");
  }

  // medium: 100 files, ~50 lines each
  if (!existsSync(MEDIUM_DIR)) {
    mkdirSync(MEDIUM_DIR, { recursive: true });
    for (let f = 0; f < 100; f++) {
      const lines: string[] = [];
      for (let i = 0; i < 50; i++) {
        if (i % 7 === 0) {
          lines.push(`// TODO: fix this needle pattern in module ${f}`);
        } else if (i % 13 === 0) {
          lines.push(`// FIXME: error handling for session timeout`);
        } else {
          lines.push(
            `const value_${i} = compute(${f}, ${i}); // generated stub`,
          );
        }
      }
      writeFileSync(
        path.join(MEDIUM_DIR, `file_${String(f).padStart(3, "0")}.txt`),
        lines.join("\n") + "\n",
        "utf8",
      );
    }
  }

  // large: 500 files, ~200 lines each (~1.5MB total)
  if (!existsSync(LARGE_DIR)) {
    mkdirSync(LARGE_DIR, { recursive: true });
    for (let f = 0; f < 500; f++) {
      const lines: string[] = [];
      for (let i = 0; i < 200; i++) {
        if (i % 20 === 0) {
          lines.push(
            `pub fn process_${f}_${i}(input: &str) -> Result<String> { // needle`,
          );
        } else if (i % 50 === 0) {
          lines.push(`    // Sherlock Holmes deduced the PM_RESUME event`);
        } else {
          lines.push(
            `    let result_${i} = transform(input, ${f * 200 + i}); // stub line padding for realistic file sizes`,
          );
        }
      }
      const subdir = path.join(LARGE_DIR, `mod_${String(f % 10)}`);
      mkdirSync(subdir, { recursive: true });
      writeFileSync(
        path.join(subdir, `component_${String(f).padStart(3, "0")}.txt`),
        lines.join("\n") + "\n",
        "utf8",
      );
    }
  }
}

// --- benchmark cases ---

function buildCases(): BenchmarkCase[] {
  const cases: BenchmarkCase[] = [
    {
      id: "single-file-literal",
      label: "Single file, literal search",
      expression: "lit:needle",
      corpus: SMALL_FIXTURE,
      extraArgs: [],
    },
    {
      id: "single-file-regex",
      label: "Single file, regex search",
      expression: "re:needle",
      corpus: SMALL_FIXTURE,
      extraArgs: [],
    },
    {
      id: "medium-dir-literal",
      label: "100 files, literal search",
      expression: "lit:needle",
      corpus: MEDIUM_DIR,
      extraArgs: [],
    },
    {
      id: "medium-dir-regex-alternation",
      label: "100 files, regex alternation",
      expression: "re:TODO|FIXME",
      corpus: MEDIUM_DIR,
      extraArgs: [],
    },
    {
      id: "medium-dir-word-boundary",
      label: "100 files, word boundary regex",
      expression: "re:\\bsession\\b",
      corpus: MEDIUM_DIR,
      extraArgs: [],
    },
    {
      id: "large-dir-literal",
      label: "500 files, literal search",
      expression: "lit:needle",
      corpus: LARGE_DIR,
      extraArgs: [],
    },
    {
      id: "large-dir-regex",
      label: "500 files, regex search",
      expression: "re:process_\\d+_\\d+",
      corpus: LARGE_DIR,
      extraArgs: [],
    },
    {
      id: "large-dir-case-insensitive",
      label: "500 files, case-insensitive literal",
      expression: "re:(?i)sherlock",
      corpus: LARGE_DIR,
      extraArgs: [],
    },
    {
      id: "large-dir-boolean-and",
      label: "500 files, boolean AND",
      expression: "lit:Result && lit:process",
      corpus: LARGE_DIR,
      extraArgs: [],
    },
    {
      id: "large-dir-prefix",
      label: "500 files, prefix match",
      expression: "prefix:pub fn",
      corpus: LARGE_DIR,
      extraArgs: [],
    },
  ];

  // add real-codebase case if this repo is big enough
  const cratesDir = path.join(ROOT, "crates");
  if (existsSync(cratesDir)) {
    cases.push({
      id: "real-codebase-literal",
      label: "Real codebase (crates/), literal search",
      expression: "lit:SearchConfig",
      corpus: cratesDir,
      extraArgs: [],
    });
    cases.push({
      id: "real-codebase-regex",
      label: "Real codebase (crates/), regex search",
      expression: "re:fn\\s+\\w+",
      corpus: cratesDir,
      extraArgs: [],
    });
  }

  return cases;
}

// --- test suite ---

describe("zig vs rust search performance", () => {
  const results: BenchmarkResult[] = [];

  beforeAll(() => {
    expect(existsSync(ZIG_IX), `Zig binary must exist at ${ZIG_IX}`).toBe(true);
    expect(existsSync(RUST_IX), `Rust binary must exist at ${RUST_IX}`).toBe(
      true,
    );
    ensureFixtures();
  });

  const cases = buildCases();

  for (const caseSpec of cases) {
    test(
      `${caseSpec.id}: ${caseSpec.label}`,
      () => {
        const result = runBenchmark(caseSpec);
        results.push(result);

        // correctness: match counts must agree
        expect(result.matchCountParity).toBe(true);
        expect(result.bytesScannedParity).toBe(true);
      },
      60_000,
    );
  }

  test("writes comparative performance report", () => {
    const report = {
      generatedAt: new Date().toISOString(),
      platform: `${os.platform()} ${os.arch()} ${os.cpus()[0]?.model ?? "unknown"}`,
      samples: SAMPLE_RUNS,
      warmup: WARMUP_RUNS,
      summary: {
        totalCases: results.length,
        parityFailures: results.filter((r) => !r.matchCountParity).length,
        zigFasterCount: results.filter((r) => r.speedup < 1).length,
        rustFasterCount: results.filter((r) => r.speedup > 1).length,
        tiedCount: results.filter((r) => r.speedup === 1).length,
      },
      cases: results.map((r) => ({
        id: r.case.id,
        label: r.case.label,
        expression: r.case.expression,
        corpus: r.case.corpus,
        matchCountParity: r.matchCountParity,
        bytesScannedParity: r.bytesScannedParity,
        filesScannedParity: r.filesScannedParity,
        rust: {
          wallMs: r.rust.median.wallMs,
          engineMs: r.rust.median.engineMs,
          scanMs: r.rust.median.scanMs,
          matchCount: r.rust.median.matchCount,
          filesScanned: r.rust.median.filesScanned,
          bytesScanned: r.rust.median.bytesScanned,
        },
        zig: {
          wallMs: r.zig.median.wallMs,
          engineMs: r.zig.median.engineMs,
          scanMs: r.zig.median.scanMs,
          matchCount: r.zig.median.matchCount,
          filesScanned: r.zig.median.filesScanned,
          bytesScanned: r.zig.median.bytesScanned,
        },
        speedup: Number(r.speedup.toFixed(2)),
        winner: r.speedup < 1 ? "zig" : r.speedup > 1 ? "rust" : "tied",
      })),
    };

    mkdirSync(path.dirname(REPORT_PATH), { recursive: true });
    writeFileSync(REPORT_PATH, `${JSON.stringify(report, null, 2)}\n`, "utf8");

    // print summary table
    const pad = (s: string, w: number) => s.length >= w ? s : " ".repeat(w - s.length) + s;
    const padR = (s: string, w: number) => s.length >= w ? s : s + " ".repeat(w - s.length);
    console.log("\n=== Zig vs Rust Search Performance ===\n");
    console.log(
      `${padR("Case", 30)} ${pad("Rust (ms)", 10)} ${pad("Zig (ms)", 10)} ${pad("Speedup", 10)} ${pad("Parity", 8)} Winner`,
    );
    console.log("-".repeat(85));
    for (const c of report.cases) {
      console.log(
        `${padR(c.id.slice(0, 30), 30)} ${pad(c.rust.wallMs.toFixed(1), 10)} ${pad(c.zig.wallMs.toFixed(1), 10)} ${pad(c.speedup.toFixed(2) + "x", 10)} ${pad(c.matchCountParity ? "OK" : "FAIL", 8)} ${c.winner}`,
      );
    }
    console.log("");
  });
});
