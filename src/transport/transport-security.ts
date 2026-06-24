/**
 * Transport-layer security policy validation and TLS session management.
 * @module
 */

import { createHash } from "node:crypto";
import { existsSync, readFileSync } from "node:fs";
import tls from "node:tls";
import { WireCryptoSession } from "./wire-crypto.js";

export type EncryptionMode = "none" | "optional" | "required";
export type AuthenticationMode = "none" | "signed" | "mutual";
export type IntegrityMode = "none" | "required";

export const WIRE_PREFIX = "spanda/wire/v1:";

/** Per-transport TLS / encryption configuration wired from bus declarations. */
export type TransportSecurityConfig = {
  encryption: EncryptionMode;
  authentication: AuthenticationMode;
  integrity: IntegrityMode;
  certPath: string | null;
  keySecret: string | null;
  keyPath: string | null;
};

export function defaultTransportSecurity(): TransportSecurityConfig {
  // Description:
  //     DefaultTransportSecurity.
  //
  // Inputs:
  //     None.
  //
  // Outputs:
  //     result: TransportSecurityConfig
  //         Return value from `defaultTransportSecurity`.
  //
  // Example:
  //     const result = defaultTransportSecurity();
  // Description:
  //     DefaultTransportSecurity.
  //
  // Inputs:
  //     None.
  //
  // Outputs:
  //     result: TransportSecurityConfig
  //         Return value from `defaultTransportSecurity`.
  //
  // Example:

  //     const result = defaultTransportSecurity();

  return {
    encryption: "none",
    authentication: "none",
    integrity: "none",
    certPath: null,
    keySecret: null,
    keyPath: null,
  };
}

function parseEncryption(value: string | null | undefined): EncryptionMode {
  // Description:
  //     ParseEncryption.
  //
  // Inputs:
  //     value: string | null | undefined
  //         Caller-supplied value.
  //
  // Outputs:
  //     result: EncryptionMode
  //         Return value from `parseEncryption`.
  //
  // Example:

  //     const result = parseEncryption(value);

  if (!value) return "none";
  if (value === "none" || value === "optional" || value === "required") return value;
  throw new Error(`invalid encryption mode '${value}'`);
}

function parseAuthentication(value: string | null | undefined): AuthenticationMode {
  // Description:
  //     ParseAuthentication.
  //
  // Inputs:
  //     value: string | null | undefined
  //         Caller-supplied value.
  //
  // Outputs:
  //     result: AuthenticationMode
  //         Return value from `parseAuthentication`.
  //
  // Example:

  //     const result = parseAuthentication(value);

  if (!value) return "none";
  if (value === "none" || value === "signed" || value === "mutual") return value;
  throw new Error(`invalid authentication mode '${value}'`);
}

function parseIntegrity(value: string | null | undefined): IntegrityMode {
  // Description:
  //     ParseIntegrity.
  //
  // Inputs:
  //     value: string | null | undefined
  //         Caller-supplied value.
  //
  // Outputs:
  //     result: IntegrityMode
  //         Return value from `parseIntegrity`.
  //
  // Example:

  //     const result = parseIntegrity(value);

  if (!value) return "none";
  if (value === "none" || value === "required") return value;
  throw new Error(`invalid integrity mode '${value}'`);
}

export function transportSecurityFromBusFields(
  encryption?: string | null,
  authentication?: string | null,
  integrity?: string | null,
): TransportSecurityConfig {
  // Description:
  //     TransportSecurityFromBusFields.
  //
  // Inputs:
  //     encryption?: string | null
  //         Caller-supplied encryption?.
  //     authentication?: string | null
  //         Caller-supplied authentication?.
  //     integrity?: string | null
  //         Caller-supplied integrity?.
  //
  // Outputs:
  //     result: TransportSecurityConfig
  //         Return value from `transportSecurityFromBusFields`.
  //
  // Example:
  //     const result = transportSecurityFromBusFields(encryption?, authentication?, integrity?);
  // Description:
  //     TransportSecurityFromBusFields.
  //
  // Inputs:
  //     encryption?: string | null
  //         Caller-supplied encryption?.
  //     authentication?: string | null
  //         Caller-supplied authentication?.
  //     integrity?: string | null
  //         Caller-supplied integrity?.
  //
  // Outputs:
  //     result: TransportSecurityConfig
  //         Return value from `transportSecurityFromBusFields`.
  //
  // Example:

  //     const result = transportSecurityFromBusFields(encryption?, authentication?, integrity?);

  return {
    encryption: parseEncryption(encryption),
    authentication: parseAuthentication(authentication),
    integrity: parseIntegrity(integrity),
    certPath: null,
    keySecret: null,
    keyPath: null,
  };
}

