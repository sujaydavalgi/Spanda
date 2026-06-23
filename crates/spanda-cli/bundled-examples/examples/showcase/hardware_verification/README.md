# Hardware verification — obstacle avoidance

Mission requires **obstacle_avoidance**, which needs a **Lidar** sensor on the deployment target.

## Fail — robot without Lidar

```bash
spanda verify examples/showcase/hardware_verification/mission_missing_lidar.sd
```

Expected: verification error — required sensor `Lidar` not on hardware profile.

## Pass — add Lidar to robot and profile

```bash
spanda verify examples/showcase/hardware_verification/mission_with_lidar.sd --json --target RoverV1
```

One command: `spanda demo verify`
