/**
 * examples module (examples.ts).
 * @module
 */

export const KILLER_DEMO_SOURCE = `// Flagship demo — safety gate + deploy + verify (see docs/killer-demo.md)
requires_hardware {
  memory >= 2 GB;
  sensors [ Camera, Lidar ];
}

hardware RoverV1 {
  cpu: CortexA78;
  memory: 4 GB;
  sensors [ Camera, Lidar, IMU ];
  actuators [ DifferentialDrive ];
  battery { capacity: 100 Wh; }
  timing { min_period: 10 ms; }
}

robot SafePatrol {
  sensor camera: Camera on "/camera";
  sensor lidar: Lidar on "/scan";
  actuator wheels: DifferentialDrive;

  ai_model planner: LLM {
    provider: "mock";
    model: "patrol-planner";
    temperature: 0.1;
  }

  safety {
    max_speed = 0.5 m/s;
    stop_if lidar.nearest_distance < 0.5 m;
  }

  behavior patrol() {
    loop every 100ms {
      let scan = lidar.read();
      let scene = camera.analyze();
      let proposal = planner.reason(
        prompt: "Plan safe forward motion",
        input: scene
      );
      let action = safety.validate(proposal);
      wheels.execute(action);
    }
  }

  verify {
    robot.velocity().linear <= 2.0 m/s;
  }
}

deploy SafePatrol to RoverV1;`;

export const DEFAULT_SOURCE = KILLER_DEMO_SOURCE;

export const EXAMPLES = [
  {
    name: "Killer demo (flagship)",
    source: KILLER_DEMO_SOURCE,
  },
  {
    name: "AI navigation",
    source: `robot Rover {
  sensor lidar: Lidar on "/scan";
  actuator wheels: DifferentialDrive;

  ai_model planner: LLM {
    provider: "mock";
    model: "safe-planner";
    temperature: 0.1;
  }

  safety {
    max_speed = 1.0 m/s;
    stop_if lidar.nearest_distance < 0.5 m;
  }

  agent Navigator {
    uses planner;
    tools [lidar, wheels];
    goal "Navigate safely";

    plan {
      let scan = lidar.read();
      let proposal = planner.reason(prompt: "Create a safe navigation action", input: scan);
      let action = safety.validate(proposal);
      wheels.execute(action);
    }
  }

  behavior run() {
    loop every 100ms {
      Navigator.plan();
    }
  }
}`,
  },
  {
    name: "Lidar avoidance",
    source: `robot Avoider {
  sensor lidar: Lidar on "/scan";
  actuator wheels: DifferentialDrive;

  safety {
    max_speed = 0.8 m/s;
    stop_if lidar.nearest_distance < 0.4 m;
  }

  behavior avoid() {
    loop every 100ms {
      let d = lidar.read();
      if d.nearest_distance < 1.0 m {
        wheels.drive(linear: 0.0 m/s, angular: 0.5 rad/s);
      } else {
        wheels.drive(linear: 0.4 m/s, angular: 0.0 rad/s);
      }
    }
  }
}`,
  },
  {
    name: "Warehouse logistics",
    source: `robot WarehouseBot {
  node logistics on "/warehouse";
  topic status: String publish on "/status";
  sensor lidar: Lidar on "/scan";
  actuator wheels: DifferentialDrive;

  safety {
    max_speed = 0.8 m/s;
  }

  behavior deliver() {
    publish status with "en route";
    loop every 100ms {
      wheels.drive(linear: 0.3 m/s, angular: 0.0 rad/s);
    }
  }
}`,
  },
];
