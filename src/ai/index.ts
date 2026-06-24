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
  // Description:
  //     RuntimeSafeAction.
  //
  // Inputs:
  //     linear: number
  //         Caller-supplied linear.
  //     angular: number
  //         Caller-supplied angular.
  //
  // Outputs:
  //     result: RuntimeValue
  //         Return value from `runtimeSafeAction`.
  //
  // Example:
  //     const result = runtimeSafeAction(linear, angular);
  // Description:
  //     RuntimeSafeAction.
  //
  // Inputs:
  //     linear: number
  //         Caller-supplied linear.
  //     angular: number
  //         Caller-supplied angular.
  //
  // Outputs:
  //     result: RuntimeValue
  //         Return value from `runtimeSafeAction`.
  //
  // Example:
  //     const result = runtimeSafeAction(linear, angular);

  // const result = runtimeSafeAction(linear, angular);
  return { kind: "safe_action", linear, angular, trusted: true };
}

export function runtimeActionProposal(
  linear: number,
  angular: number,
  source: string,
): RuntimeValue {
  // Description:
  //     RuntimeActionProposal.
  //
  // Inputs:
  //     linear: number
  //         Caller-supplied linear.
  //     angular: number
  //         Caller-supplied angular.
  //     source: string
  //         Caller-supplied source.
  //
  // Outputs:
  //     result: RuntimeValue
  //         Return value from `runtimeActionProposal`.
  //
  // Example:
  //     const result = runtimeActionProposal(linear, angular, source);
  // Description:
  //     RuntimeActionProposal.
  //
  // Inputs:
  //     linear: number
  //         Caller-supplied linear.
  //     angular: number
  //         Caller-supplied angular.
  //     source: string
  //         Caller-supplied source.
  //
  // Outputs:
  //     result: RuntimeValue
  //         Return value from `runtimeActionProposal`.
  //
  // Example:
  //     const result = runtimeActionProposal(linear, angular, source);

  // const result = runtimeActionProposal(linear, angular, source);
  return { kind: "action_proposal", linear, angular, source, trace: [], trusted: false };
}

export function isActionProposal(value: RuntimeValue): boolean {
  // Description:
  //     IsActionProposal.
  //
  // Inputs:
  //     value: RuntimeValue
  //         Caller-supplied value.
  //
  // Outputs:
  //     result: boolean
  //         Return value from `isActionProposal`.
  //
  // Example:
  //     const result = isActionProposal(value);
  // Description:
  //     IsActionProposal.
  //
  // Inputs:
  //     value: RuntimeValue
  //         Caller-supplied value.
  //
  // Outputs:
  //     result: boolean
  //         Return value from `isActionProposal`.
  //
  // Example:
  //     const result = isActionProposal(value);

  // const result = isActionProposal(value);
  return value.kind === "action_proposal";
}

export function isSafeAction(
  value: RuntimeValue,
): value is Extract<RuntimeValue, {
  // Description:
  //     IsSafeAction.
  //
  // Inputs:
  //     value: RuntimeValue
  //         Caller-supplied value.
  //
  // Outputs:
  //     result: value is Extract<RuntimeValue,
  //         Return value from `isSafeAction`.
  //
  // Example:
  //     const result = isSafeAction(value);
  // Description:
  //     IsSafeAction.
  //
  // Inputs:
  //     value: RuntimeValue
  //         Caller-supplied value.
  //
  // Outputs:
  //     result: value is Extract<RuntimeValue,
  //         Return value from `isSafeAction`.
  //
  // Example:
  //     const result = isSafeAction(value);

 // const result = isSafeAction(value);
 kind: "safe_action" }> {
  return value.kind === "safe_action";
}

export function proposalFromValue(value: RuntimeValue): {
  // Description:
  //     ProposalFromValue.
  //
  // Inputs:
  //     value: RuntimeValue
  //         Caller-supplied value.
  //
  // Outputs:
  //     None.
  //
  // Example:
  //     const result = proposalFromValue(value);
  // Description:
  //     ProposalFromValue.
  //
  // Inputs:
  //     value: RuntimeValue
  //         Caller-supplied value.
  //
  // Outputs:
  //     None.
  //
  // Example:
  //     const result = proposalFromValue(value);

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
  // Description:
  //     SafeActionFromProposal.
  //
  // Inputs:
  //     linear: number
  //         Caller-supplied linear.
  //     angular: number
  //         Caller-supplied angular.
  //
  // Outputs:
  //     result: RuntimeValue
  //         Return value from `safeActionFromProposal`.
  //
  // Example:
  //     const result = safeActionFromProposal(linear, angular);
  // Description:
  //     SafeActionFromProposal.
  //
  // Inputs:
  //     linear: number
  //         Caller-supplied linear.
  //     angular: number
  //         Caller-supplied angular.
  //
  // Outputs:
  //     result: RuntimeValue
  //         Return value from `safeActionFromProposal`.
  //
  // Example:
  //     const result = safeActionFromProposal(linear, angular);

  // const result = safeActionFromProposal(linear, angular);
  return runtimeSafeAction(linear, angular);
}

export function wrapCompletion(text: string, model: string): RuntimeValue {
  // Description:
  //     WrapCompletion.
  //
  // Inputs:
  //     text: string
  //         Caller-supplied text.
  //     model: string
  //         Caller-supplied model.
  //
  // Outputs:
  //     result: RuntimeValue
  //         Return value from `wrapCompletion`.
  //
  // Example:
  //     const result = wrapCompletion(text, model);
  // Description:
  //     WrapCompletion.
  //
  // Inputs:
  //     text: string
  //         Caller-supplied text.
  //     model: string
  //         Caller-supplied model.
  //
  // Outputs:
  //     result: RuntimeValue
  //         Return value from `wrapCompletion`.
  //
  // Example:
  //     const result = wrapCompletion(text, model);

  // const result = wrapCompletion(text, model);
  return { kind: "completion", text, model };
}

export function wrapDetection(
  label: string,
  confidence: number,
  nearestDistance: number,
): RuntimeValue {
  // Description:
  //     WrapDetection.
  //
  // Inputs:
  //     label: string
  //         Caller-supplied label.
  //     confidence: number
  //         Caller-supplied confidence.
  //     nearestDistance: number
  //         Caller-supplied nearestDistance.
  //
  // Outputs:
  //     result: RuntimeValue
  //         Return value from `wrapDetection`.
  //
  // Example:
  //     const result = wrapDetection(label, confidence, nearestDistance);
  // Description:
  //     WrapDetection.
  //
  // Inputs:
  //     label: string
  //         Caller-supplied label.
  //     confidence: number
  //         Caller-supplied confidence.
  //     nearestDistance: number
  //         Caller-supplied nearestDistance.
  //
  // Outputs:
  //     result: RuntimeValue
  //         Return value from `wrapDetection`.
  //
  // Example:
  //     const result = wrapDetection(label, confidence, nearestDistance);

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
