# spanda-grafana-dashboards

Grafana dashboard templates for Spanda Control Center observability and SRE.

## Dashboards

| File | Focus |
|------|--------|
| `dashboards/control-center-sre.json` | Availability, SLO burn-rate, incidents, alerts |
| `dashboards/control-center-ota.json` | OTA rollout status and readiness gates |

## Import

1. Open Grafana → **Dashboards** → **New** → **Import**.
2. Upload a JSON file from `dashboards/` or paste its contents.
3. Point Prometheus/OTLP data sources at your Control Center collector (`SPANDA_OTLP_METRICS_ENDPOINT`).

## Metrics labels

Templates expect OTLP metrics exported via `POST /v1/observability/otlp/export-metrics` with Spanda default resource attributes (`service.name=spanda-control-center`).

## Status

**Experimental** — import-only templates; no runtime adapter required.
