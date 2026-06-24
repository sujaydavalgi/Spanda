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
  // Description:
  //     RuntimePose.
  //
  // Inputs:
  //     x = 0: input value
  //         Caller-supplied x = 0.
  //     y = 0: input value
  //         Caller-supplied y = 0.
  //     theta = 0: input value
  //         Caller-supplied theta = 0.
  //     z = 0: input value
  //         Caller-supplied z = 0.
  //
  // Outputs:
  //     result: RuntimeValue
  //         Return value from `runtimePose`.
  //
  // Example:
  //     const result = runtimePose(x = 0, y = 0, theta = 0, z = 0);
  // Description:
  //     RuntimePose.
  //
  // Inputs:
  //     x = 0: input value
  //         Caller-supplied x = 0.
  //     y = 0: input value
  //         Caller-supplied y = 0.
  //     theta = 0: input value
  //         Caller-supplied theta = 0.
  //     z = 0: input value
  //         Caller-supplied z = 0.
  //
  // Outputs:
  //     result: RuntimeValue
  //         Return value from `runtimePose`.
  //
  // Example:
  //     const result = runtimePose(x = 0, y = 0, theta = 0, z = 0);

  // const result = runtimePose(x, y, theta, z);
  return { kind: "pose", x, y, theta, z };
}

export function runtimeVelocity(linear = 0, angular = 0): RuntimeValue {
  // Description:
  //     RuntimeVelocity.
  //
  // Inputs:
  //     linear = 0: input value
  //         Caller-supplied linear = 0.
  //     angular = 0: input value
  //         Caller-supplied angular = 0.
  //
  // Outputs:
  //     result: RuntimeValue
  //         Return value from `runtimeVelocity`.
  //
  // Example:
  //     const result = runtimeVelocity(linear = 0, angular = 0);
  // Description:
  //     RuntimeVelocity.
  //
  // Inputs:
  //     linear = 0: input value
  //         Caller-supplied linear = 0.
  //     angular = 0: input value
  //         Caller-supplied angular = 0.
  //
  // Outputs:
  //     result: RuntimeValue
  //         Return value from `runtimeVelocity`.
  //
  // Example:
  //     const result = runtimeVelocity(linear = 0, angular = 0);

  // const result = runtimeVelocity(linear, angular);
  return { kind: "velocity", linear, angular };
}

export function runtimeTrajectory(waypoints: PoseValue[]): RuntimeValue {
  // Description:
  //     RuntimeTrajectory.
  //
  // Inputs:
  //     waypoints: PoseValue[]
  //         Caller-supplied waypoints.
  //
  // Outputs:
  //     result: RuntimeValue
  //         Return value from `runtimeTrajectory`.
  //
  // Example:
  //     const result = runtimeTrajectory(waypoints);
  // Description:
  //     RuntimeTrajectory.
  //
  // Inputs:
  //     waypoints: PoseValue[]
  //         Caller-supplied waypoints.
  //
  // Outputs:
  //     result: RuntimeValue
  //         Return value from `runtimeTrajectory`.
  //
  // Example:
  //     const result = runtimeTrajectory(waypoints);

  // const result = runtimeTrajectory(waypoints);
  return { kind: "trajectory", waypoints };
}

export function poseFromState(state: { x: number; y: number; theta: number; z?: number }): RuntimeValue {
  // Description:
  //     PoseFromState.
  //
  // Inputs:
  //     state: { x: number; y: number; theta: number; z?: number }
  //         Caller-supplied state.
  //
  // Outputs:
  //     result: RuntimeValue
  //         Return value from `poseFromState`.
  //
  // Example:
  //     const result = poseFromState(state);
  // Description:
  //     PoseFromState.
  //
  // Inputs:
  //     state: { x: number; y: number; theta: number; z?: number }
  //         Caller-supplied state.
  //
  // Outputs:
  //     result: RuntimeValue
  //         Return value from `poseFromState`.
  //
  // Example:
  //     const result = poseFromState(state);

  // const result = poseFromState(state);
  return runtimePose(state.x, state.y, state.theta, state.z ?? 0);
}

export function velocityFromState(state: { linear: number; angular: number }): RuntimeValue {
  // Description:
  //     VelocityFromState.
  //
  // Inputs:
  //     state: { linear: number; angular: number }
  //         Caller-supplied state.
  //
  // Outputs:
  //     result: RuntimeValue
  //         Return value from `velocityFromState`.
  //
  // Example:
  //     const result = velocityFromState(state);
  // Description:
  //     VelocityFromState.
  //
  // Inputs:
  //     state: { linear: number; angular: number }
  //         Caller-supplied state.
  //
  // Outputs:
  //     result: RuntimeValue
  //         Return value from `velocityFromState`.
  //
  // Example:
  //     const result = velocityFromState(state);

  // const result = velocityFromState(state);
  return runtimeVelocity(state.linear, state.angular);
}

