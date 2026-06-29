# Energy Management

Solar, battery, EV charging, demand response, and backup power missions.

**Mission:** `demand_response.sd` · **Package:** `spanda-energy`

---

## Quick start

```bash
cd examples/solutions/smart-spaces
spanda check energy-management/demand_response.sd
spanda verify energy-management/demand_response.sd --target SmartSpaceGatewayV1 --config spanda.toml
```

---

## Demonstrates

- Grid outage → battery island mission
- Demand response load shed
- Energy twin mirror in device tree
- Modbus utility meter integration

**Doc:** [docs/energy-management.md](../../../docs/energy-management.md)
