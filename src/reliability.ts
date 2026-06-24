/**
 * Real-time reliability validation helpers for tasks, pipelines, and watchdogs.
 * @module
 */

import type { Span, Stmt } from "./ast/nodes.js";
import type {
  PipelineDecl,
  RecoverDecl,
  ResourceBudgetDecl,
  TaskDecl,
  TaskPriority,
  WatchdogDecl,
} from "./foundations.js";

export type Diagnostic = {
  message: string;
  line: number;
  column: number;
};

export function validateTaskTiming(task: TaskDecl): Diagnostic[] {
  // Description:
  //     ValidateTaskTiming.
  //
  // Inputs:
  //     task: TaskDecl
  //         Caller-supplied task.
  //
  // Outputs:
  //     result: Diagnostic[]
  //         Return value from `validateTaskTiming`.
  //
  // Example:
  //     const result = validateTaskTiming(task);
  // Description:
  //     ValidateTaskTiming.
  //
  // Inputs:
  //     task: TaskDecl
  //         Caller-supplied task.
  //
  // Outputs:
  //     result: Diagnostic[]
  //         Return value from `validateTaskTiming`.
  //
  // Example:

  //     const result = validateTaskTiming(task);

  const { name, intervalMs, deadlineMs, jitterMsMax, span } = task;
  const diags: Diagnostic[] = [];

  if (intervalMs <= 0) {
    diags.push({
      message: `Task '${name}' period must be positive (got ${intervalMs}ms). Suggestion: use \`every 10ms\` or larger.`,
      line: span.start.line,
      column: span.start.column,
    });
  }

  if (deadlineMs !== undefined && deadlineMs !== null) {
    if (deadlineMs <= 0) {
      diags.push({
        message: `Task '${name}' deadline must be positive (got ${deadlineMs}ms).`,
        line: span.start.line,
        column: span.start.column,
      });
    } else if (deadlineMs > intervalMs) {
      diags.push({
        message: `Task '${name}' deadline (${deadlineMs}ms) must be <= period (${intervalMs}ms). Suggestion: increase period or reduce deadline.`,
        line: span.start.line,
        column: span.start.column,
      });
    }
  }

  if (jitterMsMax !== undefined && jitterMsMax !== null) {
    if (jitterMsMax < 0) {
      diags.push({
        message: `Task '${name}' jitter must be non-negative.`,
        line: span.start.line,
        column: span.start.column,
      });
    }
    const slack = deadlineMs ?? intervalMs;
    if (jitterMsMax > slack) {
      diags.push({
        message: `Task '${name}' jitter (${jitterMsMax}ms) exceeds allowable slack (${slack}ms). Suggestion: reduce jitter or increase deadline/period.`,
        line: span.start.line,
        column: span.start.column,
      });
    }
  }

  return diags;
}

export function validateTaskPriority(task: TaskDecl): Diagnostic[] {
  // Description:
  //     ValidateTaskPriority.
  //
  // Inputs:
  //     task: TaskDecl
  //         Caller-supplied task.
  //
  // Outputs:
  //     result: Diagnostic[]
  //         Return value from `validateTaskPriority`.
  //
  // Example:
  //     const result = validateTaskPriority(task);
  // Description:
  //     ValidateTaskPriority.
  //
  // Inputs:
  //     task: TaskDecl
  //         Caller-supplied task.
  //
  // Outputs:
  //     result: Diagnostic[]
  //         Return value from `validateTaskPriority`.
  //
  // Example:

  //     const result = validateTaskPriority(task);

  const { name, priority, isolated, span } = task;
  const diags: Diagnostic[] = [];

  if (isolated && priority !== "critical" && priority !== "high") {
    diags.push({
      message: `Task '${name}' is marked isolated but priority is ${priority}. Suggestion: use \`critical isolated\` or \`high isolated\`.`,
      line: span.start.line,
      column: span.start.column,
    });
  }

  return diags;
}

