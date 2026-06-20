/**
 * index module (ai/index.ts).
 * @module
 */

export type { AIProvider, CompletionRequest, DetectionRequest, EmbedRequest } from "./AIProvider.js";
export { AIModel, createAIModel, type AiModelConfig } from "./AIModel.js";
export type { AgentRuntime, PlanExecutor } from "./Agent.js";
export {
  agentToolNames,
  agentUsesModels,
  createAgentRuntime,
  executeAgentPlan,
} from "./Agent.js";
export { MemoryStore, type MemoryKind } from "./MemoryStore.js";
export { MockAIProvider, mockAnalyzeFrame, mockCameraFrame, mockSummarize } from "./MockAIProvider.js";
export { buildPrompt } from "./PromptRuntime.js";
export { resolveAiImport, listAiLibraries, AI_LIB_REGISTRY } from "./registry.js";

import type { RuntimeValue } from "../runtime/interpreter.js";

export function runtimeSafeAction(linear: number, angular: number): RuntimeValue {
  // RuntimeSafeAction.
  //
  // Parameters:
  // - `linear` — input value
  // - `angular` — input value
  //
  // Returns:
  // `RuntimeValue`.
  //
  // Options:
  // None.
  //
  // Example:
  // const result = runtimeSafeAction(linear, angular);

  return { kind: "safe_action", linear, angular, trusted: true };
}

export function runtimeActionProposal(
  linear: number,
  angular: number,
  source: string,
): RuntimeValue {
  // RuntimeActionProposal.
  //
  // Parameters:
  // - `linear` — input value
  // - `angular` — input value
  // - `source` — input value
  //
  // Returns:
  // `RuntimeValue`.
  //
  // Options:
  // None.
  //
  // Example:
  // const result = runtimeActionProposal(linear, angular, source);

  return { kind: "action_proposal", linear, angular, source, trace: [], trusted: false };
}

export function isActionProposal(value: RuntimeValue): boolean {
  // IsActionProposal.
  //
  // Parameters:
  // - `value` — input value
  //
  // Returns:
  // `true` or `false`.
  //
  // Options:
  // None.
  //
  // Example:
  // const result = isActionProposal(value);

  return value.kind === "action_proposal";
}

export function isSafeAction(
  value: RuntimeValue,
): value is Extract<RuntimeValue, {
  // IsSafeAction.
  //
  // Parameters:
  // - `value` — input value
  //
  // Returns:
  // `value is Extract<RuntimeValue,`.
  //
  // Options:
  // None.
  //
  // Example:
  // const result = isSafeAction(value);
 kind: "safe_action" }> {
  return value.kind === "safe_action";
}

export function proposalFromValue(value: RuntimeValue): {
  // ProposalFromValue.
  //
  // Parameters:
  // - `value` — input value
  //
  // Returns:
  // ``.
  //
  // Options:
  // None.
  //
  // Example:
  // const result = proposalFromValue(value);
 linear: number; angular: number; source: string } | null {
  if (value.kind === "action_proposal") {
    return { linear: value.linear, angular: value.angular, source: value.source };
  }
  if (value.kind === "object" && value.typeName === "ActionProposal") {
    const linear = value.fields.linear?.kind === "number" ? value.fields.linear.value : 0;
    const angular = value.fields.angular?.kind === "number" ? value.fields.angular.value : 0;
    return { linear, angular, source: "object" };
  }
  if (value.kind === "velocity") {
    return { linear: value.linear, angular: value.angular, source: "velocity" };
  }
  return null;
}

export function safeActionFromProposal(
  linear: number,
  angular: number,
): RuntimeValue {
  // SafeActionFromProposal.
  //
  // Parameters:
  // - `linear` — input value
  // - `angular` — input value
  //
  // Returns:
  // `RuntimeValue`.
  //
  // Options:
  // None.
  //
  // Example:
  // const result = safeActionFromProposal(linear, angular);

  return runtimeSafeAction(linear, angular);
}

export function wrapCompletion(text: string, model: string): RuntimeValue {
  // WrapCompletion.
  //
  // Parameters:
  // - `text` — input value
  // - `model` — input value
  //
  // Returns:
  // `RuntimeValue`.
  //
  // Options:
  // None.
  //
  // Example:
  // const result = wrapCompletion(text, model);

  return { kind: "completion", text, model };
}

export function wrapDetection(
  label: string,
  confidence: number,
  nearestDistance: number,
): RuntimeValue {
  // WrapDetection.
  //
  // Parameters:
  // - `label` — input value
  // - `confidence` — input value
  // - `nearestDistance` — input value
  //
  // Returns:
  // `RuntimeValue`.
  //
  // Options:
  // None.
  //
  // Example:
  // const result = wrapDetection(label, confidence, nearestDistance);

  return {
    kind: "object",
    typeName: "Detection",
    fields: {
      label: { kind: "string", value: label },
      confidence: { kind: "number", value: confidence, unit: "none" },
      nearest_distance: { kind: "number", value: nearestDistance, unit: "m" },
    },
  };
}

export type { RuntimeValue };
