/**
 * Optional live WebSocket broker integration for the TypeScript runtime.
 * @module
 */

import type { RuntimeValue } from "../runtime/interpreter.js";

type WireEnvelope = { topic: string; payload: string };

/** Live WebSocket bridge using the ws npm client. */
export class LiveWebsocketBridge {
  private socket: import("ws").WebSocket | null = null;
  private inbound = new Map<string, RuntimeValue[]>();

  /** Connect to a ws:// or wss:// broker endpoint. */
  async connect(brokerUrl: string): Promise<void> {
    // Open a WebSocket to the broker and buffer JSON wire envelopes.
    //
    // Parameters:
    // - `brokerUrl` — ws:// or wss:// URL
    //
    // Returns:
    // Resolves when the socket is open.
    //
    // Options:
    // None.
    //
    // Example:
    // await bridge.connect("ws://localhost:9090");

    const { default: WebSocket } = await import("ws");
    const socket = new WebSocket(brokerUrl);
    this.socket = socket;
    socket.on("message", (data) => {
      try {
        const frame = JSON.parse(String(data)) as WireEnvelope;
        const queue = this.inbound.get(frame.topic) ?? [];
        queue.push({ kind: "string", value: frame.payload });
        this.inbound.set(frame.topic, queue);
      } catch {
        /* ignore malformed frames */
      }
    });
    await new Promise<void>((resolve, reject) => {
      const timer = setTimeout(() => reject(new Error("websocket connect timeout")), 3000);
      socket.once("open", () => {
        clearTimeout(timer);
        resolve();
      });
      socket.once("error", (err) => {
        clearTimeout(timer);
        reject(err);
      });
    });
  }

  publish(topic: string, payload: string): void {
    const envelope: WireEnvelope = { topic, payload };
    this.socket?.send(JSON.stringify(envelope));
  }

  subscribe(topic: string): void {
    const envelope: WireEnvelope = { topic, payload: "__subscribe__" };
    this.socket?.send(JSON.stringify(envelope));
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

export function liveWebsocketEnabled(): boolean {
  return process.env.SPANDA_LIVE_WEBSOCKET === "1";
}
