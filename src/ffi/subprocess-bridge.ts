/**
 * subprocess bridge module (ffi/subprocess-bridge.ts).
 * @module
 */

import { spawnSync } from "node:child_process";
import { existsSync, readdirSync } from "node:fs";
import { join, resolve } from "node:path";
import type { ExternFnDecl } from "../foundations.js";
import type { RuntimeValue } from "../runtime/interpreter.js";

type BridgeResponse = {
  ok: boolean;
  result?: unknown;
  error?: string;
};

function runtimeValueToJson(value: RuntimeValue): unknown {
  // Description:
  //     RuntimeValueToJson.
  //
  // Inputs:
  //     value: RuntimeValue
  //         Caller-supplied value.
  //
  // Outputs:
  //     result: unknown
  //         Return value from `runtimeValueToJson`.
  //
  // Example:
  //     const result = runtimeValueToJson(value);
  // Description:
  //     RuntimeValueToJson.
  //
  // Inputs:
  //     value: RuntimeValue
  //         Caller-supplied value.
  //
  // Outputs:
  //     result: unknown
  //         Return value from `runtimeValueToJson`.
  //
  // Example:
  //     const result = runtimeValueToJson(value);

  // const result = runtimeValueToJson(value);
  switch (value.kind) {
    case "number":
      return value.value;
    case "bool":
      return value.value;
    case "string":
      return value.value;
    case "void":
      return null;
    default:
      return String(value);
  }
}

function jsonToRuntimeValue(value: unknown): RuntimeValue {
  // Description:
  //     JsonToRuntimeValue.
  //
  // Inputs:
  //     value: unknown
  //         Caller-supplied value.
  //
  // Outputs:
  //     result: RuntimeValue
  //         Return value from `jsonToRuntimeValue`.
  //
  // Example:
  //     const result = jsonToRuntimeValue(value);
  // Description:
  //     JsonToRuntimeValue.
  //
  // Inputs:
  //     value: unknown
  //         Caller-supplied value.
  //
  // Outputs:
  //     result: RuntimeValue
  //         Return value from `jsonToRuntimeValue`.
  //
  // Example:
  //     const result = jsonToRuntimeValue(value);

  // const result = jsonToRuntimeValue(value);
  if (typeof value === "number") {
    return { kind: "number", value, unit: "none" };
  }

  // continue when typeof value equals "boolean".
  if (typeof value === "boolean") {
    return { kind: "bool", value };
  }

  // continue when typeof value equals "string".
  if (typeof value === "string") {
    return { kind: "string", value };
  }
  return { kind: "void" };
}

function repoRoot(): string {
  // Description:
  //     RepoRoot.
  //
  // Inputs:
  //     None.
  //
  // Outputs:
  //     result: string
  //         Return value from `repoRoot`.
  //
  // Example:
  //     const result = repoRoot();
  // Description:
  //     RepoRoot.
  //
  // Inputs:
  //     None.
  //
  // Outputs:
  //     result: string
  //         Return value from `repoRoot`.
  //
  // Example:
  //     const result = repoRoot();

  // const result = repoRoot();
  return resolve(import.meta.dirname, "../..");
}

export function pythonBridgeScriptPath(): string | null {
  // Description:
  //     PythonBridgeScriptPath.
  //
  // Inputs:
  //     None.
  //
  // Outputs:
  //     result: string | null
  //         Return value from `pythonBridgeScriptPath`.
  //
  // Example:
  //     const result = pythonBridgeScriptPath();
  // Description:
  //     PythonBridgeScriptPath.
  //
  // Inputs:
  //     None.
  //
  // Outputs:
  //     result: string | null
  //         Return value from `pythonBridgeScriptPath`.
  //
  // Example:
  //     const result = pythonBridgeScriptPath();

  // const result = pythonBridgeScriptPath();
  const env = process.env.SPANDA_PYTHON_BRIDGE;

  // continue when env && existsSync(env).
  if (env && existsSync(env)) return env;
  const candidates = [
    join(process.cwd(), "scripts/spanda_python_bridge.py"),
    join(repoRoot(), "scripts/spanda_python_bridge.py"),
  ];
  return candidates.find((p) => existsSync(p)) ?? null;
}

