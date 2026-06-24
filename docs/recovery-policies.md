# Recovery Policies

Recovery policies declare conditional self-healing actions in Spanda source.

## Syntax

```spanda
recovery_policy RoverRecovery {
    on gps.failed {
        switch_to visual_odometry;
        reduce_speed 0.5 m/s;
        enter degraded_mode;
    }
    on lidar.failed {
        reduce_speed 0.3 m/s;
        enter safe_mode;
    }
}
```

## Operating modes

Declare modes for verified transitions:

```spanda
operating_mode NormalMode { normal; }
operating_mode DegradedMode { degraded; }
operating_mode SafeMode { safe; }
operating_mode EmergencyMode { emergency; }
operating_mode RecoveryMode { recovery; }
```

## Relationship to mitigation

`mitigation` blocks use `if` conditions; `recovery_policy` blocks use `on` triggers. Both feed the recovery planner.

## Human approval

Actions that require operator approval:

- Resume mission
- Open gate
- Enable unsafe mode
- Restart fleet

Use `requires approval Operator` in mission declarations for high-risk recovery paths.

## Fleet recovery

```spanda
recovery_policy FleetRecovery {
    on fleet.failed {
        reassign mission;
        promote backup coordinator;
        redistribute tasks;
    }
}
```

Mesh relay: set `SPANDA_FLEET_MESH_URL` on the coordinator runtime; the mesh coordinator exposes `POST /v1/fleet/recovery`. Deployed fleet agents load programs via `POST /v1/program` and run interpreter-backed recovery (`recovery_engine: interpreter`) or assurance fallback. See [fleet-distributed.md](./fleet-distributed.md) and [self-healing.md](./self-healing.md).

## Example

See `examples/showcase/self_healing/rover.sd` and `examples/resilience/degraded_mode_recovery.sd`.
