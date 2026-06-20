/**
 * concurrency module (concurrency.ts).
 * @module
 */

import { RuntimeError } from "./runtime/interpreter.js";
import type { RuntimeValue } from "./runtime/interpreter.js";

export type SpawnHandle = {
  funcName: string;
  args: RuntimeValue[];
  result: RuntimeValue | null;
};

export type AgentRoute = {
  from: string;
  to: string;
  messageType: string;
};

function runtimeTypeTag(value: RuntimeValue): string {
  // RuntimeTypeTag.
  //
  // Parameters:
  // - `value` — input value
  //
  // Returns:
  // Text result.
  //
  // Options:
  // None.
  //
  // Example:
  // const result = runtimeTypeTag(value);

  switch (value.kind) {
    case "object":
      return `object:${value.typeName}`;
    case "enum":
      return `enum:${value.enumName}::${value.variant}`;
    case "number":
      return `number:${value.unit}`;
    case "string":
      return "string";
    case "bool":
      return "bool";
    case "pose":
      return "pose";
    case "channel":
      return "channel";
    case "task_handle":
      return "task_handle";
    case "future":
      return "future";
    default:
      return value.kind;
  }
}

export class ConcurrencyRuntime {
  private nextChannelId = 1;
  private channels = new Map<number, RuntimeValue[]>();
  private channelTypeTags = new Map<number, string>();
  private nextHandleId = 1;
  private handles = new Map<number, SpawnHandle>();
  private fireAndForgetQueue: number[] = [];
  private agentInboxes = new Map<string, RuntimeValue[]>();
  private agentRoutes: AgentRoute[] = [];

  createChannel(): RuntimeValue {
    // CreateChannel.
    //
    // Parameters:
    // None.
    //
    // Returns:
    // RuntimeValue.
    //
    // Options:
    // None.
    //
    // Example:
    // const result = createChannel();

    const id = this.nextChannelId++;
    this.channels.set(id, []);
    return { kind: "channel", id };
  }

  bindChannelType(channel: RuntimeValue, value: RuntimeValue, line: number): void {
    // BindChannelType.
    //
    // Parameters:
    // - `channel` — input value
    // - `value` — input value
    // - `line` — input value
    //
    // Returns:
    // Nothing.
    //
    // Options:
    // None.
    //
    // Example:
    // const result = bindChannelType(channel, value, line);

    if (channel.kind !== "channel") {
      throw new RuntimeError("channel type binding requires channel", line);
    }
    const next = runtimeTypeTag(value);
    const existing = this.channelTypeTags.get(channel.id);
    if (existing && existing !== next) {
      throw new RuntimeError(`Channel type mismatch: expected ${existing}, got ${next}`, line);
    }
    this.channelTypeTags.set(channel.id, next);
  }

  send(channel: RuntimeValue, value: RuntimeValue, line: number): void {
    // Send.
    //
    // Parameters:
    // - `channel` — input value
    // - `value` — input value
    // - `line` — input value
    //
    // Returns:
    // Nothing.
    //
    // Options:
    // None.
    //
    // Example:
    // const result = send(channel, value, line);

    if (channel.kind !== "channel") {
      throw new RuntimeError("send requires a channel", line);
    }
    const buf = this.channels.get(channel.id);
    if (!buf) {
      throw new RuntimeError(`Unknown channel id ${channel.id}`, line);
    }
    const expected = this.channelTypeTags.get(channel.id);
    if (expected) {
      const actual = runtimeTypeTag(value);
      if (expected !== actual) {
        throw new RuntimeError(`Channel type mismatch: expected ${expected}, got ${actual}`, line);
      }
    }
    buf.push(value);
  }

  tryRecv(channel: RuntimeValue, line: number): RuntimeValue | null {
    // TryRecv.
    //
    // Parameters:
    // - `channel` — input value
    // - `line` — input value
    //
    // Returns:
    // Some value on success, otherwise none.
    //
    // Options:
    // None.
    //
    // Example:
    // const result = tryRecv(channel, line);

    if (channel.kind !== "channel") {
      throw new RuntimeError("recv requires a channel", line);
    }
    const buf = this.channels.get(channel.id);
    if (!buf) {
      throw new RuntimeError(`Unknown channel id ${channel.id}`, line);
    }
    return buf.shift() ?? null;
  }

