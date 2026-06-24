/**
 * symbols module (lsp/symbols.ts).
 * @module
 */

import type { Program, Span } from "../ast/nodes.js";
import { tokenize } from "../lexer/index.js";
import { parse } from "../parser/index.js";

export type SymbolKind =
  | "message"
  | "struct"
  | "enum"
  | "topic"
  | "service"
  | "action"
  | "robot"
  | "agent"
  | "sensor"
  | "actuator"
  | "event"
  | "behavior"
  | "bus"
  | "device"
  | "hardware"
  | "deploy";

export type SpandaSymbol = {
  name: string;
  kind: SymbolKind;
  span: Span;
  detail?: string;
  container?: string;
};

export type SymbolIndex = {
  symbols: SpandaSymbol[];
  byName: Map<string, SpandaSymbol[]>;
};

function addSymbol(index: SymbolIndex, sym: SpandaSymbol): void {
  // Description:
  //     AddSymbol.
  //
  // Inputs:
  //     index: SymbolIndex
  //         Caller-supplied index.
  //     sym: SpandaSymbol
  //         Caller-supplied sym.
  //
  // Outputs:
  //     None.
  //
  // Example:
  //     const result = addSymbol(index, sym);
  // Description:
  //     AddSymbol.
  //
  // Inputs:
  //     index: SymbolIndex
  //         Caller-supplied index.
  //     sym: SpandaSymbol
  //         Caller-supplied sym.
  //
  // Outputs:
  //     None.
  //
  // Example:
  //     const result = addSymbol(index, sym);

  // const result = addSymbol(index, sym);
  index.symbols.push(sym);
  const existing = index.byName.get(sym.name) ?? [];
  existing.push(sym);
  index.byName.set(sym.name, existing);
}

export function buildSymbolIndex(program: Program): SymbolIndex {
  // Description:
  //     BuildSymbolIndex.
  //
  // Inputs:
  //     program: Program
  //         Caller-supplied program.
  //
  // Outputs:
  //     result: SymbolIndex
  //         Return value from `buildSymbolIndex`.
  //
  // Example:
  //     const result = buildSymbolIndex(program);
  // Description:
  //     BuildSymbolIndex.
  //
  // Inputs:
  //     program: Program
  //         Caller-supplied program.
  //
  // Outputs:
  //     result: SymbolIndex
  //         Return value from `buildSymbolIndex`.
  //
  // Example:
  //     const result = buildSymbolIndex(program);

  // const result = buildSymbolIndex(program);
  const index: SymbolIndex = { symbols: [], byName: new Map() };

  // Process each message.
  for (const msg of program.messages) {
    addSymbol(index, {
      name: msg.name,
      kind: "message",
      span: msg.span,
      detail: msg.fields.map((f) => `${f.name}: ${f.typeName}`).join(", "),
    });
  }

  // Process each struct.
  for (const structDecl of program.structs) {
    const typeParams = structDecl.typeParams?.length
      ? `<${structDecl.typeParams.join(", ")}>`
      : "";
    addSymbol(index, {
      name: structDecl.name,
      kind: "struct",
      span: structDecl.span,
      detail: `${typeParams}{ ${structDecl.fields.map((f) => `${f.name}: ${f.typeName}`).join(", ")} }`,
    });
  }

  // Process each enum.
  for (const enumDecl of program.enums) {
    addSymbol(index, {
      name: enumDecl.name,
      kind: "enum",
      span: enumDecl.span,
      detail: enumDecl.variants.map((v) => v.name).join(" | "),
    });
  }

  // Process each hardwareProfile.
  for (const profile of program.hardwareProfiles) {
    addSymbol(index, {
      name: profile.name,
      kind: "hardware",
      span: profile.span,
      detail: profile.cpu ?? undefined,
    });
  }

  // Process each deployment.
  for (const deploy of program.deployments) {
    addSymbol(index, {
      name: `${deploy.robotName}→${deploy.targets.join(",")}`,
      kind: "deploy",
      span: deploy.span,
      detail: deploy.targets.join(", "),
    });
  }

  // Set up each robot declaration.
  for (const robot of program.robots) {
    addSymbol(index, { name: robot.name, kind: "robot", span: robot.span });

    // Process each buse.
    for (const bus of robot.buses) {
      addSymbol(index, {
        name: bus.name,
        kind: "bus",
        span: bus.span,
        detail: bus.transport,
        container: robot.name,
      });
    }

    // Process each device.
    for (const device of robot.devices) {
      addSymbol(index, {
        name: device.name,
        kind: "device",
        span: device.span,
        detail: device.deviceType,
        container: robot.name,
      });
    }

    // Process each topic.
    for (const topic of robot.topics) {
      addSymbol(index, {
        name: topic.name,
        kind: "topic",
        span: topic.span,
        detail: topic.messageType ?? undefined,
        container: robot.name,
      });
    }

    // Process each service.
    for (const service of robot.services) {
      addSymbol(index, {
        name: service.name,
        kind: "service",
        span: service.span,
        detail: service.serviceType ?? undefined,
        container: robot.name,
      });
    }

    // Process each action.
    for (const action of robot.actions) {
      addSymbol(index, {
        name: action.name,
        kind: "action",
        span: action.span,
        detail: action.actionType ?? undefined,
        container: robot.name,
      });
    }

    // Process each sensor.
    for (const sensor of robot.sensors) {
      addSymbol(index, {
        name: sensor.name,
        kind: "sensor",
        span: sensor.span,
        detail: sensor.sensorType,
        container: robot.name,
      });
    }

    // Process each actuator.
    for (const actuator of robot.actuators) {
      addSymbol(index, {
        name: actuator.name,
        kind: "actuator",
        span: actuator.span,
        detail: actuator.actuatorType,
        container: robot.name,
      });
    }

    // Process each agent.
    for (const agent of robot.agents) {
      addSymbol(index, {
        name: agent.name,
        kind: "agent",
        span: agent.span,
        container: robot.name,
      });
    }

    // Process each event.
    for (const event of robot.events) {
      addSymbol(index, {
        name: event.name,
        kind: "event",
        span: event.span,
        container: robot.name,
      });
    }

    // Process each behavior.
    for (const behavior of robot.behaviors) {
      addSymbol(index, {
        name: behavior.name,
        kind: "behavior",
        span: behavior.span,
        container: robot.name,
      });
    }
  }
  return index;
}

