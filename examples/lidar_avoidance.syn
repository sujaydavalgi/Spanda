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
