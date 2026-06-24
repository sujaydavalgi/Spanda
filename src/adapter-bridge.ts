/**
 * Optional subprocess bridges for production Nav2/SLAM adapter backends.
 * @module
 */

import { execFileSync } from "node:child_process";

function bridgeCommand(envKey: string): string | undefined {
  // Description:
  //     BridgeCommand.
  //
  // Inputs:
  //     envKey: string
  //         Caller-supplied envKey.
  //
  // Outputs:
  //     result: string | undefined
  //         Return value from `bridgeCommand`.
  //
  // Example:

  //     const result = bridgeCommand(envKey);

  const value = process.env[envKey]?.trim();
  return value ? value : undefined;
}

export function invokeNav2Bridge(goal: string): string | undefined {
  // Description:
  //     InvokeNav2Bridge.
  //
  // Inputs:
  //     goal: string
  //         Caller-supplied goal.
  //
  // Outputs:
  //     result: string | undefined
  //         Return value from `invokeNav2Bridge`.
  //
  // Example:

  //     const result = invokeNav2Bridge(goal);

  const template = bridgeCommand("SPANDA_NAV2_CMD");
  if (!template) return undefined;
  const commandLine = template.replace("{goal}", goal);
  const parts = commandLine.split(/\s+/).filter(Boolean);
  const program = parts[0];
  if (!program) return undefined;
  try {
    const output = execFileSync(program, parts.slice(1), { encoding: "utf-8" });
    return output.trim();
  } catch {
    return undefined;
  }
}

export function invokeSlamBridge(operation: string): string | undefined {
  // Description:
  //     InvokeSlamBridge.
  //
  // Inputs:
  //     operation: string
  //         Caller-supplied operation.
  //
  // Outputs:
  //     result: string | undefined
  //         Return value from `invokeSlamBridge`.
  //
  // Example:

  //     const result = invokeSlamBridge(operation);

  const template = bridgeCommand("SPANDA_SLAM_CMD");
  if (!template) return undefined;
  const commandLine = template.replace("{op}", operation);
  const parts = commandLine.split(/\s+/).filter(Boolean);
  const program = parts[0];
  if (!program) return undefined;
  try {
    const output = execFileSync(program, parts.slice(1), { encoding: "utf-8" });
    return output.trim();
  } catch {
    return undefined;
  }
}
