/**
 * Signed OTA deploy artifact bundles.
 * @module
 */

import type { DeployAssignment, DeployPlan } from "./deploy-service.js";

export type DeployArtifactBundle = {
  version: string;
  program: string;
  programHash?: string;
  assignments: DeployAssignment[];
  certifications: string[];
  signature?: string;
  publicKey?: string;
};

type BundleCanonicalBody = {
  version: string;
  program: string;
  program_hash?: string;
  assignments: Array<{ robot_name: string; hardware: string }>;
  certifications: string[];
};

function canonicalBody(bundle: DeployArtifactBundle): BundleCanonicalBody {
  // Description:
  //     CanonicalBody.
  //
  // Inputs:
  //     bundle: DeployArtifactBundle
  //         Caller-supplied bundle.
  //
  // Outputs:
  //     result: BundleCanonicalBody
  //         Return value from `canonicalBody`.
  //
  // Example:

  //     const result = canonicalBody(bundle);

  return {
    version: bundle.version,
    program: bundle.program,
    program_hash: bundle.programHash,
    assignments: bundle.assignments.map((assignment) => ({
      robot_name: assignment.robotName,
      hardware: assignment.hardware,
    })),
    certifications: bundle.certifications,
  };
}

export function buildDeployBundle(plan: DeployPlan): DeployArtifactBundle {
  // Description:
  //     BuildDeployBundle.
  //
  // Inputs:
  //     plan: DeployPlan
  //         Caller-supplied plan.
  //
  // Outputs:
  //     result: DeployArtifactBundle
  //         Return value from `buildDeployBundle`.
  //
  // Example:
  //     const result = buildDeployBundle(plan);

  // Materialize the rollout manifest fields from a deploy plan.
  return {
    version: plan.version,
    program: plan.program,
    programHash: plan.programHash,
    assignments: plan.assignments,
    certifications: plan.certifications,
    signature: undefined,
    publicKey: undefined,
  };
}

export function bundleCanonicalJson(bundle: DeployArtifactBundle): string {
  // Description:
  //     BundleCanonicalJson.
  //
  // Inputs:
  //     bundle: DeployArtifactBundle
  //         Caller-supplied bundle.
  //
  // Outputs:
  //     result: string
  //         Return value from `bundleCanonicalJson`.
  //
  // Example:

  //     const result = bundleCanonicalJson(bundle);

  return JSON.stringify(canonicalBody(bundle));
}

export async function signDeployBundle(
  bundle: DeployArtifactBundle,
  keyMaterial: string,
): Promise<DeployArtifactBundle> {
  // Description:
  //     SignDeployBundle.
  //
  // Inputs:
  //     bundle: DeployArtifactBundle
  //         Caller-supplied bundle.
  //     keyMaterial: string
  //         Caller-supplied keyMaterial.
  //
  // Outputs:
  //     result: Promise<DeployArtifactBundle>
  //         Return value from `signDeployBundle`.
  //
  // Example:

  //     const result = signDeployBundle(bundle, keyMaterial);

  const { sign, publicKeyFromMaterial } = await import("./security/index.js");
  const canonical = bundleCanonicalJson(bundle);
  return {
    ...bundle,
    publicKey: publicKeyFromMaterial(keyMaterial),
    signature: sign(canonical, keyMaterial),
  };
}

export async function verifyDeployBundle(
  bundle: DeployArtifactBundle,
  keyMaterial: string,
): Promise<boolean> {
  // Description:
  //     VerifyDeployBundle.
  //
  // Inputs:
  //     bundle: DeployArtifactBundle
  //         Caller-supplied bundle.
  //     keyMaterial: string
  //         Caller-supplied keyMaterial.
  //
  // Outputs:
  //     result: Promise<boolean>
  //         Return value from `verifyDeployBundle`.
  //
  // Example:

  //     const result = verifyDeployBundle(bundle, keyMaterial);

  if (!bundle.signature) return false;
  const { verifySignature } = await import("./security/index.js");
  return verifySignature(bundleCanonicalJson(bundle), bundle.signature, keyMaterial);
}
