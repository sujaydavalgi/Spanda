/**
 * examples module (examples.ts).
 * @module
 */

export const DEFAULT_SOURCE = `robot Rover {
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
    memory short_term;
    goal "Reach destination while avoiding obstacles";

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
}`;

export const EXAMPLES = [
  { name: "AI navigation", source: DEFAULT_SOURCE },
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
  service clear_map: ClearCostmap;
  action follow_route: FollowPath;
  sensor lidar: Lidar on "/scan";
  actuator wheels: DifferentialDrive;

  safety {
    max_speed = 0.8 m/s;
    zone loading_dock rect at (2.0 m, 1.0 m) size (1.5 m, 1.0 m);
  }

  behavior deliver() {
    let start = robot.pose();
    let dock = pose(x: 2.5 m, y: 1.5 m, theta: 1.57 rad);
    let path = trajectory(from: start, to: dock, steps: 10);
    call clear_map();
    send_goal follow_route with path;
    publish status with "en route";
    loop every 100ms {
      wheels.follow(path: path);
    }
  }
}`,
  },
  {
    name: "Outdoor navigation",
    source: `robot OutdoorRover {
  sensor imu: IMU on "/imu";
  sensor lidar: Lidar on "/scan";
  actuator wheels: DifferentialDrive;

  safety {
    max_speed = 1.2 m/s;
    stop_if lidar.nearest_distance < 0.6 m;
  }

  behavior traverse() {
    loop every 50ms {
      let heading = imu.read();
      let scan = lidar.read();
      if scan.nearest_distance < 1.0 m {
        wheels.drive(linear: 0.0 m/s, angular: 0.4 rad/s);
      } else {
        wheels.drive(linear: 0.6 m/s, angular: heading.yaw * 0.05 rad/s);
      }
    }
  }
}`,
  },
];
