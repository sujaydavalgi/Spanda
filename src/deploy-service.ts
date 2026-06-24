/**
 * OTA deployment planning, rollout, rollback, and state tracking for Spanda programs.
 * @module
 */

import { createHash } from "node:crypto";
import { readFileSync, existsSync } from "node:fs";
import type { Program } from "./ast/nodes.js";
import {
  buildCertificationProofSummary,
  type CertificationProofSummary,
} from "./certify-prover.js";

export type RolloutStrategy = "all" | "canary" | "staged";

export type DeployAssignment = {
  robotName: string;
  hardware: string;
};

export type DeployPlan = {
  program: string;
  version: string;
  programHash?: string;
  assignments: DeployAssignment[];
  certifications: string[];
  certificationProof?: CertificationProofSummary;
};

export type RolloutStepStatus = "pending" | "deployed" | "rolled_back" | "skipped" | "failed";

export type RolloutStep = {
  robotName: string;
  hardware: string;
  status: RolloutStepStatus;
  version: string;
  phasePercent: number | null;
};

export type RolloutResult = {
  strategy: RolloutStrategy;
  version: string;
  dryRun: boolean;
  steps: RolloutStep[];
  success: boolean;
};

export type DeployState = {
  currentVersion: Record<string, string>;
  previousVersion: Record<string, string>;
  history: RolloutResult[];
};

export type RolloutOptions = {
  strategy: RolloutStrategy;
  canaryPercent: number;
  stagedPhases: number[];
  version: string;
  dryRun: boolean;
  requireCertify: boolean;
};

export const defaultRolloutOptions = (): RolloutOptions => ({
  strategy: "all",
  canaryPercent: 10,
  stagedPhases: [10, 50, 100],
  version: "1.0.0",
  dryRun: false,
  requireCertify: false,
});

function assignmentKey(robot: string, hardware: string): string {
  // Description:
  //     AssignmentKey.
  //
  // Inputs:
  //     robot: string
  //         Caller-supplied robot.
  //     hardware: string
  //         Caller-supplied hardware.
  //
  // Outputs:
  //     result: string
  //         Return value from `assignmentKey`.
  //
  // Example:

  //     const result = assignmentKey(robot, hardware);

  return `${robot}@${hardware}`;
}

export function deployTargetKey(robot: string, hardware: string): string {
  // Description:
  //     DeployTargetKey.
  //
  // Inputs:
  //     robot: string
  //         Caller-supplied robot.
  //     hardware: string
  //         Caller-supplied hardware.
  //
  // Outputs:
  //     result: string
  //         Return value from `deployTargetKey`.
  //
  // Example:

  //     const result = deployTargetKey(robot, hardware);

  return assignmentKey(robot, hardware);
}

export function hashProgramArtifact(programPath: string): string | undefined {
  // Description:
  //     HashProgramArtifact.
  //
  // Inputs:
  //     programPath: string
  //         Caller-supplied programPath.
  //
  // Outputs:
  //     result: string | undefined
  //         Return value from `hashProgramArtifact`.
  //
  // Example:
  //     const result = hashProgramArtifact(programPath);

  // Hash the deployment source file when it exists locally.
  if (!existsSync(programPath)) return undefined;
  const bytes = readFileSync(programPath);
  return createHash("sha256").update(bytes).digest("hex");
}

export function buildDeployPlan(program: Program, programPath: string, version: string): DeployPlan {
  // Description:
  //     BuildDeployPlan.
  //
  // Inputs:
  //     program: Program
  //         Caller-supplied program.
  //     programPath: string
  //         Caller-supplied programPath.
  //     version: string
  //         Caller-supplied version.
  //
  // Outputs:
  //     result: DeployPlan
  //         Return value from `buildDeployPlan`.
  //
  // Example:
  //     const result = buildDeployPlan(program, programPath, version);

  // Extract deploy targets and certification metadata from the program AST.
  const assignments: DeployAssignment[] = [];
  for (const deploy of program.deployments) {
    for (const hardware of deploy.targets) {
      assignments.push({ robotName: deploy.robotName, hardware });
    }
  }
  assignments.sort((a, b) =>
    a.robotName.localeCompare(b.robotName) || a.hardware.localeCompare(b.hardware),
  );
  const certifications = (program.certifications ?? []).map((cert) =>
    cert.level ? `${cert.standard}:${cert.level}` : cert.standard,
  );
  return {
    program: programPath,
    version,
    programHash: hashProgramArtifact(programPath),
    assignments,
    certifications,
    certificationProof: buildCertificationProofSummary(program, programPath),
  };
}