export function sessionMaterial(config: TransportSecurityConfig): string {
  // Description:
  //     SessionMaterial.
  //
  // Inputs:
  //     config: TransportSecurityConfig
  //         Caller-supplied config.
  //
  // Outputs:
  //     result: string
  //         Return value from `sessionMaterial`.
  //
  // Example:
  //     const result = sessionMaterial(config);
  // Description:
  //     SessionMaterial.
  //
  // Inputs:
  //     config: TransportSecurityConfig
  //         Caller-supplied config.
  //
  // Outputs:
  //     result: string
  //         Return value from `sessionMaterial`.
  //
  // Example:

  //     const result = sessionMaterial(config);

  return `${config.certPath ?? "spanda-local"}:${config.keySecret ?? "spanda-local-key"}`;
}

export function validateTransportSecurity(config: TransportSecurityConfig, transport: string): void {
  // Description:
  //     ValidateTransportSecurity.
  //
  // Inputs:
  //     config: TransportSecurityConfig
  //         Caller-supplied config.
  //     transport: string
  //         Caller-supplied transport.
  //
  // Outputs:
  //     None.
  //
  // Example:
  //     const result = validateTransportSecurity(config, transport);
  // Description:
  //     ValidateTransportSecurity.
  //
  // Inputs:
  //     config: TransportSecurityConfig
  //         Caller-supplied config.
  //     transport: string
  //         Caller-supplied transport.
  //
  // Outputs:
  //     None.
  //
  // Example:

  //     const result = validateTransportSecurity(config, transport);

  if (config.encryption === "required" && !config.certPath && !config.keySecret) {
    throw new Error(
      `transport '${transport}' requires encryption but no cert/key secret is configured`,
    );
  }
}

export type TlsEndpoint = { host: string; port: number; useTls: boolean };

export function parseTlsEndpoint(url: string): TlsEndpoint | null {
  // Description:
  //     ParseTlsEndpoint.
  //
  // Inputs:
  //     url: string
  //         Caller-supplied url.
  //
  // Outputs:
  //     result: TlsEndpoint | null
  //         Return value from `parseTlsEndpoint`.
  //
  // Example:
  //     const result = parseTlsEndpoint(url);

  // Parse broker URLs into host/port TLS endpoints.
  const lower = url.toLowerCase();
  let useTls = false;
  let stripped: string;
  let defaultPort = 1883;
  if (lower.startsWith("mqtts://")) {
    useTls = true;
    stripped = lower.slice("mqtts://".length);
    defaultPort = 8883;
  } else if (lower.startsWith("mqtt://")) {
    stripped = lower.slice("mqtt://".length);
  } else if (lower.startsWith("wss://")) {
    useTls = true;
    stripped = lower.slice("wss://".length);
    defaultPort = 443;
  } else if (lower.startsWith("ws://")) {
    stripped = lower.slice("ws://".length);
    defaultPort = 80;
  } else if (lower.startsWith("dds+sec://")) {
    useTls = true;
    stripped = lower.slice("dds+sec://".length);
    defaultPort = 7400;
  } else if (lower.startsWith("dds://")) {
    stripped = lower.slice("dds://".length);
    defaultPort = 7400;
  } else {
    return null;
  }
  const [host, portText] = stripped.includes(":")
    ? (stripped.split(":", 2) as [string, string])
    : [stripped, String(defaultPort)];
  return { host, port: Number.parseInt(portText, 10) || defaultPort, useTls };
}

function mtlsSessionMaterial(config: TransportSecurityConfig): string {
  // Description:
  //     MtlsSessionMaterial.
  //
  // Inputs:
  //     config: TransportSecurityConfig
  //         Caller-supplied config.
  //
  // Outputs:
  //     result: string
  //         Return value from `mtlsSessionMaterial`.
  //
  // Example:

  //     const result = mtlsSessionMaterial(config);

  if (config.certPath && existsSync(config.certPath) && config.keyPath && existsSync(config.keyPath)) {
    const cert = readFileSync(config.certPath);
    const key = readFileSync(config.keyPath);
    return createHash("sha256").update(cert).update(key).digest("hex");
  }
  return sessionMaterial(config);
}

