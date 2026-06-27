#!/usr/bin/env bash
# Smoke check for Control Center Tauri desktop (compile + optional bundle).
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
chmod +x "${ROOT}/scripts/build_control_center_desktop.sh"
if [[ "$(uname -s)" == "Linux" ]]; then
  export SKIP_TAURI_LINUX_CARGO_CHECK="${SKIP_TAURI_LINUX_CARGO_CHECK:-1}"
fi
"${ROOT}/scripts/build_control_center_desktop.sh"
