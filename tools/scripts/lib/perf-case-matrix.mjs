import { existsSync, mkdirSync, writeFileSync } from "node:fs";
import { spawnSync } from "node:child_process";
import path from "node:path";
import { summarizeSeries } from "./metrics.mjs";

export const DEFAULT_PERF_CASE_COUNT = 888;

const ROOT = process.cwd();
const DEFAULT_CASES = [
  "suite-en-literal",
  "suite-en-literal-casei",
  "suite-en-word",
  "suite-en-alternates",
  "suite-en-surrounding-words",
  "suite-en-no-literal",
  "suite-ru-literal",
  "suite-ru-literal-casei",
  "suite-linux-literal",
  "suite-linux-word",
  "suite-linux-alternates",
  "suite-linux-no-literal",
  "synthetic-and",
  "synthetic-or",
  "synthetic-prefix-suffix",
  "synthetic-absent-literal",
];

const EN_NAMES = [
  "Sherlock Holmes",
  "John Watson",
  "Irene Adler",
  "Inspector Lestrade",
  "Professor Moriarty",
  "Baker Street",
  "Mycroft Holmes",
  "Mrs Hudson",
];

const LINUX_TOKENS = [
  "PM_RESUME",
  "ERR_SYS",
  "PME_TURN_OFF",
  "LINK_REQ_RST",
  "CFG_BME_EVT",
  "PCI_VENDOR_ID",
  "DRM_ERROR",
  "VM_FAULT",
  "SCHED_NORMAL",
  "IRQ_HANDLED",
];

export function resolvePerfCorpora(root = ROOT) {
  const suiteDir = path.join(root, ".refs", "ripgrep", "benchsuite");
  return {
    synthetic: path.join(root, "tools", "data", "corpus"),
    enSample: path.join(suiteDir, "subtitles", "en.sample.txt"),
    ru: path.join(suiteDir, "subtitles", "ru.txt"),
    linux: path.join(suiteDir, "linux"),
  };
}

function corpusKeyForFamily(family) {
  if (family.startsWith("suite-en")) {
    return "enSample";
  }
  if (family.startsWith("suite-ru")) {
    return "ru";
  }
  if (family.startsWith("suite-linux")) {
    return "linux";
  }
  return "synthetic";
}

function expressionForFamily(family, index) {
  const enName = EN_NAMES[index % EN_NAMES.length];
  const nextEnName = EN_NAMES[(index + 3) % EN_NAMES.length];
  const linuxA = LINUX_TOKENS[index % LINUX_TOKENS.length];
  const linuxB = LINUX_TOKENS[(index + 2) % LINUX_TOKENS.length];
  const linuxC = LINUX_TOKENS[(index + 5) % LINUX_TOKENS.length];
  const linuxD = LINUX_TOKENS[(index + 7) % LINUX_TOKENS.length];
  const wordWidth = 3 + (index % 5);

  switch (family) {
    case "suite-en-literal":
      return `lit:${enName}`;
    case "suite-en-literal-casei":
      return `re:(?i)${enName}`;
    case "suite-en-word":
      return `re:\\b${enName}\\b`;
    case "suite-en-alternates":
      return `re:(${enName}|${nextEnName}|Baker Street|Inspector Lestrade)`;
    case "suite-en-surrounding-words":
      return `re:\\w+\\s+${enName.split(" ")[0]}\\s+\\w+`;
    case "suite-en-no-literal":
      return `re:\\w{${wordWidth}}\\s+\\w{${wordWidth}}\\s+\\w{${wordWidth}}\\s+\\w{${wordWidth}}\\s+\\w{${wordWidth}}`;
    case "suite-ru-literal":
      return "lit:Шерлок Холмс";
    case "suite-ru-literal-casei":
      return "re:(?i)Шерлок Холмс";
    case "suite-linux-literal":
      return `lit:${linuxA}`;
    case "suite-linux-word":
      return `re:\\b${linuxA}\\b`;
    case "suite-linux-alternates":
      return `re:(${linuxA}|${linuxB}|${linuxC}|${linuxD})`;
    case "suite-linux-no-literal":
      return `re:\\w{${wordWidth}}\\s+\\w{${wordWidth}}\\s+\\w{${wordWidth}}\\s+\\w{${wordWidth}}\\s+\\w{${wordWidth}}`;
    case "synthetic-and":
      return index % 2 === 0 ? "lit:ERROR && lit:timeout" : "lit:WARN && lit:latency";
    case "synthetic-or":
      return index % 2 === 0 ? "lit:INFO || lit:ERROR" : "lit:checkpoint || lit:timeout";
    case "synthetic-prefix-suffix":
      return `prefix:INFO && suffix:trace=iex-v2-${index % 40}-29`;
    case "synthetic-absent-literal":
      return `lit:__ix_absent_probe_${String(index).padStart(4, "0")}__`;
    default:
      throw new Error(`unsupported performance case family "${family}"`);
  }
}

