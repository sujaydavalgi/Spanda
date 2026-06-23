#!/usr/bin/env bash
# Golden path for live MQTT pub/sub against a local Mosquitto broker.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SOURCE="${ROOT}/examples/communication/mqtt_live.sd"
BROKER_HOST="127.0.0.1"
BROKER_PORT="1883"
TOPIC="/spanda/ci/ping"
MOSQUITTO_PID=""

if [[ -n "${SPANDA:-}" ]]; then
  :
elif [[ -x "${ROOT}/target/release/spanda" ]]; then
  SPANDA="${ROOT}/target/release/spanda"
else
  SPANDA="spanda"
fi

cleanup() {
  [[ -n "${MOSQUITTO_PID}" ]] && kill "${MOSQUITTO_PID}" 2>/dev/null || true
}
trap cleanup EXIT

if ! command -v mosquitto >/dev/null 2>&1; then
  echo "mosquitto not found; install mosquitto to run MQTT golden path" >&2
  exit 1
fi
if ! command -v mosquitto_sub >/dev/null 2>&1; then
  echo "mosquitto_sub not found; install mosquitto-clients" >&2
  exit 1
fi

lsof -ti ":${BROKER_PORT}" 2>/dev/null | xargs kill -9 2>/dev/null || true
mosquitto -p "${BROKER_PORT}" &
MOSQUITTO_PID=$!
sleep 1

echo "== build spanda-cli with live-mqtt =="
cargo build -p spanda --release --features live-mqtt
SPANDA="${ROOT}/target/release/spanda"

echo "== check mqtt_live example =="
"${SPANDA}" check "${SOURCE}"

echo "== sim with live MQTT bridge =="
export SPANDA_LIVE_MQTT=1
(
  timeout 8 mosquitto_sub -h "${BROKER_HOST}" -p "${BROKER_PORT}" -t "${TOPIC}" -C 1 >"${TMPDIR:-/tmp}/spanda-mqtt-golden.txt"
) &
SUB_PID=$!
sleep 1
"${SPANDA}" sim "${SOURCE}"
wait "${SUB_PID}"
grep -q "spanda-mqtt-live" "${TMPDIR:-/tmp}/spanda-mqtt-golden.txt"

echo "MQTT golden path complete."
