/**
 * Multi-host fleet mesh coordinator client.
 * @module
 */

import type { PeerDelivery } from "./fleet-orchestrator.js";
import { remoteFetch } from "./http-fetch.js";

export type MeshRelayResponse = {
  ok: boolean;
  relayed: number;
  failed: number;
  error?: string;
};

export type FleetRecoveryRequest = {
  action: string;
  fleet_name?: string;
  from_robot?: string;
  members?: string[];
};

export type FleetRecoveryResponse = {
  ok: boolean;
  relayed: number;
  failed: number;
  error?: string;
};

export function defaultFleetMeshUrl(): string | undefined {
  // Description:
  //     DefaultFleetMeshUrl.
  //
  // Inputs:
  //     None.
  //
  // Outputs:
  //     result: string | undefined
  //         Return value from `defaultFleetMeshUrl`.
  //
  // Example:

  //     const result = defaultFleetMeshUrl();

  return process.env.SPANDA_FLEET_MESH_URL;
}

async function meshFetch(
  meshUrl: string,
  path: string,
  body: string,
  token?: string,
): Promise<Response> {
  // Description:
  //     MeshFetch.
  //
  // Inputs:
  //     meshUrl: string
  //         Caller-supplied meshUrl.
  //     path: string
  //         Caller-supplied path.
  //     body: string
  //         Caller-supplied body.
  //     token?: string
  //         Caller-supplied token?.
  //
  // Outputs:
  //     result: Promise<Response>
  //         Return value from `meshFetch`.
  //
  // Example:

  //     const result = meshFetch(meshUrl, path, body, token?);

  const base = meshUrl.replace(/\/$/, "");
  const headers: Record<string, string> = {
    Accept: "application/json",
    "Content-Type": "application/json",
  };
  if (token) headers.Authorization = `Bearer ${token}`;
  return remoteFetch(`${base}${path}`, { method: "POST", headers, body });
}

export async function relayDeliveriesViaMesh(
  meshUrl: string,
  deliveries: PeerDelivery[],
  token?: string,
): Promise<MeshRelayResponse> {
  // Description:
  //     RelayDeliveriesViaMesh.
  //
  // Inputs:
  //     meshUrl: string
  //         Caller-supplied meshUrl.
  //     deliveries: PeerDelivery[]
  //         Caller-supplied deliveries.
  //     token?: string
  //         Caller-supplied token?.
  //
  // Outputs:
  //     result: Promise<MeshRelayResponse>
  //         Return value from `relayDeliveriesViaMesh`.
  //
  // Example:

  //     const result = relayDeliveriesViaMesh(meshUrl, deliveries, token?);

  const response = await meshFetch(
    meshUrl,
    "/v1/mesh/relay",
    JSON.stringify({ deliveries: deliveries.map((d) => ({
      from_robot: d.fromRobot,
      to_robot: d.toRobot,
      topic: d.topic,
      step: d.step,
      delivered: d.delivered,
    })) }),
    token,
  );
  if (!response.ok) {
    throw new Error(`fleet mesh HTTP ${response.status}: ${await response.text()}`);
  }
  const body = (await response.json()) as MeshRelayResponse;
  return body;
}

export async function relayRecoveryViaMesh(
  meshUrl: string,
  request: FleetRecoveryRequest,
  token?: string,
): Promise<FleetRecoveryResponse> {
  // Description:
  //     RelayRecoveryViaMesh.
  //
  // Inputs:
  //     meshUrl: string
  //         Caller-supplied meshUrl.
  //     request: FleetRecoveryRequest
  //         Caller-supplied request.
  //     token?: string
  //         Caller-supplied token?.
  //
  // Outputs:
  //     result: Promise<FleetRecoveryResponse>
  //         Return value from `relayRecoveryViaMesh`.
  //
  // Example:

  //     const result = relayRecoveryViaMesh(meshUrl, request, token?);

  const response = await meshFetch(
    meshUrl,
    "/v1/fleet/recovery",
    JSON.stringify(request),
    token,
  );
  if (!response.ok) {
    throw new Error(`fleet mesh recovery HTTP ${response.status}: ${await response.text()}`);
  }
  return (await response.json()) as FleetRecoveryResponse;
}
