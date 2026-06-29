# Smart Office

Occupancy-driven HVAC setback and cleaning robot missions for office floors.

**Mission:** `occupancy_climate.sd`

---

## Quick start

```bash
cd examples/solutions/smart-spaces
spanda check smart-office/occupancy_climate.sd
spanda verify smart-office/occupancy_climate.sd --target SmartSpaceGatewayV1
```

---

## Demonstrates

- BACnet gateway orchestration
- CO₂ environmental monitoring
- Cleaning mission when occupancy clears
- BACnet gateway continuity failover
