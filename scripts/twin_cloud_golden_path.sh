#!/usr/bin/env bash
# Golden path for twin replay export and cloud upload via SPANDA_CLOUD_UPLOAD_URL.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
TWIN_SOURCE="${ROOT}/examples/communication/twin_replay_golden.sd"
DEMO_SOURCE="${ROOT}/examples/communication/digital_twin_sync.sd"
UPLOAD_PORT="18770"
UPLOAD_FILE="${TMPDIR:-/tmp}/spanda-cloud-upload.json"
REPLAY_FILE="${TMPDIR:-/tmp}/spanda-twin-replay.json"
MOCK_PID=""

if [[ -n "${SPANDA:-}" ]]; then
  :
elif [[ -x "${ROOT}/target/release/spanda" ]]; then
  SPANDA="${ROOT}/target/release/spanda"
else
  SPANDA="spanda"
fi

cleanup() {
  [[ -n "${MOCK_PID}" ]] && kill "${MOCK_PID}" 2>/dev/null || true
}
trap cleanup EXIT

lsof -ti ":${UPLOAD_PORT}" 2>/dev/null | xargs kill -9 2>/dev/null || true
rm -f "${UPLOAD_FILE}" "${REPLAY_FILE}"

echo "== twin export from golden replay example =="
"${SPANDA}" twin export "${TWIN_SOURCE}" --out "${REPLAY_FILE}"
test -s "${REPLAY_FILE}"

echo "== cloud upload integration test =="
export SPANDA_CLOUD_UPLOAD_URL="http://127.0.0.1:${UPLOAD_PORT}/upload"
python3 "${ROOT}/scripts/mock_upload_server.py" "${UPLOAD_FILE}" "${UPLOAD_PORT}" &
MOCK_PID=$!
sleep 0.5
cargo test -p spanda-providers cloud_upload_posts_when_url_set -- --exact --nocapture
wait "${MOCK_PID}" 2>/dev/null || true
MOCK_PID=""
test -s "${UPLOAD_FILE}"
grep -q '"path"' "${UPLOAD_FILE}"

echo "== check digital_twin_sync example =="
"${SPANDA}" check "${DEMO_SOURCE}"

echo "Twin cloud golden path complete."