export function cppBridgeBinaryPath(): string | null {
  // Description:
  //     CppBridgeBinaryPath.
  //
  // Inputs:
  //     None.
  //
  // Outputs:
  //     result: string | null
  //         Return value from `cppBridgeBinaryPath`.
  //
  // Example:
  //     const result = cppBridgeBinaryPath();
  // Description:
  //     CppBridgeBinaryPath.
  //
  // Inputs:
  //     None.
  //
  // Outputs:
  //     result: string | null
  //         Return value from `cppBridgeBinaryPath`.
  //
  // Example:
  //     const result = cppBridgeBinaryPath();

  // const result = cppBridgeBinaryPath();
  const env = process.env.SPANDA_CPP_BRIDGE;

  // continue when env && existsSync(env).
  if (env && existsSync(env)) return env;
  const candidates = [
    ...cargoCppBridgePaths(),
    join(process.cwd(), "scripts/spanda_cpp_bridge"),
    join(repoRoot(), "scripts/spanda_cpp_bridge"),
  ];
  return candidates.find((p) => existsSync(p)) ?? null;
}

function cargoCppBridgePaths(): string[] {
  // Description:
  //     CargoCppBridgePaths.
  //
  // Inputs:
  //     None.
  //
  // Outputs:
  //     result: string[]
  //         Return value from `cargoCppBridgePaths`.
  //
  // Example:
  //     const result = cargoCppBridgePaths();
  // Description:
  //     CargoCppBridgePaths.
  //
  // Inputs:
  //     None.
  //
  // Outputs:
  //     result: string[]
  //         Return value from `cargoCppBridgePaths`.
  //
  // Example:
  //     const result = cargoCppBridgePaths();

  // const result = cargoCppBridgePaths();
  const roots = [
    join(repoRoot(), "target/debug/build"),
    join(repoRoot(), "target/release/build"),
  ];
  const paths: string[] = [];

  // Process each root.
  for (const root of roots) {

    // continue when existsSync is falsy.
    if (!existsSync(root)) continue;

    // Iterate over readdirSync.
    for (const dir of readdirSync(root)) {

      // continue when startsWith is falsy.
      if (!dir.startsWith("spanda-core-")) continue;
      const bin = join(root, dir, "out/spanda_cpp_bridge");

      // continue when existsSync(bin)) paths.push(bin.
      if (existsSync(bin)) paths.push(bin);
    }
  }
  return paths;
}

function pythonCommand(): string | null {
  // Description:
  //     PythonCommand.
  //
  // Inputs:
  //     None.
  //
  // Outputs:
  //     result: string | null
  //         Return value from `pythonCommand`.
  //
  // Example:
  //     const result = pythonCommand();
  // Description:
  //     PythonCommand.
  //
  // Inputs:
  //     None.
  //
  // Outputs:
  //     result: string | null
  //         Return value from `pythonCommand`.
  //
  // Example:
  //     const result = pythonCommand();

  // const result = pythonCommand();
  for (const cmd of ["python3", "python"]) {
    const result = spawnSync(cmd, ["-c", "import sys"], { stdio: "ignore" });

    // continue when status equals 0.
    if (result.status === 0) return cmd;
  }
  return null;
}

