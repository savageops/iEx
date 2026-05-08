import { existsSync, readFileSync } from "node:fs";
import path from "node:path";
import { describe, expect, test } from "vitest";

const ROOT = process.cwd();
const ZIG_ROOT = path.join(ROOT, "zig");

const requiredFiles = [
  "README.md",
  "build.zig",
  "src/main.zig",
  "src/cli/args.zig",
  "src/cli/output.zig",
  "src/core/expr.zig",
  "src/core/search.zig",
  "src/core/regex.zig",
  "src/core/inspect.zig",
];

function read(relativePath: string): string {
  return readFileSync(path.join(ZIG_ROOT, relativePath), "utf8");
}

describe("zig parity port contract", () => {
  test("materializes the sibling Zig root without replacing the Rust oracle", () => {
    for (const relativePath of requiredFiles) {
      expect(existsSync(path.join(ZIG_ROOT, relativePath)), relativePath).toBe(true);
    }

    expect(existsSync(path.join(ROOT, "crates", "iex-core", "src", "engine.rs"))).toBe(true);
    expect(existsSync(path.join(ROOT, "crates", "iex-cli", "src", "main.rs"))).toBe(true);
  });

  test("does not delegate Zig behavior to Rust, Cargo, or installed IX binaries", () => {
    const forbidden = [
      /std\.process\.Child/,
      /\bexec(File|Sync)?\s*\(/i,
      /spawn(Sync)?/i,
      /\bcargo\b/i,
      /target[\\/](debug|release)[\\/]ix/i,
      /target[\\/](debug|release)[\\/]iex-cli/i,
      /AppData[\\/]Local[\\/]Programs[\\/]iEx/i,
      /where\.exe\s+ix/i,
    ];

    for (const relativePath of requiredFiles.filter((file) => file.endsWith(".zig"))) {
      const source = read(relativePath);
      for (const pattern of forbidden) {
        expect(source, `${relativePath} must not match ${pattern}`).not.toMatch(pattern);
      }
    }
  });

  test("preserves the canonical command taxonomy and agent sentinels in Zig source", () => {
    const main = read("src/main.zig");
    const args = read("src/cli/args.zig");
    const output = read("src/cli/output.zig");

    for (const command of ["search", "matches", "inspect", "explain", "help"]) {
      expect(args).toContain(command);
      expect(main).toContain(`.${command}`);
    }

    expect(output).toContain("ix.result.v1");
    expect(output).toContain("ix.next.v1");
    expect(output).toContain("ix.error.v1");
    expect(output).toContain("ix.inspect.file");
    expect(output).toContain('\\"files\\"');
    expect(output).toContain('\\"matches\\"');
  });

  test("removes inert scaffold exits and owns typed source-level behavior", () => {
    const main = read("src/main.zig");
    const args = read("src/cli/args.zig");
    const expr = read("src/core/expr.zig");
    const search = read("src/core/search.zig");
    const regex = read("src/core/regex.zig");
    const inspect = read("src/core/inspect.zig");

    for (const source of [main, args, expr, search, regex, inspect]) {
      expect(source).not.toMatch(/missing_parity|MissingParityImplementation|TODO|FIXME/);
    }

    expect(search).toContain('@import("regex.zig")');
    expect(search).toContain("matchesLine");
    expect(search).toContain("predicateMatches");
    expect(regex).toContain("pub fn column");
    expect(regex).toContain("matchGroupThenRest");
    expect(inspect).toContain("has_more");
    expect(args).toContain("path_count");
    expect(args).toContain("--start-line");
    expect(args).toContain("--end-line");
    expect(args).toContain("--total-count");
    expect(args).toContain("--emit-report");
    expect(args).toContain("--before-context");
    expect(args).toContain("--after-context");
    expect(inspect).toContain("windowForPath");
    expect(inspect).toContain("contextForPath");
  });

  test("captures exposed operator workflow before private helper details", () => {
    const agents = readFileSync(path.join(ROOT, "AGENTS.md"), "utf8");
    const architecture = readFileSync(path.join(ROOT, ".docs", "zig-port-architecture.md"), "utf8");
    const output = read("src/cli/output.zig");

    expect(agents).toContain("User-Exposed Pipeline Priority");
    expect(agents).toContain("command input -> parsed intent -> execution phase -> visible metadata -> typed result sentinel -> next action");
    expect(agents).toContain("Do not expose hidden reasoning text");
    expect(architecture).toContain("User-Exposed Pipeline");

    for (const visibleSurface of ["ix.result.v1", "ix.inspect.file", "ix.next.v1", "ix.error.v1"]) {
      expect(output).toContain(visibleSurface);
    }
  });

  test("anchors the toolchain and promotion gate in repo-local docs", () => {
    const readme = read("README.md");
    const architecture = readFileSync(path.join(ROOT, ".docs", "zig-port-architecture.md"), "utf8");

    expect(readme).toContain("Pinned toolchain target: Zig `0.16.0`.");
    expect(readme).toContain("must not replace the Rust `ix` binary");
    expect(architecture).toContain("Rust remains the behavioral oracle");
    expect(architecture).toContain("Performance claims require match-count parity");
  });
});
