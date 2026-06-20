#!/usr/bin/env bash
# Build installable Spanda CLI packages for the current host using cargo-dist.
#
# Usage:
#   ./scripts/package-release.sh v0.1.0
#       Build packages for the current machine (default).
#   ./scripts/package-release.sh v0.1.0 --all
#       Build all configured Linux/macOS/Windows targets (requires cross tools).
#   ./scripts/package-release.sh v0.1.0 --all --install-cross
#       Install cross-compilation tools, then build all targets.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

TAG="${1:-}"
if [[ -z "$TAG" ]]; then
  echo "Usage: $0 <tag> [--all] [--install-cross]" >&2
  echo "Example: $0 v0.1.0" >&2
  exit 1
fi
shift || true

ARTIFACTS="host"
INSTALL_CROSS=0
while [[ $# -gt 0 ]]; do
  case "$1" in
    --all)
      ARTIFACTS="all"
      ;;
    --install-cross)
      INSTALL_CROSS=1
      ;;
    *)
      echo "Unknown option: $1" >&2
      exit 1
      ;;
  esac
  shift
done

if ! command -v dist >/dev/null 2>&1; then
  echo "Installing cargo-dist..."
  curl --proto '=https' --tlsv1.2 -LsSf \
    https://github.com/axodotdev/cargo-dist/releases/download/v0.32.0/cargo-dist-installer.sh | sh
fi

has_zig() {
  command -v zig >/dev/null 2>&1 || python3 -c "import ziglang" >/dev/null 2>&1
}

ensure_cross_tools() {
  local missing=()
  command -v cargo-zigbuild >/dev/null 2>&1 || missing+=("cargo-zigbuild")
  command -v cargo-xwin >/dev/null 2>&1 || missing+=("cargo-xwin")
  has_zig || missing+=("zig")

  if [[ ${#missing[@]} -eq 0 ]]; then
    return 0
  fi

  if [[ "$INSTALL_CROSS" -eq 1 ]]; then
    echo "Installing cross-compilation tools..."
    cargo install --locked cargo-zigbuild cargo-xwin
    if ! has_zig; then
      if command -v brew >/dev/null 2>&1; then
        brew install zig
      else
        python3 -m pip install --user ziglang
      fi
    fi
    rustup target add \
      aarch64-apple-darwin \
      aarch64-unknown-linux-gnu \
      x86_64-apple-darwin \
      x86_64-unknown-linux-gnu \
      x86_64-pc-windows-msvc
    return 0
  fi

  echo "Cross-compilation tools required for --all builds:" >&2
  for tool in "${missing[@]}"; do
    echo "  - ${tool}" >&2
  done
  echo >&2
  echo "Install them with:" >&2
  echo "  cargo install --locked cargo-zigbuild cargo-xwin" >&2
  echo "  brew install zig   # or: python3 -m pip install ziglang" >&2
  echo "  rustup target add aarch64-apple-darwin aarch64-unknown-linux-gnu \\" >&2
  echo "    x86_64-apple-darwin x86_64-unknown-linux-gnu x86_64-pc-windows-msvc" >&2
  echo >&2
  echo "Or rerun with --install-cross to install automatically:" >&2
  echo "  $0 ${TAG} --all --install-cross" >&2
  echo >&2
  echo "For official multi-platform releases, push a git tag and let GitHub Actions build all targets." >&2
  exit 1
}

build_all_targets() {
  local local_ok=0
  local global_ok=0

  if dist build --tag="$TAG" --artifacts=local; then
    local_ok=1
  elif ! command -v candle >/dev/null 2>&1; then
    echo
    echo "Note: Windows .msi was skipped (WiX is not installed on this host)."
    echo "Archives and .zip were still built; GitHub Actions produces .msi on Windows runners."
    local_ok=1
  fi

  if dist build --tag="$TAG" --artifacts=global; then
    global_ok=1
  fi

  if [[ "$local_ok" -eq 0 || "$global_ok" -eq 0 ]]; then
    return 1
  fi
}

echo "Building Spanda release artifacts for tag ${TAG} (mode: ${ARTIFACTS})..."
if [[ "$ARTIFACTS" == "all" ]]; then
  ensure_cross_tools
  build_all_targets
else
  dist build --tag="$TAG" --artifacts="$ARTIFACTS"
fi

echo
echo "Artifacts written to: ${ROOT}/target/distrib/"
ls -la "${ROOT}/target/distrib/"
