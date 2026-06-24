import { describe, expect, it } from "vitest";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import {
  deserializeMissionTrace,
  loadMissionTrace,
  replayMissionDeterministic,
} from "../src/replay.js";

const repoRoot = join(import.meta.dirname, "..");

describe("mission trace interoperability", () => {
  it("normalizes Rust snake_case trace frames", () => {
    const tracePath = join(repoRoot, "examples/end_to_end/validated_telemetry.trace");
    const trace = loadMissionTrace(tracePath);
    expect(trace.frames.length).toBeGreaterThan(0);
    expect(trace.frames[0]?.simTimeMs).toBe(0);
    expect(trace.frames[0]?.event).toBe("topic_publish");
  });

  it("accepts inline Rust JSON with snake_case fields", () => {
    const trace = deserializeMissionTrace(
      readFileSync(join(repoRoot, "examples/end_to_end/validated_telemetry.trace"), "utf8"),
    );
    expect(trace.frames[0]?.state?.emergencyStop).toBe(false);
  });
});

describe("deterministic replay", () => {
  it("verifies the validated telemetry example trace", () => {
    const tracePath = join(repoRoot, "examples/end_to_end/validated_telemetry.trace");
    const verification = replayMissionDeterministic(tracePath);
    expect(verification.ok).toBe(true);
    expect(verification.mismatches).toHaveLength(0);
  });
});
