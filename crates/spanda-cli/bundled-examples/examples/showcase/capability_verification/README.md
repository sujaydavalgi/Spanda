# Capability verification

Demonstrates hardware capability exposure, robot capability exposure, minimum-capable hardware checks, and traceability.

## Commands

```bash
spanda check examples/showcase/capability_verification/rover.sd
spanda verify examples/showcase/capability_verification/rover.sd --capabilities --traceability --json
spanda health robot examples/showcase/capability_verification/rover.sd --json
spanda sim examples/showcase/capability_verification/rover.sd
```

## What it shows

| Concept | In this demo |
|---------|----------------|
| Hardware profile | `RoverV1` with GPS, Lidar, drive |
| Robot capabilities | `exposes capabilities [ … ]` |
| Mission requirements | `requires capabilities [ gps_navigation, obstacle_avoidance ]` |
| Traceability | `spanda verify --traceability` |
| Health | `health_check` + `health_policy` |

Docs: [capability-traceability.md](../../docs/capability-traceability.md)
