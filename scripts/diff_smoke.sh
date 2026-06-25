#!/usr/bin/env bash
# Smoke mission diff between showcase rover variants.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
BASE="${ROOT}/examples/showcase/readiness/rover.sd"
OTHER="${ROOT}/examples/showcase/safety_report/rover.sd"

if [[ -n "${SPANDA_BIN:-}" && -x "${SPANDA_BIN}" ]]; then
  run_spanda() { "$SPANDA_BIN" "$@"; }
else
  run_spanda() { cargo run -q -p spanda -- "$@"; }
fi

echo "== mission diff =="
output=$(run_spanda diff "$BASE" "$OTHER" 2>&1 || true)
echo "$output"
echo "$output" | grep -q "Mission diff:"
echo "$output" | grep -q "Deploy impact: yes"

echo "== identical diff =="
run_spanda diff "$BASE" "$BASE" >/dev/null

echo "Mission diff smoke OK"