export function validateRolloutCertification(
  plan: DeployPlan,
  options: RolloutOptions,
): string | undefined {
  // Description:
  //     ValidateRolloutCertification.
  //
  // Inputs:
  //     plan: DeployPlan
  //         Caller-supplied plan.
  //     options: RolloutOptions
  //         Caller-supplied options.
  //
  // Outputs:
  //     result: string | undefined
  //         Return value from `validateRolloutCertification`.
  //
  // Example:
  //     const result = validateRolloutCertification(plan, options);

  // Enforce strict certification proof before OTA rollout proceeds.
  if (!options.requireCertify) return undefined;
  const proof = plan.certificationProof;
  if (!proof) return "Deploy plan missing certification proof summary";
  if (!proof.passedStrict) {
    return `Deploy blocked — strict certification proof failed: ${proof.summary}`;
  }
  return undefined;
}

export function planRollout(plan: DeployPlan, options: RolloutOptions): RolloutResult {
  // Description:
  //     PlanRollout.
  //
  // Inputs:
  //     plan: DeployPlan
  //         Caller-supplied plan.
  //     options: RolloutOptions
  //         Caller-supplied options.
  //
  // Outputs:
  //     result: RolloutResult
  //         Return value from `planRollout`.
  //
  // Example:

  //     const result = planRollout(plan, options);

  const certifyError = validateRolloutCertification(plan, options);
  if (certifyError) {
    return {
      strategy: options.strategy,
      version: options.version,
      dryRun: options.dryRun,
      steps: [],
      success: false,
    };
  }
  const total = plan.assignments.length;
  const steps: RolloutStep[] = [];
  if (total === 0) {
    return {
      strategy: options.strategy,
      version: options.version,
      dryRun: options.dryRun,
      steps,
      success: true,
    };
  }

  const statusFor = (deploy: boolean): RolloutStepStatus => {
    // Description:
    //     StatusFor.
    //
    // Inputs:
    //     deploy: boolean
    //         Caller-supplied deploy.
    //
    // Outputs:
    //     result: RolloutStepStatus
    //         Return value from `statusFor`.
    //
    // Example:

    //     const result = statusFor(deploy);

    if (!deploy) return "skipped";
    return options.dryRun ? "pending" : "deployed";
  };

  if (options.strategy === "all") {
    for (const a of plan.assignments) {
      steps.push({
        robotName: a.robotName,
        hardware: a.hardware,
        status: statusFor(true),
        version: options.version,
        phasePercent: 100,
      });
    }
  } else if (options.strategy === "canary") {
    const pct = Math.min(100, Math.max(1, options.canaryPercent));
    const canaryCount = Math.max(1, Math.ceil((total * pct) / 100));
    plan.assignments.forEach((a, idx) => {
      steps.push({
        robotName: a.robotName,
        hardware: a.hardware,
        status: statusFor(idx < canaryCount),
        version: options.version,
        phasePercent: idx < canaryCount ? pct : 0,
      });
    });
  } else {
    const phases = options.stagedPhases.length > 0 ? options.stagedPhases : [100];
    const finalPhase = phases[phases.length - 1] ?? 100;
    const deployCount = Math.max(1, Math.ceil((total * finalPhase) / 100));
    plan.assignments.forEach((a, idx) => {
      steps.push({
        robotName: a.robotName,
        hardware: a.hardware,
        status: statusFor(idx < deployCount),
        version: options.version,
        phasePerce
  // Description:
  //     ApplyRollout.
  //
  // Inputs:
  //     state: DeployState
  //         Caller-supplied state.
  //     result: RolloutResult
  //         Caller-supplied result.
  //
  // Outputs:
  //     None.
  //
  // Example:

// const result = applyRollout(state, result);
nt: idx < deployCount ? finalPhase : 0,
      });
    });
  }

  return {
    strategy: options.strategy,
    version: options.version,
    dryRun: options.dryRun,
    steps,
    success: !steps.some((s) => s.status === "failed"),
  };
}

export function applyRollout(state: DeployState, result: RolloutResult): void {
  // Description:
  //     ApplyRollout.
  //
  // Inputs:
  //     state: DeployState
  //         Caller-supplied state.
  //     result: RolloutResult
  //         Caller-supplied result.
  //
  // Outputs:
  //     None.
  //
  // Example:

  //     const result = applyRollout(state, result);

  if (result.dryRun) return;
  for (const step of result.steps) {
    if (step.status !== "deployed") continue;
    co
  // Description:
  //     RollbackTargets.
  //
  // Inputs:
  //     state: DeployState
  //         Caller-supplied state.
  //     plan: DeployPlan
  //         Caller-supplied plan.
  //
  // Outputs:
  //     result: RolloutResult
  //         Return value from `rollbackTargets`.
  //
  // Example:

// const result = rollbackTargets(state, plan);
nst key = assignmentKey(step.robotName, step.hardware);
    const prev = state.currentVersion[key];
    if (prev) state.previousVersion[key] = prev;
    state.currentVersion[key] = step.version;
  }
  state.history.push(result);
}

