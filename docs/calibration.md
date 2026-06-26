# Device Calibration

Calibration status affects provisioning and mission readiness.

## Fields

Declare on `[[devices]]` or fleet tree device nodes:

```toml
[[devices]]
id = "lidar-front"
calibration_status = "valid"   # valid | expired | required | unknown
calibration_expiry_ms = 1735689600000
last_self_test_ms = 1735603200000
```

## Readiness blocking

Readiness is blocked when:

- `calibration_status = expired`
- `calibration_expiry_ms` is in the past
- `health_status = critical`
- Firmware below `min_firmware_version`
- Identity not verified (`trust_level` not `verified`/`trusted`)

## CLI / reports

```bash
spanda config report --config spanda.toml
curl http://127.0.0.1:8080/v1/device-reports
curl -X POST http://127.0.0.1:8080/v1/readiness/run
```

## Related

- [device-provisioning.md](./device-provisioning.md)
- [health-checks.md](./health-checks.md)
- [fleet-readiness.md](./fleet-readiness.md)
