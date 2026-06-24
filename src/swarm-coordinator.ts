/**
 * Experimental swarm coordinator runtime built on fleet declarations and mission controllers.
 * @module
 */

import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import type { Program } from "./ast/nodes.js";
import type { SwarmDecl, SwarmPolicy } from "./foundations.js";
import {
  createMissionRuntime,
  missionAdvance,
  missionStart,
  type MissionRuntime,
} from "./robotics-platform.js";
import type { FleetMemberState, PeerDelivery } from "./fleet-orchestrator.js";

export type SwarmState = {
  roundRobinCursor: Record<string, number>;
};

export type SwarmCoordinationReport = {
  swarmName: string;
  fleetName: string;
  policy: SwarmPolicy;
  activeMember: string | null;
  members: FleetMemberState[];
  stepsAdvanced: number;
  coordinationMode: string;
  peerDeliveries: PeerDelivery[];
  roundRobinCursor: number;
  remoteRelayed?: number;
  remoteFailed?: number;
};

export type SwarmCoordinationResult = {
  program: string;
  swarms: SwarmCoordinationReport[];
  success: boolean;
};

export function defaultSwarmStatePath(): string {
  // Description:
  //     DefaultSwarmStatePath.
  //
  // Inputs:
  //     None.
  //
  // Outputs:
  //     result: string
  //         Return value from `defaultSwarmStatePath`.
  //
  // Example:

  //     const result = defaultSwarmStatePath();

  return process.env.SPANDA_SWARM_STATE ?? ".spanda/swarm-state.json";
}

export function emptySwarmState(): SwarmState {
  // Description:
  //     EmptySwarmState.
  //
  // Inputs:
  //     None.
  //
  // Outputs:
  //     result: SwarmState
  //         Return value from `emptySwarmState`.
  //
  // Example:

  //     const result = emptySwarmState();

  return { roundRobinCursor: {} };
}

export function readSwarmStateFromDisk(path = defaultSwarmStatePath()): SwarmState {
  // Description:
  //     ReadSwarmStateFromDisk.
  //
  // Inputs:
  //     path = defaultSwarmStatePath(): input value
  //         Caller-supplied path = defaultSwarmStatePath().
  //
  // Outputs:
  //     result: SwarmState
  //         Return value from `readSwarmStateFromDisk`.
  //
  // Example:

  //     const result = readSwarmStateFromDisk(path = defaultSwarmStatePath());

  if (!existsSync(path)) return emptySwarmState();
  try {
    const parsed = JSON.parse(readFileSync(path, "utf-8")) as {
      round_robin_cursor?: Record<string, number>;
      roundRobinCursor?: Record<string, number>;
    };
    return {
      roundRobinCursor: parsed.roundRobinCursor ?? parsed.round_robin_cursor ?? {},
    };
  } catch {
    return emptySwarmState();
  }
}

export function writeSwarmStateToDisk(state: SwarmState, path = defaultSwarmStatePath()): void {
  // Description:
  //     WriteSwarmStateToDisk.
  //
  // Inputs:
  //     state: SwarmState
  //         Caller-supplied state.
  //     path = defaultSwarmStatePath(): input value
  //         Caller-supplied path = defaultSwarmStatePath().
  //
  // Outputs:
  //     None.
  //
  // Example:

  //     const result = writeSwarmStateToDisk(state, path = defaultSwarmStatePath());

  const abs = resolve(path);
  mkdirSync(dirname(abs), { recursive: true });
  writeFileSync(abs, JSON.stringify({ round_robin_cursor: state.roundRobinCursor }, null, 2));
}

function missionForRobot(robot: Program["robots"][number]): MissionRuntime | null {
  // Description:
  //     MissionForRobot.
  //
  // Inputs:
  //     robot: Program["robots"][number]
  //         Caller-supplied robot.
  //
  // Outputs:
  //     result: MissionRuntime | null
  //         Return value from `missionForRobot`.
  //
  // Example:

  //     const result = missionForRobot(robot);

  if (!robot.mission) return null;
  return createMissionRuntime(
    robot.mission.name,
    [...robot.mission.steps],
    robot.mission.durationHours,
  );
}

