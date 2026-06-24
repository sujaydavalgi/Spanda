/**
 * Runtime certification gate before executing deploy-target programs.
 * @module
 */

import type { Program } from "./ast/nodes.js";
import { verifyCertificationProof } from "./certify-verify.js";

export function enforceCertificationRuntime(program: Program, strict: boolean): void {
  // Description:
  //     EnforceCertificationRuntime.
  //
  // Inputs:
  //     program: Program
  //         Caller-supplied program.
  //     strict: boolean
  //         Caller-supplied strict.
  //
  // Outputs:
  //     None.
  //
  // Example:
  //     const result = enforceCertificationRuntime(program, strict);

  // Block run/sim when certification proof checklist reports errors.
  if (!strict) return;
  const blocking = verifyCertificationProof(program, true).find((item) => item.severity === "error");
  if (blocking) {
    throw new Error(`certification runtime gate: ${blocking.message}`);
  }
}

export function certificationRuntimeEnabledFromEnv(): boolean {
  // Description:
  //     CertificationRuntimeEnabledFromEnv.
  //
  // Inputs:
  //     None.
  //
  // Outputs:
  //     result: boolean
  //         Return value from `certificationRuntimeEnabledFromEnv`.
  //
  // Example:

  //     const result = certificationRuntimeEnabledFromEnv();

  const value = process.env.SPANDA_ENFORCE_CERTIFY?.toLowerCase();
  return value === "1" || value === "true" || value === "yes";
}
