import { describe, expect, it } from "vitest";
import { compile } from "../src/compile.js";
import { verifyCertificationProof } from "../src/certify-verify.js";

describe("certify verify (TS mirror)", () => {
  it("warns when deploy lacks certification metadata", () => {
    const source = `
hardware Tiny { actuators [ DifferentialDrive ]; }
robot Rover {
  actuator wheels: DifferentialDrive;
  behavior run() { wheels.stop(); }
}
deploy Rover to Tiny;
`;
    const { program } = compile(source, "typescript");
    const items = verifyCertificationProof(program, false);
    expect(items.some((i) => i.category === "certify" && i.severity === "warning")).toBe(true);
  });

  it("errors under strict certify", () => {
    const source = `
hardware Tiny { actuators [ DifferentialDrive ]; }
robot Rover {
  actuator wheels: DifferentialDrive;
  behavior run() { wheels.stop(); }
}
deploy Rover to Tiny;
`;
    const { program } = compile(source, "typescript");
    const items = verifyCertificationProof(program, true);
    expect(items.some((i) => i.category === "certify" && i.severity === "error")).toBe(true);
  });
});
