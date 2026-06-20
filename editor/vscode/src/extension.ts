/**
 * extension module (extension.ts).
 * @module
 */

import * as path from "node:path";
import * as vscode from "vscode";
import {
  type LanguageClientOptions,
  LanguageClient,
  ServerOptions,
  TransportKind,
} from "vscode-languageclient/node";

let client: LanguageClient | undefined;

function resolveServerModule(context: vscode.ExtensionContext): string | null {
  // ResolveServerModule.
  //
  // Parameters:
  // - `context` — input value
  //
  // Returns:
  // `Some` / non-null value on success, otherwise `None` / null.
  //
  // Options:
  // None.
  //
  // Example:
  // const result = resolveServerModule(context);

  const cfg = vscode.workspace.getConfiguration("spanda");
  const configured = cfg.get<string>("languageServerPath");
  if (configured && configured.trim()) {
    return configured;
  }

  const workspace = vscode.workspace.workspaceFolders?.[0]?.uri.fsPath;
  if (workspace) {
    return path.join(workspace, "packages/lsp/dist/server.js");
  }

  // Fallback for extension-only usage.
  return path.join(context.extensionPath, "server", "dist", "server.js");
}

export async function activate(context: vscode.ExtensionContext): Promise<void> {
  // Activate.
  //
  // Parameters:
  // - `context` — input value
  //
  // Returns:
  // Success value on completion, or an error.
  //
  // Options:
  // None.
  //
  // Example:
  // const result = activate(context);

  const serverModule = resolveServerModule(context);
  if (!serverModule) {
    vscode.window.showWarningMessage("Spanda: language server path could not be resolved.");
    return;
  }

  const run: ServerOptions = {
    module: serverModule,
    transport: TransportKind.ipc,
  };

  const debug: ServerOptions = {
    module: serverModule,
    transport: TransportKind.ipc,
    options: {
      execArgv: ["--nolazy", "--inspect=6011"],
    },
  };

  const clientOptions: LanguageClientOptions = {
    documentSelector: [{ scheme: "file", language: "spanda" }],
    synchronize: {
      fileEvents: vscode.workspace.createFileSystemWatcher("**/*.sd"),
    },
  };

  client = new LanguageClient("spanda-lsp", "Spanda Language Server", { run, debug }, clientOptions);
  context.subscriptions.push(client);
  await client.start();
}

export async function deactivate(): Promise<void> {
  // Deactivate.
  //
  // Parameters:
  // None.
  //
  // Returns:
  // Success value on completion, or an error.
  //
  // Options:
  // None.
  //
  // Example:
  // const result = deactivate();

  if (client) {
    await client.stop();
  }
}
