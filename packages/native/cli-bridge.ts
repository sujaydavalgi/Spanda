/**
 * cli bridge module (cli-bridge.ts).
 * @module
 */

import { spawnSync } from "node:child_process";
import { existsSync, unlinkSync, writeFileSync } from "node:fs";
import { join, dirname } from "node:path";
import { fileURLToPath } from "node:url";
import type { CheckResult, RunResult } from "./index.js";

const repoRoot = join(dirname(fileURLToPath(import.meta.url)), "../..");

function cliPath(): string | null {
  // CliPath.
  //
  // Parameters:
  // None.
  //
  // Returns:
  // Some value on success, otherwise none.
  //
  // Options:
  // None.
  //
  // Example:
  // const result = cliPath();

  const release = join(repoRoot, "target/release/spanda");
  const debug = join(repoRoot, "target/debug/spanda");
  if (existsSync(release)) return release;
  if (existsSync(debug)) return debug;
  return null;
}

export function isCliAvailable(): boolean {
  // IsCliAvailable.
  //
  // Parameters:
  // None.
  //
  // Returns:
  // true or false.
  //
  // Options:
  // None.
  //
  // Example:
  // const result = isCliAvailable();

  return cliPath() !== null;
}

export function checkViaCli(source: string): CheckResult {
  // CheckViaCli.
  //
  // Parameters:
  // - `source` — input value
  //
  // Returns:
  // CheckResult.
  //
  // Options:
  // None.
  //
  // Example:
  // const result = checkViaCli(source);

  const bin = cliPath();
  if (!bin) {
    return { ok: false, diagnostics: [{ message: "Rust CLI not built (run: cargo build -p spanda-cli)", line: 1, column: 1 }] };
  }
  const tmp = join(repoRoot, ".spanda-check-tmp.sd");
  writeFileSync(tmp, source);
  const result = spawnSync(bin, ["check", "--json", tmp], { encoding: "utf-8" });
  try {
    unlinkSync(tmp);
  } catch {
    /* ignore */
  }
  if (!result.stdout?.trim()) {
    return {
      ok: false,
      diagnostics: [{ message: result.stderr || "CLI check failed", line: 1, column: 1 }],
    };
  }
  return JSON.parse(result.stdout) as CheckResult;
}

export function runViaCli(source: string): RunResult {
  // RunViaCli.
  //
  // Parameters:
  // - `source` — input value
  //
  // Returns:
  // RunResult.
  //
  // Options:
  // None.
  //
  // Example:
  // const result = runViaCli(source);

  const bin = cliPath();
  if (!bin) {
    throw new Error("Rust CLI not built (run: cargo build -p spanda-cli)");
  }
  const tmp = join(repoRoot, ".spanda-run-tmp.sd");
  writeFileSync(tmp, source);
  const result = spawnSync(bin, ["run", "--json", tmp], { encoding: "utf-8" });
  try {
    unlinkSync(tmp);
  } catch {
    /* ignore */
  }
  const parsed = JSON.parse(result.stdout || "{}") as {
    ok: boolean;
    result?: RunResult;
    diagnostics?: CheckResult["diagnostics"];
  };
  if (!parsed.ok || !parsed.result) {
    throw new Error(parsed.diagnostics?.[0]?.message ?? "Run failed");
  }
  return parsed.result;
}
