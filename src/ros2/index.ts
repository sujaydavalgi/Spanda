/**
 * index module (ros2/index.ts).
 * @module
 */

import type { MotionCommand, RobotBackend, RobotState, RuntimeValue } from "../runtime/interpreter.js";

export interface Ros2Adapter extends RobotBackend {
  connect(options: Ros2ConnectOptions): Promise<void>;
  disconnect(): Promise<void>;
  publishTopic(topic: string, messageType: string, message: RuntimeValue): void;
  callService(service: string, serviceType: string): RuntimeValue;
  sendAction(action: string, actionType: string, goal: RuntimeValue): RuntimeValue;
  isConnected(): boolean;
}

export type Ros2ConnectOptions = {
  nodeName?: string;
  namespace?: string;
  domainId?: number;
};

export class Ros2AdapterStub implements Ros2Adapter {
  private connected = false;
  private state: RobotState = {
    pose: { x: 0, y: 0, theta: 0 },
    velocity: { linear: 0, angular: 0 },
    emergencyStop: false,
  };
  private published: Array<{ topic: string; messageType: string; value: RuntimeValue }> = [];

  async connect(options: Ros2ConnectOptions): Promise<void> {
    // Description:
    //     Connect.
    //
    // Inputs:
    //     options: Ros2ConnectOptions
    //         Caller-supplied options.
    //
    // Outputs:
    //     result: Promise<void>
    //         Return value from `connect`.
    //
    // Example:
    //     const result = connect(options);
    // Description:
    //     Connect.
    //
    // Inputs:
    //     options: Ros2ConnectOptions
    //         Caller-supplied options.
    //
    // Outputs:
    //     result: Promise<void>
    //         Return value from `connect`.
    //
    // Example:
    //     const result = connect(options);

    // const result = connect(options);
    this.connected = true;
    console.log(`[ROS2] Connected as node '${options.nodeName ?? "spanda_node"}'`);
}

  async disconnect(): Promise<void> {
    // Description:
    //     Disconnect.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     result: Promise<void>
    //         Return value from `disconnect`.
    //
    // Example:
    //     const result = disconnect();
    // Description:
    //     Disconnect.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     result: Promise<void>
    //         Return value from `disconnect`.
    //
    // Example:
    //     const result = disconnect();

    // const result = disconnect();
    this.connected = false;
    console.log("[ROS2] Disconnected");
}

  readSensor(_sensorName: string, sensorType: string): RuntimeValue {
    // ReadSensor.
    //
    // Parameters:
    // - `_sensorName` — input value
    // - `sensorType` — input value
    //
    // Returns:
    // RuntimeValue.
    //
    // Options:
    // None.
    //
    // Example:

    // const result = readSensor(_sensorName, sensorType);

    if (!this.connected) throw new Error("ROS2 adapter not connected");
    if (sensorType === "Lidar") return { kind: "scan", nearestDistance: Infinity };
    return { kind: "void" };
  }

  executeMotion(cmd: MotionCommand): void {
    // ExecuteMotion.
    //
    // Parameters:
    // - `cmd` — input value
    //
    // Returns:
    // Nothing.
    //
    // Options:
    // None.
    //
    // Example:

    // const result = executeMotion(cmd);

    if (!this.connected) return;
    console.log(`[ROS2] Motion: ${JSON.stringify(cmd)}`);
  }

  tick(dtMs: number): void {
    // Tick.
    //
    // Parameters:
    // - `dtMs` — input value
    //
    // Returns:
    // Nothing.
    //
    // Options:
    // None.
    //
    // Example:

    // const result = tick(dtMs);

    const dt = dtMs / 1000;
    this.state.pose.x += this.state.velocity.linear * Math.cos(this.state.pose.theta) * dt;
    this.state.pose.y += this.state.velocity.linear * Math.sin(this.state.pose.theta) * dt;
    this.state.pose.theta += this.state.velocity.angular * dt;
  }

  getState(): RobotState {
    // GetState.
    //
    // Parameters:
    // None.
    //
    // Returns:
    // RobotState.
    //
    // Options:
    // None.
    //
    // Example:

    // const result = getState();

    return { ...this.state, pose: { ...this.state.pose }, velocity: { ...this.state.velocity } };
  }

  setEmergencyStop(active: boolean): void {
    // SetEmergencyStop.
    //
    // Parameters:
    // - `active` — input value
    //
    // Returns:
    // Nothing.
    //
    // Options:
    // None.
    //
    // Example:

    // const result = setEmergencyStop(active);

    this.state.emergencyStop = active;
    if (active) this.state.velocity = { linear: 0, angular: 0 };
  }

  publishTopic(topic: string, messageType: string, message: RuntimeValue): void {
    this.published.push({ topic, messageType, value: message });
    console.log(`[ROS2] publish ${topic} (${messageType})`);
  }

  callService(service: string, serviceType: string): RuntimeValue {
    console.log(`[ROS2] service ${service} (${serviceType})`);
    return { kind: "bool", value: true };
  }

  sendAction(action: string, actionType: string, goal: RuntimeValue): RuntimeValue {
    console.log(`[ROS2] action ${action} (${actionType})`);
    return { kind: "bool", value: true };
  }

  getPublishedTopics(): Array<{ topic: string; messageType: string; value: RuntimeValue }> {
    return [...this.published];
  }

  isConnected(): boolean {
    return this.connected;
  }
}

export function createRos2Adapter(): Ros2Adapter {
  // CreateRos2Adapter.
  //
  // Parameters:
  // None.
  //
  // Returns:
  // `Ros2Adapter`.
  //
  // Options:
  // None.
  //
  // Example:

  // const result = createRos2Adapter();
  return new Ros2AdapterStub();
}
