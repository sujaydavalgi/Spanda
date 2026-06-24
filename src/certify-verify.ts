/**
 * Certification proof checklist for hardware verify.
 * @module
 */

import type { Program } from "./ast/nodes.js";
import type { CompatItem } from "./rust-bridge.js";

function pass(category: string, message: string, line: number, column: number): CompatItem {
  // Description:
  //     Pass.
  //
  // Inputs:
  //     category: string
  //         Caller-supplied category.
  //     message: string
  //         Caller-supplied message.
  //     line: number
  //         Caller-supplied line.
  //     column: number
  //         Caller-supplied column.
  //
  // Outputs:
  //     result: CompatItem
  //         Return value from `pass`.
  //
  // Example:

  //     const result = pass(category, message, line, column);

  return { category, message, severity: "pass", line, column };
}

function warn(category: string, message: string, line: number, column: number): CompatItem {
  // Description:
  //     Warn.
  //
  // Inputs:
  //     category: string
  //         Caller-supplied category.
  //     message: string
  //         Caller-supplied message.
  //     line: number
  //         Caller-supplied line.
  //     column: number
  //         Caller-supplied column.
  //
  // Outputs:
  //     result: CompatItem
  //         Return value from `warn`.
  //
  // Example:

  //     const result = warn(category, message, line, column);

  return { category, message, severity: "warning", line, column };
}

function error(category: string, message: string, line: number, column: number): CompatItem {
  // Description:
  //     Error.
  //
  // Inputs:
  //     category: string
  //         Caller-supplied category.
  //     message: string
  //         Caller-supplied message.
  //     line: number
  //         Caller-supplied line.
  //     column: number
  //         Caller-supplied column.
  //
  // Outputs:
  //     result: CompatItem
  //         Return value from `error`.
  //
  // Example:

  //     const result = error(category, message, line, column);

  return { category, message, severity: "error", line, column };
}

export function verifyCertificationProof(program: Program, strict: boolean): CompatItem[] {
  // Description:
  //     VerifyCertificationProof.
  //
  // Inputs:
  //     program: Program
  //         Caller-supplied program.
  //     strict: boolean
  //         Caller-supplied strict.
  //
  // Outputs:
  //     result: CompatItem[]
  //         Return value from `verifyCertificationProof`.
  //
  // Example:
  //     const result = verifyCertificationProof(program, strict);

  // Evaluate deploy/certify/safety/mission coverage for CI gates.
  const items: CompatItem[] = [];
  const hasDeploy = program.deployments.length > 0;
  const hasCertify = (program.certifications ?? []).length > 0;

  if (hasDeploy && !hasCertify) {
    items.push(
      strict
        ? error(
            "certify",
            "Deploy targets require certification metadata — add certify ISO13849 (or IEC61508 / ISO26262)",
            1,
            1,
          )
        : warn(
            "certify",
            "Deploy targets declared without certification metadata — add certify ISO13849 (or IEC61508 / ISO26262)",
            1,
            1,
          ),
    );
  }

  if (hasCertify && !hasDeploy) {
    items.push(
      pass(
        "certify",
        "Certification metadata recorded — no deploy targets declared",
        1,
        1,
      ),
    );
  }

  for (const cert of program.certifications ?? []) {
    if (strict && cert.standard === "ISO13849" && !cert.level) {
      items.push(
        error(
          "certify",
          "ISO13849 certification should declare a performance level (e.g. PLd) under strict verify",
          cert.span.start.line,
          cert.span.start.column,
        ),
      );
    }
    if (strict && cert.standard === "ISO26262" && !cert.level) {
      items.push(
        warn(
          "certify",
          "ISO26262 certification should declare ASIL level under strict verify",
          cert.span.start.line,
          cert.span.start.column,
        ),
      );
    }
  }

  for (const deploy of program.deployments) {
    const robot = program.robots.find((r) => r.name === deploy.robotName);
    if (!robot) {
      if (strict) {
        items.push(
          error(
            "certify",
            `Deploy robot '${deploy.robotName}' not found for certification proof`,
            deploy.span.start.line,
            deploy.span.start.column,
          ),
        );
      }
      continue;
    }
    if (!robot.mission) {
      items.push(
        strict
          ? error(
              "certify",
              `Deployed robot '${deploy.robotName}' should declare a mission under strict verify`,
              deploy.span.start.line,
              deploy.span.start.column,
            )
          : warn(
              "certify",
              `Deployed robot '${deploy.robotName}' has no mission metadata`,
              deploy.span.start.line,
              deploy.span.start.column,
            ),
      );
    }
    if (!robot.safety) {
      items.push(
        strict
          ? error(
              "certify",
              `Deployed robot '${deploy.robotName}' should declare safety rules under strict verify`,
              deploy.span.start.line,
              deploy.span.start.column,
            )
          : warn(
              "certify",
              `Deployed robot '${deploy.robotName}' has no safety block`,
              deploy.span.start.line,
              deploy.span.start.column,
            ),
      );
    }
    for (const target of deploy.targets) {
      items.push(
        pass(
          "certify",
          `Deploy target ${deploy.robotName}@${target} covered by certification proof checklist`,
          deploy.span.start.line,
          deploy.span.start.column,
        ),
      );
    }
  }

  if (strict && hasDeploy && (program.programSafetyZones ?? []).length === 0) {
    items.push(
      warn(
        "certify",
        "Strict certification verify recommends program-level safety_zone declarations for deployed fleets",
        1,
        1,
      ),
    );
  }

  if (hasCertify && hasDeploy) {
    items.push(
      pass(
        "certify",
        "Certification proof checklist satisfied for declared deploy targets",
        1,
        1,
      ),
    );
  }

  return items;
}
