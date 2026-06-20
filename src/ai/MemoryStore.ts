/**
 * MemoryStore module (ai/MemoryStore.ts).
 * @module
 */

import type { RuntimeValue } from "../runtime/interpreter.js";

export type MemoryKind = "short_term" | "long_term";

export class MemoryStore {
  private entries: Array<{ key: string; value: RuntimeValue; at: number }> = [];
  private readonly limit: number;

  constructor(
    public readonly kind: MemoryKind,
    limit = kind === "short_term" ? 32 : 256,
  ) {
    this.limit = limit;
  }

  remember(key: string, value: RuntimeValue): void {
    // Remember.
    //
    // Parameters:
    // - `key` — input value
    // - `value` — input value
    //
    // Returns:
    // Nothing.
    //
    // Options:
    // None.
    //
    // Example:
    // const result = remember(key, value);

    this.entries.push({ key, value, at: Date.now() });
    if (this.entries.length > this.limit) {
      this.entries.shift();
    }
  }

  recall(key: string): RuntimeValue | undefined {
    // Recall.
    //
    // Parameters:
    // - `key` — input value
    //
    // Returns:
    // Some value on success, otherwise none.
    //
    // Options:
    // None.
    //
    // Example:
    // const result = recall(key);

    for (let i = this.entries.length - 1; i >= 0; i--) {
      if (this.entries[i]!.key === key) return this.entries[i]!.value;
    }
    return undefined;
  }

  recent(count = 5): RuntimeValue[] {
    // Recent.
    //
    // Parameters:
    // - `count` — optional input
    //
    // Returns:
    // RuntimeValue[].
    //
    // Options:
    // - `count` — optional parameter
    //
    // Example:
    // const result = recent(count);

    return this.entries.slice(-count).map((e) => e.value);
  }

  clear(): void {
    // Clear the value.
    //
    // Parameters:
    // None.
    //
    // Returns:
    // Nothing.
    //
    // Options:
    // None.
    //
    // Example:
    // const result = clear();

    this.entries = [];
  }

  summaryForPrompt(): string | undefined {
    // SummaryForPrompt.
    //
    // Parameters:
    // None.
    //
    // Returns:
    // Some value on success, otherwise none.
    //
    // Options:
    // None.
    //
    // Example:
    // const result = summaryForPrompt();

    if (this.entries.length === 0) return undefined;
    const keys = this.entries
      .slice(-5)
      .map((e) => e.key)
      .reverse();
    return `Agent memory (${this.kind}): ${keys.join(", ")}`;
  }
}
