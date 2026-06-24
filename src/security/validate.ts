/**
 * Static security validation and audit reporting for Spanda programs.
 * @module
 */

import { tokenize } from "../lexer/index.js";
import { parse } from "../parser/index.js";
import type { Program, RobotDecl, TopicDecl } from "../ast/nodes.js";
import type { BusDecl } from "../comm/index.js";
import type { SecretDecl, SecureBlockDecl } from "../foundations.js";
import { isKnownCapability } from "./index.js";
import { parseTrustBoundary, TrustBoundaryRegistry } from "./trust-boundary.js";

export type SecuritySeverity = "error" | "warning" | "info";

export type SecurityFinding = {
  severity: SecuritySeverity;
  message: string;
  line: number;
  column: number;
};

export type SecurityReport = {
  findings: SecurityFinding[];
};

function emptyReport(): SecurityReport {
  // Description:
  //     EmptyReport.
  //
  // Inputs:
  //     None.
  //
  // Outputs:
  //     result: SecurityReport
  //         Return value from `emptyReport`.
  //
  // Example:

  //     const result = emptyReport();

  return { findings: [] };
}

function pushError(report: SecurityReport, message: string, line: number, column: number): void {
  // Description:
  //     PushError.
  //
  // Inputs:
  //     report: SecurityReport
  //         Caller-supplied report.
  //     message: string
  //         Caller-supplied message.
  //     line: number
  //         Caller-supplied line.
  //     column: number
  //         Caller-supplied column.
  //
  // Outputs:
  //     None.
  //
  // Example:

  //     const result = pushError(report, message, line, column);

  report.findings.push({ severity: "error", message, line, column });
}

function pushWarning(report: SecurityReport, message: string, line: number, column: number): void {
  // Description:
  //     PushWarning.
  //
  // Inputs:
  //     report: SecurityReport
  //         Caller-supplied report.
  //     message: string
  //         Caller-supplied message.
  //     line: number
  //         Caller-supplied line.
  //     column: number
  //         Caller-supplied column.
  //
  // Outputs:
  //     None.
  //
  // Example:

  //     const result = pushWarning(report, message, line, column);

  report.findings.push({ severity: "warning", message, line, column });
}

function pushInfo(report: SecurityReport, message: string, line: number, column: number): void {
  // Description:
  //     PushInfo.
  //
  // Inputs:
  //     report: SecurityReport
  //         Caller-supplied report.
  //     message: string
  //         Caller-supplied message.
  //     line: number
  //         Caller-supplied line.
  //     column: number
  //         Caller-supplied column.
  //
  // Outputs:
  //     None.
  //
  // Example:

  //     const result = pushInfo(report, message, line, column);

  report.findings.push({ severity: "info", message, line, column });
}

function secretIsCryptoMaterial(secret: SecretDecl): boolean {
  // Description:
  //     SecretIsCryptoMaterial.
  //
  // Inputs:
  //     secret: SecretDecl
  //         Caller-supplied secret.
  //
  // Outputs:
  //     result: boolean
  //         Return value from `secretIsCryptoMaterial`.
  //
  // Example:

  //     const result = secretIsCryptoMaterial(secret);

  return (
    secret.name.includes("key") ||
    secret.name.includes("cert") ||
    secret.source.source === "file"
  );
}

function parseMode(field: string, value: string): boolean {
  // Description:
  //     ParseMode.
  //
  // Inputs:
  //     field: string
  //         Caller-supplied field.
  //     value: string
  //         Caller-supplied value.
  //
  // Outputs:
  //     result: boolean
  //         Return value from `parseMode`.
  //
  // Example:

  //     const result = parseMode(field, value);

  switch (field) {
    case "encryption":
      return value === "none" || value === "optional" || value === "required";
    case "authentication":
      return value === "none" || value === "signed" || value === "mutual";
    case "integrity":
      return value === "none" || value === "required";
    default:
      return true;
  }
}

