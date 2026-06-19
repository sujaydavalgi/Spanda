import type { Environment, RobotState } from "../runtime/interpreter.js";

export type SafetyZoneRuntime = {
  name: string;
  shape: "circle" | "rect";
  x: number;
  y: number;
  radius?: number;
  width?: number;
  height?: number;
};

export type SafetyEvaluation = {
  allowed: boolean;
  reason?: string;
  emergencyStop: boolean;
};

export type SafetyConfig = {
  maxSpeed: number;
  stopIfRules: Array<(env: Environment) => boolean>;
  zones: SafetyZoneRuntime[];
};

export class SafetyMonitor {
  private emergencyStop = false;

  constructor(private config: SafetyConfig) {}

  evaluateBeforeMotion(env: Environment, pose: { x: number; y: number }): SafetyEvaluation {
    if (this.emergencyStop) {
      return { allowed: false, reason: "Emergency stop active", emergencyStop: true };
    }

    for (const rule of this.config.stopIfRules) {
      if (rule(env)) {
        this.emergencyStop = true;
        return {
          allowed: false,
          reason: "stop_if safety rule triggered",
          emergencyStop: true,
        };
      }
    }

    for (const zone of this.config.zones) {
      if (this.isPointInZone(pose.x, pose.y, zone)) {
        this.emergencyStop = true;
        return {
          allowed: false,
          reason: `Robot entered safety zone '${zone.name}'`,
          emergencyStop: true,
        };
      }
    }

    return { allowed: true, emergencyStop: false };
  }

  isInZone(zoneName: string, pose: { x: number; y: number }): boolean {
    const zone = this.config.zones.find((z) => z.name === zoneName);
    if (!zone) return false;
    return this.isPointInZone(pose.x, pose.y, zone);
  }

  clampSpeed(requested: number): number {
    return Math.min(Math.abs(requested), this.config.maxSpeed) * Math.sign(requested || 1);
  }

  isEmergencyStop(): boolean {
    return this.emergencyStop;
  }

  setEmergencyStop(active: boolean): void {
    this.emergencyStop = active;
  }

  reset(): void {
    this.emergencyStop = false;
  }

  private isPointInZone(x: number, y: number, zone: SafetyZoneRuntime): boolean {
    if (zone.shape === "circle" && zone.radius !== undefined) {
      const dx = x - zone.x;
      const dy = y - zone.y;
      return Math.sqrt(dx * dx + dy * dy) <= zone.radius;
    }
    if (zone.shape === "rect" && zone.width !== undefined && zone.height !== undefined) {
      return x >= zone.x && x <= zone.x + zone.width && y >= zone.y && y <= zone.y + zone.height;
    }
    return false;
  }
}

export function createSafetyConfigFromRobot(
  maxSpeed: number,
  stopIfRules: Array<(env: Environment) => boolean>,
  zones: SafetyZoneRuntime[] = [],
): SafetyConfig {
  return { maxSpeed, stopIfRules, zones };
}

export function applyEmergencyStop(state: RobotState): RobotState {
  return {
    ...state,
    emergencyStop: true,
    velocity: { linear: 0, angular: 0 },
  };
}

export function interpolatePoses(
  from: { x: number; y: number; theta: number; z?: number },
  to: { x: number; y: number; theta: number; z?: number },
  steps: number,
): Array<{ x: number; y: number; theta: number; z: number }> {
  const count = Math.max(2, Math.floor(steps));
  const waypoints: Array<{ x: number; y: number; theta: number; z: number }> = [];
  for (let i = 0; i < count; i++) {
    const t = i / (count - 1);
    waypoints.push({
      x: from.x + (to.x - from.x) * t,
      y: from.y + (to.y - from.y) * t,
      theta: from.theta + (to.theta - from.theta) * t,
      z: (from.z ?? 0) + ((to.z ?? 0) - (from.z ?? 0)) * t,
    });
  }
  return waypoints;
}
