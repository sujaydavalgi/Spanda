/**
 * Durable mission checkpoint persistence (TypeScript mirror).
 * @module
 */

import type { MissionStateSnapshot } from "./mission-continuity.js";

export type ContinuityCheckpointStore = {
  entries: Record<string, MissionStateSnapshot>;
};

function storeKey(mission: string, robot: string): string {
  return `${mission}::${robot}`;
}

export function defaultCheckpointStorePath(): string {
  return process.env.SPANDA_CONTINUITY_CHECKPOINTS ?? ".spanda/mission-checkpoints.json";
}

export function recordCheckpoint(
  store: ContinuityCheckpointStore,
  mission: string,
  robot: string,
  snapshot: MissionStateSnapshot,
): ContinuityCheckpointStore {
  return {
    entries: {
      ...store.entries,
      [storeKey(mission, robot)]: snapshot,
    },
  };
}

export function loadCheckpoint(
  store: ContinuityCheckpointStore,
  mission: string,
  robot: string,
): MissionStateSnapshot | undefined {
  return store.entries[storeKey(mission, robot)];
}
