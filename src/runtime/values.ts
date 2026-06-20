/**
 * values module (runtime/values.ts).
 * @module
 */

import type { RuntimeValue } from "../runtime/interpreter.js";

export type { RuntimeValue } from "../runtime/interpreter.js";
import type { SafetyZoneRuntime } from "../safety/index.js";

export type PoseValue = { x: number; y: number; theta: number; z: number };
export type VelocityValue = { linear: number; angular: number };

export function runtimePose(x = 0, y = 0, theta = 0, z = 0): RuntimeValue {
  // RuntimePose.
  //
  // Parameters:
  // - `x` — optional input
  // - `y` — optional input
  // - `theta` — optional input
  // - `z` — optional input
  //
  // Returns:
  // `RuntimeValue`.
  //
  // Options:
  // - `x` — optional parameter
  // - `y` — optional parameter
  // - `theta` — optional parameter
  // - `z` — optional parameter
  //
  // Example:
  // const result = runtimePose(x, y, theta, z);

  return { kind: "pose", x, y, theta, z };
}

export function runtimeVelocity(linear = 0, angular = 0): RuntimeValue {
  // RuntimeVelocity.
  //
  // Parameters:
  // - `linear` — optional input
  // - `angular` — optional input
  //
  // Returns:
  // `RuntimeValue`.
  //
  // Options:
  // - `linear` — optional parameter
  // - `angular` — optional parameter
  //
  // Example:
  // const result = runtimeVelocity(linear, angular);

  return { kind: "velocity", linear, angular };
}

export function runtimeTrajectory(waypoints: PoseValue[]): RuntimeValue {
  // RuntimeTrajectory.
  //
  // Parameters:
  // - `waypoints` — input value
  //
  // Returns:
  // `RuntimeValue`.
  //
  // Options:
  // None.
  //
  // Example:
  // const result = runtimeTrajectory(waypoints);

  return { kind: "trajectory", waypoints };
}

export function poseFromState(state: { x: number; y: number; theta: number; z?: number }): RuntimeValue {
  // PoseFromState.
  //
  // Parameters:
  // - `state` — optional input
  //
  // Returns:
  // `RuntimeValue`.
  //
  // Options:
  // - `state` — optional parameter
  //
  // Example:
  // const result = poseFromState(state);

  return runtimePose(state.x, state.y, state.theta, state.z ?? 0);
}

export function velocityFromState(state: { linear: number; angular: number }): RuntimeValue {
  // VelocityFromState.
  //
  // Parameters:
  // - `state` — input value
  //
  // Returns:
  // `RuntimeValue`.
  //
  // Options:
  // None.
  //
  // Example:
  // const result = velocityFromState(state);

  return runtimeVelocity(state.linear, state.angular);
}

export function getPoseFields(val: RuntimeValue): PoseValue | null {
  // GetPoseFields.
  //
  // Parameters:
  // - `val` — input value
  //
  // Returns:
  // `Some` / non-null value on success, otherwise `None` / null.
  //
  // Options:
  // None.
  //
  // Example:
  // const result = getPoseFields(val);

  if (val.kind !== "pose") return null;
  return { x: val.x, y: val.y, theta: val.theta, z: val.z };
}

export function getVelocityFields(val: RuntimeValue): VelocityValue | null {
  // GetVelocityFields.
  //
  // Parameters:
  // - `val` — input value
  //
  // Returns:
  // `Some` / non-null value on success, otherwise `None` / null.
  //
  // Options:
  // None.
  //
  // Example:
  // const result = getVelocityFields(val);

  if (val.kind !== "velocity") return null;
  return { linear: val.linear, angular: val.angular };
}

export function getTrajectoryWaypoints(val: RuntimeValue): PoseValue[] | null {
  // GetTrajectoryWaypoints.
  //
  // Parameters:
  // - `val` — input value
  //
  // Returns:
  // `Some` / non-null value on success, otherwise `None` / null.
  //
  // Options:
  // None.
  //
  // Example:
  // const result = getTrajectoryWaypoints(val);

  if (val.kind !== "trajectory") return null;
  return val.waypoints;
}

export function getNumber(val: RuntimeValue, defaultVal = 0): number {
  // GetNumber.
  //
  // Parameters:
  // - `val` — input value
  // - `defaultVal` — optional input
  //
  // Returns:
  // Numeric result.
  //
  // Options:
  // - `defaultVal` — optional parameter
  //
  // Example:
  // const result = getNumber(val, defaultVal);

  return val.kind === "number" ? val.value : defaultVal;
}

export function getString(val: RuntimeValue, defaultVal = ""): string {
  // GetString.
  //
  // Parameters:
  // - `val` — input value
  // - `defaultVal` — optional input
  //
  // Returns:
  // Text result.
  //
  // Options:
  // - `defaultVal` — optional parameter
  //
  // Example:
  // const result = getString(val, defaultVal);

  return val.kind === "string" ? val.value : defaultVal;
}

export type TopicBinding = {
  name: string;
  messageType: string;
  topic: string;
};

export type ServiceBinding = {
  name: string;
  serviceType: string;
};

export type ActionBinding = {
  name: string;
  actionType: string;
};

export type ZoneBinding = SafetyZoneRuntime;