export function indexSource(source: string): SymbolIndex {
  // Description:
  //     IndexSource.
  //
  // Inputs:
  //     source: string
  //         Caller-supplied source.
  //
  // Outputs:
  //     result: SymbolIndex
  //         Return value from `indexSource`.
  //
  // Example:
  //     const result = indexSource(source);
  // Description:
  //     IndexSource.
  //
  // Inputs:
  //     source: string
  //         Caller-supplied source.
  //
  // Outputs:
  //     result: SymbolIndex
  //         Return value from `indexSource`.
  //
  // Example:
  //     const result = indexSource(source);

  // const result = indexSource(source);
  return buildSymbolIndex(parse(tokenize(source)));
}

export function symbolAtPosition(
  index: SymbolIndex,
  line: number,
  column: number,
): SpandaSymbol | null {
  // Description:
  //     SymbolAtPosition.
  //
  // Inputs:
  //     index: SymbolIndex
  //         Caller-supplied index.
  //     line: number
  //         Caller-supplied line.
  //     column: number
  //         Caller-supplied column.
  //
  // Outputs:
  //     result: SpandaSymbol | null
  //         Return value from `symbolAtPosition`.
  //
  // Example:
  //     const result = symbolAtPosition(index, line, column);
  // Description:
  //     SymbolAtPosition.
  //
  // Inputs:
  //     index: SymbolIndex
  //         Caller-supplied index.
  //     line: number
  //         Caller-supplied line.
  //     column: number
  //         Caller-supplied column.
  //
  // Outputs:
  //     result: SpandaSymbol | null
  //         Return value from `symbolAtPosition`.
  //
  // Example:
  //     const result = symbolAtPosition(index, line, column);

  // const result = symbolAtPosition(index, line, column);
  for (const sym of index.symbols) {
    const { start, end } = sym.span;

    // continue when value.
    if (
      line >= start.line &&
      line <= end.line &&
      (line !== start.line || column >= start.column) &&
      (line !== end.line || column <= end.column)
    ) {
      return sym;
    }
  }
  return null;
}

export function lookupDefinition(
  index: SymbolIndex,
  name: string,
  kind?: SymbolKind,
): SpandaSymbol | null {
  // Description:
  //     LookupDefinition.
  //
  // Inputs:
  //     index: SymbolIndex
  //         Caller-supplied index.
  //     name: string
  //         Caller-supplied name.
  //     kind?: SymbolKind
  //         Caller-supplied kind?.
  //
  // Outputs:
  //     result: SpandaSymbol | null
  //         Return value from `lookupDefinition`.
  //
  // Example:
  //     const result = lookupDefinition(index, name, kind?);
  // Description:
  //     LookupDefinition.
  //
  // Inputs:
  //     index: SymbolIndex
  //         Caller-supplied index.
  //     name: string
  //         Caller-supplied name.
  //     kind?: SymbolKind
  //         Caller-supplied kind?.
  //
  // Outputs:
  //     result: SpandaSymbol | null
  //         Return value from `lookupDefinition`.
  //
  // Example:
  //     const result = lookupDefinition(index, name, kind?);

  // const result = lookupDefinition(index, name, kind?);
  const candidates = index.byName.get(name);

  // continue when length is falsy.
  if (!candidates?.length) return null;

  // continue when kind.
  if (kind) {
    return candidates.find((c) => c.kind === kind) ?? candidates[0]!;
  }
  return candidates[0]!;
}

