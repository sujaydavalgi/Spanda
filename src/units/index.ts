/**
 * index module (units/index.ts).
 * @module
 */

import type { UnitKind } from "../ast/nodes.js";

export type PhysicalCategory =
  | "scalar"
  | "distance"
  | "duration"
  | "velocity"
  | "acceleration"
  | "angle"
  | "angular_velocity"
  | "mass"
  | "force"
  | "power"
  | "voltage"
  | "current"
  | "temperature"
  | "pressure"
  | "frequency"
  | "humidity"
  | "illuminance"
  | "luminance"
  | "concentration"
  | "sound_level"
  | "magnetic_field"
  | "rotational_speed"
  | "torque"
  | "energy"
  | "uv_index"
  | "ph"
  | "conductivity"
  | "particulate_matter"
  | "turbidity"
  | "salinity"
  | "radiation"
  | "soil_moisture";

const DEG_TO_RAD = Math.PI / 180;

export function canonicalUnit(category: PhysicalCategory): UnitKind {
  // Description:
  //     CanonicalUnit.
  //
  // Inputs:
  //     category: PhysicalCategory
  //         Caller-supplied category.
  //
  // Outputs:
  //     result: UnitKind
  //         Return value from `canonicalUnit`.
  //
  // Example:
  //     const result = canonicalUnit(category);
  // Description:
  //     CanonicalUnit.
  //
  // Inputs:
  //     category: PhysicalCategory
  //         Caller-supplied category.
  //
  // Outputs:
  //     result: UnitKind
  //         Return value from `canonicalUnit`.
  //
  // Example:
  //     const result = canonicalUnit(category);

  // const result = canonicalUnit(category);
  switch (category) {
    case "scalar": return "none";
    case "distance": return "m";
    case "duration": return "s";
    case "velocity": return "m/s";
    case "acceleration": return "m/s²";
    case "angle": return "rad";
    case "angular_velocity": return "rad/s";
    case "mass": return "kg";
    case "force": return "N";
    case "power": return "W";
    case "voltage": return "V";
    case "current": return "A";
    case "temperature": return "celsius";
    case "pressure": return "Pa";
    case "frequency": return "Hz";
    case "humidity": return "rh";
    case "illuminance": return "lux";
    case "luminance": return "cd/m²";
    case "concentration": return "ppm";
    case "sound_level": return "dB";
    case "magnetic_field": return "uT";
    case "rotational_speed": return "rpm";
    case "torque": return "N·m";
    case "energy": return "J";
    case "uv_index": return "uvi";
    case "ph": return "pH";
    case "conductivity": return "uS/cm";
    case "particulate_matter": return "ug/m3";
    case "turbidity": return "NTU";
    case "salinity": return "ppt";
    case "radiation": return "uSv/h";
    case "soil_moisture": return "%VWC";
  }
}

export function unitCategory(unit: UnitKind): PhysicalCategory {
  // Description:
  //     UnitCategory.
  //
  // Inputs:
  //     unit: UnitKind
  //         Caller-supplied unit.
  //
  // Outputs:
  //     result: PhysicalCategory
  //         Return value from `unitCategory`.
  //
  // Example:
  //     const result = unitCategory(unit);
  // Description:
  //     UnitCategory.
  //
  // Inputs:
  //     unit: UnitKind
  //         Caller-supplied unit.
  //
  // Outputs:
  //     result: PhysicalCategory
  //         Return value from `unitCategory`.
  //
  // Example:
  //     const result = unitCategory(unit);

  // const result = unitCategory(unit);
  switch (unit) {
    case "none":
      return "scalar";
    case "m":
    case "mm":
    case "cm":
    case "km":
    case "ft":
    case "in":
      return "distance";
    case "s":
    case "ms":
    case "us":
    case "min":
    case "h":
      return "duration";
    case "m/s":
    case "km/h":
    case "mph":
      return "velocity";
    case "m/s²":
    case "g":
      return "acceleration";
    case "rad":
    case "deg":
      return "angle";
    case "rad/s":
    case "deg/s":
      return "angular_velocity";
    case "kg":
    case "gram":
    case "lb":
      return "mass";
    case "N":
    case "kN":
      return "force";
    case "W":
    case "kW":
    case "MW":
      return "power";
    case "V":
    case "mV":
    case "kV":
      return "voltage";
    case "A":
    case "mA":
      return "current";
    case "celsius":
    case "fahrenheit":
    case "kelvin":
      return "temperature";
    case "Pa":
    case "kPa":
    case "bar":
    case "psi":
    case "mbar":
      return "pressure";
    case "Hz":
    case "kHz":
    case "MHz":
      return "frequency";
    case "rh":
    case "%RH":
      return "humidity";
    case "lux":
    case "lx":
      return "illuminance";
    case "cd/m²":
    case "nit":
      return "luminance";
    case "ppm":
    case "ppb":
      return "concentration";
    case "dB":
    case "dBA":
      return "sound_level";
    case "uT":
    case "gauss":
      return "magnetic_field";
    case "rpm":
      return "rotational_speed";
    case "N·m":
    case "Nm":
      return "torque";
    case "J":
    case "Wh":
    case "kWh":
      return "energy";
    case "uvi":
      return "uv_index";
    case "pH":
      return "ph";
    case "uS/cm":
    case "mS/cm":
    case "S/m":
      return "conductivity";
    case "ug/m3":
      return "particulate_matter";
    case "NTU":
    case "FNU":
      return "turbidity";
    case "ppt":
    case "psu":
      return "salinity";
    case "uSv/h":
    case "mSv/h":
      return "radiation";
    case "%VWC":
    case "vwc":
      return "soil_moisture";
    default:
      return "scalar";
  }
}

