#!/usr/bin/env bash
# Prepare Human Interaction security audit artifacts (health opt-in + AR session RBAC).
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT="${ROOT}/.spanda/hri-security-audit-prep.json"
mkdir -p "$(dirname "$OUT")"

echo "== HRI security audit prep =="

HRI_API_OK=false
if cargo test -p spanda-api --test humans_api_tests -q >/tmp/hri-humans-api-tests.log 2>&1; then
  grep -q "test result: ok" /tmp/hri-humans-api-tests.log && HRI_API_OK=true
fi

HEALTH_GATE_OK=false
if cargo test -p spanda-security --test human_health_tests -q >/tmp/hri-health-tests.log 2>&1; then
  grep -q "test result: ok" /tmp/hri-health-tests.log && HEALTH_GATE_OK=true
fi

GATE_OK=false
if SPANDA_HRI_SKIP_SOAK=1 SPANDA_HRI_SKIP_AUDIT=1 ./scripts/hri_stable_promotion_gate.sh >/tmp/hri-gate.log 2>&1; then
  GATE_OK=true
fi

export ROOT HRI_API_OK HEALTH_GATE_OK GATE_OK
python3 - <<'PY' > "$OUT"
import json, os, time
report = {
    "generated_at_ms": int(time.time() * 1000),
    "scope": [
        "human_health_opt_in",
        "wearable_telemetry_rbac",
        "hri_session_annotate_rbac",
        "operator_mission_approval",
    ],
    "checks": {
        "humans_api_tests": os.environ.get("HRI_API_OK") == "true",
        "human_health_gate_tests": os.environ.get("HEALTH_GATE_OK") == "true",
        "promotion_gate_smoke": os.environ.get("GATE_OK") == "true",
    },
    "reviewer_packet": [
        "docs/stable-hardening-human-interaction.md",
        "docs/security-audit-third-party.md",
        "GET /v1/human-health/policy",
        "GET /v1/hri/sessions",
        "POST /v1/hri/sessions/{id}/annotate",
        "GET /v1/operator/mission/approvals",
        "POST /v1/operator/mission/approve",
    ],
}
print(json.dumps(report, indent=2))
PY

echo "Wrote $OUT"
cat "$OUT"
echo "hri-security-audit-prep ok"
