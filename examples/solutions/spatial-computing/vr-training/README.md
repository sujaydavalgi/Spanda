# VR Operator Training

Trainee operator practices AMR control in VR (`vr-training-001` device) with mission replay for certification evidence.

## Run

```bash
spanda sim training_mission.sd --record
spanda replay training_mission.trace --playback --deterministic
spanda readiness training_mission.sd --profile human_collaboration --config ../spanda.toml
```

## Demonstrates

- VR training linked to sim + replay (no VR engine in core)
- Operator certification readiness gate
- Training twin mirror (planned) for progress tracking

## Packages (planned)

`spanda-openxr` for VR viewport
