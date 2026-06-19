robot ArmBot {
  sensor ft_sensor: ForceTorque on "/ft";
  actuator arm: RoboticArm;
  actuator gripper: Gripper;

  safety {
    max_speed = 0.5 m/s;
  }

  behavior pick_and_place() {
    // Move to pick position
    arm.move_to(x: 0.3 m, y: 0.1 m, z: 0.2 m);
    gripper.open();

    loop every 100ms {
      arm.move_to(x: 0.3 m, y: 0.1 m, z: 0.05 m);
      gripper.close();

      let force = ft_sensor.read();
      arm.move_to(x: 0.5 m, y: 0.3 m, z: 0.2 m);
      gripper.open();
    }
  }
}
