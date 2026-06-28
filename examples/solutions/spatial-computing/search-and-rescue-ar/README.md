# Search and Rescue AR

Multi-human SAR team with ground robot, aerial drone (`drone-sar-001`), and AR team map overlay.

## Run

```bash
spanda check sar_mission.sd
spanda verify sar_mission.sd --capabilities --config ../spanda.toml
spanda swarm coordinate sar_mission.sd
```

## Demonstrates

- `search_rescue_operator` and `drone_pilot` capabilities
- Collaborative mission with human takeover on geofence breach
- Mission continuity reassign + checkpoint
- Team readiness (min team size in `spanda.readiness.toml`)

## Context awareness

Geofence breach → wearable alert → AR warning → human takeover request
