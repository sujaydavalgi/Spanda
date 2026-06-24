/**
 * Deterministic mission trace recording and replay for simulation runs.
 * @module
 */

import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { tokenize } from "./lexer/index.js";
import { parse } from "./parser/index.js";
import { Interpreter } from "./runtime/interpreter.js";
import { createDefaultSimulator } from "./simulator/index.js";

export type TraceFrame = {
  simTimeMs: number;
  event: string;
  payload?: unknown;
  state?: ReplayStateSnapshot;
};

export type ReplayStateSnapshot = {
  pose: { x: number; y: number; theta: number; z?: number };
  velocity: { linear: number; angular: number };
  emergencyStop: boolean;
  activeMode?: string;
};

export type PlaybackReport = {
  framesApplied: number;
  statesApplied: number;
  events: string[];
};

export type MissionTrace = {
  version: number;
  source: string;
  deterministic: boolean;
  frames: TraceFrame[];
};

export type TraceVerification = {
  ok: boolean;
  matched: number;
  mismatches: string[];
};

export function createMissionTrace(source: string): MissionTrace {
  // Description:
  //     CreateMissionTrace.
  //
  // Inputs:
  //     source: string
  //         Caller-supplied source.
  //
  // Outputs:
  //     result: MissionTrace
  //         Return value from `createMissionTrace`.
  //
  // Example:
  //     const result = createMissionTrace(source);
  // Description:
  //     CreateMissionTrace.
  //
  // Inputs:
  //     source: string
  //         Caller-supplied source.
  //
  // Outputs:
  //     result: MissionTrace
  //         Return value from `createMissionTrace`.
  //
  // Example:

  //     const result = createMissionTrace(source);

  return {
    version: 1,
    source,
    deterministic: true,
    frames: [],
  };
}

export function recordTraceFrame(
  trace: MissionTrace,
  simTimeMs: number,
  event: string,
  payload: unknown = {},
): void {
  // Description:
  //     RecordTraceFrame.
  //
  // Inputs:
  //     trace: MissionTrace
  //         Caller-supplied trace.
  //     simTimeMs: number
  //         Caller-supplied simTimeMs.
  //     event: string
  //         Caller-supplied event.
  //     payload: unknown = {}
  //         Caller-supplied payload.
  //
  // Outputs:
  //     None.
  //
  // Example:
  //     const result = recordTraceFrame(trace, simTimeMs, event, payload);
  // Description:
  //     RecordTraceFrame.
  //
  // Inputs:
  //     trace: MissionTrace
  //         Caller-supplied trace.
  //     simTimeMs: number
  //         Caller-supplied simTimeMs.
  //     event: string
  //         Caller-supplied event.
  //     payload: unknown = {}
  //         Caller-supplied payload.
  //
  // Outputs:
  //     None.
  //
  // Example:

  //     const result = recordTraceFrame(trace, simTimeMs, event, payload);

  trace.frames.push({ simTimeMs, event, payload, state: undefined });
}

export function recordTraceFrameWithState(
  trace: MissionTrace,
  simTimeMs: number,
  event: string,
  payload: unknown,
  state: ReplayStateSnapshot,
): void {
  // Description:
  //     RecordTraceFrameWithState.
  //
  // Inputs:
  //     trace: MissionTrace
  //         Caller-supplied trace.
  //     simTimeMs: number
  //         Caller-supplied simTimeMs.
  //     event: string
  //         Caller-supplied event.
  //     payload: unknown
  //         Caller-supplied payload.
  //     state: ReplayStateSnapshot
  //         Caller-supplied state.
  //
  // Outputs:
  //     None.
  //
  // Example:

  //     const result = recordTraceFrameWithState(trace, simTimeMs, event, payload, state);

  trace.frames.push({ simTimeMs, event, payload, state });
  if (state) {
    trace.version = 2;
  }
}

export function playbackFrames(
  frames: TraceFrame[],
  applyState: (state: ReplayStateSnapshot) => void,
  wallClock = false,
): PlaybackReport {
  // Description:
  //     PlaybackFrames.
  //
  // Inputs:
  //     frames: TraceFrame[]
  //         Caller-supplied frames.
  //
  // Outputs:
  //     result: PlaybackReport
  //         Return value from `playbackFrames`.
  //
  // Example:

  //     const result = playbackFrames(frames);

  let statesApplied = 0;
  const events: string[] = [];
  let prevSim = 0;
  const wallStart = performance.now();
  for (const frame of frames) {
    if (wallClock) {
      const delta = frame.simTimeMs - prevSim;
      if (delta > 0) {
        const end = performance.now() + delta;
        while (performance.now() < end) {
          /* wall pacing */
        }
      }
      prevSim = frame.simTimeMs;
    }
    if (frame.state) {
      applyState(frame.state);
      statesApplied += 1;
    }
    events.push(frame.event);
  }
  void wallStart;
  return { framesApplied: frames.length, statesApplied, events };
}