export function unitsCompatible(a: UnitKind, b: UnitKind): boolean {
  // Description:
  //     UnitsCompatible.
  //
  // Inputs:
  //     a: UnitKind
  //         Caller-supplied a.
  //     b: UnitKind
  //         Caller-supplied b.
  //
  // Outputs:
  //     result: boolean
  //         Return value from `unitsCompatible`.
  //
  // Example:
  //     const result = unitsCompatible(a, b);
  // Description:
  //     UnitsCompatible.
  //
  // Inputs:
  //     a: UnitKind
  //         Caller-supplied a.
  //     b: UnitKind
  //         Caller-supplied b.
  //
  // Outputs:
  //     result: boolean
  //         Return value from `unitsCompatible`.
  //
  // Example:
  //     const result = unitsCompatible(a, b);

  // const result = unitsCompatible(a, b);
  if (a === b) return true;

  // continue when a equals "none" || b === "none".
  if (a === "none" || b === "none") return true;
  return unitCategory(a) === unitCategory(b);
}

export function unitMatchesNamedType(typeName: string, unit: UnitKind): boolean {
  // Description:
  //     UnitMatchesNamedType.
  //
  // Inputs:
  //     typeName: string
  //         Caller-supplied typeName.
  //     unit: UnitKind
  //         Caller-supplied unit.
  //
  // Outputs:
  //     result: boolean
  //         Return value from `unitMatchesNamedType`.
  //
  // Example:
  //     const result = unitMatchesNamedType(typeName, unit);
  // Description:
  //     UnitMatchesNamedType.
  //
  // Inputs:
  //     typeName: string
  //         Caller-supplied typeName.
  //     unit: UnitKind
  //         Caller-supplied unit.
  //
  // Outputs:
  //     result: boolean
  //         Return value from `unitMatchesNamedType`.
  //
  // Example:
  //     const result = unitMatchesNamedType(typeName, unit);

  // const result = unitMatchesNamedType(typeName, unit);
  switch (typeName) {
    case "Distance":
      return unitCategory(unit) === "distance";
    case "Duration":
      return unitCategory(unit) === "duration";
    case "Velocity":
      return unitCategory(unit) === "velocity";
    case "Acceleration":
      return unitCategory(unit) === "acceleration";
    case "Angle":
      return unitCategory(unit) === "angle";
    case "AngularVelocity":
      return unitCategory(unit) === "angular_velocity";
    case "Mass":
      return unitCategory(unit) === "mass";
    case "Force":
      return unitCategory(unit) === "force";
    case "Power":
      return unitCategory(unit) === "power";
    case "Voltage":
      return unitCategory(unit) === "voltage";
    case "Current":
      return unitCategory(unit) === "current";
    case "Temperature":
      return unitCategory(unit) === "temperature";
    case "Pressure":
      return unitCategory(unit) === "pressure";
    case "Humidity":
      return unitCategory(unit) === "humidity";
    case "Illuminance":
      return unitCategory(unit) === "illuminance";
    case "Luminance":
      return unitCategory(unit) === "luminance";
    case "Concentration":
      return unitCategory(unit) === "concentration";
    case "SoundLevel":
      return unitCategory(unit) === "sound_level";
    case "MagneticField":
      return unitCategory(unit) === "magnetic_field";
    case "RotationalSpeed":
      return unitCategory(unit) === "rotational_speed";
    case "Torque":
      return unitCategory(unit) === "torque";
    case "Energy":
      return unitCategory(unit) === "energy";
    case "UvIndex":
      return unitCategory(unit) === "uv_index";
    case "Ph":
      return unitCategory(unit) === "ph";
    case "Conductivity":
      return unitCategory(unit) === "conductivity";
    case "ParticulateMatter":
      return unitCategory(unit) === "particulate_matter";
    case "Turbidity":
      return unitCategory(unit) === "turbidity";
    case "Salinity":
      return unitCategory(unit) === "salinity";
    case "Radiation":
      return unitCategory(unit) === "radiation";
    case "SoilMoisture":
      return unitCategory(unit) === "soil_moisture";
    default:
      return false;
  }
}

