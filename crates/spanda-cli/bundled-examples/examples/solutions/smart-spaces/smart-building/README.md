# Smart Building

Commercial tower floor readiness, lockdown, and service robot inspection missions.

**Mission:** `floor_readiness.sd`

---

## Quick start

```bash
cd examples/solutions/smart-spaces
spanda check smart-building/floor_readiness.sd
spanda verify smart-building/floor_readiness.sd --target BuildingEdgeV1 --config spanda.toml
spanda control-center serve --config spanda.toml --program smart-building/floor_readiness.sd
```

---

## Demonstrates

- Multi-gateway BACnet redundancy
- Fire panel and emergency notification paths
- Building lockdown with operator approval
- Service robot continuity reassignment
- Digital twin rollup for `tower-demo` facility
