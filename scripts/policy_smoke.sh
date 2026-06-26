#!/usr/bin/env bash
# Smoke verify-time operational policy evaluation.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
FILE="${ROOT}/examples/showcase/policy/warehouse.sd"

if [[ -n "${SPANDA_BIN:-}" && -x "${SPANDA_BIN}" ]]; then
  run_spanda() { "$SPANDA_BIN" "$@"; }
else
  run_spanda() { cargo run -q -p spanda -- "$@"; }
fi

echo "== policy verify pass =="
run_spanda verify "$FILE" --policy WarehousePolicy >/dev/null

echo "== policy verify json =="
run_spanda verify "$FILE" --policy WarehousePolicy --json >/dev/null

echo "== policy readiness merge =="
run_spanda readiness "$FILE" --policy WarehousePolicy >/dev/null

echo "== policy deploy gate (operational_policy gate) =="
set +e
gate_json=$(run_spanda deploy gate "$FILE" --operational-policy WarehousePolicy --json 2>/dev/null)
set -e
printf '%s' "$gate_json" | python3 -c "
import json, sys
data = json.load(sys.stdin)
gate = next(
    (g for g in data.get('gates', []) if g.get('name') == 'operational_policy:WarehousePolicy'),
    None,
)
if not gate or not gate.get('passed'):
    print('operational policy gate missing or failed:', gate, file=sys.stderr)
    sys.exit(1)
"

echo "Policy smoke OK"
