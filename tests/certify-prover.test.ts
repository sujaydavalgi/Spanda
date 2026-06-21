import { join } from "node:path";
import { describe, expect, it } from "vitest";
import { compile } from "../src/compile.js";
import { buildCertificationProof } from "../src/certify-prover.js";
import { invokeNav2Bridge } from "../src/adapter-bridge.js";

describe("certify prover (TS mirror)", () => {
  it("passes for certified deployment example", () => {
    const source = `
certify ISO13849 { level PLd; }
hardware Tiny { actuators [ DifferentialDrive ]; }
robot Rover {
  actuator wheels: DifferentialDrive;
  safety { max_speed = 0.8 m/s; }
  mission Patrol { navigate; }
  behavior run() { wheels.stop(); }
}
deploy Rover to Tiny;
`;
    const { program } = compile(source, "typescript");
    const proof = buildCertificationProof(program, "certified.sd", true);
    expect(proof.passed).toBe(true);
  });
});

describe("adapter bridge (TS mirror)", () => {
  it("invokes configured Nav2 bridge command", () => {
    const previous = process.env.SPANDA_NAV2_CMD;
    const script = join(import.meta.dirname, "..", "examples/adapters/nav2_bridge.sh");
    process.env.SPANDA_NAV2_CMD = `bash ${script} {goal}`;
    expect(invokeNav2Bridge("Dock")).toContain("accepted goal=Dock");
    if (previous === undefined) delete process.env.SPANDA_NAV2_CMD;
    else process.env.SPANDA_NAV2_CMD = previous;
  });
});
