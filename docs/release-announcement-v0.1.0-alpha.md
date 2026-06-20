# Spanda v0.1.0-alpha is live

Spanda v0.1.0-alpha is now available for public evaluation.

Release: <https://github.com/sujaydavalgi/Spanda/releases/tag/v0.1.0-alpha>

## What to try first

Start with the showcase examples:

- `examples/showcase/rover_navigation.sd` — sensors + AI planning + SafeAction
- `examples/showcase/warehouse_robot.sd` — tasks + communication + safety zones
- `examples/showcase/ai_safety_violation.sd` — unsafe AI proposal rejection
- `examples/showcase/hardware_compatibility.sd` — deploy target verification
- `examples/showcase/communication_demo.sd` — message/topic/service/action
- `examples/showcase/digital_twin_demo.sd` — twin telemetry + replay

## Quick commands

```bash
spanda check examples/showcase/rover_navigation.sd
spanda run examples/showcase/rover_navigation.sd
spanda verify examples/showcase/hardware_compatibility.sd --json
spanda check examples/showcase/ai_safety_violation.sd
```

## Scope of this alpha

v0.1.0-alpha focuses on:

- Stability
- Examples
- Documentation
- CI/CD
- Developer experience

This is a public evaluation release, not a production robotics release.

## Feedback requested

Please share:

- parser/typechecker/runtime bugs
- safety and verification edge cases
- docs gaps in onboarding
- showcase examples that should be added next

Open issues: <https://github.com/sujaydavalgi/Spanda/issues>
