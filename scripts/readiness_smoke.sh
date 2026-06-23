#!/usr/bin/env bash
# Smoke operational readiness commands in CI.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
ROVER="${ROOT}/examples/showcase/readiness/rover.sd"

if [[ -n "${SPANDA_BIN:-}" && -x "${SPANDA_BIN}" ]]; then
  run_spanda() { "$SPANDA_BIN" "$@"; }
else
  run_spanda() { cargo run -q -p spanda -- "$@"; }
fi

echo "== spanda-readiness crate tests =="
cargo test -p spanda-readiness --quiet

echo "== readiness CLI =="
run_spanda readiness "$ROVER" --json >/dev/null
runtime_json="$(run_spanda readiness "$ROVER" --target RoverV1 --runtime --inject-health-faults --json || true)"
if ! grep -q 'Runtime' <<<"$runtime_json"; then
  echo "readiness runtime fault injection produced no runtime issues"
  exit 1
fi

echo "== check --readiness-json =="
run_spanda check "$ROVER" --readiness-json --json | grep -q '"readiness"'

echo "== demo readiness =="
run_spanda demo readiness

echo "Readiness smoke OK"
