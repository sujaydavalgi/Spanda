# Emergency Response

Fire response, evacuation, and life-safety continuity for any smart space scale.

**Mission:** `fire_response.sd` · **Minimum readiness:** 95

---

## Quick start

```bash
cd examples/solutions/smart-spaces
spanda check emergency-response/fire_response.sd
spanda readiness emergency-response/fire_response.sd --profile smart_space --config spanda.toml --json
```

---

## Demonstrates

- Fire panel integration via BACnet
- Emergency lighting continuity failover
- Evacuation mission with egress lighting
- Life-safety `kill_switch` precedence over comfort automation

**Simulation scenarios:** fire, flood, power failure, gateway failure — see [docs/solutions/smart-spaces.md](../../../docs/solutions/smart-spaces.md#digital-twin)
