import { describe, it, expect } from "vitest";
import { tokenize } from "../src/lexer/index.js";
import { parse } from "../src/parser/index.js";

const SAMPLE = `
robot Rover {
  sensor lidar: Lidar on "/scan";
  actuator wheels: DifferentialDrive;

  safety {
    max_speed = 1.5 m/s;
    stop_if lidar.read().nearest_distance < 0.5 m;
  }

  behavior avoid_obstacles() {
    loop every 50ms {
      let scan = lidar.read();
      if scan.nearest_distance < 0.5 m {
        wheels.stop();
      } else {
        wheels.drive(linear: 0.8 m/s, angular: 0.2 rad/s);
      }
    }
  }
}
`;

describe("parser", () => {
  it("parses a complete robot program", () => {
    const tokens = tokenize(SAMPLE);
    const ast = parse(tokens);

    expect(ast.robots).toHaveLength(1);
    const robot = ast.robots[0];
    expect(robot.name).toBe("Rover");
    expect(robot.sensors).toHaveLength(1);
    expect(robot.sensors[0].name).toBe("lidar");
    expect(robot.sensors[0].binding).toEqual({ kind: "topic", path: "/scan" });
    expect(robot.actuators).toHaveLength(1);
    expect(robot.safety).not.toBeNull();
    expect(robot.safety!.rules).toHaveLength(2);
    expect(robot.behaviors).toHaveLength(1);
  });

  it("parses loop every syntax", () => {
    const tokens = tokenize("robot R { behavior b() { loop every 50ms { } } }");
    const ast = parse(tokens);
    const loop = ast.robots[0].behaviors[0].body[0];
    expect(loop.kind).toBe("LoopStmt");
    if (loop.kind === "LoopStmt") {
      expect(loop.intervalMs).toBe(50);
    }
  });

  it("parses if/else statements", () => {
    const tokens = tokenize("robot R { behavior b() { if true { } else { } } }");
    const ast = parse(tokens);
    const ifStmt = ast.robots[0].behaviors[0].body[0];
    expect(ifStmt.kind).toBe("IfStmt");
    if (ifStmt.kind === "IfStmt") {
      expect(ifStmt.elseBranch).not.toBeNull();
    }
  });

  it("parses method calls with named arguments", () => {
    const tokens = tokenize("robot R { behavior b() { wheels.drive(linear: 0.8 m/s, angular: 0.2 rad/s); } actuator wheels: DifferentialDrive; }");
    const ast = parse(tokens);
    const behavior = ast.robots[0].behaviors[0];
    const stmt = behavior.body[0];
    expect(stmt.kind).toBe("ExprStmt");
  });

  it("parses safety max_speed rule", () => {
    const tokens = tokenize("robot R { safety { max_speed = 1.5 m/s; } }");
    const ast = parse(tokens);
    const rule = ast.robots[0].safety!.rules[0];
    expect(rule.kind).toBe("MaxSpeedRule");
  });
});