function expectedPressureForFamily(family) {
  if (family.endsWith("no-literal")) {
    return {
      expectedHotspot: "scan",
      pressure: "regex_without_required_literal",
      retentionRule: "must expose scan cost instead of hiding behind aggregate timing",
    };
  }
  if (family.includes("linux")) {
    return {
      expectedHotspot: "scan",
      pressure: "linux_tree_and_giant_header_tail",
      retentionRule: "must preserve predecessor parity before promotion",
    };
  }
  if (family.includes("casei")) {
    return {
      expectedHotspot: "scan",
      pressure: "casefold_or_unicode_comparison",
      retentionRule: "non-ascii case-insensitive lanes require match-count authority",
    };
  }
  if (family.includes("alternates")) {
    return {
      expectedHotspot: "scan",
      pressure: "multi_literal_prefilter_and_leftmost_semantics",
      retentionRule: "must preserve match-count parity across alternates",
    };
  }
  return {
    expectedHotspot: "scan",
    pressure: "literal_or_structural_baseline",
    retentionRule: "must remain neutral-or-better against current canonical binary",
  };
}

export function buildPerformanceCaseMatrix(options = {}) {
  const count = Math.max(1, Math.floor(Number(options.count ?? DEFAULT_PERF_CASE_COUNT)));
  const families = options.families ?? DEFAULT_CASES;
  const root = options.root ?? ROOT;
  const corpora = resolvePerfCorpora(root);
  const cases = [];

  for (let index = 0; index < count; index += 1) {
    const family = families[index % families.length];
    const corpusKey = corpusKeyForFamily(family);
    const pressure = expectedPressureForFamily(family);
    cases.push({
      id: `ix-perf-${String(index + 1).padStart(4, "0")}`,
      ordinal: index + 1,
      family,
      expression: expressionForFamily(family, index),
      corpusKey,
      corpus: corpora[corpusKey],
      statsOnly: true,
      expectedHotspot: pressure.expectedHotspot,
      pressure: pressure.pressure,
      retentionRule: pressure.retentionRule,
      tags: [
        family.includes("linux") ? "linux-suite" : "portable",
        family.endsWith("no-literal") ? "no-literal-regex" : "prefilterable",
        family.includes("casei") ? "case-insensitive" : "case-sensitive",
      ],
    });
  }

  return cases;
}

export function filterAvailablePerfCases(cases) {
  return cases.filter((caseSpec) => existsSync(caseSpec.corpus));
}

export function buildIxSearchArgs(caseSpec, options = {}) {
  const args = ["search", caseSpec.expression, caseSpec.corpus, "--json", "--stats-only"];
  const threads = Number(options.threads ?? 0);
  if (Number.isFinite(threads) && threads > 0) {
    args.push("--threads", String(Math.floor(threads)));
  }
  return args;
}

function parseIxReport(stdout) {
  try {
    return JSON.parse((stdout || "{}").replace(/^\uFEFF/, ""));
  } catch {
    return null;
  }
}