function toCanonicalLinear(value: number, unit: UnitKind): number {
  // Description:
  //     ToCanonicalLinear.
  //
  // Inputs:
  //     value: number
  //         Caller-supplied value.
  //     unit: UnitKind
  //         Caller-supplied unit.
  //
  // Outputs:
  //     result: number
  //         Return value from `toCanonicalLinear`.
  //
  // Example:
  //     const result = toCanonicalLinear(value, unit);
  // Description:
  //     ToCanonicalLinear.
  //
  // Inputs:
  //     value: number
  //         Caller-supplied value.
  //     unit: UnitKind
  //         Caller-supplied unit.
  //
  // Outputs:
  //     result: number
  //         Return value from `toCanonicalLinear`.
  //
  // Example:
  //     const result = toCanonicalLinear(value, unit);

  // const result = toCanonicalLinear(value, unit);
  switch (unit) {
    case "m": return value;
    case "mm": return value / 1000;
    case "cm": return value / 100;
    case "km": return value * 1000;
    case "ft": return value * 0.3048;
    case "in": return value * 0.0254;
    case "s": return value;
    case "ms": return value / 1000;
    case "us": return value / 1_000_000;
    case "min": return value * 60;
    case "h": return value * 3600;
    case "m/s": return value;
    case "km/h": return value / 3.6;
    case "mph": return value / 2.2369362920544;
    case "m/s²": return value;
    case "g": return value * 9.80665;
    case "rad": return value;
    case "deg": return value * DEG_TO_RAD;
    case "rad/s": return value;
    case "deg/s": return value * DEG_TO_RAD;
    case "kg": return value;
    case "gram": return value / 1000;
    case "lb": return value * 0.45359237;
    case "N": return value;
    case "kN": return value * 1000;
    case "W": return value;
    case "kW": return value * 1000;
    case "MW": return value * 1_000_000;
    case "V": return value;
    case "mV": return value / 1000;
    case "kV": return value * 1000;
    case "A": return value;
    case "mA": return value / 1000;
    case "celsius": return value;
    case "fahrenheit": return (value - 32) * 5 / 9;
    case "kelvin": return value - 273.15;
    case "Pa": return value;
    case "kPa": return value * 1000;
    case "bar": return value * 100_000;
    case "mbar": return value * 100;
    case "psi": return value * 6894.757293168;
    case "Hz": return value;
    case "kHz": return value * 1000;
    case "MHz": return value * 1_000_000;
    case "rh":
    case "%RH": return value;
    case "lux":
    case "lx": return value;
    case "cd/m²":
    case "nit": return value;
    case "ppm": return value;
    case "ppb": return value / 1000;
    case "dB":
    case "dBA": return value;
    case "uT": return value;
    case "gauss": return value * 100;
    case "rpm": return value;
    case "N·m":
    case "Nm": return value;
    case "J": return value;
    case "Wh": return value * 3600;
    case "kWh": return value * 3_600_000;
    case "uvi": return value;
    case "pH": return value;
    case "uS/cm": return value;
    case "mS/cm": return value * 1000;
    case "S/m": return value * 10_000;
    case "ug/m3": return value;
    case "NTU":
    case "FNU": return value;
    case "ppt":
    case "psu": return value;
    case "uSv/h": return value;
    case "mSv/h": return value * 1000;
    case "%VWC":
    case "vwc": return value;
    default:
      return value;
  }
}

