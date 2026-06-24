/**
 * MockAIProvider module (ai/MockAIProvider.ts).
 * @module
 */

import type { RuntimeValue } from "../runtime/interpreter.js";
import { buildPrompt } from "./PromptRuntime.js";
import type { AIProvider, CompletionRequest, DetectionRequest, EmbedRequest } from "./AIProvider.js";

function scanDistance(input?: RuntimeValue): number {
  // Description:
  //     ScanDistance.
  //
  // Inputs:
  //     input?: RuntimeValue
  //         Caller-supplied input?.
  //
  // Outputs:
  //     result: number
  //         Return value from `scanDistance`.
  //
  // Example:
  //     const result = scanDistance(input?);
  // Description:
  //     ScanDistance.
  //
  // Inputs:
  //     input?: RuntimeValue
  //         Caller-supplied input?.
  //
  // Outputs:
  //     result: number
  //         Return value from `scanDistance`.
  //
  // Example:
  //     const result = scanDistance(input?);

  // const result = scanDistance(input?);
  if (!input) return 5;

  // continue when kind equals "scan".
  if (input.kind === "scan") return input.nearestDistance;

  // continue when kind equals typeName === "Detection".
  if (input.kind === "object" && input.typeName === "Detection") {
    const nearest = input.fields.nearest_distance;

    // continue when kind equals "number".
    if (nearest?.kind === "number") return nearest.value;
  }
  return 5;
}

function actionProposal(linear: number, angular: number, source: string, trace: string[]): RuntimeValue {
  // Description:
  //     ActionProposal.
  //
  // Inputs:
  //     linear: number
  //         Caller-supplied linear.
  //     angular: number
  //         Caller-supplied angular.
  //     source: string
  //         Caller-supplied source.
  //     trace: string[]
  //         Caller-supplied trace.
  //
  // Outputs:
  //     result: RuntimeValue
  //         Return value from `actionProposal`.
  //
  // Example:
  //     const result = actionProposal(linear, angular, source, trace);
  // Description:
  //     ActionProposal.
  //
  // Inputs:
  //     linear: number
  //         Caller-supplied linear.
  //     angular: number
  //         Caller-supplied angular.
  //     source: string
  //         Caller-supplied source.
  //     trace: string[]
  //         Caller-supplied trace.
  //
  // Outputs:
  //     result: RuntimeValue
  //         Return value from `actionProposal`.
  //
  // Example:
  //     const result = actionProposal(linear, angular, source, trace);

  // const result = actionProposal(linear, angular, source, trace);
  return { kind: "action_proposal", linear, angular, source, trace, trusted: false };
}

export class MockAIProvider implements AIProvider {
  complete(request: CompletionRequest): RuntimeValue {
    // Complete.
    //
    // Parameters:
    // - `request` — input value
    //
    // Returns:
    // RuntimeValue.
    //
    // Options:
    // None.
    //
    // Example:

    // const result = complete(request);

    const prompt = buildPrompt(request.prompt, request.input);
    const dist = scanDistance(request.input);

    if (/stop|halt|wait/i.test(request.prompt)) {
      return actionProposal(0, 0, request.model, [
        `model=${request.model}`,
        `prompt=${prompt}`,
        "decision=stop",
      ]);
    }

    if (/turn|avoid|obstacle/i.test(request.prompt) || dist < 0.8) {
      const angular = dist < 0.4 ? 0.6 : 0.25;
      const linear = dist < 0.4 ? 0 : Math.min(0.4, dist * 0.3);
      return actionProposal(linear, angular, request.model, [
        `model=${request.model}`,
        `prompt=${prompt}`,
        `nearest_distance=${dist.toFixed(2)}`,
        "decision=avoid_obstacle",
      ]);
    }

    const linear = Math.min(0.8, dist * 0.45);
    return actionProposal(linear, 0, request.model, [
      `model=${request.model}`,
      `prompt=${prompt}`,
      `nearest_distance=${dist.toFixed(2)}`,
      "decision=forward",
    ]);
  }