export function validatePipeline(pipeline: PipelineDecl): Diagnostic[] {
  // Description:
  //     ValidatePipeline.
  //
  // Inputs:
  //     pipeline: PipelineDecl
  //         Caller-supplied pipeline.
  //
  // Outputs:
  //     result: Diagnostic[]
  //         Return value from `validatePipeline`.
  //
  // Example:
  //     const result = validatePipeline(pipeline);
  // Description:
  //     ValidatePipeline.
  //
  // Inputs:
  //     pipeline: PipelineDecl
  //         Caller-supplied pipeline.
  //
  // Outputs:
  //     result: Diagnostic[]
  //         Return value from `validatePipeline`.
  //
  // Example:

  //     const result = validatePipeline(pipeline);

  const { name, budgetMs, span } = pipeline;
  const diags: Diagnostic[] = [];

  if (budgetMs <= 0) {
    diags.push({
      message: `Pipeline '${name}' budget must be positive (got ${budgetMs}ms).`,
      line: span.start.line,
      column: span.start.column,
    });
  }

  return diags;
}

export function validateWatchdog(watchdog: WatchdogDecl, taskNames: string[]): Diagnostic[] {
  // Description:
  //     ValidateWatchdog.
  //
  // Inputs:
  //     watchdog: WatchdogDecl
  //         Caller-supplied watchdog.
  //     taskNames: string[]
  //         Caller-supplied taskNames.
  //
  // Outputs:
  //     result: Diagnostic[]
  //         Return value from `validateWatchdog`.
  //
  // Example:
  //     const result = validateWatchdog(watchdog, taskNames);
  // Description:
  //     ValidateWatchdog.
  //
  // Inputs:
  //     watchdog: WatchdogDecl
  //         Caller-supplied watchdog.
  //     taskNames: string[]
  //         Caller-supplied taskNames.
  //
  // Outputs:
  //     result: Diagnostic[]
  //         Return value from `validateWatchdog`.
  //
  // Example:

  //     const result = validateWatchdog(watchdog, taskNames);

  const { name, target, timeoutMs, span } = watchdog;
  const diags: Diagnostic[] = [];

  if (timeoutMs <= 0) {
    diags.push({
      message: `Watchdog '${name}' timeout must be positive.`,
      line: span.start.line,
      column: span.start.column,
    });
  }

  if (target && !taskNames.includes(target)) {
    diags.push({
      message: `Watchdog '${name}' target task '${target}' not found. Suggestion: declare the task before the watchdog or fix the task name.`,
      line: span.start.line,
      column: span.start.column,
    });
  }

  return diags;
}