function fromCanonical(value: number, category: PhysicalCategory, to: UnitKind): number {
  // Description:
  //     FromCanonical.
  //
  // Inputs:
  //     value: number
  //         Caller-supplied value.
  //     category: PhysicalCategory
  //         Caller-supplied category.
  //     to: UnitKind
  //         Caller-supplied to.
  //
  // Outputs:
  //     result: number
  //         Return value from `fromCanonical`.
  //
  // Example:
  //     const result = fromCanonical(value, category, to);
  // Description:
  //     FromCanonical.
  //
  // Inputs:
  //     value: number
  //         Caller-supplied value.
  //     category: PhysicalCategory
  //         Caller-supplied category.
  //     to: UnitKind
  //         Caller-supplied to.
  //
  // Outputs:
  //     result: number
  //         Return value from `fromCanonical`.
  //
  // Example:
  //     const result = fromCanonical(value, category, to);

  // const result = fromCanonical(value, category, to);
  switch (category) {
    case "distance":

      // Branch on to.
      switch (to) {
        case "m": return value;
        case "mm": return value * 1000;
        case "cm": return value * 100;
        case "km": return value / 1000;
        case "ft": return value / 0.3048;
        case "in": return value / 0.0254;
        default: return value;
      }
    case "duration":

      // Branch on to.
      switch (to) {
        case "s": return value;
        case "ms": return value * 1000;
        case "us": return value * 1_000_000;
        case "min": return value / 60;
        case "h": return value / 3600;
        default: return value;
      }
    case "velocity":

      // Branch on to.
      switch (to) {
        case "m/s": return value;
        case "km/h": return value * 3.6;
        case "mph": return value * 2.2369362920544;
        default: return value;
      }
    case "acceleration":

      // Branch on to.
      switch (to) {
        case "m/s²": return value;
        case "g": return value / 9.80665;
        default: return value;
      }
    case "angle":

      // Branch on to.
      switch (to) {
        case "rad": return value;
        case "deg": return value / DEG_TO_RAD;
        default: return value;
      }
    case "angular_velocity":

      // Branch on to.
      switch (to) {
        case "rad/s": return value;
        case "deg/s": return value / DEG_TO_RAD;
        default: return value;
      }
    case "mass":

      // Branch on to.
      switch (to) {
        case "kg": return value;
        case "gram": return value * 1000;
        case "lb": return value / 0.45359237;
        default: return value;
      }
    case "force":

      // Branch on to.
      switch (to) {
        case "N": return value;
        case "kN": return value / 1000;
        default: return value;
      }
    case "power":

      // Branch on to.
      switch (to) {
        case "W": return value;
        case "kW": return value / 1000;
        case "MW": return value / 1_000_000;
        default: return value;
      }
    case "voltage":

      // Branch on to.
      switch (to) {
        case "V": return value;
        case "mV": return value * 1000;
        case "kV": return value / 1000;
        default: return value;
      }
    case "current":

      // Branch on to.
      switch (to) {
        case "A": return value;
        case "mA": return value * 1000;
        default: return value;
      }
    case "temperature":

      // Branch on to.
      switch (to) {
        case "celsius": return value;
        case "fahrenheit": return value * 9 / 5 + 32;
        case "kelvin": return value + 273.15;
        default: return value;
      }
    case "pressure":

      // Branch on to.
      switch (to) {
        case "Pa": return value;
        case "kPa": return value / 1000;
        case "bar": return value / 100_000;
        case "mbar": return value / 100;
        case "psi": return value / 6894.757293168;
        default: return value;
      }
    case "frequency":

      // Branch on to.
      switch (to) {
        case "Hz": return value;
        case "kHz": return value / 1000;
        case "MHz": return value / 1_000_000;
        default: return value;
      }
    case "humidity":

      // Branch on to.
      switch (to) {
        case "rh":
        case "%RH": return value;
        default: return value;
      }
    case "illuminance":

      // Branch on to.
      switch (to) {
        case "lux":
        case "lx": return value;
        default: return value;
      }
    case "luminance":

      // Branch on to.
      switch (to) {
        case "cd/m²":
        case "nit": return value;
        default: return value;
      }
    case "concentration":

      // Branch on to.
      switch (to) {
        case "ppm": return value;
        case "ppb": return value * 1000;
        default: return value;
      }
    case "sound_level":

      // Branch on to.
      switch (to) {
        case "dB":
        case "dBA": return value;
        default: return value;
      }
    case "magnetic_field":

      // Branch on to.
      switch (to) {
        case "uT": return value;
        case "gauss": return value / 100;
        default: return value;
      }
    case "rotational_speed":

      // Branch on to.
      switch (to) {
        case "rpm": return value;
        default: return value;
      }
    case "torque":

      // Branch on to.
      switch (to) {
        case "N·m":
        case "Nm": return value;
        default: return value;
      }
    case "energy":

      // Branch on to.
      switch (to) {
        case "J": return value;
        case "Wh": return value / 3600;
        case "kWh": return value / 3_600_000;
        default: return value;
      }
    case "uv_index":

      // Branch on to.
      switch (to) {
        case "uvi": return value;
        default: return value;
      }
    case "ph":

      // Branch on to.
      switch (to) {
        case "pH": return value;
        default: return value;
      }
    case "conductivity":

      // Branch on to.
      switch (to) {
        case "uS/cm": return value;
        case "mS/cm": return value / 1000;
        case "S/m": return value / 10_000;
        default: return value;
      }
    case "particulate_matter":

      // Branch on to.
      switch (to) {
        case "ug/m3": return value;
        default: return value;
      }
    case "turbidity":

      // Branch on to.
      switch (to) {
        case "NTU":
        case "FNU": return value;
        default: return value;
      }
    case "salinity":

      // Branch on to.
      switch (to) {
        case "ppt":
        case "psu": return value;
        default: return value;
      }
    case "radiation":

      // Branch on to.
      switch (to) {
        case "uSv/h": return value;
        case "mSv/h": return value / 1000;
        default: return value;
      }
    case "soil_moisture":

      // Branch on to.
      switch (to) {
        case "%VWC":
        case "vwc": return value;
        default: return value;
      }
    default:
      return value;
  }
}

