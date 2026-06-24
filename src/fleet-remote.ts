/**
 * Remote fleet peer relay client and agent registry.
 * @module
 */

import { readFileSync, writeFileSync, mkdirSync, existsSync } from "node:fs";
import { dirname, resolve } from "node:path";
import type { PeerDelivery } from "./fleet-orchestrator.js";
import { remoteFetch } from "./http-fetch.js";

export type FleetAgentEntry = {
  robotName: string;
  url: string;
  token?: string;
};

export type FleetAgentRegistry = {
  agents: FleetAgentEntry[];
};

export type PeerRelayResponse = {
  ok: boolean;
  toRobot: string;
  topic: string;
  step: string;
  error?: string;
};

export function defaultFleetAgentsPath(): string {
  // Description:
  //     DefaultFleetAgentsPath.
  //
  // Inputs:
  //     None.
  //
  // Outputs:
  //     result: string
  //         Return value from `defaultFleetAgentsPath`.
  //
  // Example:

  //     const result = defaultFleetAgentsPath();

  return process.env.SPANDA_FLEET_AGENTS ?? ".spanda/fleet-agents.json";
}

export function emptyFleetAgentRegistry(): FleetAgentRegistry {
  // Description:
  //     EmptyFleetAgentRegistry.
  //
  // Inputs:
  //     None.
  //
  // Outputs:
  //     result: FleetAgentRegistry
  //         Return value from `emptyFleetAgentRegistry`.
  //
  // Example:

  //     const result = emptyFleetAgentRegistry();

  return { agents: [] };
}

export function loadFleetAgentRegistry(text: string | null): FleetAgentRegistry {
  // Description:
  //     LoadFleetAgentRegistry.
  //
  // Inputs:
  //     text: string | null
  //         Caller-supplied text.
  //
  // Outputs:
  //     result: FleetAgentRegistry
  //         Return value from `loadFleetAgentRegistry`.
  //
  // Example:

  //     const result = loadFleetAgentRegistry(text);

  if (!text) return emptyFleetAgentRegistry();
  try {
    const parsed = JSON.parse(text) as {
      agents?: Array<{ robot_name?: string; robotName?: string; url: string; token?: string }>;
    };
    return {
      agents: (parsed.agents ?? []).map((entry) => ({
        robotName: entry.robotName ?? entry.robot_name ?? "",
        url: entry.url,
        token: entry.token,
      })),
    };
  } catch {
    return emptyFleetAgentRegistry();
  }
}

export function serializeFleetAgentRegistry(registry: FleetAgentRegistry): string {
  // Description:
  //     SerializeFleetAgentRegistry.
  //
  // Inputs:
  //     registry: FleetAgentRegistry
  //         Caller-supplied registry.
  //
  // Outputs:
  //     result: string
  //         Return value from `serializeFleetAgentRegistry`.
  //
  // Example:

  //     const result = serializeFleetAgentRegistry(registry);

  return JSON.stringify(
    {
      agents: registry.agents.map((entry) => ({
        robot_name: entry.robotName,
        url: entry.url,
        token: entry.token,
      })),
    },
    null,
    2,
  );
}

export function readFleetAgentRegistryFromDisk(path = defaultFleetAgentsPath()): FleetAgentRegistry {
  // Description:
  //     ReadFleetAgentRegistryFromDisk.
  //
  // Inputs:
  //     path = defaultFleetAgentsPath(): input value
  //         Caller-supplied path = defaultFleetAgentsPath().
  //
  // Outputs:
  //     result: FleetAgentRegistry
  //         Return value from `readFleetAgentRegistryFromDisk`.
  //
  // Example:

  //     const result = readFleetAgentRegistryFromDisk(path = defaultFleetAgentsPath());

  if (!existsSync(path)) return emptyFleetAgentRegistry();
  return loadFleetAgentRegistry(readFileSync(path, "utf-8"));
}

