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
  // FfiBridgeKind.
  //
  // Parameters:
  // - `path` — input value
  //
  // Returns:
  // `Some` / non-null value on success, otherwise `None` / null.
  //
  // Options:
  // None.
  //
  // Example:
  // const result = ffiBridgeKind(path);

  if (path.startsWith("python.")) return "python";
  if (path.startsWith("cpp.")) return "cpp";
  return null;
}

export function resolveFfiImport(path: string): boolean {
  // ResolveFfiImport.
  //
  // Parameters:
  // - `path` — input value
  //
  // Returns:
  // `true` or `false`.
  //
  // Options:
  // None.
  //
  // Example:
  // const result = resolveFfiImport(path);

  if (FFI_BRIDGE_IMPORTS.has(path)) return true;
  const kind = ffiBridgeKind(path);
  if (!kind) return false;
  const suffix = path.slice(kind.length + 1);
  return suffix.length > 0 && /^[a-z][a-z0-9_]*(\.[a-z][a-z0-9_]*)*$/.test(suffix);
}
