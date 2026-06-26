#!/usr/bin/env bash
# Smoke check for Control Center Tauri desktop (compile + optional bundle).
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
chmod +x "${ROOT}/scripts/build_control_center_desktop.sh"
"${ROOT}/scripts/build_control_center_desktop.sh"