export function writeFleetAgentRegistryToDisk(
  registry: FleetAgentRegistry,
  path = defaultFleetAgentsPath(),
): void {
  // Description:
  //     WriteFleetAgentRegistryToDisk.
  //
  // Inputs:
  //     registry: FleetAgentRegistry
  //         Caller-supplied registry.
  //     path = defaultFleetAgentsPath(): input value
  //         Caller-supplied path = defaultFleetAgentsPath().
  //
  // Outputs:
  //     None.
  //
  // Example:

  //     const result = writeFleetAgentRegistryToDisk(registry, path = defaultFleetAgentsPath());

  const abs = resolve(path);
  mkdirSync(dirname(abs), { recursive: true });
  writeFileSync(abs, serializeFleetAgentRegistry(registry));
}

export function registerFleetAgent(
  registry: FleetAgentRegistry,
  robotName: string,
  url: string,
  token?: string,
): FleetAgentRegistry {
  // Description:
  //     RegisterFleetAgent.
  //
  // Inputs:
  //     registry: FleetAgentRegistry
  //         Caller-supplied registry.
  //     robotName: string
  //         Caller-supplied robotName.
  //     url: string
  //         Caller-supplied url.
  //     token?: string
  //         Caller-supplied token?.
  //
  // Outputs:
  //     result: FleetAgentRegistry
  //         Return value from `registerFleetAgent`.
  //
  // Example:

  //     const result = registerFleetAgent(registry, robotName, url, token?);

  if (!url.startsWith("http://") && !url.startsWith("https://")) {
    throw new Error(`fleet agent URL must start with http:// or https:// (got ${url})`);
  }
  const agents = registry.agents.filter((entry) => entry.robotName !== robotName);
  agents.push({ robotName, url, token });
  agents.sort((a, b) => a.robotName.localeCompare(b.robotName));
  return { agents };
}

export function lookupFleetAgent(
  registry: FleetAgentRegistry,
  robotName: string,
): FleetAgentEntry | undefined {
  // Description:
  //     LookupFleetAgent.
  //
  // Inputs:
  //     registry: FleetAgentRegistry
  //         Caller-supplied registry.
  //     robotName: string
  //         Caller-supplied robotName.
  //
  // Outputs:
  //     result: FleetAgentEntry | undefined
  //         Return value from `lookupFleetAgent`.
  //
  // Example:

  //     const result = lookupFleetAgent(registry, robotName);

  return registry.agents.find((entry) => entry.robotName === robotName);
}

async function agentFetch(
  entry: FleetAgentEntry,
  method: string,
  path: string,
  body?: string,
): Promise<Response> {
  // Description:
  //     AgentFetch.
  //
  // Inputs:
  //     entry: FleetAgentEntry
  //         Caller-supplied entry.
  //     method: string
  //         Caller-supplied method.
  //     path: string
  //         Caller-supplied path.
  //     body?: string
  //         Caller-supplied body?.
  //
  // Outputs:
  //     result: Promise<Response>
  //         Return value from `agentFetch`.
  //
  // Example:

  //     const result = agentFetch(entry, method, path, body?);

  const base = entry.url.replace(/\/$/, "");
  const headers: Record<string, string> = { Accept: "application/json" };
  if (body) headers["Content-Type"] = "application/json";
  if (entry.token) headers.Authorization = `Bearer ${entry.token}`;
  return remoteFetch(`${base}${path}`, { method, headers, body });
}

export async function fleetAgentHealth(entry: FleetAgentEntry): Promise<boolean> {
  // Description:
  //     FleetAgentHealth.
  //
  // Inputs:
  //     entry: FleetAgentEntry
  //         Caller-supplied entry.
  //
  // Outputs:
  //     result: Promise<boolean>
  //         Return value from `fleetAgentHealth`.
  //
  // Example:

  //     const result = fleetAgentHealth(entry);

  const response = await agentFetch(entry, "GET", "/v1/health");
  if (!response.ok) return false;
  const body = (await response.json()) as { ok?: boolean };
  return body.ok === true;
}

