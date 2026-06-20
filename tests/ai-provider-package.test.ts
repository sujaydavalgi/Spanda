import { describe, it, expect } from "vitest";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { compile } from "../src/compile.js";

const repoRoot = join(import.meta.dirname, "..");

describe("ai provider package example", () => {
  it("type-checks ai_provider_package provider.sd", () => {
    const source = readFileSync(
      join(repoRoot, "examples/packages/ai_provider_package/src/provider.sd"),
      "utf-8",
    );
    expect(() => compile(source, "typescript")).not.toThrow();
  });
});
