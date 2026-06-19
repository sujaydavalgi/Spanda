/** Standard library namespace paths for `import std.*;` */
export const STD_NAMESPACES = new Set([
  "std.time",
  "std.units",
  "std.spatial",
  "std.ai",
  "std.robotics",
  "std.sensors",
  "std.safety",
  "std.twin",
  "std.hri",
]);

export function resolveStdImport(path: string): boolean {
  return STD_NAMESPACES.has(path);
}
