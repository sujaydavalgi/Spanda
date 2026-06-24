/**
 * registry module (ai/registry.ts).
 * @module
 */

export type AiLibModule = {
  id: string;
  vendor: string;
  name: string;
  version: string;
  description: string;
  runtime: "onnx" | "tflite" | "tensorrt" | "openvino";
};

export const AI_LIB_REGISTRY: Record<string, AiLibModule> = {
  "onnx.runtime": {
    id: "onnx.runtime",
    vendor: "ONNX",
    name: "runtime",
    version: "1.0.0",
    description: "ONNX Runtime inference backend",
    runtime: "onnx",
  },
  "tflite.runtime": {
    id: "tflite.runtime",
    vendor: "TensorFlow",
    name: "runtime",
    version: "1.0.0",
    description: "TensorFlow Lite inference backend",
    runtime: "tflite",
  },
  "tensorrt.runtime": {
    id: "tensorrt.runtime",
    vendor: "NVIDIA",
    name: "runtime",
    version: "1.0.0",
    description: "TensorRT inference backend for Jetson",
    runtime: "tensorrt",
  },
  "openvino.runtime": {
    id: "openvino.runtime",
    vendor: "Intel",
    name: "runtime",
    version: "1.0.0",
    description: "OpenVINO inference backend for Intel CPUs and VPUs",
    runtime: "openvino",
  },
};

export function resolveAiImport(path: string): AiLibModule | undefined {
  // Description:
  //     ResolveAiImport.
  //
  // Inputs:
  //     path: string
  //         Caller-supplied path.
  //
  // Outputs:
  //     result: AiLibModule | undefined
  //         Return value from `resolveAiImport`.
  //
  // Example:
  //     const result = resolveAiImport(path);
  // Description:
  //     ResolveAiImport.
  //
  // Inputs:
  //     path: string
  //         Caller-supplied path.
  //
  // Outputs:
  //     result: AiLibModule | undefined
  //         Return value from `resolveAiImport`.
  //
  // Example:
  //     const result = resolveAiImport(path);

  // const result = resolveAiImport(path);
  return AI_LIB_REGISTRY[path];
}

export function listAiLibraries(): AiLibModule[] {
  // Description:
  //     ListAiLibraries.
  //
  // Inputs:
  //     None.
  //
  // Outputs:
  //     result: AiLibModule[]
  //         Return value from `listAiLibraries`.
  //
  // Example:
  //     const result = listAiLibraries();
  // Description:
  //     ListAiLibraries.
  //
  // Inputs:
  //     None.
  //
  // Outputs:
  //     result: AiLibModule[]
  //         Return value from `listAiLibraries`.
  //
  // Example:
  //     const result = listAiLibraries();

  // const result = listAiLibraries();
  return Object.values(AI_LIB_REGISTRY);
}
