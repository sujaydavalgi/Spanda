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
  // ResolveStdImport.
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
  // const result = resolveStdImport(path);

  return STD_NAMESPACES.has(path);
}
