import { existsSync, mkdirSync, readFileSync } from "node:fs";
import path from "node:path";
import { spawnSync } from "node:child_process";
import { writeFileSync } from "node:fs";
import { beforeAll, describe, expect, test } from "vitest";

const ROOT = process.cwd();
const ZIG_ROOT = path.join(ROOT, "zig");
const ZIG_EXE =
  process.env.ZIG_EXE ??
  "C:\\Users\\Savage\\.local\\zig\\zig-x86_64-windows-0.16.0\\zig.exe";
const ZIG_IX = path.join(ZIG_ROOT, "zig-out", "bin", "ix-zig.exe");
const RUST_IX = path.join(ROOT, "target", "release", "ix.exe");

function run(command: string, args: string[], cwd = ZIG_ROOT): string {
  const result = spawnSync(command, args, {
    cwd,
    encoding: "utf8",
    windowsHide: true,
  });
  expect(result.status, `${command} ${args.join(" ")}\n${result.stderr}`).toBe(0);
  return result.stdout.trim().replaceAll("\\", "/");
}

function runFailure(command: string, args: string[], cwd = ZIG_ROOT) {
  const result = spawnSync(command, args, {
    cwd,
    encoding: "utf8",
    windowsHide: true,
  });
  expect(result.status, `${command} ${args.join(" ")}`).not.toBe(0);
  return {
    status: result.status,
    output: `${result.stdout}${result.stderr}`.trim().replaceAll("\\", "/"),
  };
}

function parseResultSentinel(output: string) {
  const match = output.match(/^-- ix\.result\.v1 (.*) --$/m);
  expect(match, output).not.toBeNull();
  return JSON.parse(match?.[1] ?? "{}");
}

function normalizePath(value: string): string {
  return value.replaceAll("\\", "/").replace(/\/+/g, "/");
}

function normalizeInspectContextJson(value: string) {
  const parsed = JSON.parse(value);
  for (const report of parsed.reports ?? []) {
    if (typeof report.path === "string") report.path = normalizePath(report.path);
  }
  return parsed;
}