export function traceFramesFrom(trace: MissionTrace, offsetMs: number): TraceFrame[] {
  // Description:
  //     TraceFramesFrom.
  //
  // Inputs:
  //     trace: MissionTrace
  //         Caller-supplied trace.
  //     offsetMs: number
  //         Caller-supplied offsetMs.
  //
  // Outputs:
  //     result: TraceFrame[]
  //         Return value from `traceFramesFrom`.
  //
  // Example:
  //     const result = traceFramesFrom(trace, offsetMs);
  // Description:
  //     TraceFramesFrom.
  //
  // Inputs:
  //     trace: MissionTrace
  //         Caller-supplied trace.
  //     offsetMs: number
  //         Caller-supplied offsetMs.
  //
  // Outputs:
  //     result: TraceFrame[]
  //         Return value from `traceFramesFrom`.
  //
  // Example:

  //     const result = traceFramesFrom(trace, offsetMs);

  const idx = trace.frames.findIndex((frame) => frame.simTimeMs >= offsetMs);
  return idx === -1 ? [] : trace.frames.slice(idx);
}

export function parseReplayOffset(raw: string): number {
  // Description:
  //     ParseReplayOffset.
  //
  // Inputs:
  //     raw: string
  //         Caller-supplied raw.
  //
  // Outputs:
  //     result: number
  //         Return value from `parseReplayOffset`.
  //
  // Example:
  //     const result = parseReplayOffset(raw);
  // Description:
  //     ParseReplayOffset.
  //
  // Inputs:
  //     raw: string
  //         Caller-supplied raw.
  //
  // Outputs:
  //     result: number
  //         Return value from `parseReplayOffset`.
  //
  // Example:

  //     const result = parseReplayOffset(raw);

  const asNumber = Number(raw);
  if (!Number.isNaN(asNumber)) {
    return asNumber;
  }

  if (!raw.startsWith("T+")) {
    throw new Error(`Invalid replay offset '${raw}'; expected T+mm:ss or milliseconds`);
  }

  const value = raw.slice(2);
  const parts = value.split(":");
  let totalSecs = 0;

  if (parts.length === 2) {
    totalSecs = Number(parts[0]) * 60 + Number(parts[1]);
  } else if (parts.length === 3) {
    totalSecs = Number(parts[0]) * 3600 + Number(parts[1]) * 60 + Number(parts[2]);
  } else {
    throw new Error(`Invalid replay offset '${raw}'; expected T+mm:ss`);
  }

  return totalSecs * 1000;
}

export function verifyTraces(
  expected: MissionTrace,
  actual: MissionTrace,
  fromMs: number,
): TraceVerification {
  // Description:
  //     VerifyTraces.
  //
  // Inputs:
  //     expected: MissionTrace
  //         Caller-supplied expected.
  //     actual: MissionTrace
  //         Caller-supplied actual.
  //     fromMs: number
  //         Caller-supplied fromMs.
  //
  // Outputs:
  //     result: TraceVerification
  //         Return value from `verifyTraces`.
  //
  // Example:
  //     const result = verifyTraces(expected, actual, fromMs);
  // Description:
  //     VerifyTraces.
  //
  // Inputs:
  //     expected: MissionTrace
  //         Caller-supplied expected.
  //     actual: MissionTrace
  //         Caller-supplied actual.
  //     fromMs: number
  //         Caller-supplied fromMs.
  //
  // Outputs:
  //     result: TraceVerification
  //         Return value from `verifyTraces`.
  //
  // Example:

  //     const result = verifyTraces(expected, actual, fromMs);

  const exp = traceFramesFrom(expected, fromMs);
  const act = traceFramesFrom(actual, fromMs);
  const mismatches: string[] = [];
  const shared = Math.min(exp.length, act.length);

  for (let index = 0; index < shared; index++) {
    if (exp[index]!.event !== act[index]!.event) {
      mismatches.push(
        `frame ${index}: expected event '${exp[index]!.event}', got '${act[index]!.event}'`,
      );
    } else if (Math.abs(exp[index]!.simTimeMs - act[index]!.simTimeMs) > 0.001) {
      mismatches.push(
        `frame ${index} event '${exp[index]!.event}': expected t=${exp[index]!.simTimeMs.toFixed(3)}ms, got t=${act[index]!.simTimeMs.toFixed(3)}ms`,
      );
    }
  }

  if (exp.length !== act.length) {
    mismatches.push(`frame count mismatch: expected ${exp.length}, got ${act.length}`);
  }

  return {
    ok: mismatches.length === 0,
    matched: shared,
    mismatches,
  };
}