  detect(request: DetectionRequest): RuntimeValue {
    // Detect.
    //
    // Parameters:
    // - `request` — input value
    //
    // Returns:
    // RuntimeValue.
    //
    // Options:
    // None.
    //
    // Example:

    // const result = detect(request);

    const dist = scanDistance(request.frame);
    const label = dist < 0.6 ? "obstacle" : dist < 1.2 ? "object" : "clear";
    const confidence = dist < 0.6 ? 0.94 : dist < 1.2 ? 0.82 : 0.71;

    return {
      kind: "object",
      typeName: "Detection",
      fields: {
        label: { kind: "string", value: label },
        confidence: { kind: "number", value: confidence, unit: "none" },
        nearest_distance: { kind: "number", value: dist, unit: "m" },
      },
    };
  }

  embed(request: EmbedRequest): RuntimeValue {
    const vector = Array.from({ length: 8 }, (_, i) =>
      Math.sin(request.text.length * 0.13 + i) * 0.5 + 0.5,
    );
    return { kind: "embedding", dimensions: vector.length, vector };
  }
}

export function mockSummarize(input?: RuntimeValue, model = "mock"): RuntimeValue {
  // Description:
  //     MockSummarize.
  //
  // Inputs:
  //     input?: RuntimeValue
  //         Caller-supplied input?.
  //     model = "mock": input value
  //         Caller-supplied model = "mock".
  //
  // Outputs:
  //     result: RuntimeValue
  //         Return value from `mockSummarize`.
  //
  // Example:
  //     const result = mockSummarize(input?, model = "mock");
  // Description:
  //     MockSummarize.
  //
  // Inputs:
  //     input?: RuntimeValue
  //         Caller-supplied input?.
  //     model = "mock": input value
  //         Caller-supplied model = "mock".
  //
  // Outputs:
  //     result: RuntimeValue
  //         Return value from `mockSummarize`.
  //
  // Example:
  //     const result = mockSummarize(input?, model = "mock");

  // const result = mockSummarize(input?, model);
  const summary =
    input?.kind === "scan"
      ? `Nearest obstacle at ${input.nearestDistance.toFixed(2)} m`
      : input?.kind === "object" && input.typeName === "Detection"
        ? `Detected ${(input.fields.label as { value: string })?.value ?? "object"}`
        : "Environment stable";
  return { kind: "completion", text: `[${model}] ${summary}`, model };
}

export function mockAnalyzeFrame(frame?: RuntimeValue, model = "mock"): RuntimeValue {
  // Description:
  //     MockAnalyzeFrame.
  //
  // Inputs:
  //     frame?: RuntimeValue
  //         Caller-supplied frame?.
  //     model = "mock": input value
  //         Caller-supplied model = "mock".
  //
  // Outputs:
  //     result: RuntimeValue
  //         Return value from `mockAnalyzeFrame`.
  //
  // Example:
  //     const result = mockAnalyzeFrame(frame?, model = "mock");
  // Description:
  //     MockAnalyzeFrame.
  //
  // Inputs:
  //     frame?: RuntimeValue
  //         Caller-supplied frame?.
  //     model = "mock": input value
  //         Caller-supplied model = "mock".
  //
  // Outputs:
  //     result: RuntimeValue
  //         Return value from `mockAnalyzeFrame`.
  //
  // Example:
  //     const result = mockAnalyzeFrame(frame?, model = "mock");

  // const result = mockAnalyzeFrame(frame?, model);
  const dist = scanDistance(frame);
  return {
    kind: "object",
    typeName: "Detection",
    fields: {
      label: { kind: "string", value: dist < 0.7 ? "cluttered_scene" : "open_scene" },
      confidence: { kind: "number", value: 0.86, unit: "none" },
      nearest_distance: { kind: "number", value: dist, unit: "m" },
    },
  };
}

export function mockCameraFrame(): RuntimeValue {
  // Description:
  //     MockCameraFrame.
  //
  // Inputs:
  //     None.
  //
  // Outputs:
  //     result: RuntimeValue
  //         Return value from `mockCameraFrame`.
  //
  // Example:
  //     const result = mockCameraFrame();
  // Description:
  //     MockCameraFrame.
  //
  // Inputs:
  //     None.
  //
  // Outputs:
  //     result: RuntimeValue
  //         Return value from `mockCameraFrame`.
  //
  // Example:
  //     const result = mockCameraFrame();

  // const result = mockCameraFrame();
  return {
    kind: "object",
    typeName: "CameraFrame",
    fields: {
      width: { kind: "number", value: 640, unit: "none" },
      height: { kind: "number", value: 480, unit: "none" },
      nearest_distance: { kind: "number", value: 1.5, unit: "m" },
    },
  };
}
