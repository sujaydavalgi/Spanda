# Mission continuity walkthrough

Hands-on path for takeover, delegation, succession, and durable checkpoints. Prerequisite: [getting-started.md](./getting-started.md).

## 1. Evaluate continuity

```bash
spanda continuity examples/showcase/continuity/warehouse.sd \
  --failed ScannerAlpha --progress 72 --trigger robot_failed
```

Expect: successor **ScannerBeta**, mode **resume**, checkpoint at **72%**.

## 2. Rank successors

```bash
spanda succession examples/showcase/continuity/warehouse.sd \
  --failed ScannerAlpha --scope fleet --json
```

## 3. Policy diagnostics

```bash
spanda check examples/showcase/continuity/warehouse.sd --readiness-json --json
```

Look for `continuity:policy`, `continuity:fleet`, and `continuity:mission` categories.

## 4. Swarm member lost

```bash
spanda takeover examples/showcase/swarm_takeover/swarm.sd \
  --failed DroneTwo --trigger swarm_lost --scope swarm

spanda swarm coordinate examples/showcase/swarm_takeover/swarm.sd \
  --failed DroneTwo --progress 55 --json
```

## 5. Fleet runtime (staging)

With mesh coordinator and deployed agents:

```bash
export SPANDA_FLEET_MESH_URL=http://coordinator:9700
spanda continuity examples/showcase/continuity/warehouse.sd \
  --failed ScannerAlpha --progress 72
```

Checkpoints persist under `.spanda/mission-checkpoints.json`.

## Related

- [mission-continuity.md](./mission-continuity.md)
- [continuity-policies.md](./continuity-policies.md)
- [self-healing.md](./self-healing.md)
- [fleet-distributed.md](./fleet-distributed.md)
