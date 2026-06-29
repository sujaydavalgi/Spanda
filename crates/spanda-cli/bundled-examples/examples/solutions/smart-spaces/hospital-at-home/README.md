# Hospital-at-Home

Patient monitoring with opt-in wearables — bridges [Connected Healthcare](../../../docs/solutions/spatial-computing.md) and Smart Spaces blueprints.

**Mission:** `patient_monitoring.sd`

---

## Quick start

```bash
cd examples/solutions/smart-spaces
spanda check hospital-at-home/patient_monitoring.sd
spanda readiness hospital-at-home/patient_monitoring.sd --profile smart_space --config spanda.toml
```

---

## Demonstrates

- `health_opt_in` on patient human entity
- Wearable fall detection and vital monitoring
- Emergency notification to facilities operator
- Privacy policy in `spanda.security.toml`

**Related:** [examples/solutions/spatial-computing/wearable-health/](../../spatial-computing/wearable-health/)
