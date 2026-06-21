# Secure Communication

Spanda extends its existing identity, capability, audit, and signed-topic security model with **optional encrypted communication** across buses, topics, services, and actions.

## Philosophy

| Boundary | Encryption | Authentication |
|----------|------------|----------------|
| In-process simulation | Optional | None |
| Inter-process (local) | Recommended | Signed (optional) |
| Inter-device / robot-to-robot | **Required** | Signed |
| Robot-to-cloud | **Required** | Mutual |
| Operator-to-robot | **Required** | **Mutual** |
| Actuator commands over network | Encrypted + signed + verified | Mutual |

Encryption is optional for internal simulation and development. Deployments crossing trust boundaries should declare explicit policies.

Publish-time **trusted-source enforcement** runs when `trusted_sources` is set on a secure topic. Inbound **receive** and transport poll paths verify trusted sources when publisher identity is available on the message envelope.

Broker URLs using TLS schemes (`mqtts://`, `wss://`, `dds+sec://`) automatically upgrade encryption to `required`. Adapters validate that a negotiated TLS session exists before publishing. When a certificate path is configured and the PEM file exists on disk, rustls parses it during session negotiation.

Supported transports: `local`, `ros2`, `dds`, `mqtt`, `websocket`, `ble`, `wifi`, and `cellular`. Live broker integrations use the same wire frame and session material derived from configured cert/key secrets.

| Transport | Rust (optional) | TypeScript (optional) |
|-----------|-----------------|------------------------|
| MQTT | `--features live-mqtt` + `SPANDA_LIVE_MQTT=1` | `SPANDA_LIVE_MQTT=1` (uses `mqtt` npm client) |
| WebSocket | `--features live-websocket` + `SPANDA_LIVE_WEBSOCKET=1` | `SPANDA_LIVE_WEBSOCKET=1` (uses `ws`) |
| DDS | `--features live-dds` + `SPANDA_LIVE_DDS=1` | `SPANDA_LIVE_DDS=1` (UDP multicast) |

Build all live adapters at once with `--features live-transport`.

### mTLS handshake (optional)

When mutual authentication is configured and cert/key PEM files exist on disk, the runtime can perform a real TLS handshake against the broker URL (Rust: rustls; TypeScript: Node `tls` when `SPANDA_MTLS_HANDSHAKE=1`). Session keys for wire encryption are derived from the negotiated peer certificate hash. Set `SPANDA_MTLS_REQUIRED=1` to fail configuration when the handshake cannot complete (instead of falling back to cert-derived AES-GCM material).

## Robot-wide policy

```spanda
secure_comm {
    encryption: required;
    authentication: mutual;
    integrity: required;
}
```

Modes:

- `encryption`: `none` | `optional` | `required`
- `authentication`: `none` | `signed` | `mutual`
- `integrity`: `none` | `required`

## Secure buses

```spanda
bus robot_mesh {
    transport: "dds";
    url: "dds+sec://fleet.local:7400";
    encryption: required;
    authentication: mutual;
}
```

Broker URLs can also come from the `SPANDA_BROKER_URL` environment variable when the bus block omits `url`.

Legacy shorthand remains supported: `bus ros2;`

## Secure topics, services, and actions

```spanda
topic lidar_scan: Topic<LidarScan> {
    secure {
        encryption required;
        signed required;
        trusted_sources [LidarFront];
    }
}

service GetBattery: Service<BatteryRequest, BatteryStatus> {
    secure {
        encryption required;
        authentication mutual;
    }
}
```

Signed messages protect integrity. Mutual authentication protects identity on operator and cloud links.

## Message types

- `EncryptedMessage<T>` — payload inaccessible until decrypted
- `SignedMessage<T>` — must be verified before trusted use
- `VerifiedMessage<T>` — signature-checked envelope for actuator paths
- `TrustedSource`, `Certificate`, `PublicKey`, `PrivateKey`, `SessionKey`

Actuator commands crossing trust boundaries must use `VerifiedMessage<SafeAction>`.

## Capabilities

Crypto and secure comm operations require declared capabilities:

- `crypto.encrypt`, `crypto.decrypt`, `crypto.sign`, `crypto.verify`
- `identity.read`, `secret.read`
- `secure_topic.publish`, `secure_topic.subscribe`

## CLI

```bash
spanda security check robot.sd
spanda security audit robot.sd
spanda run robot.sd --secure
spanda sim robot.sd --inject-security-faults
```

## Transport adapters

Spanda transport uses a versioned **wire frame** (`TransportWireFrame` v1) with JSON payload, optional `source_id`, and AES-256-GCM encryption when bus policy requires it. Frames on the wire are prefixed with `spanda/wire/v1:` followed by hex-encoded ciphertext.

Broker URLs using TLS schemes (`mqtts://`, `wss://`, `dds+sec://`) automatically upgrade encryption to `required`. Adapters validate that a negotiated TLS session exists before publishing.

Supported transports: `local`, `ros2`, `dds`, `mqtt`, `websocket`, `ble`, `wifi`, and `cellular`. Live broker integrations use the same wire frame and session material derived from configured cert/key secrets.

## Examples

See `examples/security/` for encrypted topics, robot-to-robot mesh, operator commands, cloud links, and fault injection scenarios.

## Related docs

- [Identity and trust](identity.md)
- [Secrets management](secrets.md)
- [Trust boundaries](trust-boundaries.md)
- [Security foundation](../security.md)
