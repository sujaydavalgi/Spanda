robot Drone {
  sensor altimeter: AltitudeSensor on "/altitude";
  actuator rotors: DroneRotors;

  safety {
    max_speed = 2.0 m/s;
    stop_if altimeter.read() < 0.5 m;
  }

  behavior altitude_hold() {
    loop every 50ms {
      let altitude = altimeter.read();

      if altitude < 1.0 m {
        rotors.set_thrust(thrust: 0.7);
      } else {
        if altitude > 2.0 m {
          rotors.set_thrust(thrust: 0.3);
        } else {
          rotors.hover();
        }
      }
    }
  }
}
