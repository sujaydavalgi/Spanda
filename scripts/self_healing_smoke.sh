#!/usr/bin/env bash
# Smoke self-healing and recovery commands in CI.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
HEALING="${ROOT}/examples/showcase/self_healing/rover.sd"
FLEET="${ROOT}/examples/showcase/fleet_recovery/fleet.sd"

resolve_spanda_bin() {
  if [[ -n "${SPANDA_BIN:-}" && -x "${SPANDA_BIN}" ]]; then
    echo "${SPANDA_BIN}"
    return
  fi
  local bases=("${ROOT}/target")
  if [[ -n "${CARGO_TARGET_DIR:-}" ]]; then
    bases+=("${CARGO_TARGET_DIR}")
  fi
  for base in "${bases[@]}"; do
    for candidate in "${base}/debug/spanda" "${base}/release/spanda"; do
      if [[ -x "${candidate}" ]]; then
        echo "${candidate}"
        return
      fi
    done
  done
}

if resolved="$(resolve_spanda_bin)"; [[ -n "${resolved}" ]]; then
  export SPANDA_BIN="${resolved}"
  run_spanda() { "${SPANDA_BIN}" "$@"; }
else
  unset SPANDA_BIN
  run_spanda() { cargo run -q -p spanda -- "$@"; }
fi

echo "== spanda-assurance recovery tests =="
cargo test -p spanda-assurance --test recovery_tests --quiet
cargo test -p spanda-interpreter --test recovery_runtime --quiet
cargo test -p spanda-interpreter --test mission_approval_runtime --quiet
cargo test -p spanda-fleet --test mesh_integration mesh_coordinator_relays_fleet_recovery --quiet

echo "== recovery runtime auto-trigger =="
cargo test -p spanda-interpreter recovery_auto_triggers --quiet

echo "== recovery CLI =="
run_spanda heal "$HEALING" >/dev/null
run_spanda recover "$HEALING" --failure gps >/dev/null
run_spanda recovery knowledge "$HEALING" >/dev/null
run_spanda analyze-failure "$HEALING" --with-recovery >/dev/null
run_spanda heal "$FLEET" >/dev/null

echo "== demo self-healing =="
export SPANDA_ROOT="${ROOT}"
run_spanda demo self-healing

echo "Self-healing smoke OK"
