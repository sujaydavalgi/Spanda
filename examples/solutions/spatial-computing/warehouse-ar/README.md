# Warehouse AR Picking

Operator wears AR glasses (`hololens-001` in device tree) for pick-path overlay while coordinating with AMR `amr-001`.

## Run

```bash
spanda check pick_mission.sd
spanda verify pick_mission.sd --capabilities --traceability --config ../spanda.toml
spanda readiness pick_mission.sd --profile human_collaboration --config ../spanda.toml
```

## Demonstrates

- `operate_robot` and `forklift_operator` operator capabilities
- Robot + human collaborative mission
- Proximity alert → audit (context awareness pattern)
- Mission continuity checkpoint on failure

## Packages (planned)

`spanda-hololens` or `spanda-arkit` for AR overlay · `spanda-opencv` for AMR camera