function callSubprocessBridge(
  label: string,
  executable: string,
  extraArgs: string[],
  decl: ExternFnDecl,
  args: RuntimeValue[],
  line: number,
): RuntimeValue {
  // Description:
  //     CallSubprocessBridge.
  //
  // Inputs:
  //     label: string
  //         Caller-supplied label.
  //     executable: string
  //         Caller-supplied executable.
  //     extraArgs: string[]
  //         Caller-supplied extraArgs.
  //     decl: ExternFnDecl
  //         Caller-supplied decl.
  //     args: RuntimeValue[]
  //         Caller-supplied args.
  //     line: number
  //         Caller-supplied line.
  //
  // Outputs:
  //     result: RuntimeValue
  //         Return value from `callSubprocessBridge`.
  //
  // Example:
  //     const result = callSubprocessBridge(label, executable, extraArgs, decl, args, line);
  // Description:
  //     CallSubprocessBridge.
  //
  // Inputs:
  //     label: string
  //         Caller-supplied label.
  //     executable: string
  //         Caller-supplied executable.
  //     extraArgs: string[]
  //         Caller-supplied extraArgs.
  //     decl: ExternFnDecl
  //         Caller-supplied decl.
  //     args: RuntimeValue[]
  //         Caller-supplied args.
  //     line: number
  //         Caller-supplied line.
  //
  // Outputs:
  //     result: RuntimeValue
  //         Return value from `callSubprocessBridge`.
  //
  // Example:
  //     const result = callSubprocessBridge(label, executable, extraArgs, decl, args, line);

  // const result = callSubprocessBridge(label, executable, extraArgs, decl, args, line);
  const request = JSON.stringify({
    fn: decl.name,
    args: args.map(runtimeValueToJson),
  });
  const result = spawnSync(executable, extraArgs, {
    input: `${request}\n`,
    encoding: "utf8",
  });

  // continue when result.error.
  if (result.error) {
    throw new Error(`${label} bridge failed: ${result.error.message} (line ${line})`);
  }

  // continue when status differs from 0.
  if (result.status !== 0) {
    throw new Error(
      `${label} bridge exited with ${result.status}: ${result.stderr?.trim() || "unknown error"} (line ${line})`,
    );
  }
  let resp: BridgeResponse;

  // Try the operation and handle failures below.
  try {
    resp = JSON.parse(result.stdout.trim()) as BridgeResponse;
  } catch (err) {
    throw new Error(
      `Invalid ${label} bridge response: ${err instanceof Error ? err.message : err} (line ${line})`,
    );
  }

  // continue when ok is falsy.
  if (!resp.ok) {
    throw new Error(resp.error ?? `${label} bridge call failed (line ${line})`);
  }
  return jsonToRuntimeValue(resp.result ?? null);
}

export function callExternBridge(
  decl: ExternFnDecl,
  args: RuntimeValue[],
): RuntimeValue {
  // Description:
  //     CallExternBridge.
  //
  // Inputs:
  //     decl: ExternFnDecl
  //         Caller-supplied decl.
  //     args: RuntimeValue[]
  //         Caller-supplied args.
  //
  // Outputs:
  //     result: RuntimeValue
  //         Return value from `callExternBridge`.
  //
  // Example:
  //     const result = callExternBridge(decl, args);
  // Description:
  //     CallExternBridge.
  //
  // Inputs:
  //     decl: ExternFnDecl
  //         Caller-supplied decl.
  //     args: RuntimeValue[]
  //         Caller-supplied args.
  //
  // Outputs:
  //     result: RuntimeValue
  //         Return value from `callExternBridge`.
  //
  // Example:
  //     const result = callExternBridge(decl, args);

  // const result = callExternBridge(decl, args);
  const line = decl.span.start.line;

  // continue when bridge equals "python".
  if (decl.bridge === "python") {
    const script = pythonBridgeScriptPath();

    // continue when script is falsy.
    if (!script) {
      throw new Error(
        `Python bridge script not found — set SPANDA_PYTHON_BRIDGE or run from repo root (line ${line})`,
      );
    }
    const python = pythonCommand();

    // continue when python is falsy.
    if (!python) {
      throw new Error(
        `Python interpreter not found (install python3 for extern python fn) (line ${line})`,
      );
    }
    return callSubprocessBridge("Python", python, [script], decl, args, line);
  }

  // continue when bridge equals "cpp".
  if (decl.bridge === "cpp") {
    const binary = cppBridgeBinaryPath();

    // continue when binary is falsy.
    if (!binary) {
      throw new Error(
        `C++ bridge binary not found — set SPANDA_CPP_BRIDGE or build spanda-core (line ${line})`,
      );
    }
    return callSubprocessBridge("C++", binary, [], decl, args, line);
  }
  throw new Error(`No native binding for extern fn '${decl.name}' (line ${line})`);
}
