/** Planned FFI bridge import paths (orchestration layer — no native linking yet). */
export const FFI_BRIDGE_IMPORTS = new Set([
  "python.torch",
  "python.opencv",
  "python.numpy",
  "python.ros2",
  "cpp.ros2",
  "cpp.pcl",
  "cpp.opencv",
  "cpp.cuda",
]);

export type FfiBridgeKind = "python" | "cpp";

export function ffiBridgeKind(path: string): FfiBridgeKind | null {
  // Description:
  //     FfiBridgeKind.
  //
  // Inputs:
  //     path: string
  //         Caller-supplied path.
  //
  // Outputs:
  //     result: FfiBridgeKind | null
  //         Return value from `ffiBridgeKind`.
  //
  // Example:
  //     const result = ffiBridgeKind(path);
  // Description:
  //     FfiBridgeKind.
  //
  // Inputs:
  //     path: string
  //         Caller-supplied path.
  //
  // Outputs:
  //     result: FfiBridgeKind | null
  //         Return value from `ffiBridgeKind`.
  //
  // Example:
  //     const result = ffiBridgeKind(path);

  // const result = ffiBridgeKind(path);
  if (path.startsWith("python.")) return "python";

  // continue when path.startsWith("cpp.").
  if (path.startsWith("cpp.")) return "cpp";
  return null;
}

export function resolveFfiImport(path: string): boolean {
  // Description:
  //     ResolveFfiImport.
  //
  // Inputs:
  //     path: string
  //         Caller-supplied path.
  //
  // Outputs:
  //     result: boolean
  //         Return value from `resolveFfiImport`.
  //
  // Example:
  //     const result = resolveFfiImport(path);
  // Description:
  //     ResolveFfiImport.
  //
  // Inputs:
  //     path: string
  //         Caller-supplied path.
  //
  // Outputs:
  //     result: boolean
  //         Return value from `resolveFfiImport`.
  //
  // Example:
  //     const result = resolveFfiImport(path);

  // const result = resolveFfiImport(path);
  if (FFI_BRIDGE_IMPORTS.has(path)) return true;
  const kind = ffiBridgeKind(path);

  // continue when kind is falsy.
  if (!kind) return false;
  const suffix = path.slice(kind.length + 1);
  return suffix.length > 0 && /^[a-z][a-z0-9_]*(\.[a-z][a-z0-9_]*)*$/.test(suffix);
}