describe("zig runtime parity", () => {
  beforeAll(() => {
    expect(existsSync(ZIG_EXE), "Zig 0.16.0 executable must exist").toBe(true);
    expect(existsSync(RUST_IX), "Rust oracle ix.exe must exist").toBe(true);
    run(ZIG_EXE, ["build", "--summary", "all"]);
  }, 120_000);

  test("explain JSON matches the Rust oracle for literal expressions", () => {
    const zig = run(ZIG_IX, ["explain", "lit:Zig"]);
    const rust = run(RUST_IX, ["explain", "lit:Zig"]);
    expect(JSON.parse(zig)).toEqual(JSON.parse(rust));
  });

  test("top-level help text matches the Rust oracle exactly", () => {
    expect(run(ZIG_IX, ["--help"])).toBe(run(RUST_IX, ["--help"]));
  });

  test("subcommand help text matches the Rust oracle exactly", () => {
    for (const args of [["search", "--help"], ["matches", "--help"], ["inspect", "--help"], ["explain", "--help"]]) {
      expect(run(ZIG_IX, args)).toBe(run(RUST_IX, args));
    }
  });

  test("top-level compat unsupported flags emit the same guided error as the Rust oracle", () => {
    const zig = runFailure(ZIG_IX, ["--bad"]);
    const rust = runFailure(RUST_IX, ["--bad"]);
    expect(zig.status).toBe(rust.status);
    expect(zig.output).toBe(rust.output);
  });

  test("inspect window output matches the Rust oracle for bounded file windows", () => {
    const args = ["inspect", "../AGENTS.md", "--range", "1:3"];
    expect(run(ZIG_IX, args)).toBe(run(RUST_IX, args));
  });

  test("inspect file windows preserve total-count and json output contracts", () => {
    for (const args of [
      ["inspect", "../AGENTS.md", "--total-count", "2"],
      ["inspect", "../AGENTS.md", "--range", "1:2", "--json"],
      ["inspect", "../AGENTS.md", "--range", "1:2", "--format", "records"],
      ["inspect", "../AGENTS.md", "--range", "1:2", "--format", "json"],
    ]) {
      expect(run(ZIG_IX, args)).toBe(run(RUST_IX, args));
    }
  });

  test("inspect file-window flags preserve Rust bounds, continuation, and multipath contracts", () => {
    const first = path.join(process.env.TEMP ?? ROOT, "ix-zig-runtime-parity-inspect-first.txt");
    const second = path.join(process.env.TEMP ?? ROOT, "ix-zig-runtime-parity-inspect-second.txt");
    writeFileSync(first, "alpha\nneedle\nomega\ntail\n", "utf8");
    writeFileSync(second, "one\ntwo\nthree\n", "utf8");

    for (const args of [
      ["inspect", first, "--skip", "1", "--limit", "2"],
      ["inspect", first, "--start-line", "2", "--end-line", "3"],
      ["inspect", first, "--head", "2"],
      ["inspect", first, second, "--range", "1:2", "--format", "records"],
    ]) {
      expect(run(ZIG_IX, args)).toBe(run(RUST_IX, args));
    }
    const jsonArgs = ["inspect", first, second, "--range", "1:2", "--json"];
    expect(normalizeInspectContextJson(run(ZIG_IX, jsonArgs))).toEqual(normalizeInspectContextJson(run(RUST_IX, jsonArgs)));
  });

  test("search sentinel preserves Rust-visible functional state", () => {
    const args = ["search", "lit:Zig", "../AGENTS.md", "--max-hits", "5"];
    const zig = parseResultSentinel(run(ZIG_IX, args));
    const rust = parseResultSentinel(run(RUST_IX, args));

    expect(zig.expr).toBe(rust.expr);
    expect(zig.cmd).toBe(rust.cmd);
    expect(zig.status).toBe(rust.status);
    expect(zig.bytes).toBe(rust.bytes);
    expect(zig.files).toEqual(rust.files);
    expect(zig.matches).toBe(rust.matches);
    expect(zig.slowest.bytes).toBe(rust.slowest.bytes);
    expect(zig.slowest.path).toBe(rust.slowest.path);
  });

  test("search and matches emit the same literal hit rows as the Rust oracle", () => {
    const fixture = path.join(process.env.TEMP ?? ROOT, "ix-zig-runtime-parity-hitrows.txt");
    writeFileSync(fixture, "alpha needle beta\nneedle again\nno hit\n", "utf8");

    const searchArgs = ["search", "lit:needle", fixture];
    const zigSearch = run(ZIG_IX, searchArgs);
    const rustSearch = run(RUST_IX, searchArgs);
    expect(zigSearch.split("\n").filter((line) => !line.startsWith("-- ix.result.v1"))).toEqual(
      rustSearch.split("\n").filter((line) => !line.startsWith("-- ix.result.v1")),
    );

    const matchesArgs = ["matches", "lit:needle", fixture];
    expect(run(ZIG_IX, matchesArgs)).toBe(run(RUST_IX, matchesArgs));
  });

  test("max-hits limits retained rows without truncating Rust-visible match counts", () => {
    const fixture = path.join(process.env.TEMP ?? ROOT, "ix-zig-runtime-parity-max-hits.txt");
    writeFileSync(fixture, "needle one\nneedle two\nneedle three\n", "utf8");

    const args = ["search", "lit:needle", fixture, "--max-hits", "1"];
    const zig = run(ZIG_IX, args);
    const rust = run(RUST_IX, args);
    const zigRows = zig.split("\n").filter((line) => line && !line.startsWith("-- ix.result.v1"));
    const rustRows = rust.split("\n").filter((line) => line && !line.startsWith("-- ix.result.v1"));

    expect(zigRows).toEqual(rustRows);
    expect(parseResultSentinel(zig).matches).toBe(parseResultSentinel(rust).matches);
    expect(parseResultSentinel(zig).matches).toBe(3);
  });

  test("stats-only literal search counts occurrences rather than matching lines", () => {
    const fixture = path.join(process.env.TEMP ?? ROOT, "ix-zig-runtime-parity-literal-occurrences.txt");
    writeFileSync(fixture, "needle needle\nneedle\n", "utf8");

    const args = ["search", "lit:needle", fixture, "--json", "--stats-only"];
    const zig = JSON.parse(run(ZIG_IX, args));
    const rust = JSON.parse(run(RUST_IX, args));

    expect(zig.hits).toEqual([]);
    expect(zig.stats.matches_found).toBe(rust.stats.matches_found);
    expect(zig.stats.matches_found).toBe(3);
    expect(zig.stats.bytes_scanned).toBe(rust.stats.bytes_scanned);
  });

  test("search json exposes Rust-compatible hit payloads for literal expressions", () => {
    const fixture = path.join(process.env.TEMP ?? ROOT, "ix-zig-runtime-parity-json.txt");
    writeFileSync(fixture, "alpha needle beta\nneedle again\nno hit\n", "utf8");

    const args = ["search", "lit:needle", fixture, "--json"];
    const zig = JSON.parse(run(ZIG_IX, args));
    const rust = JSON.parse(run(RUST_IX, args));

    expect(zig.expression).toBe(rust.expression);
    expect(zig.hits).toEqual(rust.hits.map((hit: { path: string; line: number; column: number; preview: string }) => ({
      ...hit,
      path: normalizePath(hit.path),
    })));
    expect(zig.stats.matches_found).toBe(rust.stats.matches_found);
    expect(zig.stats.bytes_scanned).toBe(rust.stats.bytes_scanned);
    expect(Object.keys(zig.stats).sort()).toEqual(Object.keys(rust.stats).sort());
    expect(Object.keys(zig.stats.concurrency).sort()).toEqual(Object.keys(rust.stats.concurrency).sort());
    expect(Object.keys(zig.stats.regex_decomposition).sort()).toEqual(Object.keys(rust.stats.regex_decomposition).sort());
    expect(Object.keys(zig.stats.slowest_files[0]).sort()).toEqual(Object.keys(rust.stats.slowest_files[0]).sort());
  });

  test("search scans explicitly supplied hidden roots with Rust-equivalent workload accounting", () => {
    const hiddenDir = path.join(process.env.TEMP ?? ROOT, ".ix-zig-runtime-hidden-root");
    const hiddenFile = path.join(hiddenDir, "fixture.txt");
    mkdirSync(hiddenDir, { recursive: true });
    writeFileSync(hiddenFile, "needle\nmiss\nneedle\n", "utf8");

    for (const target of [hiddenFile, hiddenDir]) {
      const args = ["search", "lit:needle", target, "--json", "--stats-only"];
      const zig = JSON.parse(run(ZIG_IX, args));
      const rust = JSON.parse(run(RUST_IX, args));

      expect(zig.stats.matches_found).toBe(rust.stats.matches_found);
      expect(zig.stats.files_discovered).toBe(rust.stats.files_discovered);
      expect(zig.stats.files_scanned).toBe(rust.stats.files_scanned);
      expect(zig.stats.files_skipped).toBe(rust.stats.files_skipped);
      expect(zig.stats.bytes_scanned).toBe(rust.stats.bytes_scanned);
      expect(zig.stats.slowest_files[0].bytes).toBe(rust.stats.slowest_files[0].bytes);
    }
  });

  test("search root normalization dedupes duplicate and overlapping roots like Rust", () => {
    const root = path.join(process.env.TEMP ?? ROOT, "ix-zig-runtime-root-pruning");
    const child = path.join(root, "child");
    mkdirSync(child, { recursive: true });
    const fixture = path.join(child, "fixture.txt");
    writeFileSync(fixture, "needle\n", "utf8");

    for (const args of [
      ["search", "lit:needle", fixture, fixture, "--json", "--stats-only"],
      ["search", "lit:needle", child, root, "--json", "--stats-only"],
    ]) {
      const zig = JSON.parse(run(ZIG_IX, args));
      const rust = JSON.parse(run(RUST_IX, args));

      expect(zig.stats.input_roots).toBe(rust.stats.input_roots);
      expect(zig.stats.effective_roots).toBe(rust.stats.effective_roots);
      expect(zig.stats.pruned_roots).toBe(rust.stats.pruned_roots);
      expect(zig.stats.overlap_pruned_roots).toBe(rust.stats.overlap_pruned_roots);
      expect(zig.stats.discovered_duplicate_paths).toBe(rust.stats.discovered_duplicate_paths);
      expect(zig.stats.matches_found).toBe(rust.stats.matches_found);
      expect(zig.stats.files_discovered).toBe(rust.stats.files_discovered);
      expect(zig.stats.files_scanned).toBe(rust.stats.files_scanned);
      expect(zig.stats.bytes_scanned).toBe(rust.stats.bytes_scanned);
    }
  });

  test("search skips likely binary files using Rust sniff semantics", () => {
    const fixture = path.join(process.env.TEMP ?? ROOT, "ix-zig-runtime-binary-skip.bin");
    writeFileSync(fixture, Buffer.from([0, 65, 66, 67, 110, 101, 101, 100, 108, 101, 10]));

    const args = ["search", "lit:needle", fixture, "--json", "--stats-only"];
    const zig = JSON.parse(run(ZIG_IX, args));
    const rust = JSON.parse(run(RUST_IX, args));

    expect(zig.stats.files_discovered).toBe(rust.stats.files_discovered);
    expect(zig.stats.files_scanned).toBe(rust.stats.files_scanned);
    expect(zig.stats.files_skipped).toBe(rust.stats.files_skipped);
    expect(zig.stats.matches_found).toBe(rust.stats.matches_found);
    expect(zig.stats.bytes_scanned).toBe(rust.stats.bytes_scanned);
    expect(zig.stats.slowest_files).toEqual(rust.stats.slowest_files);
  });

  test("matches json and emit-report preserve the Rust-visible report contract", () => {
    const fixture = path.join(process.env.TEMP ?? ROOT, "ix-zig-runtime-parity-report.txt");
    const zigReportPath = path.join(process.env.TEMP ?? ROOT, "ix-zig-runtime-parity-zig-report.json");
    const rustReportPath = path.join(process.env.TEMP ?? ROOT, "ix-zig-runtime-parity-rust-report.json");
    writeFileSync(fixture, "alpha needle beta\nneedle again\n", "utf8");

    const matchesArgs = ["matches", "lit:needle", fixture, "--json"];
    const zigMatches = JSON.parse(run(ZIG_IX, matchesArgs));
    const rustMatches = JSON.parse(run(RUST_IX, matchesArgs));
    expect(zigMatches.expression).toBe(rustMatches.expression);
    expect(zigMatches.hits).toEqual(rustMatches.hits.map((hit: { path: string; line: number; column: number; preview: string }) => ({
      ...hit,
      path: normalizePath(hit.path),
    })));
    expect(zigMatches.stats.matches_found).toBe(rustMatches.stats.matches_found);
    expect(zigMatches.stats.bytes_scanned).toBe(rustMatches.stats.bytes_scanned);

    run(ZIG_IX, ["search", "lit:needle", fixture, "--emit-report", zigReportPath]);
    run(RUST_IX, ["search", "lit:needle", fixture, "--emit-report", rustReportPath]);
    const zigReport = JSON.parse(readFileSync(zigReportPath, "utf8"));
    const rustReport = JSON.parse(readFileSync(rustReportPath, "utf8"));
    expect(zigReport.expression).toBe(rustReport.expression);
    expect(zigReport.hits).toEqual(rustReport.hits.map((hit: { path: string; line: number; column: number; preview: string }) => ({
      ...hit,
      path: normalizePath(hit.path),
    })));
    expect(zigReport.stats.matches_found).toBe(rustReport.stats.matches_found);
    expect(zigReport.stats.bytes_scanned).toBe(rustReport.stats.bytes_scanned);
  });

  test("search telemetry exposes measured operator-visible timings instead of inert counters", () => {
    const fixture = path.join(process.env.TEMP ?? ROOT, "ix-zig-runtime-parity-telemetry.txt");
    writeFileSync(fixture, "needle\nother needle\n", "utf8");

    const zig = JSON.parse(run(ZIG_IX, ["search", "lit:needle", fixture, "--json", "--stats-only"]));
    const rust = JSON.parse(run(RUST_IX, ["search", "lit:needle", fixture, "--json", "--stats-only"]));

    expect(zig.hits).toEqual([]);
    expect(zig.stats.matches_found).toBe(rust.stats.matches_found);
    expect(zig.stats.bytes_scanned).toBe(rust.stats.bytes_scanned);
    expect(zig.stats.input_roots).toBe(rust.stats.input_roots);
    expect(zig.stats.effective_roots).toBe(rust.stats.effective_roots);
    expect(zig.stats.linux_strategy.collect_hits).toBe(false);
    expect(zig.stats.timings.scan_ms).toBeGreaterThan(0);
    expect(zig.stats.timings.total_ms).toBeGreaterThanOrEqual(zig.stats.timings.scan_ms);
    expect(zig.stats.timings.scan_work_ms_total).toBeGreaterThan(0);
    expect(zig.stats.slowest_files[0].duration_ms).toBeGreaterThan(0);
    expect(zig.stats.slowest_files[0].bytes).toBe(rust.stats.slowest_files[0].bytes);
  });

  test("search telemetry reflects requested thread context instead of inert constants", () => {
    const fixture = path.join(process.env.TEMP ?? ROOT, "ix-zig-runtime-parity-thread-context.txt");
    writeFileSync(fixture, "needle\n", "utf8");

    const zig = JSON.parse(run(ZIG_IX, ["search", "lit:needle", fixture, "--json", "--stats-only", "--threads", "3"]));

    expect(zig.stats.concurrency.available_threads).toBeGreaterThanOrEqual(1);
    expect(zig.stats.concurrency.outer_scan_threads).toBe(3);
    expect(zig.stats.concurrency.execution_mode).toBe("materialized");
    expect(zig.stats.concurrency.sharding_enabled).toBe(false);
  });

  test("stats-only regex counts occurrences like the Rust fast-count contract", () => {
    const fixture = path.join(process.env.TEMP ?? ROOT, "ix-zig-runtime-parity-stats-count.txt");
    writeFileSync(fixture, "abc def ghi jkl mno abc def ghi jkl mno\nabc def ghi jkl mno\n", "utf8");

    const args = ["search", "re:\\w{3}\\s+\\w{3}\\s+\\w{3}\\s+\\w{3}\\s+\\w{3}", fixture, "--json", "--stats-only"];
    const zig = JSON.parse(run(ZIG_IX, args));
    const rust = JSON.parse(run(RUST_IX, args));

    expect(zig.stats.matches_found).toBe(rust.stats.matches_found);
    expect(zig.stats.matches_found).toBe(3);
    expect(zig.stats.bytes_scanned).toBe(rust.stats.bytes_scanned);
  });

  test("stats-only surrounding-word regex follows Rust decomposition line-count semantics", () => {
    const fixture = path.join(process.env.TEMP ?? ROOT, "ix-zig-runtime-parity-decomposition-count.txt");
    writeFileSync(fixture, "aa Holmes bb Holmes cc\nxx Holmes yy\nHolmes\n", "utf8");

    const args = ["search", "re:\\w+\\s+Holmes\\s+\\w+", fixture, "--json", "--stats-only"];
    const zig = JSON.parse(run(ZIG_IX, args));
    const rust = JSON.parse(run(RUST_IX, args));

    expect(zig.stats.matches_found).toBe(rust.stats.matches_found);
    expect(zig.stats.matches_found).toBe(2);
    expect(zig.stats.bytes_scanned).toBe(rust.stats.bytes_scanned);
  });

  test("inspect match-context output matches the Rust oracle for literal file targets", () => {
    const fixture = path.join(process.env.TEMP ?? ROOT, "ix-zig-runtime-parity-context.txt");
    writeFileSync(fixture, "alpha needle beta\nneedle again\nomega\n", "utf8");

    const args = ["inspect", "--expr", "lit:needle", fixture, "--context", "1"];
    expect(run(ZIG_IX, args)).toBe(run(RUST_IX, args));
  });

  test("inspect match-context records and json formats match the Rust oracle", () => {
    const fixture = path.join(process.env.TEMP ?? ROOT, "ix-zig-runtime-parity-context-formats.txt");
    writeFileSync(fixture, "alpha\nneedle\nomega\n", "utf8");

    expect(run(ZIG_IX, ["inspect", "--expr", "lit:needle", fixture, "--context", "1", "--format", "records"])).toBe(
      run(RUST_IX, ["inspect", "--expr", "lit:needle", fixture, "--context", "1", "--format", "records"]),
    );

    const jsonArgs = ["inspect", "--expr", "lit:needle", fixture, "--context", "1", "--format", "json"];
    expect(normalizeInspectContextJson(run(ZIG_IX, jsonArgs))).toEqual(normalizeInspectContextJson(run(RUST_IX, jsonArgs)));
  });

  test("inspect match-context asymmetric before and after flags match the Rust oracle", () => {
    const fixture = path.join(process.env.TEMP ?? ROOT, "ix-zig-runtime-parity-context-asymmetric.txt");
    writeFileSync(fixture, "alpha\nneedle\nomega\ntail\n", "utf8");

    for (const args of [
      ["inspect", "--expr", "lit:needle", fixture, "-B", "1", "-A", "0", "--format", "records"],
      ["inspect", "--expr", "lit:needle", fixture, "-B", "0", "-A", "1", "--format", "records"],
    ]) {
      expect(run(ZIG_IX, args)).toBe(run(RUST_IX, args));
    }
  });

  test("regex search covers literal alternation and word-boundary operator workflows", () => {
    const fixture = path.join(process.env.TEMP ?? ROOT, "ix-zig-runtime-parity-regex.txt");
    writeFileSync(fixture, "alpha needle beta\nTODO item\nFIXME item\nsession handshake\nunsessionized\n", "utf8");

    for (const args of [
      ["search", "re:needle", fixture],
      ["search", "re:TODO|FIXME", fixture],
      ["search", "re:\\b(session|handshake)\\b", fixture],
    ]) {
      const zig = run(ZIG_IX, args);
      const rust = run(RUST_IX, args);
      expect(zig.split("\n").filter((line) => !line.startsWith("-- ix.result.v1"))).toEqual(
        rust.split("\n").filter((line) => !line.startsWith("-- ix.result.v1")),
      );
      expect(parseResultSentinel(zig).matches).toBe(parseResultSentinel(rust).matches);
    }
  });

  test("top-level repeated -e compat lowering matches Rust hit rows and result expression", () => {
    const fixture = path.join(process.env.TEMP ?? ROOT, "ix-zig-runtime-parity-repeated-e.txt");
    writeFileSync(fixture, "alpha needle beta\nneedle again\nomega\n", "utf8");

    const args = ["-e", "alpha", "-e", "needle", fixture];
    const zig = run(ZIG_IX, args);
    const rust = run(RUST_IX, args);
    expect(zig.split("\n").filter((line) => !line.startsWith("-- ix.result.v1"))).toEqual(
      rust.split("\n").filter((line) => !line.startsWith("-- ix.result.v1")),
    );
    expect(parseResultSentinel(zig).expr).toBe(parseResultSentinel(rust).expr);
    expect(parseResultSentinel(zig).matches).toBe(parseResultSentinel(rust).matches);
  });

  test("top-level compat json uses the canonical search report path", () => {
    const fixture = path.join(process.env.TEMP ?? ROOT, "ix-zig-runtime-parity-compat-json.txt");
    writeFileSync(fixture, "alpha needle beta\nneedle again\n", "utf8");

    const args = ["-e", "needle", "--json", fixture];
    const zig = JSON.parse(run(ZIG_IX, args));
    const rust = JSON.parse(run(RUST_IX, args));

    expect(zig.expression).toBe(rust.expression);
    expect(zig.hits).toEqual(rust.hits.map((hit: { path: string; line: number; column: number; preview: string }) => ({
      ...hit,
      path: normalizePath(hit.path),
    })));
    expect(zig.stats.matches_found).toBe(rust.stats.matches_found);
    expect(zig.stats.files_scanned).toBe(rust.stats.files_scanned);
    expect(Object.keys(zig.stats).sort()).toEqual(Object.keys(rust.stats).sort());
  });

  test("top-level compat accepts long regexp and thread flags without changing match semantics", () => {
    const fixture = path.join(process.env.TEMP ?? ROOT, "ix-zig-runtime-parity-compat-flags.txt");
    writeFileSync(fixture, "alpha needle beta\nneedle again\nomega\n", "utf8");

    for (const args of [
      ["--regexp", "needle", "-j", "1", fixture],
      ["--regexp=needle", "--threads=1", fixture],
      ["-eneedle", "-j1", fixture],
    ]) {
      const zig = run(ZIG_IX, args);
      const rust = run(RUST_IX, args);
      expect(zig.split("\n").filter((line) => !line.startsWith("-- ix.result.v1"))).toEqual(
        rust.split("\n").filter((line) => !line.startsWith("-- ix.result.v1")),
      );
      expect(parseResultSentinel(zig).matches).toBe(parseResultSentinel(rust).matches);
    }
  });

  test("regex parity covers inline ignore-case and fixed-string boolean operator escaping", () => {
    const fixture = path.join(process.env.TEMP ?? ROOT, "ix-zig-runtime-parity-regex-inline.txt");
    writeFileSync(fixture, "Alpha\nIX&&Rust\nTimeout.*\n", "utf8");

    for (const args of [
      ["search", "re:(?i)alpha", fixture],
      ["-F", "IX&&Rust", fixture],
      ["-F", "-i", "Timeout.*", fixture],
    ]) {
      const zig = run(ZIG_IX, args);
      const rust = run(RUST_IX, args);
      expect(zig.split("\n").filter((line) => !line.startsWith("-- ix.result.v1"))).toEqual(
        rust.split("\n").filter((line) => !line.startsWith("-- ix.result.v1")),
      );
      expect(parseResultSentinel(zig).expr).toBe(parseResultSentinel(rust).expr);
      expect(parseResultSentinel(zig).matches).toBe(parseResultSentinel(rust).matches);
    }
  });

  test("regex and literal matching preserve ignore-case, anchors, and plus repetition", () => {
    const fixture = path.join(process.env.TEMP ?? ROOT, "ix-zig-runtime-parity-regex-operators.txt");
    writeFileSync(fixture, "Alpha needle\nbeta   needle token\nneedle suffix\nsuffix needle\n", "utf8");

    for (const args of [
      ["-i", "alpha", fixture],
      ["search", "re:^needle", fixture],
      ["search", "re:needle$", fixture],
      ["search", "re:\\w{4}\\s+needle", fixture],
    ]) {
      const zig = run(ZIG_IX, args);
      const rust = run(RUST_IX, args);
      expect(zig.split("\n").filter((line) => !line.startsWith("-- ix.result.v1"))).toEqual(
        rust.split("\n").filter((line) => !line.startsWith("-- ix.result.v1")),
      );
      expect(parseResultSentinel(zig).matches).toBe(parseResultSentinel(rust).matches);
    }
  });

  test("regex parity covers classes, digit classes, optional, star, and group consumption", () => {
    const fixture = path.join(process.env.TEMP ?? ROOT, "ix-zig-runtime-parity-regex-depth.txt");
    writeFileSync(
      fixture,
      [
        "ERR42: colour",
        "WARN7: color",
        "INFO: colouur",
        "group suffix",
        "group other",
        "prefix suffix",
      ].join("\n") + "\n",
      "utf8",
    );

    for (const args of [
      ["search", "re:[A-Z]+\\d+: colo?r", fixture],
      ["search", "re:WARN\\d+", fixture],
      ["search", "re:colou*r", fixture],
      ["search", "re:(group|prefix) suffix", fixture],
    ]) {
      const zig = run(ZIG_IX, args);
      const rust = run(RUST_IX, args);
      expect(zig.split("\n").filter((line) => !line.startsWith("-- ix.result.v1"))).toEqual(
        rust.split("\n").filter((line) => !line.startsWith("-- ix.result.v1")),
      );
      expect(parseResultSentinel(zig).matches).toBe(parseResultSentinel(rust).matches);
    }
  });
});