export function validateResourceBudget(budget: ResourceBudgetDecl, span: Span): Diagnostic[] {
  // Description:
  //     ValidateResourceBudget.
  //
  // Inputs:
  //     budget: ResourceBudgetDecl
  //         Caller-supplied budget.
  //     span: Span
  //         Caller-supplied span.
  //
  // Outputs:
  //     result: Diagnostic[]
  //         Return value from `validateResourceBudget`.
  //
  // Example:
  //     const result = validateResourceBudget(budget, span);
  // Description:
  //     ValidateResourceBudget.
  //
  // Inputs:
  //     budget: ResourceBudgetDecl
  //         Caller-supplied budget.
  //     span: Span
  //         Caller-supplied span.
  //
  // Outputs:
  //     result: Diagnostic[]
  //         Return value from `validateResourceBudget`.
  //
  // Example:

  //     const result = validateResourceBudget(budget, span);

  const {
    batteryPctMax,
    memoryMbMax,
    cpuPctMax,
    gpuPctMax,
    networkMbpsMax,
    storageMbMax,
  } = budget;
  const diags: Diagnostic[] = [];

  const checkPct = (label: string, value: number | null | undefined) => {
    // Description:
    //     CheckPct.
    //
    // Inputs:
    //     label: string
    //         Caller-supplied label.
    //     value: number | null | undefined
    //         Caller-supplied value.
    //
    // Outputs:
    //     None.
    //
    // Example:

    //     const result = checkPct(label, value);

    if (value !== undefined && value !== null) {
      if (value <= 0 || value > 100) {
        diags.push({
          message: `Resource budget ${label} must be in (0, 100] (got ${value}).`,
          line: span.start.line,
          column: span.start.column,
        });
      }
    }
  };

  checkPct("cpu", cpuPctMax);
  checkPct("gpu", gpuPctMax);
  checkPct("battery", batteryPctMax);

  for (const [label, value] of [
    ["memory", memoryMbMax],
    ["network", networkMbpsMax],
    ["storage", storageMbMax],
  ] as const) {
    if (value !== undefined && value !== null && value <= 0) {
      diags.push({
        message: `Resource budget ${label} must be positive (got ${value}).`,
        line: span.start.line,
        column: span.start.column,
      });
    }
  }

  if (
    cpuPctMax !== undefined &&
    cpuPctMax !== null &&
    gpuPctMax !== undefined &&
    gpuPctMax !
  // Description:
  //     HasSafeRecoverAction.
  //
  // Inputs:
  //     body: Stmt[]
  //         Caller-supplied body.
  //
  // Outputs:
  //     result: boolean
  //         Return value from `hasSafeRecoverAction`.
  //
  // Example:

// const result = hasSafeRecoverAction(body);
== null &&
    cpuPctMax + gpuPctMax > 100
  ) {
    diags.push({
      message: `Resource budget cpu (${cpuPctMax}%) + gpu (${gpuPctMax}%) exceeds 100%. Suggestion: reduce one ceilin
  // Description:
  //     ValidateRecover.
  //
  // Inputs:
  //     recover: RecoverDecl
  //         Caller-supplied recover.
  //
  // Outputs:
  //     result: Diagnostic[]
  //         Return value from `validateRecover`.
  //
  // Example:

// const result = validateRecover(recover);
g.`,
      line: span.start.line,
      column: span.start.column,
    });
  }

  return diags;
}

function hasSafeRecoverAction(body: Stmt[]): boolean {
  // Description:
  //     HasSafeRecoverAction.
  //
  // Inputs:
  //     body: Stmt[]
  //         Caller-supplied body.
  //
  // Outputs:
  //     result: boolean
  //         Return value from `hasSafeRecoverAction`.
  //
  // Example:

  //     const result = hasSafeRecoverAction(body);

  return body.some(
    (stmt) => stmt.kind === "StopAllActuatorsStmt" || stmt.kind === "EnterModeStmt",
  );
}

export function validateRecover(recover: RecoverDecl): Diagnostic[] {
  // Description:
  //     ValidateRecover.
  //
  // Inputs:
  //     recover: RecoverDecl
  //         Caller-supplied recover.
  //
  // Outputs:
  //     result: Diagnostic[]
  //         Return value from `validateRecover`.
  //
  // Example:

  //     const result = validateRecover(recover);

  const { errorName, body, span } = recover;
  const diags: Diagnostic[] = [];

  if (errorName === "RuntimeError" && !hasSafeRecoverAction(body)) {
  // Description:
  //     TaskPriorityLabel.
  //
  // Inputs:
  //     priority: TaskPriority
  //         Caller-supplied priority.
  //
  // Outputs:
  //     result: string
  //         Return value from `taskPriorityLabel`.
  //
  // Example:

  //     const result = taskPriorityLabel(priority);
  diags.push({
      message:
        "Recovery from RuntimeError should stop actuators or enter degraded mode. Suggestion: add stop_all_actuators() or enter degraded_mode;",
      line: span.start.line,
      column: span.start.column,
    });
  }

  return diags;
}

export function taskPriorityLabel(priority: TaskPriority): string {
  // Description:
  //     TaskPriorityLabel.
  //
  // Inputs:
  //     priority: TaskPriority
  //         Caller-supplied priority.
  //
  // Outputs:
  //     result: string
  //         Return value from `taskPriorityLabel`.
  //
  // Example:

  //     const result = taskPriorityLabel(priority);

  return priority;
}
