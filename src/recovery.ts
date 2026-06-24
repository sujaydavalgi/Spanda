/**
 * TypeScript self-healing recovery analysis (native CLI fallback).
 * @module
 */

import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";
import type { Program } from "./ast/nodes.js";
import { evaluateReadinessTs, type ReadinessOptions } from "./readiness.js";

export type RecoveryStatus =
  | "Success"
  | "PartialSuccess"
  | "Failed"
  | "Aborted"
  | "Unsafe";

export type RecoveryContext = {
  issue: string;
  diagnosis?: string;
  classification?: string;
  level: number;
};

export type PlannedRecoveryAction = {
  description: string;
  risk: string;
  requiresApproval: boolean;
  order: number;
};

export type RecoveryPlan = {
  name: string;
  failure: string;
  diagnosis: string;
  risk: string;
  actions: PlannedRecoveryAction[];
};

export type RecoveryEvidence = {
  failure: string;
  diagnosis: string;
  plan: string;
  safety_validation: string;
  recovery_actions: string[];
  outcome: string;
  operator_approval: string | null;
  verification: string;
};

export type RecoveryResult = {
  plan: string;
  status: RecoveryStatus;
  executed_actions: string[];
  failed_actions: string[];
  verification_outcome: string;
  evidence: RecoveryEvidence;
};

export type RecoveryReadiness = {
  recovery_ready: boolean;
  risk: string;
  readiness_score: number;
  blockers: string[];
};

export type RecoveryReport = {
  policies: Array<{ name: string; triggers: Array<[string, string[]]> }>;
  plans: RecoveryPlan[];
  results: RecoveryResult[];
  readiness: RecoveryReadiness;
  passed: boolean;
};

export type RecoveryKnowledgeEntry = {
  failure_pattern: string;
  recovery_pattern: string;
  success_rate: number;
  recommendation: string;
};

export type RecoveryKnowledgeBase = {
  entries: RecoveryKnowledgeEntry[];
};

function defaultKnowledgeStorePath(): string {
  // Description:
  //     DefaultKnowledgeStorePath.
  //
  // Inputs:
  //     None.
  //
  // Outputs:
  //     result: string
  //         Return value from `defaultKnowledgeStorePath`.
  //
  // Example:

  //     const result = defaultKnowledgeStorePath();

  return join(process.cwd(), ".spanda", "recovery_knowledge.json");
}

export function loadRecoveryKnowledgeStore(path = defaultKnowledgeStorePath()): RecoveryKnowledgeBase {
  // Description:
  //     LoadRecoveryKnowledgeStore.
  //
  // Inputs:
  //     path = defaultKnowledgeStorePath(): input value
  //         Caller-supplied path = defaultKnowledgeStorePath().
  //
  // Outputs:
  //     result: RecoveryKnowledgeBase
  //         Return value from `loadRecoveryKnowledgeStore`.
  //
  // Example:

  //     const result = loadRecoveryKnowledgeStore(path = defaultKnowledgeStorePath());

  if (!existsSync(path)) return { entries: [] };
  try {
    return JSON.parse(readFileSync(path, "utf-8")) as RecoveryKnowledgeBase;
  } catch {
    return { entries: [] };
  }
}

export function bestKnowledgeEntry(
  kb: RecoveryKnowledgeBase,
  issue: string,
): RecoveryKnowledgeEntry | undefined {
  // Description:
  //     BestKnowledgeEntry.
  //
  // Inputs:
  //     kb: RecoveryKnowledgeBase
  //         Caller-supplied kb.
  //     issue: string
  //         Caller-supplied issue.
  //
  // Outputs:
  //     result: RecoveryKnowledgeEntry | undefined
  //         Return value from `bestKnowledgeEntry`.
  //
  // Example:

  //     const result = bestKnowledgeEntry(kb, issue);

  const lower = issue.toLowerCase();
  return kb.entries
    .filter(
      (e) =>
        lower.includes(e.failure_pattern.toLowerCase()) ||
        e.failure_pattern.toLowerCase().includes(lower),
    )
    .sort((a, b) => b.success_rate - a.success_rate)[0];
}

