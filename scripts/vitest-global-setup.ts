/**
 * Ensure the native `spanda` CLI is built before Vitest runs Rust-backed tests.
 * @module
 */

import { spawnSync } from "node:child_process";
import { existsSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const repoRoot = join(dirname(fileURLToPath(import.meta.url)), "..");
const cli = join(repoRoot, "target/debug/spanda");

function cliIsCurrent(path: string): boolean {
  if (!existsSync(path)) {
    return false;
  }
  const probe = spawnSync(path, ["state", "estimate"], { encoding: "utf-8" });
  const output = `${probe.stdout ?? ""}${probe.stderr ?? ""}`;
  return !output.includes("Unknown command") && !output.includes("Unknown argument");
}

export default function globalSetup(): void {
  if (cliIsCurrent(cli)) {
    return;
  }

  spawnSync("cargo", ["build", "-p", "spanda"], {
    cwd: repoRoot,
    stdio: "inherit",
    env: { ...process.env, CARGO_TARGET_DIR: join(repoRoot, "target") },
  });
}
