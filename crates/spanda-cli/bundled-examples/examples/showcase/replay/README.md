# Mission replay

Record simulation output, then inspect or replay for regression and incident review.

## Record

```bash
cd examples/showcase/replay
spanda sim mission.sd --record
# writes mission.trace in the current directory
```

## Replay

```bash
spanda replay mission.trace
spanda replay mission.trace --deterministic
spanda replay mission.trace --playback --from T+00:01
```

Fault injection is declared via `simulate_compatibility { fault LidarFailure; }` for verify/sim walks.

Docs: [replay.md](../../docs/replay.md)