function advanceMember(program: Program, memberName: string): {
  // Description:
  //     AdvanceMember.
  //
  // Inputs:
  //     program: Program
  //         Caller-supplied program.
  //     memberName: string
  //         Caller-supplied memberName.
  //
  // Outputs:
  //     None.
  //
  // Example:

  //     const result = advanceMember(program, memberName);

  state: FleetMemberState;
  deliveries: PeerDelivery[];
} {
  const robot = program.robots.find((entry) => entry.name === memberName);
  if (!robot) {
    return {
      state: {
        robotName: memberName,
        missionName: null,
        missionState: "MissingRobot",
        currentStep: "",
        hasPeerLink: false,
      },
      deliveries: [],
    };
  }
  const runtime = missionForRobot(robot);
  if (!runtime) {
    return {
      state: {
        robotName: memberName,
        missionName: null,
        missionState: "NoMission",
        currentStep: "",
        hasPeerLink: (robot.peerRobots?.length ?? 0) > 0,
      },
      deliveries: [],
    };
  }
  missionStart(runtime);
  const step = missionAdvance(runtime) ?? "";
  const peerHandoffs = (robot.peerRobots ?? []).flatMap((peer) =>
    step ? [`${memberName}->${peer.name}:step=${step}`] : [],
  );
  const deliveries: PeerDelivery[] = (robot.peerRobots ?? []).flatMap((peer) =>
    step
      ? [{
          fromRobot: memberName,
          toRobot: peer.name,
          topic: "mission_step",
          step,
          delivered: true,
        }]
      : [],
  );
  return {
    state: {
      robotName: memberName,
      missionName: runtime.name,
      missionState: runtime.state,
      currentStep: step,
      hasPeerLink: (robot.peerRobots?.length ?? 0) > 0,
      peerHandoffs,
    },
    deliveries,
  };
}

function leaderFollowDeliveries(
  leader: string,
  step: string,
  members: string[],
): PeerDelivery[] {
  // Description:
  //     LeaderFollowDeliveries.
  //
  // Inputs:
  //     leader: string
  //         Caller-supplied leader.
  //     step: string
  //         Caller-supplied step.
  //     members: string[]
  //         Caller-supplied members.
  //
  // Outputs:
  //     result: PeerDelivery[]
  //         Return value from `leaderFollowDeliveries`.
  //
  // Example:

  //     const result = leaderFollowDeliveries(leader, step, members);

  if (!step) return [];
  return members
    .filter((member) => member !== leader)
    .map((follower) => ({
      fromRobot: leader,
      toRobot: follower,
      topic: "mission_step",
      step,
      delivered: true,
    }));
}

function coordinateSwarmGroup(
  program: Program,
  swarm: SwarmDecl,
  cursor: number,
): SwarmCoordinationReport {
  // Description:
  //     CoordinateSwarmGroup.
  //
  // Inputs:
  //     program: Program
  //         Caller-supplied program.
  //     swarm: SwarmDecl
  //         Caller-supplied swarm.
  //     cursor: number
  //         Caller-supplied cursor.
  //
  // Outputs:
  //     result: SwarmCoordinationReport
  //         Return value from `coordinateSwarmGroup`.
  //
  // Example:

  //     const result = coordinateSwarmGroup(program, swarm, cursor);

  const fleet = program.fleets.find((entry) => entry.name === swarm.fleetName);
  const members = fleet?.members ?? [];
  const memberStates: FleetMemberState[] = [];
  let peerDeliveries: PeerDelivery[] = [];
  let stepsAdvanced = 0;
  let activeMember: string | null = null;
  let nextCursor = cursor;

  if (swarm.policy === "round_robin") {
    if (members.length > 0) {
      const index = cursor % members.length;
      nextCursor = (index + 1) % members.length;
      const memberName = members[index]!;
      activeMember = memberName;
      const { state, deliveries } = advanceMember(program, memberName);
      peerDeliveries.push(...deliveries);
      if (state.currentStep) stepsAdvanced = 1;
      memberStates.push(state);
    }
  } else if (swarm.policy === "broadcast") {
    for (const memberName of members) {
      const { state, deliveries } = advanceMember(program, memberName);
      peerDeliveries.push(...deliveries);
      if (state.currentStep) stepsAdvanced += 1;
      memberStates.push(state);
    }
  } else {
    const leader = members[0];
    if (leader) {
      activeMember = leader;
      const { state } = advanceMember(program, leader);
      if (state.currentStep) stepsAdvanced = 1;
      peerDeliveries.push(...leaderFollowDeliveries(leader, state.currentStep, members));
      memberStates.push(state);
    }
  }

  const coordinationMode =
    swarm.policy === "round_robin"
      ? (peerDeliveries.length > 0 ? "swarm_round_robin_peer" : "swarm_round_robin")
      : swarm.policy === "broadcast"
        ? (peerDeliveries.length > 0 ? "swarm_broadcast_peer" : "swarm_broadcast")
        : "swarm_leader_follow";

  return {
    swarmName: swarm.name,
    fleetName: swarm.fleetName,
    policy: swarm.policy,
    activeMember,
    members: memberStates,
    stepsAdvanced,
    coordinationMode,
    peerDeliveries,
    roundRobinCursor: nextCursor,
  };
}

