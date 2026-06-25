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

echo "Policy smoke OK"
