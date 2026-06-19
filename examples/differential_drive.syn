robot Rover {
  actuator wheels: DifferentialDrive;

  safety {
    max_speed = 1.0 m/s;
  }

  behavior drive_forward() {
    loop every 100ms {
      wheels.drive(linear: 0.5 m/s, angular: 0.1 rad/s);
    }
  }
}