function validateSecureBlock(
  block: SecureBlockDecl,
  hasIdentity: boolean,
  kind: string,
  line: number,
  column: number,
  report: SecurityReport,
): void {
  // Description:
  //     ValidateSecureBlock.
  //
  // Inputs:
  //     block: SecureBlockDecl
  //         Caller-supplied block.
  //     hasIdentity: boolean
  //         Caller-supplied hasIdentity.
  //     kind: string
  //         Caller-supplied kind.
  //     line: number
  //         Caller-supplied line.
  //     column: number
  //         Caller-supplied column.
  //     report: SecurityReport
  //         Caller-supplied report.
  //
  // Outputs:
  //     None.
  //
  // Example:

  //     const result = validateSecureBlock(block, hasIdentity, kind, line, column, report);

  const needsIdentity =
    block.signed ||
    block.encryption === "required" ||
    block.authentication === "mutual";
  if (needsIdentity && !hasIdentity) {
    pushError(report, `secure ${kind} requires robot identity declaration`, line, column);
  }
  for (const [field, value] of [
    ["encryption", block.encryption],
    ["authentication", block.authentication],
    ["integrity", block.integrity],
  ] as const) {
    if (value && !parseMode(field, value)) {
      pushError(report, `invalid ${field} mode '${value}' in secure block`, line, column);
    }
  }
  for (const cap of block.requires) {
    if (!isKnownCapability(cap)) {
      pushError(report, `unknown capability '${cap}' in secure block`, block.span.start.line, block.span.start.column);
    }
  }
}

function validateTopic(
  topic: TopicDecl,
  hasIdentity: boolean,
  hasKeyOrCert: boolean,
  boundaries: TrustBoundaryRegistry,
  report: SecurityReport,
): void {
  // Description:
  //     ValidateTopic.
  //
  // Inputs:
  //     topic: TopicDecl
  //         Caller-supplied topic.
  //     hasIdentity: boolean
  //         Caller-supplied hasIdentity.
  //     hasKeyOrCert: boolean
  //         Caller-supplied hasKeyOrCert.
  //     boundaries: TrustBoundaryRegistry
  //         Caller-supplied boundaries.
  //     report: SecurityReport
  //         Caller-supplied report.
  //
  // Outputs:
  //     None.
  //
  // Example:

  //     const result = validateTopic(topic, hasIdentity, hasKeyOrCert, boundaries, report);

  if (topic.secure) {
    validateSecureBlock(
      topic.secure,
      hasIdentity,
      "topic",
      topic.span.start.line,
      topic.span.start.column,
      report,
    );
    if (topic.secure.encryption === "required" && !hasKeyOrCert) {
      pushError(
        report,
        `encrypted topic '${topic.name}' requires key or certificate config`,
        topic.span.start.line,
        topic.span.start.column,
      );
    }
    if (topic.secure.trustedSources.length > 0 && topic.secure.rejectUntrusted) {
      pushInfo(
        report,
        `topic '${topic.name}' rejects untrusted actuator sources`,
        topic.span.start.line,
        topic.span.start.column,
      );
    }
  }
  if (
    topic.messageType.includes("SafeAction") &&
    boundaries.contains("robot_to_robot") &&
    topic.secure?.encryption !== "required"
  ) {
    pushError(
      report,
      `SafeAction topic '${topic.name}' over robot-to-robot must require encryption`,
      topic.span.start.line,
      topic.span.start.column,
    );
  }
}

function validateBus(bus: BusDecl, hasKeyOrCert: boolean, report: SecurityReport): void {
  // Description:
  //     ValidateBus.
  //
  // Inputs:
  //     bus: BusDecl
  //         Caller-supplied bus.
  //     hasKeyOrCert: boolean
  //         Caller-supplied hasKeyOrCert.
  //     report: SecurityReport
  //         Caller-supplied report.
  //
  // Outputs:
  //     None.
  //
  // Example:

  //     const result = validateBus(bus, hasKeyOrCert, report);

  if (bus.encryption === "required" && !hasKeyOrCert) {
    pushError(
      report,
      `encrypted bus '${bus.name}' requires key or certificate in secrets`,
      bus.span.start.line,
      bus.span.start.column,
    );
  }
  if (bus.encryption === "required" && bus.transport === "local") {
    pushWarning(
      report,
      `bus '${bus.name}' requires encryption on local transport — OK for dev, not for deployment`,
      bus.span.start.line,
      bus.span.start.column,
    );
  }
}

