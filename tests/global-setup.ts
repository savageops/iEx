import { execFileSync } from "node:child_process";

export default function globalSetup() {
  execFileSync("cargo", ["build", "-p", "iex-cli"], {
    cwd: process.cwd(),
    stdio: "inherit",
  });
}
