import { createServer } from "node:http";
import { describe, expect, it } from "vitest";
import { compile } from "../src/compile.js";
import { orchestrateFleetsRemote } from "../src/fleet-orchestrator.js";
import {
  registerFleetAgent,
  relayPeerDeliveries,
  type FleetAgentRegistry,
} from "../src/fleet-remote.js";

describe("fleet remote (TS mirror)", () => {
  it("relays peer deliveries to a local fleet agent", async () => {
    const server = createServer((req, res) => {
      if (req.method === "GET" && req.url === "/v1/health") {
        res.writeHead(200, { "Content-Type": "application/json" });
        res.end(JSON.stringify({ ok: true }));
        return;
      }
      if (req.method === "POST" && req.url === "/v1/peer") {
        res.writeHead(200, { "Content-Type": "application/json" });
        res.end(JSON.stringify({ ok: true, to_robot: "ScoutB", topic: "mission_step", step: "inspect" }));
        return;
      }
      res.writeHead(404);
      res.end();
    });
    await new Promise<void>((resolveListen) => server.listen(0, resolveListen));
    const port = (server.address() as { port: number }).port;
    let registry: FleetAgentRegistry = { agents: [] };
    registry = registerFleetAgent(registry, "ScoutB", `http://127.0.0.1:${port}`);
    const { relayed, failed } = await relayPeerDeliveries(
      [
        {
          fromRobot: "ScoutA",
          toRobot: "ScoutB",
          topic: "mission_step",
          step: "inspect",
          delivered: true,
        },
      ],
      registry,
    );
    expect(relayed).toBe(1);
    expect(failed).toBe(0);
    server.close();
  });

  it("orchestrates with remote relay mode", async () => {
    const server = createServer((req, res) => {
      if (req.method === "GET" && req.url === "/v1/health") {
        res.writeHead(200, { "Content-Type": "application/json" });
        res.end(JSON.stringify({ ok: true }));
        return;
      }
      if (req.method === "POST" && req.url === "/v1/peer") {
        res.writeHead(200, { "Content-Type": "application/json" });
        res.end(JSON.stringify({ ok: true, to_robot: "ScoutB", topic: "mission_step", step: "inspect" }));
        return;
      }
      res.writeHead(404);
      res.end();
    });
    await new Promise<void>((resolveListen) => server.listen(0, resolveListen));
    const port = (server.address() as { port: number }).port;
    let registry: FleetAgentRegistry = { agents: [] };
    registry = registerFleetAgent(registry, "ScoutB", `http://127.0.0.1:${port}`);
    const source = `
robot ScoutA {
  robot ScoutB;
  mission Patrol { navigate; inspect; }
}
robot ScoutB {
  mission Patrol { navigate; inspect; }
}
fleet Recon { ScoutA; ScoutB; }
`;
    const { program } = compile(source, "typescript");
    const result = await orchestrateFleetsRemote(program, "peer_fleet.sd", registry);
    expect(result.success).toBe(true);
    expect(result.fleets[0]?.coordinationMode).toBe("distributed_peer_mesh");
    expect(result.fleets[0]?.remoteRelayed).toBe(1);
    server.close();
  });
});