export function convertValue(value: number, from: UnitKind, to: UnitKind): number | undefined {
  // Description:
  //     ConvertValue.
  //
  // Inputs:
  //     value: number
  //         Caller-supplied value.
  //     from: UnitKind
  //         Caller-supplied from.
  //     to: UnitKind
  //         Caller-supplied to.
  //
  // Outputs:
  //     result: number | undefined
  //         Return value from `convertValue`.
  //
  // Example:
  //     const result = convertValue(value, from, to);
  // Description:
  //     ConvertValue.
  //
  // Inputs:
  //     value: number
  //         Caller-supplied value.
  //     from: UnitKind
  //         Caller-supplied from.
  //     to: UnitKind
  //         Caller-supplied to.
  //
  // Outputs:
  //     result: number | undefined
  //         Return value from `convertValue`.
  //
  // Example:
  //     const result = convertValue(value, from, to);

  // const result = convertValue(value, from, to);
  if (from === to) return value;

  // continue when unitsCompatible is falsy.
  if (!unitsCompatible(from, to)) return undefined;
  const canonical = toCanonicalLinear(value, from);
  return fromCanonical(canonical, unitCategory(from), to);
}

export function alignForBinary(
  left: number,
  leftUnit: UnitKind,
  right: number,
  rightUnit: UnitKind,
): [number, number, UnitKind] | undefined {
  // Description:
  //     AlignForBinary.
  //
  // Inputs:
  //     left: number
  //         Caller-supplied left.
  //     leftUnit: UnitKind
  //         Caller-supplied leftUnit.
  //     right: number
  //         Caller-supplied right.
  //     rightUnit: UnitKind
  //         Caller-supplied rightUnit.
  //
  // Outputs:
  //     result: [number, number, UnitKind] | undefined
  //         Return value from `alignForBinary`.
  //
  // Example:
  //     const result = alignForBinary(left, leftUnit, right, rightUnit);
  // Description:
  //     AlignForBinary.
  //
  // Inputs:
  //     left: number
  //         Caller-supplied left.
  //     leftUnit: UnitKind
  //         Caller-supplied leftUnit.
  //     right: number
  //         Caller-supplied right.
  //     rightUnit: UnitKind
  //         Caller-supplied rightUnit.
  //
  // Outputs:
  //     result: [number, number, UnitKind] | undefined
  //         Return value from `alignForBinary`.
  //
  // Example:
  //     const result = alignForBinary(left, leftUnit, right, rightUnit);

  // const result = alignForBinary(left, leftUnit, right, rightUnit);
  if (!unitsCompatible(leftUnit, rightUnit)) return undefined;

  // continue when leftUnit equals rightUnit.
  if (leftUnit === rightUnit) return [left, right, leftUnit];
  const rightInLeft = convertValue(right, rightUnit, leftUnit);

  // continue when rightInLeft equals undefined.
  if (rightInLeft === undefined) return undefined;
  return [left, rightInLeft, leftUnit];
}
