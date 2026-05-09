import { execFileSync } from "node:child_process";
import { existsSync } from "node:fs";
import path from "node:path";

const ROOT = path.resolve(process.cwd());
const DEBUG_BIN = process.platform === "win32"
  ? path.join(ROOT, "target", "debug", "iex-cli.exe")
  : path.join(ROOT, "target", "debug", "iex-cli");

export function ensureBinary(): string {
  if (!existsSync(DEBUG_BIN)) {
    execFileSync("cargo", ["build", "-p", "iex-cli"], {
      cwd: ROOT,
      stdio: "inherit",
    });
  }
  return DEBUG_BIN;
}

export function runExplain(expr: string): any {
  const bin = ensureBinary();
  const output = execFileSync(bin, ["explain", expr], {
    cwd: ROOT,
    encoding: "utf8",
  });
  return JSON.parse(output);
}

export function runSearchJson(expr: string, pathArg: string, extra: string[] = []): any {
  const bin = ensureBinary();
  const args = ["search", expr, pathArg, "--json", ...extra];
  const output = execFileSync(bin, args, {
    cwd: ROOT,
    encoding: "utf8",
    maxBuffer: 64 * 1024 * 1024,
  });
  return JSON.parse(output);
}