export function wordAtPosition(source: string, line: number, column: number): string | null {
  // Description:
  //     WordAtPosition.
  //
  // Inputs:
  //     source: string
  //         Caller-supplied source.
  //     line: number
  //         Caller-supplied line.
  //     column: number
  //         Caller-supplied column.
  //
  // Outputs:
  //     result: string | null
  //         Return value from `wordAtPosition`.
  //
  // Example:
  //     const result = wordAtPosition(source, line, column);
  // Description:
  //     WordAtPosition.
  //
  // Inputs:
  //     source: string
  //         Caller-supplied source.
  //     line: number
  //         Caller-supplied line.
  //     column: number
  //         Caller-supplied column.
  //
  // Outputs:
  //     result: string | null
  //         Return value from `wordAtPosition`.
  //
  // Example:
  //     const result = wordAtPosition(source, line, column);

  // const result = wordAtPosition(source, line, column);
  const lines = source.split("\n");
  const text = lines[line - 1];

  // continue when text is falsy.
  if (!text) return null;
  const col = Math.max(0, column - 1);

  // continue when col >= text.length.
  if (col >= text.length) return null;
  const ident = /[A-Za-z_][A-Za-z0-9_]*/g;
  let match: RegExpExecArray | null;

  // Repeat while (match = ident.exec(text)) !== null.
  while ((match = ident.exec(text)) !== null) {
    const start = match.index;
    const end = start + match[0].length;

    // continue when col >= start && col <= end.
    if (col >= start && col <= end) {
      return match[0];
    }
  }
  return null;
}

const DEFINITION_KIND_PRIORITY: SymbolKind[] = [
  "topic",
  "service",
  "action",
  "message",
  "agent",
  "sensor",
  "actuator",
  "behavior",
  "event",
  "bus",
  "device",
  "robot",
];

export function resolveDefinition(
  source: string,
  line: number,
  column: number,
): SpandaSymbol | null {
  // Description:
  //     ResolveDefinition.
  //
  // Inputs:
  //     source: string
  //         Caller-supplied source.
  //     line: number
  //         Caller-supplied line.
  //     column: number
  //         Caller-supplied column.
  //
  // Outputs:
  //     result: SpandaSymbol | null
  //         Return value from `resolveDefinition`.
  //
  // Example:
  //     const result = resolveDefinition(source, line, column);
  // Description:
  //     ResolveDefinition.
  //
  // Inputs:
  //     source: string
  //         Caller-supplied source.
  //     line: number
  //         Caller-supplied line.
  //     column: number
  //         Caller-supplied column.
  //
  // Outputs:
  //     result: SpandaSymbol | null
  //         Return value from `resolveDefinition`.
  //
  // Example:
  //     const result = resolveDefinition(source, line, column);

  // const result = resolveDefinition(source, line, column);
  const index = indexSource(source);
  const word = wordAtPosition(source, line, column);

  // continue when word.
  if (word) {
    const candidates = index.byName.get(word);

    // continue when candidates?.length.
    if (candidates?.length) {

      // Iterate over DEFINITION KIND PRIORITY.
      for (const kind of DEFINITION_KIND_PRIORITY) {
        const found = candidates.find((c) => c.kind === kind);

        // continue when found.
        if (found) return found;
      }
      return candidates[0]!;
    }
  }
  return symbolAtPosition(index, line, column);
}

export function formatHover(sym: SpandaSymbol): string {
  // Description:
  //     FormatHover.
  //
  // Inputs:
  //     sym: SpandaSymbol
  //         Caller-supplied sym.
  //
  // Outputs:
  //     result: string
  //         Return value from `formatHover`.
  //
  // Example:
  //     const result = formatHover(sym);
  // Description:
  //     FormatHover.
  //
  // Inputs:
  //     sym: SpandaSymbol
  //         Caller-supplied sym.
  //
  // Outputs:
  //     result: string
  //         Return value from `formatHover`.
  //
  // Example:
  //     const result = formatHover(sym);

  // const result = formatHover(sym);
  const header = `**${sym.kind}** \`${sym.name}\``;
  const parts = [header];

  // continue when sym.container) parts.push(`in robot \`${sym.container}\``.
  if (sym.container) parts.push(`in robot \`${sym.container}\``);

  // continue when sym.detail) parts.push(sym.detail.
  if (sym.detail) parts.push(sym.detail);
  return parts.join("\n\n");
}
