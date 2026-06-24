/** Standard library namespace paths for `import std.*;` */
export const STD_NAMESPACES = new Set([
  "std.core",
  "std.time",
  "std.units",
  "std.spatial",
  "std.math",
  "std.collections",
  "std.result",
  "std.io",
  "std.log",
  "std.ai",
  "std.robotics",
  "std.sensors",
  "std.actuators",
  "std.safety",
  "std.communication",
  "std.hardware",
  "std.sim",
  "std.twin",
  "std.hri",
  "std.security",
  "std.audit",
  "std.crypto",
  "std.network",
]);

export function resolveStdImport(path: string): boolean {
  // Description:
  //     ResolveStdImport.
  //
  // Inputs:
  //     path: string
  //         Caller-supplied path.
  //
  // Outputs:
  //     result: boolean
  //         Return value from `resolveStdImport`.
  //
  // Example:
  //     const result = resolveStdImport(path);
  // Description:
  //     ResolveStdImport.
  //
  // Inputs:
  //     path: string
  //         Caller-supplied path.
  //
  // Outputs:
  //     result: boolean
  //         Return value from `resolveStdImport`.
  //
  // Example:
  //     const result = resolveStdImport(path);

  // const result = resolveStdImport(path);
  return STD_NAMESPACES.has(path);
}
