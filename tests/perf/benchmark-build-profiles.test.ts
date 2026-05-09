import path from "node:path";
import { describe, expect, test } from "vitest";

import { listBuildProfiles, resolveBuildProfile } from "../../tools/scripts/lib/build-profiles.mjs";

describe("benchmark build profiles", () => {
  test("lists the canonical build profiles for benchmark candidates", () => {
    expect(listBuildProfiles()).toEqual([
      {
        id: "debug",
        description: "debug benchmark candidate",
        binaryDir: "debug",
      },
      {
        id: "native-lto",
        description: "host-native proof candidate with target-cpu=native, fat LTO, and one codegen unit",
        binaryDir: path.join("native-lto", "release-lto"),
      },
      {
        id: "release",
        description: "baseline release benchmark candidate",
        binaryDir: "release",
      },
      {
        id: "release-lto",
        description: "single-root proof candidate with fat LTO and one codegen unit",
        binaryDir: "release-lto",
      },
    ]);
  });

  test("resolves the release-lto profile to its cargo command and binary path", () => {
    const rootDir = process.cwd();
    const profile = resolveBuildProfile("release-lto", rootDir);

    expect(profile.id).toBe("release-lto");
    expect(profile.cargoArgs).toEqual(["build", "--profile", "release-lto", "-p", "iex-cli"]);
    expect(profile.binaryPath).toBe(
      path.join(rootDir, "target", "release-lto", process.platform === "win32" ? "ix.exe" : "ix"),
    );
  });

  test("fails fast on unsupported benchmark build profiles", () => {
    expect(() => resolveBuildProfile("release-bolt")).toThrow(
      /supported profiles: debug, native-lto, release, release-lto/i,
    );
  });
});
