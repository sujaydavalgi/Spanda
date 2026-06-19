import { describe, it, expect } from "vitest";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { compile, run } from "../src/compile.js";
import { tokenize } from "../src/lexer/index.js";
import { parse, ParseError } from "../src/parser/index.js";
import { TypeCheckError } from "../src/types/index.js";
import { createDefaultSimulator } from "../src/simulator/index.js";

function expectTypeCheckError(source: string, messagePart: string): void {
  try {
    compile(source);
    expect.fail("expected type check to fail");
  } catch (err) {
    expect(err).toBeInstanceOf(TypeCheckError);
    const messages = (err as TypeCheckError).errors.map((e) => e.message).join("\n");
    expect(messages).toContain(messagePart);
  }
}

function expectParseError(source: string, messagePart: string): void {
  try {
    parse(tokenize(source));
    expect.fail("expected parse to fail");
  } catch (err) {
    expect(err).toBeInstanceOf(ParseError);
    expect((err as ParseError).message).toContain(messagePart);
  }
}

describe("type system", () => {
  it("accepts foundation types with annotations", () => {
    expect(() =>
      compile(`
        robot R {
          actuator wheels: DifferentialDrive;
          behavior run() {
            let count: Int = 3;
            let label: String = "rover";
            let active: Bool = true;
            let _ = count;
            wheels.stop();
          }
        }
      `),
    ).not.toThrow();
  });

  it("type-checks generic collection annotations", () => {
    expect(() =>
      compile(`
        robot R {
          actuator wheels: DifferentialDrive;
          behavior run() {
            let goals: Array<Goal>;
            let scan_topic: Topic<LidarScan>;
            let svc: Service<Command, Feedback>;
            wheels.stop();
          }
        }
      `),
    ).not.toThrow();
  });

  it("rejects undefined initializer with generic annotation", () => {
    expectTypeCheckError(
      `
        robot R {
          actuator wheels: DifferentialDrive;
          behavior run() {
            let goals: Array<Goal> = goals_placeholder;
            wheels.stop();
          }
        }
      `,
      "Undefined",
    );
  });

  it("rejects generic arity mismatch at parse time", () => {
    expectParseError(
      `
        robot R {
          actuator wheels: DifferentialDrive;
          behavior run() {
            let bad: Array<Int, Float>;
            wheels.stop();
          }
        }
      `,
      "expects 1",
    );
  });

  it("accepts valid unit literals and operations", () => {
    expect(() =>
      compile(`
        robot R {
          actuator wheels: DifferentialDrive;
          behavior run() {
            let timeout: Duration = 500 ms;
            let speed: Velocity = 1.5 m/s;
            let distance: Distance = 2.0 m;
            let total: Distance = distance + 1.0 m;
            let _ = total;
            wheels.stop();
          }
        }
      `),
    ).not.toThrow();
  });

  it("rejects incompatible unit addition", () => {
    expectTypeCheckError(
      `
        robot R {
          actuator wheels: DifferentialDrive;
          behavior run() {
            let speed: Velocity = 1.0 m/s;
            let distance: Distance = 2.0 m;
            let bad = speed + distance;
            wheels.stop();
          }
        }
      `,
      "incompatible",
    );
  });

  it("rejects distance plus duration", () => {
    expectTypeCheckError(
      `
        robot R {
          actuator wheels: DifferentialDrive;
          behavior run() {
            let d: Distance = 1.0 m;
            let t: Duration = 500 ms;
            let bad = d + t;
            wheels.stop();
          }
        }
      `,
      "incompatible",
    );
  });

  it("accepts spatial, sensor, and AI type annotations", () => {
    expect(() =>
      compile(`
        robot R {
          sensor lidar: Lidar on "/scan";
          actuator wheels: DifferentialDrive;
          ai_model planner: LLM { provider: "mock"; model: "p"; temperature: 0.1; }
          behavior run() {
            let pose: Pose;
            let path: Path;
            let scan: LidarScan;
            let frame: CameraFrame;
            let prompt: Prompt;
            wheels.stop();
          }
        }
      `),
    ).not.toThrow();
  });

  it("rejects ActionProposal passed directly to execute", () => {
    expectTypeCheckError(
      `
        robot R {
          sensor lidar: Lidar on "/scan";
          actuator wheels: DifferentialDrive;
          ai_model planner: LLM { provider: "mock"; model: "p"; temperature: 0.1; }
          behavior run() {
            let proposal: ActionProposal = planner.reason(prompt: "go");
            wheels.execute(proposal);
          }
        }
      `,
      "ActionProposal",
    );
  });

  it("accepts SafeAction from safety.validate before execute", () => {
    expect(() =>
      compile(`
        robot R {
          sensor lidar: Lidar on "/scan";
          actuator wheels: DifferentialDrive;
          ai_model planner: LLM { provider: "mock"; model: "p"; temperature: 0.1; }
          safety { max_speed = 1.0 m/s; }
          behavior run() {
            let proposal: ActionProposal = planner.reason(prompt: "go");
            let action: SafeAction = safety.validate(proposal);
            wheels.execute(action);
          }
        }
      `),
    ).not.toThrow();
  });

  it("rejects unknown type at parse time", () => {
    expectParseError(
      `
        robot R {
          actuator wheels: DifferentialDrive;
          behavior run() {
            let x: NotARealType;
            wheels.stop();
          }
        }
      `,
      "Unknown type",
    );
  });

  it("runs safety example", () => {
    const source = readFileSync(join(import.meta.dirname, "../examples/types/safety.sd"), "utf-8");
    const { program } = compile(source);
    expect(() =>
      run(program, {
        backend: createDefaultSimulator(),
        entryBehavior: "run",
      }),
    ).not.toThrow();
  });
});
