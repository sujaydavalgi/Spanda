#!/usr/bin/env bash
# Bundle @spanda/lsp into editor/vscode for marketplace VSIX packaging.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

npm run build --workspace=@spanda/lsp

DEST="$ROOT/editor/vscode/server"
rm -rf "$DEST/dist" "$DEST/node_modules"
mkdir -p "$DEST/dist"
cp -r packages/lsp/dist/* "$DEST/dist/"

# Production deps for the language server process.
cat > "$DEST/package.json" <<'EOF'
{
  "name": "spanda-lsp-bundled",
  "private": true,
  "type": "module",
  "dependencies": {
    "vscode-languageserver": "^9.0.1",
    "vscode-languageserver-textdocument": "^1.0.12"
  }
}
EOF
npm install --prefix "$DEST" --omit=dev

npm run build --prefix editor/vscode
echo "✓ Bundled LSP into editor/vscode/server/"