export function getPoseFields(val: RuntimeValue): PoseValue | null {
  // Description:
  //     GetPoseFields.
  //
  // Inputs:
  //     val: RuntimeValue
  //         Caller-supplied val.
  //
  // Outputs:
  //     result: PoseValue | null
  //         Return value from `getPoseFields`.
  //
  // Example:
  //     const result = getPoseFields(val);
  // Description:
  //     GetPoseFields.
  //
  // Inputs:
  //     val: RuntimeValue
  //         Caller-supplied val.
  //
  // Outputs:
  //     result: PoseValue | null
  //         Return value from `getPoseFields`.
  //
  // Example:
  //     const result = getPoseFields(val);

  // const result = getPoseFields(val);
  if (val.kind !== "pose") return null;
  return { x: val.x, y: val.y, theta: val.theta, z: val.z };
}

export function getVelocityFields(val: RuntimeValue): VelocityValue | null {
  // Description:
  //     GetVelocityFields.
  //
  // Inputs:
  //     val: RuntimeValue
  //         Caller-supplied val.
  //
  // Outputs:
  //     result: VelocityValue | null
  //         Return value from `getVelocityFields`.
  //
  // Example:
  //     const result = getVelocityFields(val);
  // Description:
  //     GetVelocityFields.
  //
  // Inputs:
  //     val: RuntimeValue
  //         Caller-supplied val.
  //
  // Outputs:
  //     result: VelocityValue | null
  //         Return value from `getVelocityFields`.
  //
  // Example:
  //     const result = getVelocityFields(val);

  // const result = getVelocityFields(val);
  if (val.kind !== "velocity") return null;
  return { linear: val.linear, angular: val.angular };
}

export function getTrajectoryWaypoints(val: RuntimeValue): PoseValue[] | null {
  // Description:
  //     GetTrajectoryWaypoints.
  //
  // Inputs:
  //     val: RuntimeValue
  //         Caller-supplied val.
  //
  // Outputs:
  //     result: PoseValue[] | null
  //         Return value from `getTrajectoryWaypoints`.
  //
  // Example:
  //     const result = getTrajectoryWaypoints(val);
  // Description:
  //     GetTrajectoryWaypoints.
  //
  // Inputs:
  //     val: RuntimeValue
  //         Caller-supplied val.
  //
  // Outputs:
  //     result: PoseValue[] | null
  //         Return value from `getTrajectoryWaypoints`.
  //
  // Example:
  //     const result = getTrajectoryWaypoints(val);

  // const result = getTrajectoryWaypoints(val);
  if (val.kind !== "trajectory") return null;
  return val.waypoints;
}

export function getNumber(val: RuntimeValue, defaultVal = 0): number {
  // Description:
  //     GetNumber.
  //
  // Inputs:
  //     val: RuntimeValue
  //         Caller-supplied val.
  //     defaultVal = 0: input value
  //         Caller-supplied defaultVal = 0.
  //
  // Outputs:
  //     result: number
  //         Return value from `getNumber`.
  //
  // Example:
  //     const result = getNumber(val, defaultVal = 0);
  // Description:
  //     GetNumber.
  //
  // Inputs:
  //     val: RuntimeValue
  //         Caller-supplied val.
  //     defaultVal = 0: input value
  //         Caller-supplied defaultVal = 0.
  //
  // Outputs:
  //     result: number
  //         Return value from `getNumber`.
  //
  // Example:
  //     const result = getNumber(val, defaultVal = 0);

  // const result = getNumber(val, defaultVal);
  return val.kind === "number" ? val.value : defaultVal;
}

export function getString(val: RuntimeValue, defaultVal = ""): string {
  // Description:
  //     GetString.
  //
  // Inputs:
  //     val: RuntimeValue
  //         Caller-supplied val.
  //     defaultVal = "": input value
  //         Caller-supplied defaultVal = "".
  //
  // Outputs:
  //     result: string
  //         Return value from `getString`.
  //
  // Example:
  //     const result = getString(val, defaultVal = "");
  // Description:
  //     GetString.
  //
  // Inputs:
  //     val: RuntimeValue
  //         Caller-supplied val.
  //     defaultVal = "": input value
  //         Caller-supplied defaultVal = "".
  //
  // Outputs:
  //     result: string
  //         Return value from `getString`.
  //
  // Example:
  //     const result = getString(val, defaultVal = "");

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
