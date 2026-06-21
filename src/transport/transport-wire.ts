/**
 * Canonical JSON wire frames for Spanda transport adapters.
 * @module
 */

import type { RuntimeValue } from "../runtime/interpreter.js";
import type { TransportKind } from "../comm/index.js";
import {
  TlsTransportSession,
  type TransportSecurityConfig,
  WIRE_PREFIX,
} from "./transport-security.js";

export type TransportWireFrame = {
  v: number;
  topic: string;
  message_type: string;
  payload: string;
  source_id?: string;
  transport: string;
};

export type FullTransportConfig = {
  brokerUrl?: string | null;
  nodeName?: string | null;
  namespace?: string | null;
  domainId?: number | null;
  clientId?: string | null;
  security: TransportSecurityConfig;
  tls: TlsTransportSession;
};

function runtimeValueToPlain(value: RuntimeValue): unknown {
  // Convert a runtime value into JSON-serializable data.
  switch (value.kind) {
    case "number":
      return { kind: "number", value: value.value, unit: value.unit };
    case "bool":
      return { kind: "bool", value: value.value };
    case "string":
      return { kind: "string", value: value.value };
    case "velocity":
      return { kind: "velocity", linear: value.linear, angular: value.angular };
    case "pose":
      return { kind: "pose", x: value.x, y: value.y, theta: value.theta, z: value.z };
    case "scan":
      return { kind: "scan", nearestDistance: value.nearestDistance };
    case "object":
      return {
        kind: "object",
        typeName: value.typeName,
        fields: Object.fromEntries(
          Object.entries(value.fields).map(([k, v]) => [k, runtimeValueToPlain(v)]),
        ),
      };
    default:
      return { kind: value.kind };
  }
}

export function runtimeValueToJson(value: RuntimeValue): string {
  // Serialize a runtime value to JSON for wire transport frames.
  //
  // Parameters:
  // - `value` — runtime value to encode
  //
  // Returns:
  // JSON string payload.
  //
  // Options:
  // None.
  //
  // Example:
  // const json = runtimeValueToJson(velocityValue);

  return JSON.stringify(runtimeValueToPlain(value));
}

function plainToRuntimeValue(data: Record<string, unknown>): RuntimeValue {
  // Rehydrate a runtime value from decoded wire JSON.
  const kind = String(data.kind ?? "string");
  switch (kind) {
    case "number":
      return {
        kind: "number",
        value: Number(data.value ?? 0),
        unit: (data.unit as "none") ?? "none",
      };
    case "bool":
      return { kind: "bool", value: Boolean(data.value) };
    case "string":
      return { kind: "string", value: String(data.value ?? "") };
    case "velocity":
      return { kind: "velocity", linear: Number(data.linear ?? 0), angular: Number(data.angular ?? 0) };
    case "pose":
      return {
        kind: "pose",
        x: Number(data.x ?? 0),
        y: Number(data.y ?? 0),
        theta: Number(data.theta ?? 0),
        z: Number(data.z ?? 0),
      };
    case "scan":
      return { kind: "scan", nearestDistance: Number(data.nearestDistance ?? 0) };
    case "object":
      return {
        kind: "object",
        typeName: String(data.typeName ?? "Object"),
        fields: Object.fromEntries(
          Object.entries((data.fields as Record<string, unknown>) ?? {}).map(([k, v]) => [
            k,
            plainToRuntimeValue(v as Record<string, unknown>),
          ]),
        ),
      };
    default:
      return { kind: "string", value: JSON.stringify(data) };
  }
}

export function runtimeValueFromJson(json: string): RuntimeValue {
  // Parse JSON wire payload back into a runtime value.
  //
  // Parameters:
  // - `json` — JSON payload string
  //
  // Returns:
  // RuntimeValue decoded from wire JSON.
  //
  // Options:
  // None.
  //
  // Example:
  // const value = runtimeValueFromJson(frame.payload);

  const data = JSON.parse(json) as Record<string, unknown>;
  return plainToRuntimeValue(data);
}

export function createTransportWireFrame(
  topic: string,
  messageType: string,
  value: RuntimeValue,
  sourceId: string | null | undefined,
  transport: TransportKind,
): TransportWireFrame {
  // Build a versioned transport wire frame envelope.
  //
  // Parameters:
  // - `topic` — topic path
  // - `messageType` — message type name
  // - `value` — payload runtime value
  // - `sourceId` — optional publisher identity
  // - `transport` — active transport kind
  //
  // Returns:
  // TransportWireFrame v1 object.
  //
  // Options:
  // None.
  //
  // Example:
  // const frame = createTransportWireFrame("/motion", "Velocity", val, "Nav", "mqtt");

  const frame: TransportWireFrame = {
    v: 1,
    topic,
    message_type: messageType,
    payload: runtimeValueToJson(value),
    transport,
  };
  if (sourceId) frame.source_id = sourceId;
  return frame;
}

export function encodeWireValue(
  config: FullTransportConfig,
  topic: string,
  messageType: string,
  value: RuntimeValue,
  sourceId: string | null | undefined,
  transport: TransportKind,
): RuntimeValue {
  // Encode a runtime value into a wire frame string, optionally encrypted.
  //
  // Parameters:
  // - `config` — full transport configuration with TLS session
  // - `topic` — topic path
  // - `messageType` — message type name
  // - `value` — payload runtime value
  // - `sourceId` — optional publisher identity
  // - `transport` — active transport kind
  //
  // Returns:
  // String runtime value containing wire frame JSON or ciphertext.
  //
  // Options:
  // None.
  //
  // Example:
  // const wire = encodeWireValue(cfg, "/t", "Velocity", val, "Nav", "mqtt");

  const frame = createTransportWireFrame(topic, messageType, value, sourceId, transport);
  const json = JSON.stringify(frame);
  if (config.security.encryption === "none") {
    return { kind: "string", value: json };
  }
  return { kind: "string", value: config.tls.encryptFrame(json) };
}

export function decodeWireValue(
  config: FullTransportConfig,
  value: RuntimeValue,
): { value: RuntimeValue; sourceId: string | null } {
  // Decode an adapter wire value back to payload and optional source id.
  //
  // Parameters:
  // - `config` — full transport configuration with TLS session
  // - `value` — wire runtime value from adapter receive
  //
  // Returns:
  // Decoded payload value and optional source_id.
  //
  // Options:
  // None.
  //
  // Example:
  // const { value, sourceId } = decodeWireValue(cfg, wireValue);

  if (value.kind !== "string") {
    return { value, sourceId: null };
  }
  const wireText =
    config.security.encryption === "none"
      ? value.value
      : config.tls.decryptFrame(value.value);
  const frame = JSON.parse(wireText) as TransportWireFrame;
  return {
    value: runtimeValueFromJson(frame.payload),
    sourceId: frame.source_id ?? null,
  };
}

export { WIRE_PREFIX };
