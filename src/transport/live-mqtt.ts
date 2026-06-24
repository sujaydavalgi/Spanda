/**
 * Optional live MQTT broker integration for the TypeScript runtime.
 * @module
 */

import type { RuntimeValue } from "../runtime/interpreter.js";

type MqttClient = {
  publish(topic: string, payload: string): void;
  subscribe(topic: string): void;
  on(event: "message", handler: (topic: string, payload: Buffer) => void): void;
  end(): void;
};

/** Live MQTT bridge backed by the mqtt npm client when available. */
export class LiveMqttBridge {
  private client: MqttClient | null = null;
  private inbound = new Map<string, RuntimeValue[]>();

  /** Connect to a broker URL using the mqtt client library. */
  async connect(brokerUrl: string, clientId: string): Promise<void> {
    // Description:
    //     Connect.
    //
    // Inputs:
    //     brokerUrl: string
    //         Caller-supplied brokerUrl.
    //     clientId: string
    //         Caller-supplied clientId.
    //
    // Outputs:
    //     result: Promise<void>
    //         Return value from `connect`.
    //
    // Example:

    //     const result = connect(brokerUrl, clientId);

    const mqtt = await import("mqtt");
    const client = mqtt.connect(brokerUrl, { clientId }) as MqttClient;
    this.client = client;
    client.on("message", (topic, payload) => {
      const text = payload.toString("utf8");
      const queue = this.inbound.get(topic) ?? [];
      queue.push({ kind: "string", value: text });
      this.inbound.set(topic, queue);
    });
    await new Promise<void>((resolve, reject) => {
      const timer = setTimeout(() => reject(new Error("mqtt connect timeout")), 3000);
      (client as unknown as { on: (e: string, h: () => void) => void }).on("connect", () => {
        clearTimeout(timer);
        resolve();
      });
    });
  }

  publish(topic: string, payload: string): void {
    this.client?.publish(topic, payload);
  }

  subscribe(topic: string): void {
    this.client?.subscribe(topic);
  }

  receive(topic: string): RuntimeValue | null {
    const queue = this.inbound.get(topic);
    return queue?.shift() ?? null;
  }

  disconnect(): void {
    this.client?.end();
    this.client = null;
  }
}

export function liveMqttEnabled(): boolean {
  // Description:
  //     LiveMqttEnabled.
  //
  // Inputs:
  //     None.
  //
  // Outputs:
  //     result: boolean
  //         Return value from `liveMqttEnabled`.
  //
  // Example:

  //     const result = liveMqttEnabled();

  return process.env.SPANDA_LIVE_MQTT === "1";
}