export async function fleetAgentReadiness(
  entry: FleetAgentEntry,
  runtime = false,
  injectHealthFaults = false,
): Promise<{
  // Description:
  //     FleetAgentReadiness.
  //
  // Inputs:
  //     entry: FleetAgentEntry
  //         Caller-supplied entry.
  //     runtime = false: input value
  //         Caller-supplied runtime = false.
  //     injectHealthFaults = false: input value
  //         Caller-supplied injectHealthFaults = false.
  //
  // Outputs:
  //     result: Promise<
  //         Return value from `fleetAgentReadiness`.
  //
  // Example:

 // const result = fleetAgentReadiness(entry, runtime = false, injectHealthFaults = false);
 ok: boolean; mission_ready?: boolean; readiness?: unknown }> {
  const query = new URLSearchParams();
  if (runtime) query.set("runtime", "true");
  if (injectHealthFaults) query.set("inject_health_faults", "true");
  const suffix = query.toString() ? `?${query.toString()}` : "";
  const response = await agentFetch(entry, "GET", `/v1/readiness${suffix}`);
  if (!response.ok) {
    throw new Error(`fleet agent readiness HTTP ${response.status}`);
  }
  return (await response.json()) as { ok: boolean; mission_ready?: boolean; readiness?: unknown };
}

export async function fleetAgentUploadProgram(entry: FleetAgentEntry, program: string): Promise<void> {
  // Description:
  //     FleetAgentUploadProgram.
  //
  // Inputs:
  //     entry: FleetAgentEntry
  //         Caller-supplied entry.
  //     program: string
  //         Caller-supplied program.
  //
  // Outputs:
  //     result: Promise<void>
  //         Return value from `fleetAgentUploadProgram`.
  //
  // Example:

  //     const result = fleetAgentUploadProgram(entry, program);

  const response = await agentFetch(entry, "POST", "/v1/program", JSON.stringify({ program }));
  if (!response.ok) {
    throw new Error(`fleet agent program upload HTTP ${response.status}`);
  }
  const body = (await response.json()) as { ok?: boolean };
  if (!body.ok) {
    throw new Error("fleet agent program upload failed");
  }
}

export async function relayPeerDelivery(
  entry: FleetAgentEntry,
  delivery: PeerDelivery,
): Promise<PeerRelayResponse> {
  // Description:
  //     RelayPeerDelivery.
  //
  // Inputs:
  //     entry: FleetAgentEntry
  //         Caller-supplied entry.
  //     delivery: PeerDelivery
  //         Caller-supplied delivery.
  //
  // Outputs:
  //     result: Promise<PeerRelayResponse>
  //         Return value from `relayPeerDelivery`.
  //
  // Example:

  //     const result = relayPeerDelivery(entry, delivery);

  const response = await agentFetch(
    entry,
    "POST",
    "/v1/peer",
    JSON.stringify({
      from_robot: delivery.fromRobot,
      to_robot: delivery.toRobot,
      topic: delivery.topic,
      step: delivery.step,
    }),
  );
  if (!response.ok) {
    throw new Error(`fleet agent HTTP ${response.status}: ${await response.text()}`);
  }
  const body = (await response.json()) as {
    ok?: boolean;
    to_robot?: string;
    toRobot?: string;
    topic?: string;
    step?: string;
    error?: string;
  };
  return {
    ok: body.ok === true,
    toRobot: body.toRobot ?? body.to_robot ?? delivery.toRobot,
    topic: body.topic ?? delivery.topic,
    step: body.step ?? delivery.step,
    error: body.error,
  };
}

export async function relayPeerDeliveries(
  deliveries: PeerDelivery[],
  registry: FleetAgentRegistry,
): Promise<{
  // Description:
  //     RelayPeerDeliveries.
  //
  // Inputs:
  //     deliveries: PeerDelivery[]
  //         Caller-supplied deliveries.
  //     registry: FleetAgentRegistry
  //         Caller-supplied registry.
  //
  // Outputs:
  //     result: Promise<
  //         Return value from `relayPeerDeliveries`.
  //
  // Example:

 // const result = relayPeerDeliveries(deliveries, registry);
 relayed: number; failed: number }> {
  let relayed = 0;
  let failed = 0;
  for (const delivery of deliveries) {
    const agent = lookupFleetAgent(registry, delivery.toRobot);
    if (!agent) {
      failed += 1;
      continue;
    }
    try {
      const resp = await relayPeerDelivery(agent, delivery);
      if (resp.ok) relayed += 1;
      else failed += 1;
    } catch {
      failed += 1;
    }
  }
  return { relayed, failed };
}
