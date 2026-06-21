import { readFileSync } from "node:fs";
import { join } from "node:path";
import { describe, expect, it } from "vitest";
import { tokenize } from "../src/lexer/index.js";
import { parse } from "../src/parser/index.js";
import { run } from "../src/compile.js";
import { createDefaultSimulator } from "../src/simulator/index.js";
import { applyGpsPositionFaults } from "../src/connectivity-positioning.js";

describe("connectivity runtime", () => {
  it("failovers link and transport on network outage", () => {
    const source = readFileSync(
      join(import.meta.dirname, "..", "examples", "connectivity", "wifi_lte_failover.sd"),
      "utf8",
    );
    const augmented = `${source}\nsimulate_compatibility { fault NetworkOutage; }\n`;
    const program = parse(tokenize(augmented));
    const logs: string[] = [];
    run(program, {
      backend: createDefaultSimulator(),
      onLog: (msg) => logs.push(msg),
    });
    expect(logs.some((l) => l.includes("emit network.disconnected"))).toBe(true);
    expect(logs.some((l) => l.includes("failover wifi -> cellular"))).toBe(true);
  });

  it("dispatches gps.lost when GPSLost fault is simulated", () => {
    const program = parse(
      tokenize(`
simulate_compatibility { fault GPSLost; }
robot R {
  on gps.lost { }
  behavior idle() { }
}
`),
    );
    const logs: string[] = [];
    run(program, {
      backend: createDefaultSimulator(),
      onLog: (msg) => logs.push(msg),
    });
    expect(logs.some((l) => l.includes("emit gps.lost"))).toBe(true);
  });

  it("offsets GPS fix under GpsSpoofing and fires gps.spoofed", () => {
    const program = parse(
      tokenize(`
simulate_compatibility { fault GpsSpoofing; }
robot R {
  sensor gps: GPS;
  on gps.spoofed { }
  behavior idle() {
    let fix = gps.read();
    let _ = fix;
  }
}
`),
    );
    const logs: string[] = [];
    run(program, {
      backend: createDefaultSimulator(),
      onLog: (msg) => logs.push(msg),
    });
    expect(logs.some((l) => l.includes("emit gps.spoofed"))).toBe(true);
  });

  it("applyGpsPositionFaults drifts coordinates over time", () => {
    const faults = new Set(["GpsDrift"]);
    const a = applyGpsPositionFaults(faults, 30.0, -97.0, 0);
    const b = applyGpsPositionFaults(faults, 30.0, -97.0, 60_000);
    expect(b.lat).toBeGreaterThan(a.lat);
  });
});