export function coordinateSwarms(
  program: Program,
  programPath: string,
  state: SwarmState,
): SwarmCoordinationResult {
  // Description:
  //     CoordinateSwarms.
  //
  // Inputs:
  //     program: Program
  //         Caller-supplied program.
  //     programPath: string
  //         Caller-supplied programPath.
  //     state: SwarmState
  //         Caller-supplied state.
  //
  // Outputs:
  //     result: SwarmCoordinationResult
  //         Return value from `coordinateSwarms`.
  //
  // Example:
  //     const result = coordinateSwarms(program, programPath, state);

  // Execute one coordination tick for each swarm declaration in the program.
  const reports: SwarmCoordinationReport[] = [];
  for (const swarm of program.swarms ?? []) {
    const fleet = program.fleets.find((entry) => entry.name === swarm.fleetName);
    if (!fleet) {
      reports.push({
        swarmName: swarm.name,
        fleetName: swarm.fleetName,
        policy: swarm.policy,
        activeMember: null,
        members: [{
          robotName: "",
          missionName: null,
          missionState: "MissingFleet",
          currentStep: "",
          hasPeerLink: false,
        }],
        stepsAdvanced: 0,
        coordinationMode: "missing_fleet",
        peerDeliveries: [],
        roundRobinCursor: 0,
      });
      continue;
    }
    const cursor = state.roundRobinCursor[swarm.name] ?? 0;
    const report = coordinateSwarmGroup(program, swarm, cursor);
    state.roundRobinCursor[swarm.name] = report.roundRobinCursor;
    reports.push(report);
  }
  const success = reports.every(
    (report) =>
      report.coordinationMode !== "missing_fleet"
      && report.members.every((member) => member.missionState !== "MissingRobot"),
  );
  return { program: programPath, swarms: reports, success };
}

export async function coordinateSwarmsMesh(
  program: Program,
  programPath: string,
  state: SwarmState,
  meshUrl: string,
  token?: string,
): Promise<SwarmCoordinationResult> {
  // Description:
  //     CoordinateSwarmsMesh.
  //
  // Inputs:
  //     program: Program
  //         Caller-supplied program.
  //     programPath: string
  //         Caller-supplied programPath.
  //     state: SwarmState
  //         Caller-supplied state.
  //     meshUrl: string
  //         Caller-supplied meshUrl.
  //     token?: string
  //         Caller-supplied token?.
  //
  // Outputs:
  //     result: Promise<SwarmCoordinationResult>
  //         Return value from `coordinateSwarmsMesh`.
  //
  // Example:
  //     const result = coordinateSwarmsMesh(program, programPath, state, meshUrl, token?);

  // Execute swarm coordination locally, then push peer deliveries to the mesh coordinator.
  const { relayDeliveriesViaMesh } = await import("./fleet-mesh.js");
  const result = coordinateSwarms(program, programPath, state);
  let success = result.success;
  for (const swarm of result.swarms) {
    if (swarm.peerDeliveries.length === 0) continue;
    try {
      const resp = await relayDeliveriesViaMesh(meshUrl, swarm.peerDeliveries, token);
      swarm.remoteRelayed = resp.relayed;
      swarm.remoteFailed = resp.failed;
      if (resp.relayed > 0) swarm.coordinationMode = `${swarm.coordinationMode}_mesh`;
      if (resp.failed > 0) success = false;
    } catch {
      swarm.remoteFailed = swarm.peerDeliveries.length;
      success = false;
    }
  }
  return { ...result, success };
}