export function loadMergedRecoveryKnowledge(program: Program): RecoveryKnowledgeBase {
  // Description:
  //     LoadMergedRecoveryKnowledge.
  //
  // Inputs:
  //     program: Program
  //         Caller-supplied program.
  //
  // Outputs:
  //     result: RecoveryKnowledgeBase
  //         Return value from `loadMergedRecoveryKnowledge`.
  //
  // Example:

  //     const result = loadMergedRecoveryKnowledge(program);

  const persisted = loadRecoveryKnowledgeStore();
  const staticEntries: RecoveryKnowledgeEntry[] = [];
  for (const policy of extractPolicies(program)) {
    for (const [condition, actions] of policy.triggers) {
      if (actions[0]) {
        staticEntries.push({
          failure_pattern: condition,
          recovery_pattern: actions[0],
          success_rate: 0.5,
          recommendation: `Policy ${policy.name}`,
        });
      }
    }
  }
  const merged = [...staticEntries];
  for (const entry of persisted.entries) {
    const existing = merged.find((e) => e.failure_pattern === entry.failure_pattern);
    if (existing) {
      existing.success_rate = (existing.success_rate + entry.success_rate) / 2;
      if (entry.success_rate > existing.success_rate) {
        existing.recovery_pattern = entry.recovery_pattern;
        existing.recommendation = entry.recommendation;
      }
    } else {
      merged.push(entry);
    }
  }
  return { entries: merged };
}

export function formatRecoveryKnowledge(kb: RecoveryKnowledgeBase): string {
  // Description:
  //     FormatRecoveryKnowledge.
  //
  // Inputs:
  //     kb: RecoveryKnowledgeBase
  //         Caller-supplied kb.
  //
  // Outputs:
  //     result: string
  //         Return value from `formatRecoveryKnowledge`.
  //
  // Example:

  //     const result = formatRecoveryKnowledge(kb);

  if (kb.entries.length === 0) return "No recovery knowledge entries.\n";
  return kb.entries
    .map(
      (e) =>
        `${e.failure_pattern} -> ${e.recovery_pattern} (${Math.round(e.success_rate * 100)}% success)\n  ${e.recommendation}`,
    )
    .join("\n");
}

function classifyFailure(issue: string): string {
  // Description:
  //     ClassifyFailure.
  //
  // Inputs:
  //     issue: string
  //         Caller-supplied issue.
  //
  // Outputs:
  //     result: string
  //         Return value from `classifyFailure`.
  //
  // Example:

  //     const result = classifyFailure(issue);

  const lower = issue.toLowerCase();
  if (lower.includes("gps") || lower.includes("sensor")) return "SensorFailure";
  if (lower.includes("actuator") || lower.includes("motor")) return "ActuatorFailure";
  if (lower.includes("lte") || lower.includes("wifi") || lower.includes("connect")) {
    return "ConnectivityFailure";
  }
  if (lower.includes("provider")) return "ProviderFailure";
  if (lower.includes("fleet")) return "FleetFailure";
  if (lower.includes("safety")) return "SafetyFailure";
  return "Unknown";
}

function inferDiagnosis(issue: string): string {
  // Description:
  //     InferDiagnosis.
  //
  // Inputs:
  //     issue: string
  //         Caller-supplied issue.
  //
  // Outputs:
  //     result: string
  //         Return value from `inferDiagnosis`.
  //
  // Example:

  //     const result = inferDiagnosis(issue);

  const lower = issue.toLowerCase();
  if (lower.includes("gps")) return "Satellite lock lost";
  if (lower.includes("lidar")) return "Lidar point cloud unavailable";
  if (lower.includes("lte") || lower.includes("wifi")) return "Connectivity link down";
  return "Root cause under investigation";
}

