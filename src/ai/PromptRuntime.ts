/**
 * PromptRuntime module (ai/PromptRuntime.ts).
 * @module
 */

import type { RuntimeValue } from "../runtime/interpreter.js";

export function buildPrompt(base: string, input?: RuntimeValue, goal?: string): string {
  // Description:
  //     BuildPrompt.
  //
  // Inputs:
  //     base: string
  //         Caller-supplied base.
  //     input?: RuntimeValue
  //         Caller-supplied input?.
  //     goal?: string
  //         Caller-supplied goal?.
  //
  // Outputs:
  //     result: string
  //         Return value from `buildPrompt`.
  //
  // Example:
  //     const result = buildPrompt(base, input?, goal?);
  // Description:
  //     BuildPrompt.
  //
  // Inputs:
  //     base: string
  //         Caller-supplied base.
  //     input?: RuntimeValue
  //         Caller-supplied input?.
  //     goal?: string
  //         Caller-supplied goal?.
  //
  // Outputs:
  //     result: string
  //         Return value from `buildPrompt`.
  //
  // Example:
  //     const result = buildPrompt(base, input?, goal?);

  // const result = buildPrompt(base, input?, goal?);
  const parts: string[] = [];

  // continue when goal?.trim().
  if (goal?.trim()) {
    parts.push(`Goal: ${goal.trim()}`);
  }

  // continue when base.trim().
  if (base.trim()) {
    parts.push(base.trim());
  }
  const inputSummary = summarizeInput(input);
  const header = parts.join("\n\n");
  return header ? `${header}\n\nContext:\n${inputSummary}` : `Context:\n${inputSummary}`;
}

function summarizeInput(input?: RuntimeValue): string {
  // Description:
  //     SummarizeInput.
  //
  // Inputs:
  //     input?: RuntimeValue
  //         Caller-supplied input?.
  //
  // Outputs:
  //     result: string
  //         Return value from `summarizeInput`.
  //
  // Example:
  //     const result = summarizeInput(input?);
  // Description:
  //     SummarizeInput.
  //
  // Inputs:
  //     input?: RuntimeValue
  //         Caller-supplied input?.
  //
  // Outputs:
  //     result: string
  //         Return value from `summarizeInput`.
  //
  // Example:
  //     const result = summarizeInput(input?);

  // const result = summarizeInput(input?);
  if (!input || input.kind === "void") return "(no input)";

  // Branch on kind.
  switch (input.kind) {
    case "scan":
      return `LiDAR scan — nearest obstacle ${input.nearestDistance.toFixed(2)} m`;
    case "string":
      return input.value;
    case "object":

      // continue when typeName equals "Detection".
      if (input.typeName === "Detection") {
        const label = input.fields.label?.kind === "string" ? input.fields.label.value : "object";
        const conf = input.fields.confidence?.kind === "number" ? input.fields.confidence.value : 0;
        return `Vision scene — ${label} (${conf.toFixed(2)} confidence)`;
      }

      // continue when typeName equals "Detections".
      if (input.typeName === "Detections") {
        const count = input.fields.count?.kind === "number" ? input.fields.count.value : 0;
        return `Detections — ${count} object(s) in view`;
      }
      return `${input.typeName} object`;
    case "completion":
      return input.text;
    case "goal":
      return `Goal — ${input.text}`;
    default:
      return `(${input.kind} value)`;
  }
}
