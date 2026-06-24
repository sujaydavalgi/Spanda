/**
 * Robotics platform primitives: mission lifecycle, fleet grouping, and program safety zones.
 * @module
 */

export type MissionState = "Pending" | "Running" | "Paused" | "Completed" | "Failed";

export type MissionRuntime = {
  name: string | null;
  steps: string[];
  state: MissionState;
  stepIndex: number;
  durationHours: number | null;
};

export function createMissionRuntime(
  name: string | null,
  steps: string[],
  durationHours: number | null,
): MissionRuntime {
  // Description:
  //     CreateMissionRuntime.
  //
  // Inputs:
  //     name: string | null
  //         Caller-supplied name.
  //     steps: string[]
  //         Caller-supplied steps.
  //     durationHours: number | null
  //         Caller-supplied durationHours.
  //
  // Outputs:
  //     result: MissionRuntime
  //         Return value from `createMissionRuntime`.
  //
  // Example:
  //     const result = createMissionRuntime(name, steps, durationHours);
  // Description:
  //     CreateMissionRuntime.
  //
  // Inputs:
  //     name: string | null
  //         Caller-supplied name.
  //     steps: string[]
  //         Caller-supplied steps.
  //     durationHours: number | null
  //         Caller-supplied durationHours.
  //
  // Outputs:
  //     result: MissionRuntime
  //         Return value from `createMissionRuntime`.
  //
  // Example:

  //     const result = createMissionRuntime(name, steps, durationHours);

  return {
    name,
    steps,
    state: "Pending",
    stepIndex: 0,
    durationHours,
  };
}

export function missionStart(runtime: MissionRuntime): void {
  // Description:
  //     MissionStart.
  //
  // Inputs:
  //     runtime: MissionRuntime
  //         Caller-supplied runtime.
  //
  // Outputs:
  //     None.
  //
  // Example:
  //     const result = missionStart(runtime);
  // Description:
  //     MissionStart.
  //
  // Inputs:
  //     runtime: MissionRuntime
  //         Caller-supplied runtime.
  //
  // Outputs:
  //     None.
  //
  // Example:

  //     const result = missionStart(runtime);

  if (runtime.state === "Pending") {
    runtime.state = "Running";
  }
}

export function missionPause(runtime: MissionRuntime): void {
  // Description:
  //     MissionPause.
  //
  // Inputs:
  //     runtime: MissionRuntime
  //         Caller-supplied runtime.
  //
  // Outputs:
  //     None.
  //
  // Example:
  //     const result = missionPause(runtime);

  // Pause an active mission without losing step progress.
  if (runtime.state === "Running") {
    runtime.state = "Paused";
  }
}

export function missionResume(runtime: MissionRuntime): void {
  // Description:
  //     MissionResume.
  //
  // Inputs:
  //     runtime: MissionRuntime
  //         Caller-supplied runtime.
  //
  // Outputs:
  //     None.
  //
  // Example:
  //     const result = missionResume(runtime);

  // Resume a paused mission from the current step.
  if (runtime.state === "Paused") {
    runtime.state = "Running";
  }
}

export function missionAdvance(runtime: MissionRuntime): string {
  // Description:
  //     MissionAdvance.
  //
  // Inputs:
  //     runtime: MissionRuntime
  //         Caller-supplied runtime.
  //
  // Outputs:
  //     result: string
  //         Return value from `missionAdvance`.
  //
  // Example:
  //     const result = missionAdvance(runtime);

  // Move to the next mission step and return its name when one remains.
  if (runtime.state !== "Running") {
    return "";
  }
  if (runtime.stepIndex >= runtime.steps.length) {
    runtime.state = "Completed";
    return "";
  }
  const step = runtime.steps[runtime.stepIndex] ?? "";
  runtime.stepIndex += 1;
  if (runtime.stepIndex >= runtime.steps.length) {
    runtime.state = "Completed";
  }
  return step;
}

export function missionComplete(runtime: MissionRuntime): void {
  // Description:
  //     MissionComplete.
  //
  // Inputs:
  //     runtime: MissionRuntime
  //         Caller-supplied runtime.
  //
  // Outputs:
  //     None.
  //
  // Example:
  //     const result = missionComplete(runtime);

  // Mark the mission completed regardless of remaining steps.
  runtime.state = "Completed";
  runtime.stepIndex = runtime.steps.length;
}

export function missionFail(runtime: MissionRuntime): void {
  // Description:
  //     MissionFail.
  //
  // Inputs:
  //     runtime: MissionRuntime
  //         Caller-supplied runtime.
  //
  // Outputs:
  //     None.
  //
  // Example:
  //     const result = missionFail(runtime);

  // Mark the mission failed and stop step progression.
  runtime.state = "Failed";
}

export function missionCurrentStep(runtime: MissionRuntime): string {
  // Description:
  //     MissionCurrentStep.
  //
  // Inputs:
  //     runtime: MissionRuntime
  //         Caller-supplied runtime.
  //
  // Outputs:
  //     result: string
  //         Return value from `missionCurrentStep`.
  //
  // Example:
  //     const result = missionCurrentStep(runtime);

  // Return the active step name while the mission is running.
  if (runtime.state !== "Running") {
    return "";
  }
  return runtime.steps[runtime.stepIndex] ?? "";
}

export class FleetRegistry {
  private fleets = new Map<string, string[]>();

  register(name: string, members: string[]): void {

    // Store a fleet name and its member robot identifiers.
    this.fleets.set(name, members);
  }

  members(name: string): string[] | undefined {

    // Look up fleet members by fleet name.
    return this.fleets.get(name);
  }

  names(): string[] {

    // Return all declared fleet names.
    return [...this.fleets.keys()];
  }

  clone(): FleetRegistry {

    // Clone fleet registrations for env re-binding after robot setup.
    const copy = new FleetRegistry();
    for (const [name, members] of this.fleets) {
      copy.register(name, [...members]);
    }
    return copy;
  }
}

export class ProgramSafetyZoneRegistry {
  private zones = new Map<string, number>();

  register(name: string, maxSpeedMps: number): void {

    // Register a zone-specific maximum speed in meters per second.
    this.zones.set(name, maxSpeedMps);
  }

  maxSpeedFor(zoneName: string): number | undefined {

    // Resolve the configured speed cap for a named zone.
    return this.zones.get(zoneName);
  }

  speedCaps(): Map<string, number> {

    // Return all registered zone speed caps.
    return new Map(this.zones);
  }
}
