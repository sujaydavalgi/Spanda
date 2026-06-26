# spanda-otel-collector

Optional **OpenTelemetry collector** integration for Spanda Control Center distributed traces and SRE metrics.

## Status

**Experimental** — package contract stub. Control Center pushes OTLP/JSON to any OTLP HTTP endpoint (Jaeger, Grafana Alloy, vendor collectors).

## Configuration

Set a single collector base URL (paths `/v1/traces` and `/v1/metrics` are appended when missing):

```bash
export SPANDA_OTEL_COLLECTOR_URL=http://127.0.0.1:4318
export SPANDA_OTLP_TRACE_AUTO_PUSH=1   # optional: push each API span
```

Or use explicit endpoints:

```bash
export SPANDA_OTLP_TRACES_ENDPOINT=http://127.0.0.1:4318/v1/traces
export SPANDA_OTLP_METRICS_ENDPOINT=http://127.0.0.1:4318/v1/metrics
```

## API

```bash
curl http://127.0.0.1:8080/v1/observability/backend
curl -H "Authorization: Bearer $SPANDA_API_KEY" -X POST http://127.0.0.1:8080/v1/observability/otlp/export
```

## Related

- [control-center.md](../../../docs/control-center.md)
- [enterprise-operations-roadmap.md](../../../docs/enterprise-operations-roadmap.md)
