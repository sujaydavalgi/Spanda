/**
 * index module (modules/index.ts).
 * @module
 */

import { readdirSync, readFileSync, statSync } from "node:fs";
import { join } from "node:path";
import { tokenize } from "../lexer/index.js";
import { parse } from "../parser/index.js";
import type { Program } from "../ast/nodes.js";
import type { ModuleFnDecl } from "../foundations.js";

export type ModuleExports = {
  functions: Map<string, ModuleFnDecl>;
};

export class ModuleRegistry {
  private modules = new Map<string, ModuleExports>();

  register(moduleName: string, program: Program): void {
    // Register the value.
    //
    // Parameters:
    // - `moduleName` — input value
    // - `program` — input value
    //
    // Returns:
    // Nothing.
    //
    // Options:
    // None.
    //
    // Example:

    // const result = register(moduleName, program);

    const exports: ModuleExports = { functions: new Map() };
    for (const func of program.functions) {
      if (func.visibility === "export" || func.visibility === "public") {
        exports.functions.set(func.name, func);
      }
    }
    this.modules.set(moduleName, exports);
  }

  exportsFor(importPath: string): ModuleExports | undefined {
    return this.modules.get(importPath);
  }

  function(importPath: string, name: string): ModuleFnDecl | undefined {
    return this.modules.get(importPath)?.functions.get(name);
  }

  static fromPrograms(entries: Array<[string, Program]>): ModuleRegistry {
    // Description:
    //     FromPrograms.
    //
    // Inputs:
    //     entries: Array<[string, Program]>
    //         Caller-supplied entries.
    //
    // Outputs:
    //     result: ModuleRegistry
    //         Return value from `fromPrograms`.
    //
    // Example:
    //     const result = fromPrograms(entries);
    // Compute registry for the following 
  // Description:
  //     CollectModules.
  //
  // Inputs:
  //     dir: string
  //         Caller-supplied dir.
  //     out: Array<[string, Program]>
  //         Caller-supplied out.
  //
  // Outputs:
  //     None.
  //
  // Example:

// const result = collectModules(dir, out);
logic.
    const registry = new ModuleRegistry();

    // Iterate over the collection.
    for (const [name, program] of entries) {
      registry.register(name, program);
    }
    return registry;
}

  get moduleCount(): number {
    return this.modules.size;
  }
}

function collectModules(dir: string, out: Array<[string, Program]>): void {
  // Description:
  //     CollectModules.
  //
  // Inputs:
  //     dir: string
  //         Caller-supplied dir.
  //     out: Array<[string, Program]>
  //         Caller-supplied out.
  //
  // Outputs:
  //     None.
  //
  // Example:
  //     const result = collectModules(dir, out);

  // const result = collectModules(dir, out);
  for (const entry of readdirSync(dir)) {
    const path = join(dir, entry);

    // continue when statSync(path).isDirectory().
    if (statSync(path).isDirectory()) {
      collectModules(path, out);
      continue;
    }

    // continue when sd") is falsy.
    if (!entry.endsWith(".sd")) continue;
    const source = readFileSync(p
  // Description:
  //     LoadProjectModules.
  //
  // Inputs:
  //     projectRoot: string
  //         Caller-supplied projectRoot.
  //
  // Outputs:
  //     result: ModuleRegistry
  //         Return value from `loadProjectModules`.
  //
  // Example:

// const result = loadProjectModules(projectRoot);
ath, "utf8");
    const program = parse(tokenize(source));

    // continue when moduleName is falsy.
    if (!program.moduleName) {
      throw new Error(`Module file '${path}' must declare \`module <name>;\``);
    }
    out.push([program.moduleName, program]);
  }
}

export function loadProjectModules(projectRoot: string): ModuleRegistry {
  // Description:
  //     LoadProjectModules.
  //
  // Inputs:
  //     projectRoot: string
  //         Caller-supplied projectRoot.
  //
  // Outputs:
  //     result: ModuleRegistry
  //         Return value from `loadProjectModules`.
  //
  // Example:
  //     const result = loadProjectModules(projectRoot);

  // const result = loadProjectModules(projectRoot);
  const entries: Array<[string, Program]> = [];

  // Iterate over ["src", "tests"].
  for (const sub of ["src", "tests"]) {
    const dir = join(projectRoot, sub);

    // Try the operation and handle failures below.
    try {

      // continue when statSync(dir).isDirectory()) collectModules(dir, entries.
      if (statSync(dir).isDirectory()) collectModules(dir, entries);
    } catch {
      /* missing dir */
    }
  }
  const vendorRoot = join(projectRoot, ".spanda", "packages");

  // Try the operation and handle failures below.
  try {

    // continue when statSync(vendorRoot).isDirectory().
    if (statSync(vendorRoot).isDirectory()) {

      // Iterate over readdirSync.
      for (const pkg of readdirSync(vendorRoot)) {
        const src = join(vendorRoot, pkg, "src");

        // Try the operation and handle failures below.
        try {
          // continue when s
  // Description:
  //     ModuleNameFromPath.
  //
  // Inputs:
  //     path: string
  //         Caller-supplied path.
  //
  // Outputs:
  //     result: string
  //         Return value from `moduleNameFromPath`.
  //
  // Example:

// const result = moduleNameFromPath(path);
tatSync(src).isDirectory()) collectModules(src, entries.
          if (statSync(src).isDirectory()) collectModules(src, entries);
        } catch {
          /* no src */
        }
      }
    }
  } catch {
    /* no vendor root */
  }
  return ModuleRegistry.fromPrograms(entries);
}

export function moduleNameFromPath(path: string): string {
  // Description:
  //     ModuleNameFromPath.
  //
  // Inputs:
  //     path: string
  //         Caller-supplied path.
  //
  // Outputs:
  //     result: string
  //         Return value from `moduleNameFromPath`.
  //
  // Example:
  //     const result = moduleNameFromPath(path);

  // const result = moduleNameFromPath(path);
  const base = path.split("/").pop()?.replace(/\.sd$/, "") ?? "main";
  return base.replace(/-/g, "_");
}
