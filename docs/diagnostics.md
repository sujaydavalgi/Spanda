# Diagnostics

Diagnostics link anomalies and mission traces to **root causes** and **recovery actions**.

## Syntax

Mitigation plans feed static diagnosis:

```spanda
mitigation GPSLostMitigation {
    if gps.failed {
        switch_to visual_odometry;
        reduce_speed 0.5 m/s;
        enter degraded_mode;
    }
}
```

## Core types

| Type | Role |
|------|------|
| `Diagnosis` | Subject + root causes + fault tree |
| `RootCause` | Description, confidence, contributing factors |
| `FaultTree` | Top event and gate structure |
| `CausalGraph` | Cause-effect edges |

## CLI

```bash
spanda diagnose mission.trace [--json]
spanda diagnose rover.sd          # static program diagnosis
```

Trace diagnosis uses the existing **mission replay** infrastructure (`spanda-runtime`).

## Package

Advanced causal inference: **`spanda-diagnosis`** (`assurance.diagnosis`).

## Example

See `examples/diagnostics/gps_failure.sd`.