export function serializeMissionTrace(trace: MissionTrace): string {
  // Description:
  //     SerializeMissionTrace.
  //
  // Inputs:
  //     trace: MissionTrace
  //         Caller-supplied trace.
  //
  // Outputs:
  //     result: string
  //         Return value from `serializeMissionTrace`.
  //
  // Example:

  //     const result = serializeMissionTrace(trace);

  return JSON.stringify(trace, null, 2);
}

function normalizeReplayState(raw: Record<string, unknown>): ReplayStateSnapshot | undefined {
  if (!raw) {
    return undefined;
  }
  const pose = raw.pose as Record<string, number> | undefined;
  const velocity = raw.velocity as Record<string, number> | undefined;
  if (!pose || !velocity) {
    return undefined;
  }
  return {
    pose: {
      x: pose.x,
      y: pose.y,
      theta: pose.theta,
      z: pose.z,
    },
    velocity: {
      linear: velocity.linear,
      angular: velocity.angular,
    },
    emergencyStop: Boolean(raw.emergencyStop ?? raw.emergency_stop),
    activeMode: (raw.activeMode ?? raw.active_mode) as string | undefined,
  };
}

function normalizeTraceFrame(raw: Record<string, unknown>): TraceFrame {
  return {
    simTimeMs: Number(raw.simTimeMs ?? raw.sim_time_ms ?? 0),
    event: String(raw.event ?? ""),
    payload: raw.payload,
    state: raw.state ? normalizeReplayState(raw.state as Record<string, unknown>) : undefined,
  };
}

export function deserializeMissionTrace(text: string): MissionTrace {
  const parsed = JSON.parse(text) as Record<string, unknown>;
  const frames = Array.isArray(parsed.frames)
    ? parsed.frames.map((frame) => normalizeTraceFrame(frame as Record<string, unknown>))
    : [];
  return {
    version: Number(parsed.version ?? 1),
    source: String(parsed.source ?? ""),
    deterministic: Boolean(parsed.deterministic),
    frames,
  };
}

export function loadMissionTrace(path: string): MissionTrace {
  return deserializeMissionTrace(readFileSync(path, "utf8"));
}

export function resolveTraceOutputPath(traceSource?: string): string {
  const source = traceSource ?? "program.sd";
  if (source.endsWith(".sd")) {
    return `${source.slice(0, -3)}.trace`;
  }
  return `${source}.trace`;
}

export function saveMissionTrace(trace: MissionTrace, path: string): void {
  const parent = dirname(path);
  if (!existsSync(parent)) {
    mkdirSync(parent, { recursive: true });
  }
  writeFileSync(path, serializeMissionTrace(trace), "utf8");
}

export function resolveTraceSource(traceFile: string, source: string): string {
  if (existsSync(source)) {
    return source;
  }
  const candidate = join(dirname(traceFile), source);
  if (existsSync(candidate)) {
    return candidate;
  }
  return source;
}

export function replayMissionDeterministic(
  traceFile: string,
  options?: { fromMs?: number; maxLoopIterations?: number },
): TraceVerification {
  const expected = loadMissionTrace(traceFile);
  const sourcePath = resolveTraceSource(traceFile, expected.source);
  const source = readFileSync(sourcePath, "utf8");
  const program = parse(tokenize(source));
  const interpreter = new Interpreter({
    backend: createDefaultSimulator(),
    maxLoopIterations: options?.maxLoopIterations ?? 20,
    recordTrace: true,
    traceSource: expected.source,
  });
  interpreter.run(program);
  const actual = interpreter.takeMissionTrace();
  if (!actual) {
    return {
      ok: false,
      matched: 0,
      mismatches: ["No mission trace recorded during replay run"],
    };
  }
  return verifyTraces(expected, actual, options?.fromMs ?? 0);
}
