/**
 * Trust boundary model for cross-domain secure communication validation.
 * @module
 */

import type { AuthenticationMode, EncryptionMode, IntegrityMode } from "../transport/transport-security.js";

export type TrustBoundaryKind =
  | "robot_internal"
  | "robot_to_robot"
  | "robot_to_cloud"
  | "operator_to_robot";

export function boundaryForTransportName(transport: string): TrustBoundaryKind | null {
  // Description:
  //     BoundaryForTransportName.
  //
  // Inputs:
  //     transport: string
  //         Caller-supplied transport.
  //
  // Outputs:
  //     result: TrustBoundaryKind | null
  //         Return value from `boundaryForTransportName`.
  //
  // Example:
  //     const result = boundaryForTransportName(transport);

  // Map a transport name to the trust boundary it typically crosses.
  switch (transport) {
    case "local":
    case "sim":
    case "ble":
      return "robot_internal";
    case "ros2":
    case "dds":
    case "mqtt":
      return "robot_to_robot";
    case "websocket":
      return "operator_to_robot";
    case "wifi":
    case "cellular":
      return "robot_to_cloud";
    default:
      return null;
  }
}

export function parseTrustBoundary(name: string): TrustBoundaryKind {
  // Description:
  //     ParseTrustBoundary.
  //
  // Inputs:
  //     name: string
  //         Caller-supplied name.
  //
  // Outputs:
  //     result: TrustBoundaryKind
  //         Return value from `parseTrustBoundary`.
  //
  // Example:
  //     const result = parseTrustBoundary(name);
  // Description:
  //     ParseTrustBoundary.
  //
  // Inputs:
  //     name: string
  //         Caller-supplied name.
  //
  // Outputs:
  //     result: TrustBoundaryKind
  //         Return value from `parseTrustBoundary`.
  //
  // Example:

  //     const result = parseTrustBoundary(name);

  switch (name) {
    case "robot_internal":
      return "robot_internal";
    case "robot_to_robot":
      return "robot_to_robot";
    case "robot_to_cloud":
      return "robot_to_cloud";
    case "operator_to_robot":
      return "operator_to_robot";
    default:
      throw new Error(`unknown trust boundary '${name}'`);
  }
}

export function requiredEncryption(boundary: TrustBoundaryKind): EncryptionMode {
  // Description:
  //     RequiredEncryption.
  //
  // Inputs:
  //     boundary: TrustBoundaryKind
  //         Caller-supplied boundary.
  //
  // Outputs:
  //     result: EncryptionMode
  //         Return value from `requiredEncryption`.
  //
  // Example:
  //     const result = requiredEncryption(boundary);

  // Return the minimum encryption mode required for a trust boundary.
  switch (boundary) {
    case "robot_internal":
      return "optional";
    default:
      return "required";
  }
}

export function requiredAuthentication(boundary: TrustBoundaryKind): AuthenticationMode {
  // Description:
  //     RequiredAuthentication.
  //
  // Inputs:
  //     boundary: TrustBoundaryKind
  //         Caller-supplied boundary.
  //
  // Outputs:
  //     result: AuthenticationMode
  //         Return value from `requiredAuthentication`.
  //
  // Example:
  //     const result = requiredAuthentication(boundary);

  // Return the minimum authentication mode required for a trust boundary.
  if (boundary === "operator_to_robot") return "mutual";
  return "none";
}

export function requiresVerifiedActuator(boundary: TrustBoundaryKind): boolean {
  // Description:
  //     RequiresVerifiedActuator.
  //
  // Inputs:
  //     boundary: TrustBoundaryKind
  //         Caller-supplied boundary.
  //
  // Outputs:
  //     result: boolean
  //         Return value from `requiresVerifiedActuator`.
  //
  // Example:
  //     const result = requiresVerifiedActuator(boundary);

  // True when actuator commands crossing this boundary need verified envelopes.
  return boundary !== "robot_internal";
}

/** Registry of declared trust boundaries for compile-time and runtime checks. */
export class TrustBoundaryRegistry {
  private boundaries = new Set<TrustBoundaryKind>();

  declare(boundary: TrustBoundaryKind): void {

    // Register a declared trust boundary on the robot.
    this.boundaries.add(boundary);
  }

  contains(boundary: TrustBoundaryKind): boolean {

    // Return whether the robot declared a specific trust boundary.
    return this.boundaries.has(boundary);
  }

  validateChannel(
    boundary: TrustBoundaryKind,
    encryption: EncryptionMode,
    authentication: AuthenticationMode,
    integrity: IntegrityMode,
    messageType: string,
  ): void {
    // Validate endpoint security against trust boundary requirements.
    //
    // Parameters:
    // - `boundary` — trust boundary being crossed
    // - `encryption` — endpoint encryption mode
    // - `authentication` — endpoint authentication mode
    // - `integrity` — endpoint integrity mode
    // - `messageType` — message type name for error context
    //
    // Returns:
    // Nothing; throws when policy is insufficient.
    //
    // Options:
    // None.
    //
    // Example:

    // registry.validateChannel("robot_to_robot", "none", "none", "none", "Velocity");

    const reqEnc = requiredEncryption(boundary);
    if (reqEnc === "required" && encryption !== "required") {
      throw new Error(
        `encryption required for ${messageType} crossing ${boundary}`,
      );
    }
    const reqAuth = requiredAuthentication(boundary);
    if (reqAuth === "mutual" && authentication !== "mutual") {
      throw new Error(
        `mutual authentication required for ${messageType} crossing ${boundary}`,
      );
    }
    if (
      requiresVerifiedActuator(boundary) &&
      messageType === "SafeAction" &&
      (encryption !== "required" || integrity !== "required")
    ) {
      throw new Error(
        "SafeAction crossing trust boundary requires encryption and integrity",
      );
    }
  }
}
