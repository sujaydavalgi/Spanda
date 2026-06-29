# Smart Home

Residential night mode and water leak response — readiness-gated orchestration above Matter/Zigbee gateways.

**Mission:** `night_mode.sd` · **Profile:** `smart_space` residential

---

## Quick start

```bash
cd examples/solutions/smart-spaces
spanda check smart-home/night_mode.sd
spanda readiness smart-home/night_mode.sd --profile smart_space --config spanda.toml --json
```

---

## Demonstrates

- Night mode readiness (locks, smoke, leak sensors, gateway)
- Gateway failover via `continuity_policy`
- Leak response escalation hook
- Matter + Zigbee device tree nodes in `spanda.devices.toml`

---

## Related

- [docs/building-automation.md](../../../docs/building-automation.md#smart-home)
- [docs/smart-space-readiness.md](../../../docs/smart-space-readiness.md)