function extractPolicies(program: Program): RecoveryReport["policies"] {
  // Description:
  //     ExtractPolicies.
  //
  // Inputs:
  //     program: Program
  //         Caller-supplied program.
  //
  // Outputs:
  //     result: RecoveryReport["policies"]
  //         Return value from `extractPolicies`.
  //
  // Example:

  //     const result = extractPolicies(program);

  const specs: RecoveryReport["policies"] = [];
  for (const decl of program.recoveryPolicies ?? []) {
    specs.push({
      name: decl.name,
      triggers: decl.branches.map((b) => [b.condition, b.actions]),
    });
  }
  for (const decl of program.mitigations ?? []) {
    specs.push({
      name: decl.name,
      triggers: decl.branches.map((b) => [b.condition, b.actions]),
    });
  }
  return specs;
}

function parseAction(text: string, order: number): PlannedRecoveryAction {
  // Description:
  //     ParseAction.
  //
  // Inputs:
  //     text: string
  //         Caller-supplied text.
  //     order: number
  //         Caller-supplied order.
  //
  // Outputs:
  //     result: PlannedRecoveryAction
  //         Return value from `parseAction`.
  //
  // Example:

  //     const result = parseAction(text, order);

  const lower = text.toLowerCase();
  const risk =
    lower.includes("unsafe") || lower.includes("restart fleet") || lower.includes("open gate")
      ? "High"
      : lower.includes("halt") || lower.includes("emergency")
        ? "Critical"
        : "Low";
  return {
    description: text,
    risk,
    requiresApproval: risk === "High" || risk === "Critical" || lower.includes("resume mission"),
    order,
  };
}

function actionsForIssue(program: Program, issue: string): PlannedRecoveryAction[] {
  // Description:
  //     ActionsForIssue.
  //
  // Inputs:
  //     program: Program
  //         Caller-supplied program.
  //     issue: string
  //         Caller-supplied issue.
  //
  // Outputs:
  //     result: PlannedRecoveryAction[]
  //         Return value from `actionsForIssue`.
  //
  // Example:

  //     const result = actionsForIssue(program, issue);

  const lower = issue.toLowerCase();
  const actions: PlannedRecoveryAction[] = [];
  let order = 0;
  for (const policy of extractPolicies(program)) {
    for (const [condition, policyActions] of policy.triggers) {
      if (lower.includes(condition.toLowerCase())) {
        for (const action of policyActions) {
          order += 1;
          actions.push(parseAction(action, order));
        }
      }
    }
  }
  if (actions.length > 0) return actions;
  const knowledge = loadMergedRecoveryKnowledge(program);
  const entry = bestKnowledgeEntry(knowledge, issue);
  if (entry) {
    return [parseAction(entry.recovery_pattern, 1)];
  }
  if (lower.includes("gps")) {
    return [
      "switch_to visual_odometry",
      "reduce_speed 0.5 m/s",
      "enter degraded_mode",
    ].map((a, i) => parseAction(a, i + 1));
  }
  return [parseAction("enter safe_mode", 1)];
}

function planRecovery(program: Program, context: RecoveryContext): RecoveryPlan {
  // Description:
  //     PlanRecovery.
  //
  // Inputs:
  //     program: Program
  //         Caller-supplied program.
  //     context: RecoveryContext
  //         Caller-supplied context.
  //
  // Outputs:
  //     result: RecoveryPlan
  //         Return value from `planRecovery`.
  //
  // Example:

  //     const result = planRecovery(program, context);

  const diagnosis = context.diagnosis ?? inferDiagnosis(context.issue);
  const actions = actionsForIssue(program, context.issue);
  const risk = actions.some((a) => a.risk === "Critical")
    ? "Critical"
    : actions.some((a) => a.risk === "High")
      ? "High"
      : "Low";
  return {
    name: `recovery_${context.issue.replace(/[. ]/g, "_")}`,
    failure: context.issue,
    diagnosis,
    risk,
    actions,
  };
}

