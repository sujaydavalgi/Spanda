/**
 * Optional live DDS domain integration via UDP multicast for TypeScript.
 * @module
 */

import { createSocket, type Socket } from "node:dgram";
import type { RuntimeValue } from "../runtime/interpreter.js";

type DdsWireEnvelope = { topic: string; payload: string };

/** Live DDS bridge over UDP multicast for a domain id. */
export class LiveDdsBridge {
  private socket: Socket | null = null;
  private group = "239.255.0.0";
  private port = 7400;
  private inbound = new Map<string, RuntimeValue[]>();

  /** Join the DDS domain multicast group for the given domain id. */
  connect(domainId: number): void {
    // Bind a UDP socket and join the domain multicast group.
    //
    // Parameters:
    // - `domainId` — DDS domain id
    //
    // Returns:
    // Nothing.
    //
    // Options:
    // None.
    //
    // Example:
    // bridge.connect(0);

    const port = 7400 + domainId;
    const octet = Math.min(domainId, 255);
    this.group = `239.255.0.${octet}`;
    this.port = port;
    const socket = createSocket({ type: "udp4", reuseAddr: true });
    socket.bind(port);
    socket.addMembership(this.group);
    socket.on("message", (buf) => {
      try {
        const frame = JSON.parse(buf.toString("utf8")) as DdsWireEnvelope;
        const queue = this.inbound.get(frame.topic) ?? [];
        queue.push({ kind: "string", value: frame.payload });
        this.inbound.set(frame.topic, queue);
      } catch {
        /* ignore malformed frames */
      }
    });
    this.socket = socket;
  }

  publish(topic: string, payload: string): void {
    const envelope: DdsWireEnvelope = { topic, payload };
    this.socket?.send(JSON.stringify(envelope), this.port, this.group);
  }

  subscribe(_topic: string): void {
    /* multicast receive is topic-filtered in software */
  }

  receive(topic: string): RuntimeValue | null {
    const queue = this.inbound.get(topic);
    return queue?.shift() ?? null;
  }

  disconnect(): void {
    this.socket?.close();
    this.socket = null;
  }
}

export function liveDdsEnabled(): boolean {
  return process.env.SPANDA_LIVE_DDS === "1";
}
