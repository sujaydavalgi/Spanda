# Device Quarantine

Unknown or untrusted devices enter **quarantine** until an operator approves trust.

## Rules

While quarantined, a device:

- **Cannot** control actuators (remote drives, arms, motor controllers)
- **Cannot** publish trusted safety data
- **Cannot** satisfy mission capability requirements
- **Requires** operator approval to move to `verified`

## Triggers

- Discovery with `trust_level = unknown`
- Failed provisioning workflow
- `POST /v1/devices/{id}/quarantine` or `spanda device quarantine <id>`
- `POST /v1/operator/quarantine` (operator API)

## Trust approval

```bash
# CLI: re-provision after setting trust in config
spanda device provision cam-001 --robot rover-001

# API: patch lifecycle after updating trust
curl -X PATCH http://127.0.0.1:8080/v1/devices/cam-001 \
  -H "Authorization: Bearer $SPANDA_API_KEY" \
  -d '{"lifecycle_state":"verified"}'
```

Set `trust_level = "verified"` or `"trusted"` in `spanda.devices.toml` before provisioning.

## Identity anomalies

Quarantine may also follow validation findings:

- Duplicate serial, MAC, or IP
- Unknown certificate (identity without fingerprint)
- Insecure endpoint on safety-critical actuator
- Unsupported firmware

## Related

- [device-pool.md](./device-pool.md)
- [device-discovery.md](./device-discovery.md)
- [security-assurance.md](./security-assurance.md)
