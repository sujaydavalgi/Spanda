#!/usr/bin/env bash
# ADAS Solution Blueprint smoke — verify, readiness, replay, compliance.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
ADAS="$ROOT/examples/solutions/adas"
MAIN="$ADAS/src/highway_drive.sd"
TRACE="$ADAS/src/highway_drive.trace"

cd "$ROOT"
export SPANDA_ROOT="${SPANDA_ROOT:-$ROOT}"

# shellcheck source=lib/registry_env.sh
source "${ROOT}/scripts/lib/registry_env.sh"
ensure_spanda_registry_url "$ROOT"
cargo build -p spanda -q

if [[ -n "${SPANDA_BIN:-}" && -x "${SPANDA_BIN}" ]]; then
  run_spanda() { "$SPANDA_BIN" "$@"; }
else
  run_spanda() { cargo run -q -p spanda -- "$@"; }
fi

echo "== ADAS Solution Blueprint smoke =="

check() {
  echo "--- $* ---"
  run_spanda "$@"
}

check check "$MAIN"
check verify "$MAIN" --profile iso26262 --capabilities --traceability
check readiness "$MAIN" --profile iso26262
echo "--- replay src/highway_drive.trace --deterministic ---"
( cd "$ADAS" && run_spanda replay src/highway_drive.trace --deterministic )
check compliance report "$MAIN" --profile iso26262

for example in \
  "$ADAS/lane_keeping/lane_keeping.sd" \
  "$ADAS/adaptive_cruise/adaptive_cruise.sd" \
  "$ADAS/automatic_emergency_braking/aeb.sd" \
  "$ADAS/sensor_failure_recovery/camera_failure.sd" \
  "$ADAS/driver_takeover/driver_takeover.sd"
do
  check check "$example"
  check verify "$example" --capabilities
done

echo "--- continuity (camera_failure.sd) ---"
run_spanda continuity "$ADAS/sensor_failure_recovery/camera_failure.sd" \
  --failed front_camera --trigger sensor_failed || true

echo ""
echo "ADAS smoke complete."
