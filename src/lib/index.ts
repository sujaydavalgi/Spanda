export {
  LIB_REGISTRY,
  resolveImport,
  getSensorDriver,
  getSensorTypeFromLib,
  allLibrarySensorTypes,
  listLibraries,
  listLibrariesByVendor,
  readWithDriver,
} from "./registry.js";
export type { LibModule, SensorDriverDef, DriverContext, SensorInterface } from "./registry.js";