function runOneIxProbe(binaryPath, caseSpec, options = {}) {
  const args = buildIxSearchArgs(caseSpec, options);
  const started = process.hrtime.bigint();
  const result = spawnSync(binaryPath, args, {
    cwd: options.root ?? ROOT,
    encoding: "utf8",
    maxBuffer: 128 * 1024 * 1024,
    windowsHide: true,
  });
  const ended = process.hrtime.bigint();
  const wallMs = Number(ended - started) / 1_000_000;
  const stdout = result.stdout?.toString() ?? "";
  const stderr = result.stderr?.toString() ?? "";
  const status = result.status ?? 0;
  const report = parseIxReport(stdout);

  if (status !== 0) {
    throw new Error(`ix probe failed for ${caseSpec.id}: code=${status}\n${stderr}`);
  }
  if (!report) {
    throw new Error(`ix probe emitted non-json output for ${caseSpec.id}`);
  }

  const timings = report?.stats?.timings ?? {};
  return {
    wallMs,
    engineMs: timings.total_ms ?? wallMs,
    discoverMs: timings.discover_ms ?? 0,
    scanMs: timings.scan_ms ?? 0,
    aggregateMs: timings.aggregate_ms ?? 0,
    filesDiscovered: report?.stats?.files_discovered ?? 0,
    filesScanned: report?.stats?.files_scanned ?? 0,
    bytesScanned: report?.stats?.bytes_scanned ?? 0,
    matchCount: report?.stats?.matches_found ?? report?.stats?.matches ?? report?.matches?.length ?? 0,
    slowestFiles: report?.stats?.slowest_files ?? [],
  };
}

export function measurePerformanceCase(binaryPath, caseSpec, options = {}) {
  const warmup = Math.max(0, Math.floor(Number(options.warmup ?? 0)));
  const samples = Math.max(1, Math.floor(Number(options.samples ?? 1)));

  for (let index = 0; index < warmup; index += 1) {
    runOneIxProbe(binaryPath, caseSpec, options);
  }

  const samplesOut = [];
  for (let index = 0; index < samples; index += 1) {
    samplesOut.push(runOneIxProbe(binaryPath, caseSpec, options));
  }

  const wallSummary = summarizeSeries(samplesOut.map((sample) => sample.wallMs));
  const engineSummary = summarizeSeries(samplesOut.map((sample) => sample.engineMs));
  const scanSummary = summarizeSeries(samplesOut.map((sample) => sample.scanMs));
  const selected = [...samplesOut].sort((left, right) => left.wallMs - right.wallMs)[Math.floor(samplesOut.length / 2)];

  return {
    case: caseSpec,
    samples: samplesOut,
    wall: wallSummary,
    engine: engineSummary,
    scan: scanSummary,
    selected,
    scanSharePct: selected.engineMs > 0 ? (selected.scanMs / selected.engineMs) * 100 : 0,
  };
}

export function summarizePerformanceMatrix(results) {
  const byFamily = new Map();
  for (const result of results) {
    const family = result.case.family;
    if (!byFamily.has(family)) {
      byFamily.set(family, []);
    }
    byFamily.get(family).push(result);
  }

  const families = [...byFamily.entries()].map(([family, familyResults]) => {
    return {
      family,
      count: familyResults.length,
      wallMs: summarizeSeries(familyResults.map((result) => result.selected.wallMs)),
      engineMs: summarizeSeries(familyResults.map((result) => result.selected.engineMs)),
      scanMs: summarizeSeries(familyResults.map((result) => result.selected.scanMs)),
      scanSharePct: summarizeSeries(familyResults.map((result) => result.scanSharePct)),
    };
  });

  const slowest = [...results]
    .sort((left, right) => right.selected.wallMs - left.selected.wallMs)
    .slice(0, 25)
    .map((result) => ({
      id: result.case.id,
      family: result.case.family,
      expression: result.case.expression,
      corpus: result.case.corpus,
      wallMs: result.selected.wallMs,
      engineMs: result.selected.engineMs,
      scanMs: result.selected.scanMs,
      scanSharePct: result.scanSharePct,
      filesScanned: result.selected.filesScanned,
      bytesScanned: result.selected.bytesScanned,
      matchCount: result.selected.matchCount,
      slowestFiles: result.selected.slowestFiles.slice(0, 5),
    }));

  return {
    count: results.length,
    families,
    slowest,
  };
}

export function writePerformanceMatrixReport(report, outPath) {
  mkdirSync(path.dirname(outPath), { recursive: true });
  writeFileSync(outPath, `${JSON.stringify(report, null, 2)}\n`, "utf8");
}
