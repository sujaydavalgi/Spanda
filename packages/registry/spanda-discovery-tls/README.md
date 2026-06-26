# spanda-discovery-tls

Production TLS certificate policy for Spanda device discovery transports.

## Environment

| Variable | Purpose |
|----------|---------|
| `SPANDA_DISCOVERY_REQUIRE_TLS` | Reject `http://` and `mqtt://` discovery endpoints |
| `SPANDA_DISCOVERY_TLS_CA_BUNDLE` | Path to vendor/enterprise CA bundle (PEM) |
| `SPANDA_PRODUCTION_POLICY=production` | Enables TLS requirement with OTA certify defaults |

## Vendor CA bundles

Place vendor-specific roots under a fleet-managed directory, for example:

```text
/etc/spanda/certs/
  vendor-a-ca.pem
  vendor-b-ca.pem
```

Point `SPANDA_DISCOVERY_TLS_CA_BUNDLE` at the concatenated bundle or per-vendor file.

## API

`GET /v1/discovery` includes a `tls` summary when the core transport responds.

## Related packages

- `spanda-discovery-mdns`
- `spanda-discovery-wifi`
- `spanda-discovery-cellular`