function analyzeRobot(robot: RobotDecl, report: SecurityReport, audit: boolean): void {
  // Description:
  //     AnalyzeRobot.
  //
  // Inputs:
  //     robot: RobotDecl
  //         Caller-supplied robot.
  //     report: SecurityReport
  //         Caller-supplied report.
  //     audit: boolean
  //         Caller-supplied audit.
  //
  // Outputs:
  //     None.
  //
  // Example:

  //     const result = analyzeRobot(robot, report, audit);

  const boundaries = new TrustBoundaryRegistry();
  for (const tb of robot.trustBoundaries) {
    try {
      boundaries.declare(parseTrustBoundary(tb.name));
    } catch (e) {
      pushError(report, String(e), tb.span.start.line, tb.span.start.column);
    }
  }

  const hasIdentity = robot.identity !== null;
  const hasKeyOrCert = (robot.secrets ?? []).some(secretIsCryptoMaterial);

  if (robot.secureComm) {
    for (const [field, value] of [
      ["encryption", robot.secureComm.encryption],
      ["authentication", robot.secureComm.authentication],
      ["integrity", robot.secureComm.integrity],
    ] as const) {
      if (value && !parseMode(field, value)) {
        pushError(
          report,
          `invalid ${field} mode '${value}' in secure_comm`,
          robot.secureComm.span.start.line,
          robot.secureComm.span.start.column,
        );
      }
    }
    if (audit) {
      pushInfo(report, "encryption policy declared via secure_comm", robot.span.start.line, robot.span.start.column);
    }
  }

  for (const bus of robot.buses) validateBus(bus, hasKeyOrCert, report);
  for (const topic of robot.topics) validateTopic(topic, hasIdentity, hasKeyOrCert, boundaries, report);

  for (const service of robot.services) {
    if (service.secure) {
      validateSecureBlock(
        service.secure,
        hasIdentity,
        "service",
        service.span.start.line,
        service.span.start.column,
        report,
      );
    }
  }

  for (const action of robot.actions) {
    if (action.secure) {
      validateSecureBlock(
        action.secure,
        hasIdentity,
        "action",
        action.span.start.line,
        action.span.start.column,
        report,
      );
      if (
        action.actionType === "SafeAction" &&
        boundaries.contains("robot_to_robot") &&
        action.secure.encryption !== "required"
      ) {
        pushError(
          report,
          "SafeAction crossing robot_to_robot trust boundary requires encryption",
          action.span.start.line,
          action.span.start.column,
        );
      }
    }
  }

  if (robot.permissions) {
    for (const secret of robot.secrets ?? []) {
      if (!robot.permissions.capabilities.includes("secret.read")) {
        pushError(
          report,
          `secret '${secret.name}' used without secret.read capability in permissions`,
          secret.span.start.line,
          secret.span.start.column,
        );
      }
    }
  } else if ((robot.secrets ?? []).length > 0) {
    for (const secret of robot.secrets ?? []) {
      pushError(
        report,
        `secret '${secret.name}' declared without secret.read capability`,
        secret.span.start.line,
        secret.span.start.column,
      );
    }
  }

  if (audit && robot.identity?.fields.some(([k]) => k === "cert")) {
    pushInfo(report, "certificate-backed identity configured", robot.span.start.line, robot.span.start.column);
  }
}

export function analyzeProgram(program: Program, audit = false): SecurityReport {
  // Description:
  //     AnalyzeProgram.
  //
  // Inputs:
  //     program: Program
  //         Caller-supplied program.
  //     audit = false: input value
  //         Caller-supplied audit = false.
  //
  // Outputs:
  //     result: SecurityReport
  //         Return value from `analyzeProgram`.
  //
  // Example:
  //     const result = analyzeProgram(program, audit = false);
  // Description:
  //     AnalyzeProgram.
  //
  // Inputs:
  //     program: Program
  //         Caller-supplied program.
  //     audit = false: input value
  //         Caller-supplied audit = false.
  //
  // Outputs:
  //     result: SecurityReport
  //         Return value from `analyzeProgram`.
  //
  // Example:

  //     const result = analyzeProgram(program, audit = false);

  const report = emptyReport();
  for (const robot of program.robots) analyzeRobot(robot, report, audit);
  return report;
}

export function securityCheck(source: string): SecurityReport {
  // Description:
  //     SecurityCheck.
  //
  // Inputs:
  //     source: string
  //         Caller-supplied source.
  //
  // Outputs:
  //     result: SecurityReport
  //         Return value from `securityCheck`.
  //
  // Example:
  //     const result = securityCheck(source);

  // Run static security validation on Spanda source text.
  const tokens = tokenize(source);
  const program = parse(tokens);
  return analyzeProgram(program, false);
}

export function securityAudit(source: string): SecurityReport {
  // Description:
  //     SecurityAudit.
  //
  // Inputs:
  //     source: string
  //         Caller-supplied source.
  //
  // Outputs:
  //     result: SecurityReport
  //         Return value from `securityAudit`.
  //
  // Example:
  //     const result = securityAudit(source);

  // Produce an audit-oriented security report including informational events.
  const tokens = tokenize(source);
  const program = parse(tokens);
  return analyzeProgram(program, true);
}

export function reportHasErrors(report: SecurityReport): boolean {
  // Description:
  //     ReportHasErrors.
  //
  // Inputs:
  //     report: SecurityReport
  //         Caller-supplied report.
  //
  // Outputs:
  //     result: boolean
  //         Return value from `reportHasErrors`.
  //
  // Example:

  //     const result = reportHasErrors(report);

  return report.findings.some((f) => f.severity === "error");
}