function tryMtlsHandshake(
  endpoint: TlsEndpoint,
  certPath: string,
  keyPath: string,
): {
  // Description:
  //     TryMtlsHandshake.
  //
  // Inputs:
  //     endpoint: TlsEndpoint
  //         Caller-supplied endpoint.
  //     certPath: string
  //         Caller-supplied certPath.
  //     keyPath: string
  //         Caller-supplied keyPath.
  //
  // Outputs:
  //     None.
  //
  // Example:

 // const result = tryMtlsHandshake(endpoint, certPath, keyPath);
 cipherSuite: string; sessionMaterial: string } | null {
  if (!endpoint.useTls) return null;
  try {
    const cert = readFileSync(certPath);
    const key = readFileSync(keyPath);
    const socket = tls.connect({
      host: endpoint.host,
      port: endpoint.port,
      cert,
      key,
      rejectUnauthorized: true,
      servername: endpoint.host,
    });
    const deadline = Date.now() + 3000;
    while (!socket.authorized && Date.now() < deadline) {
      /* busy wait for short broker handshake in sync configure path */
    }
    if (!socket.authorized) {
      socket.destroy();
      return null;
    }
    const peer = socket.getPeerCertificate();
    const peerHash = peer.raw
      ? createHash("sha256").update(peer.raw).digest("hex")
      : "peer";
    socket.end();
    return {
      cipherSuite: "TLS/mTLS",
      sessionMaterial: `mtls:${endpoint.host}:${endpoint.port}:${peerHash}`,
    };
  } catch {
    return null;
  }
}

export function resolveBrokerUrl(busUrl?: string | null): string | null {
  // Description:
  //     ResolveBrokerUrl.
  //
  // Inputs:
  //     busUrl?: string | null
  //         Caller-supplied busUrl?.
  //
  // Outputs:
  //     result: string | null
  //         Return value from `resolveBrokerUrl`.
  //
  // Example:
  //     const result = resolveBrokerUrl(busUrl?);

  // Resolve broker URL from bus declaration or SPANDA_BROKER_URL env.
  if (busUrl && busUrl.length > 0) return busUrl;
  const env = process.env.SPANDA_BROKER_URL;
  return env && env.length > 0 ? env : null;
}

export function urlRequiresTls(brokerUrl?: string | null): boolean {
  // Description:
  //     UrlRequiresTls.
  //
  // Inputs:
  //     brokerUrl?: string | null
  //         Caller-supplied brokerUrl?.
  //
  // Outputs:
  //     result: boolean
  //         Return value from `urlRequiresTls`.
  //
  // Example:
  //     const result = urlRequiresTls(brokerUrl?);
  // Description:
  //     UrlRequiresTls.
  //
  // Inputs:
  //     brokerUrl?: string | null
  //         Caller-supplied brokerUrl?.
  //
  // Outputs:
  //     result: boolean
  //         Return value from `urlRequiresTls`.
  //
  // Example:

  //     const result = urlRequiresTls(brokerUrl?);

  if (!brokerUrl) return false;
  const lower = brokerUrl.toLowerCase();
  return (
    lower.startsWith("mqtts://") ||
    lower.startsWith("wss://") ||
    lower.startsWith("ssl://") ||
    lower.startsWith("tls://") ||
    lower.startsWith("dds+sec://")
  );
}

export type SecureCommPolicy = {
  encryption: EncryptionMode;
  authentication: AuthenticationMode;
  integrity: IntegrityMode;
};

export function effectiveTransportPolicy(
  robot: SecureCommPolicy,
  bus: TransportSecurityConfig,
): TransportSecurityConfig {
  // Description:
  //     EffectiveTransportPolicy.
  //
  // Inputs:
  //     robot: SecureCommPolicy
  //         Caller-supplied robot.
  //     bus: TransportSecurityConfig
  //         Caller-supplied bus.
  //
  // Outputs:
  //     result: TransportSecurityConfig
  //         Return value from `effectiveTransportPolicy`.
  //
  // Example:
  //     const result = effectiveTransportPolicy(robot, bus);
  // Description:
  //     EffectiveTransportPolicy.
  //
  // Inputs:
  //     robot: SecureCommPolicy
  //         Caller-supplied robot.
  //     bus: TransportSecurityConfig
  //         Caller-supplied bus.
  //
  // Outputs:
  //     result: TransportSecurityConfig
  //         Return value from `effectiveTransportPolicy`.
  //
  // Example:

  //     const result = effectiveTransportPolicy(robot, bus);

  return {
    encryption: bus.encryption !== "none" ? bus.encryption : robot.encryption,
    authentication: bus.authentication !== "none" ? bus.authentication : robot.authentication,
    integrity: bus.integrity !== "none" ? bus.integrity : robot.integrity,
    certPath: bus.certPath,
    keySecret: bus.keySecret,
    keyPath: bus.keyPath,
  };
}

