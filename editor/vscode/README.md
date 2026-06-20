# Spanda VS Code Extension (Experimental)

This extension provides:

- `.sd` language registration
- baseline syntax highlighting
- LSP client wiring to `spanda-lsp`
- VSIX packaging for local install

## Build

```bash
cd editor/vscode
npm install
npm run build
```

## Package a VSIX

```bash
cd editor/vscode
npm install
npm run package
```

Install with:

```bash
code --install-extension spanda-vscode-0.1.0.vsix
```

## Run in Extension Development Host

1. Open this repository in VS Code
2. Open `editor/vscode`
3. Press `F5` to launch Extension Development Host
4. Set `spanda.languageServerPath` if needed (defaults to `packages/lsp/dist/server.js`)

## Prerequisites

- Build LSP server first:

```bash
npm run build --workspace=@spanda/lsp
```
