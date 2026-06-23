# IoT examples

Runnable programs for IoT provider contracts, package dispatch, and optional live bridges.

**Guide:** [docs/iot.md](../../docs/iot.md) · **Golden path:** `./scripts/live_iot_golden_path.sh`

```bash
spanda check examples/iot/modbus_dispatch.sd
SPANDA_LIVE_MODBUS=1 spanda run examples/iot/modbus_dispatch.sd
```

---

## Files

| File | Protocol | Live env |
|------|----------|----------|
| [`modbus_dispatch.sd`](modbus_dispatch.sd) | Modbus `read_register` (mirrors `spanda-modbus`) | `SPANDA_LIVE_MODBUS=1` |

---

## Registry package stubs

Additional protocol stubs live under `packages/registry/` (`spanda-opcua`, `spanda-zigbee`, `spanda-lora`, `spanda-matter`, `spanda-canbus`). Copy the `modbus_dispatch.sd` module pattern or add a path dependency on the registry package.

Live bridge env flags: see [docs/iot.md](../../docs/iot.md#live-hardware-optional).