function executePlan(
  program: Program,
  plan: RecoveryPlan,
  options: ReadinessOptions,
): RecoveryResult {
  // Description:
  //     ExecutePlan.
  //
  // Inputs:
  //     program: Program
  //         Caller-supplied program.
  //     plan: RecoveryPlan
  //         Caller-supplied plan.
  //     options: ReadinessOptions
  //         Caller-supplied options.
  //
  // Outputs:
  //     result: RecoveryResult
  //         Return value from `executePlan`.
  //
  // Example:

  //     const result = executePlan(program, plan, options);

  const readiness = evaluateReadinessTs(program, options);
  const executed: string[] = [];
  const failed: string[] = [];
  for (const action of plan.actions) {
    const unsafe = action.description.toLowerCase().includes("unsafe");
    const readinessOk =
      readiness.mission_ready || (action.risk === "Low" && readiness.score.total > 0);
    if (unsafe || !readinessOk) {
      failed.push(action.description);
      continue;
    }
    executed.push(action.description);
  }
  const status: RecoveryStatus =
    failed.length === 0 && executed.length > 0
      ? "Success"
      : executed.length > 0
        ? "PartialSuccess"
        : "Failed";
  const evidence: RecoveryEvidence = {
    failure: plan.failure,
    diagnosis: plan.diagnosis,
    plan: plan.name,
    safety_validation: plan.actions.every((a) => !a.description.toLowerCase().includes("unsafe"))
      ? "PASS"
      : "FAIL",
    recovery_actions: executed,
    outcome: status,
    operator_approval: null,
    verification: status === "Success" ? "Recovery verified" : "Recovery incomplete",
  };
  return {
    plan: plan.name,
    status,
    executed_actions: executed,
    failed_actions: failed,
    verification_outcome: evidence.verification,
    evidence,
  };
}

function validateOperatingModes(program: Program): boolean {
  // Description:
  //     ValidateOperatingModes.
  //
  // Inputs:
  //     program: Program
  //         Caller-supplied program.
  //
  // Outputs:
  //     result: boolean
  //         Return value from `validateOperatingModes`.
  //
  // Example:

  //     const result = validateOperatingModes(program);

  const modes = program.operatingModes ?? [];
  if (modes.length === 0) return true;
  const hasSafe = modes.some((m) => /safe/i.test(m.modeKind));
  const hasDegraded = modes.some((m) => /degraded/i.test(m.modeKind));
  return hasSafe && hasDegraded;
}

export function evaluateRecoveryTs(
  program: Program,
  context?: RecoveryContext,
  options: ReadinessOptions = {},
): RecoveryReport {
  // Description:
  //     EvaluateRecoveryTs.
  //
  // Inputs:
  //     program: Program
  //         Caller-supplied program.
  //     context?: RecoveryContext
  //         Caller-supplied context?.
  //     options: ReadinessOptions = {}
  //         Caller-supplied options.
  //
  // Outputs:
  //     result: RecoveryReport
  //         Return value from `evaluateRecoveryTs`.
  //
  // Example:

  //     const result = evaluateRecoveryTs(program, context?, options);

  const policies = extractPolicies(program);
  const contexts: RecoveryContext[] = context
    ? [context]
    : policies.length > 0
      ? policies.flatMap((p) =>
          p.triggers.map(([cond]) => ({
            issue: cond,
            level: 2,
          })),
        )
      : [{ issue: "gps.failed", diagnosis: "Satellite lock lost", level: 2 }];
  const plans = contexts.map((ctx) => planRecovery(program, ctx));
  const results = plans.map((plan) => executePlan(program, plan, options));
  const readinessScore = evaluateReadinessTs(program, options).score.total;
  const passed =
    plans.length > 0 &&
    results.every((r) => r.status !== "Unsafe" && r.status !== "Failed") &&
    validateOperatingModes(program);
  return {
    policies,
    plans,
    results,
    readiness: {
      recovery_ready: passed,
      risk: plans[0]?.risk ?? "Unknown",
      readiness_score: readinessScore,
      blockers: passed ? [] : ["One or more recovery actions failed validation"],
    },
    passed,
  };
}