export function rollbackTargets(state: DeployState, plan: DeployPlan): RolloutResult {
  // Description:
  //     RollbackTargets.
  //
  // Inputs:
  //     state: DeployState
  //         Caller-supplied state.
  //     plan: DeployPlan
  //         Caller-supplied plan.
  //
  // Outputs:
  //     result: RolloutResult
  //         Return value from `rollbackTargets`.
  //
  // Example:

  //     const result = rollbackTargets(state, plan);

  const steps: RolloutStep[] = [];
  for (const a of plan.assignments) {
    const key = assignmentKey(a.robotName, a.hardware);
    const targetVersion = state.previousVersion[key];
    if (targetVersion) {
      const current = state.currentVersion[key];
      if (current) state.previousVersion[key] = current;
      state.currentVersion[key] = targetVersion;
      steps.push({
        robotName: a.robotName,
        hardware: a.hardware,
        status: "rolled_back",
        version: targetVersion,
        phasePercent: null,
      });
    } else {
      steps.push({
        robotName: a.robotName,
        hardware: a.hardware,
        status: "skipped",
        version: "unknown",
  // Description:
  //     DefaultStatePath.
  //
  // Inputs:
  //     None.
  //
  // Outputs:
  //     result: string
  //         Return value from `defaultStatePath`.
  //
  // Example:

        // const result = defaultStatePath();

        phasePercent: null,
      });
    }
  }
  const result: RolloutResult = {
    stra
  // Description:
  //     EmptyDeployState.
  //
  // Inputs:
  //     None.
  //
  // Outputs:
  //     result: DeployState
  //         Return value from `emptyDeployState`.
  //
  // Example:

// const result = emptyDeployState();
tegy: "all",
    version: "rollback",
    dryRun: false,
    steps,
    success: steps.some((s) => s.status === "rolled_back"),
  };
  sta
  // Description:
  //     LoadDeployState.
  //
  // Inputs:
  //     text: string | null
  //         Caller-supplied text.
  //
  // Outputs:
  //     result: DeployState
  //         Return value from `loadDeployState`.
  //
  // Example:

// const result = loadDeployState(text);
te.history.push(result);
  return result;
}

export function defaultStatePath(): string {
  // Description:
  //     DefaultStatePath.
  //
  // Inputs:
  //     None.
  //
  // Outputs:
  //     result: string
  //         Return value from `defaultStatePath`.
  //
  // Example:

  //     const result = defaultStatePath();

  return ".spanda/deploy-state.json";
}

export function emptyDeployState(): DeployState {
  // Description:
  //     EmptyDeployState.
  //
  // Inputs:
  //     None.
  //
  // Outputs:
  //     result: DeployState
  //         Return value from `emptyDeployState`.
  //
  // Example:

  //     const result = emptyDeployState();

  return { currentVersion: {}, previousVersion: {}, history: [] };
}

export function loadDeployState(text: string | null): DeployState {
  // Description:
  //     LoadDeployState.
  //
  // Inputs:
  //     text: string | null
  //         Caller-supplied text.
  //
  // Outputs:
  //     result: DeployState
  //         Return value from `loadDeployState`.
  //
  // Example:

  //     const result = loadDeployState(text);

  if (!text) return emptyDeployState();
  try {
    const p
  // Description:
  //     SerializeDeployState.
  //
  // Inputs:
  //     state: DeployState
  //         Caller-supplied state.
  //
  // Outputs:
  //     result: string
  //         Return value from `serializeDeployState`.
  //
  // Example:

// const result = serializeDeployState(state);
arsed = JSON.parse(text) as DeployState;
    return {
      currentVersion: parsed.currentVersion ?? {},
      previousVersion: parsed.previousVersion ?? {},
      history: parsed.history ?? [],
    };
  } catch {
    return emptyDeployState();
  }
}

export function serializeDeployState(state: DeployState): string {
  // Description:
  //     SerializeDeployState.
  //
  // Inputs:
  //     state: DeployState
  //         Caller-supplied state.
  //
  // Outputs:
  //     result: string
  //         Return value from `serializeDeployState`.
  //
  // Example:

  //     const result = serializeDeployState(state);

  return JSON.stringify(state, null, 2);
}
