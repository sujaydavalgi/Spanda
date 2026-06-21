import { readFileSync } from "node:fs";
import { join } from "node:path";
import { describe, expect, it } from "vitest";
import { tokenize } from "../src/lexer/index.js";
import { parse } from "../src/parser/index.js";
import { verifyHardwareProgram } from "../src/hardware-verify.js";
import {
  applyGpsPositionFaults,
  faultToConnectivity,
  connectivityLinkToTransport,
  verifyRequiresConnectivity,
} from "../src/connectivity-positioning.js";
import { hardwareProfileFromDecl } from "../src/hardware-profile.js";

const examplesDir = join(import.meta.dirname, "..", "examples", "connectivity");

describe("connectivity verify (TS fallback)", () => {
  it("verifies connectivity requirements on RoverV2 with simulate faults", () => {
    const source = readFileSync(join(examplesDir, "connectivity_hardware_verify.sd"), "utf8");
    const program = parse(tokenize(source));
    const result = verifyHardwareProgram(program, { target: "RoverV2" });
    expect(result.target).toBe("RoverV2");
    expect(result.items.some((i) => i.category === "simulate")).toBe(true);
    expect(result.items.some((i) => i.category === "connectivity" && i.severity === "pass")).toBe(true);
    expect(result.items.some((i) => i.category === "sensors" && i.severity === "error")).toBe(true);
  });

  it("maps faults and link transports", () => {
    expect(faultToConnectivity("NetworkOutage")).toEqual({
      domain: "network",
      event: "disconnected",
    });
    expect(faultToConnectivity("GpsSpoofing")).toEqual({ domain: "gps", event: "spoofed" });
    expect(connectivityLinkToTransport("wifi")).toBe("mqtt");
    expect(connectivityLinkToTransport("cellular")).toBe("dds");
  });

  it("fails when required cellular missing from profile", () => {
    const program = parse(
      tokenize(`
requires_connectivity { cellular: required; }
hardware Tiny { connectivity [ WiFi6, GPS ]; }
robot R { actuator wheels: DifferentialDrive; }
deploy R to Tiny;
`),
    );
    const profile = hardwareProfileFromDecl(program.hardwareProfiles[0]!);
    const items = verifyRequiresConnectivity(program.requiresConnectivity!, profile);
    expect(items.some((i) => i.severity === "error")).toBe(true);
  });

  it("passes rover_deploy against RoverV1 when using builtins", () => {
    const source = readFileSync(
      join(import.meta.dirname, "..", "examples/hardware/rover_deploy.sd"),
      "utf8",
    );
    const program = parse(tokenize(source));
    const result = verifyHardwareProgram(program);
    expect(result.ok).toBe(true);
    expect(result.items.some((i) => i.category === "sensors" && i.severity === "pass")).toBe(true);
  });
});
