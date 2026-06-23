#!/usr/bin/env bash
# Build the VS Code extension VSIX without publishing (CI / maintainer smoke test).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
if [[ ! -d node_modules ]]; then
  npm ci
fi
if [[ ! -d editor/vscode/node_modules ]]; then
  npm install --prefix editor/vscode
fi
./scripts/bundle-vscode-extension.sh
cd editor/vscode
npm run package
VERSION="$(node -p "require('./package.json').version")"
VSIX="spanda-vscode-${VERSION}.vsix"
test -f "$VSIX"
echo "✓ VS Code VSIX built: editor/vscode/$VSIX"