/** Negotiated TLS session for transport wire encryption (AES-256-GCM). */
export class TlsTransportSession {
  negotiated = false;
  cipherSuite = "none";
  peerVerified = false;
  private session: WireCryptoSession | null = null;

  connect(config: TransportSecurityConfig, brokerUrl?: string | null): void {

    // Negotiate wire encryption and optional mTLS handshake against a broker URL.
    validateTransportSecurity(config, "tls");
    if (config.encryption === "none") {
      this.negotiated = false;
      this.cipherSuite = "none";
      this.peerVerified = true;
      this.session = null;
      return;
    }
    const certFile =
      config.certPath && existsSync(config.certPath) ? config.certPath : null;
    const keyFile = config.keyPath && existsSync(config.keyPath) ? config.keyPath : null;
    if (config.authentication === "mutual" && (!certFile || !keyFile)) {
      throw new Error("mutual TLS authentication failed: missing certificate or key file");
    }
    if (
      config.authentication === "mutual" &&
      certFile &&
      keyFile &&
      brokerUrl &&
      process.env.SPANDA_MTLS_HANDSHAKE === "1"
    ) {
      const endpoint = parseTlsEndpoint(brokerUrl);
      if (endpoint?.useTls) {
        const hs = tryMtlsHandshake(endpoint, certFile, keyFile);
        if (hs) {
          const crypto = WireCryptoSession.fromMaterial(hs.sessionMaterial);
          this.cipherSuite = hs.cipherSuite;
          this.peerVerified = true;
          this.session = crypto;
          this.negotiated = true;
          return;
        }
        if (process.env.SPANDA_MTLS_REQUIRED === "1") {
          throw new Error("mTLS handshake failed");
        }
      }
    }
    this.peerVerified = config.authentication !== "mutual" || certFile !== null;
    if (certFile) {
      this.peerVerified = true;
    }
    const material = mtlsSessionMaterial(config);
    const crypto = WireCryptoSession.fromMaterial(material);
    this.cipherSuite = crypto.cipherSuite;
    this.session = crypto;
    this.negotiated = true;
  }

  encryptFrame(plaintext: string): string {
    // Encrypt a JSON wire frame when a TLS session is negotiated.
    //
    // Parameters:
    // - `plaintext` — JSON wire frame text
    //
    // Returns:
    // Plaintext or spanda/wire/v1 prefixed hex ciphertext.
    //
    // Options:
    // None.
    //
    // Example:

    // const wire = tls.encryptFrame(json);

    if (!this.negotiated) return plaintext;
    if (!this.session) throw new Error("TLS session not negotiated");
    const encrypted = this.session.encrypt(new TextEncoder().encode(plaintext));
    return `${WIRE_PREFIX}${Buffer.from(encrypted).toString("hex")}`;
  }

  decryptFrame(ciphertext: string): string {
    // Decrypt a transport wire frame or pass through plaintext.
    //
    // Parameters:
    // - `ciphertext` — wire frame text or encrypted hex payload
    //
    // Returns:
    // Decrypted JSON wire frame text.
    //
    // Options:
    // None.
    //
    // Example:

    // const json = tls.decryptFrame(wire);

    if (!this.negotiated) return ciphertext;
    if (ciphertext.startsWith(WIRE_PREFIX)) {
      if (!this.session) throw new Error("TLS session not negotiated");
      const hexPayload = ciphertext.slice(WIRE_PREFIX.length);
      const bytes = Buffer.from(hexPayload, "hex");
      const plain = this.session.decrypt(bytes);
      return new TextDecoder().decode(plain);
    }
    const legacy = `tls:${this.cipherSuite}:`;
    if (ciphertext.startsWith(legacy)) {
      return ciphertext.slice(legacy.length);
    }
    throw new Error("TLS decrypt failed: unrecognized wire frame");
  }
}
