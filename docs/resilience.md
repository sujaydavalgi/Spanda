# Resilience

Resilience policies describe **fault tolerance strategies** and integrate with **readiness scoring**.

## Syntax

```spanda
resilience_policy RoverResilience {
    strategy dual_navigation;
    strategy graceful_degradation;
}

operating_mode DegradedMode { degraded; }
operating_mode SafeMode { safe; }

mitigation SensorFailover {
    if lidar.failed {
        reduce_speed 0.3 m/s;
        enter degraded_mode;
    }
}
```

## Core types

| Type | Role |
|------|------|
| `ResiliencePolicy` | Named strategies |
| `RecoveryPolicy` | Mitigation-derived recovery actions |
| `FaultToleranceStrategy` | Strategy name and description |
| `RedundancyModel` | Component replicas and failover |
| `SafeModeTransition` | Mode transition triggers |

## CLI

```bash
spanda resilience check rover.sd [--json]
```

Reports include readiness score from `spanda-readiness` — resilience does not replace fleet health or readiness engines.

## Package

Runtime failover backends: **`spanda-resilience`** (`assurance.resilience`).

## Example

See `examples/resilience/degraded_mode_recovery.sd`.
