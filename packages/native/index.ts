export type Diagnostic = {
  message: string;
  line: number;
  column: number;
};

export type CheckResult = {
  ok: boolean;
  diagnostics: Diagnostic[];
};

export type PoseState = {
  x: number;
  y: number;
  theta: number;
  z?: number;
};

export type VelocityState = {
  linear: number;
  angular: number;
};

export type RobotState = {
  pose: PoseState;
  velocity: VelocityState;
  emergency_stop: boolean;
};

export type TaskMetrics = {
  name: string;
  priority: string;
  interval_ms: number;
  ticks: number;
  skipped: number;
  missed_deadlines: number;
  budget_violations: number;
  last_duration_ms: number;
  max_duration_ms: number;
};

export type SchedulerMetrics = {
  multiplexed_tasks: number;
  scheduler_ticks: number;
  base_tick_ms: number;
  emergency_stops: number;
};

export type ExecutionMetrics = {
  spawns: number;
  joins: number;
  parallel_blocks: number;
  fire_and_forget_spawns: number;
};

export type RuntimeTelemetry = {
  tasks: TaskMetrics[];
  scheduler: SchedulerMetrics;
  execution: ExecutionMetrics;
  replay_frames: number;
};

export type RunResult = {
  state: RobotState;
  events: string[];
  logs: string[];
  metrics: RuntimeTelemetry;
};

export type RunOptions = {
  entryBehavior?: string;
  maxLoopIterations?: number;
  traceScheduler?: boolean;
  traceTasks?: boolean;
  replayTrace?: boolean;
};

export interface SpandaNative {
  checkSource(source: string): CheckResult;
  runSource(source: string, options?: RunOptions): RunResult;
  coreVersion(): string;
}

let native: SpandaNative | null = null;
let loadAttempted = false;

export function isNativeAvailable(): boolean {
  loadNative();
  return native !== null;
}

function loadNative(): void {
  if (loadAttempted) return;
  loadAttempted = true;
  try {
    // eslint-disable-next-line @typescript-eslint/no-require-imports
    const mod = require("./native.js") as SpandaNative;
    native = mod;
  } catch {
    native = null;
  }
}

export function checkSource(source: string): CheckResult | null {
  loadNative();
  if (!native) return null;
  return native.checkSource(source);
}

export function runSource(source: string, options?: RunOptions): RunResult | null {
  loadNative();
  if (!native) return null;
  return native.runSource(source, options);
}

export function coreVersion(): string | null {
  loadNative();
  return native?.coreVersion() ?? null;
}