  createTaskHandle(funcName: string, args: RuntimeValue[]): RuntimeValue {
    // CreateTaskHandle.
    //
    // Parameters:
    // - `funcName` — input value
    // - `args` — input value
    //
    // Returns:
    // RuntimeValue.
    //
    // Options:
    // None.
    //
    // Example:
    // const result = createTaskHandle(funcName, args);

    const id = this.nextHandleId++;
    this.handles.set(id, { funcName, args, result: null });
    return { kind: "task_handle", id };
  }

  queueFireAndForget(funcName: string, args: RuntimeValue[]): void {
    // QueueFireAndForget.
    //
    // Parameters:
    // - `funcName` — input value
    // - `args` — input value
    //
    // Returns:
    // Nothing.
    //
    // Options:
    // None.
    //
    // Example:
    // const result = queueFireAndForget(funcName, args);

    const handle = this.createTaskHandle(funcName, args);
    if (handle.kind === "task_handle") {
      this.fireAndForgetQueue.push(handle.id);
    }
  }

  getHandle(id: number): SpawnHandle | undefined {
    // GetHandle.
    //
    // Parameters:
    // - `id` — input value
    //
    // Returns:
    // Some value on success, otherwise none.
    //
    // Options:
    // None.
    //
    // Example:
    // const result = getHandle(id);

    return this.handles.get(id);
  }

  setHandleResult(id: number, result: RuntimeValue): void {
    // SetHandleResult.
    //
    // Parameters:
    // - `id` — input value
    // - `result` — input value
    //
    // Returns:
    // Nothing.
    //
    // Options:
    // None.
    //
    // Example:
    // const result = setHandleResult(id, result);

    const handle = this.handles.get(id);
    if (handle) handle.result = result;
  }

  drainFireAndForgetQueue(): number[] {
    // DrainFireAndForgetQueue.
    //
    // Parameters:
    // None.
    //
    // Returns:
    // number[].
    //
    // Options:
    // None.
    //
    // Example:
    // const result = drainFireAndForgetQueue();

    const queue = [...this.fireAndForgetQueue];
    this.fireAndForgetQueue = [];
    return queue;
  }

  registerAgentRoute(from: string, to: string, messageType: string): void {
    // RegisterAgentRoute.
    //
    // Parameters:
    // - `from` — input value
    // - `to` — input value
    // - `messageType` — input value
    //
    // Returns:
    // Nothing.
    //
    // Options:
    // None.
    //
    // Example:
    // const result = registerAgentRoute(from, to, messageType);

    this.agentRoutes.push({ from, to, messageType });
  }

  sendAgent(from: string, to: string, value: RuntimeValue, line: number): void {
    // SendAgent.
    //
    // Parameters:
    // - `from` — input value
    // - `to` — input value
    // - `value` — input value
    // - `line` — input value
    //
    // Returns:
    // Nothing.
    //
    // Options:
    // None.
    //
    // Example:
    // const result = sendAgent(from, to, value, line);

    const allowed = this.agentRoutes.some((route) => route.from === from && route.to === to);
    if (!allowed) {
      throw new RuntimeError(`No agent channel from '${from}' to '${to}'`, line);
    }
    const route = this.agentRoutes.find((r) => r.from === from && r.to === to);
    if (route?.messageType) {
      const actual = runtimeTypeTag(value);
      const expected = `object:${route.messageType}`;
      if (actual !== expected && actual !== route.messageType) {
        throw new RuntimeError(
          `Agent message type mismatch: expected ${route.messageType}, got ${actual}`,
          line,
        );
      }
    }
    const inbox = this.agentInboxes.get(to) ?? [];
    inbox.push(value);
    this.agentInboxes.set(to, inbox);
  }

  tryRecvAgent(agent: string): RuntimeValue | null {
    // TryRecvAgent.
    //
    // Parameters:
    // - `agent` — input value
    //
    // Returns:
    // Some value on success, otherwise none.
    //
    // Options:
    // None.
    //
    // Example:
    // const result = tryRecvAgent(agent);

    const inbox = this.agentInboxes.get(agent);
    return inbox?.shift() ?? null;
  }
}