export function simulateFailureRecoveryTs(
  program: Program,
  failureKind: string,
  options: ReadinessOptions = {},
): RecoveryReport {
  // Description:
  //     SimulateFailureRecoveryTs.
  //
  // Inputs:
  //     program: Program
  //         Caller-supplied program.
  //     failureKind: string
  //         Caller-supplied failureKind.
  //     options: ReadinessOptions = {}
  //         Caller-supplied options.
  //
  // Outputs:
  //     result: RecoveryReport
  //         Return value from `simulateFailureRecoveryTs`.
  //
  // Example:

  //     const result = simulateFailureRecoveryTs(program, failureKind, options);

  return evaluateRecoveryTs(
    program,
    {
      issue: `${failureKind} failure`,
      diagnosis: inferDiagnosis(failureKind),
      classification: classifyFailure(failureKind),
      level: 3,
    },
    options,
  );
}

export function formatRecoveryReport(report: RecoveryReport): string {
  // Description:
  //     FormatRecoveryReport.
  //
  // Inputs:
  //     report: RecoveryReport
  //         Caller-supplied report.
  //
  // Outputs:
  //     result: string
  //         Return value from `formatRecoveryReport`.
  //
  // Example:

  //     const result = formatRecoveryReport(report);

  const plan = report.plans[0];
  const result = report.results[0];
  if (!plan || !result) {
    return `Recovery Ready: ${report.readiness.recovery_ready ? "YES" : "NO"}\n`;
  }
  return [
    `Issue:\n${plan.failure}\n`,
    `Diagnosis:\n${plan.diagnosis}\n`,
    `Recovery:\n${plan.actions[0]?.description ?? "none"}\n`,
    `Risk:\n${plan.risk}\n`,
    `Safety Validation:\n${result.evidence.safety_validation}\n`,
    `Outcome:\n${result.status}`,
    `Recovery Ready: ${report.readiness.recovery_ready ? "YES" : "NO"}`,
  ].join("\n");
}

/** Relay a fleet recovery command through the mesh when `SPANDA_FLEET_MESH_URL` is set. */
export async function coordinateFleetRecoveryViaMesh(
  action: string,
  options: {
    fleetName?: string;
    fromRobot?: string;
    members?: string[];
  } = {},
): Promise<{
  // Description:
  //     CoordinateFleetRecoveryViaMesh.
  //
  // Inputs:
  //     action: string
  //         Caller-supplied action.
  //     options: { fleetName?: string; fromRobot?: string; members?: string[]; } = {}
  //         Caller-supplied options.
  //
  // Outputs:
  //     result: Promise<
  //         Return value from `coordinateFleetRecoveryViaMesh`.
  //
  // Example:
  //     const result = coordinateFleetRecoveryViaMesh(action, options);

  // Description:
  //     CoordinateFleetRecoveryViaMesh.
  //
  // Inputs:
  //     action: string
  //         Caller-supplied action.
    // options: {
    fleetName?: string;
    fromRobot?: string;
    members?: string[];
  } = {}
  //         Caller-supplied options.
  //
  // Outputs:
  //     result: Promise<
  //         Return value from `coordinateFleetRecoveryViaMesh`.
  //
  // Example:

 // const result = coordinateFleetRecoveryViaMesh(action, options);
 relayed: number; failed: number } | null> {
  const meshUrl = process.env.SPANDA_FLEET_MESH_URL;
  if (!meshUrl) return null;
  const { relayRecoveryViaMesh } = await import("./fleet-mesh.js");
  const token = process.env.SPANDA_FLEET_MESH_TOKEN;
  const response = await relayRecoveryViaMesh(
    meshUrl,
    {
      action,
      fleet_name: options.fleetName,
      from_robot: options.fromRobot,
      members: options.members,
    },
    token,
  );
  return { relayed: response.relayed, failed: response.failed };
}
