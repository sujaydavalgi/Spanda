/**
 * Project-scoped provider bootstrap mirrored from Rust `providers/bootstrap.rs`.
 * @module
 */

import type { TransportKind } from "../comm/index.js";
import {
  createTransportStub,
  defaultTransportSecurity,
  type RoutingCommBus,
  type TransportConfig,
  TlsTransportSession,
} from "../transport/index.js";
import { ProviderRegistry, transportRegistryKey } from "./registry.js";

/** Map a transport kind to the official package that backs it when installed. */
export function officialPackageForTransport(kind: TransportKind): string | null {
  // Description:
  //     OfficialPackageForTransport.
  //
  // Inputs:
  //     kind: TransportKind
  //         Caller-supplied kind.
  //
  // Outputs:
  //     result: string | null
  //         Return value from `officialPackageForTransport`.
  //
  // Example:

  //     const result = officialPackageForTransport(kind);

  switch (kind) {
    case "ros2":
      return "spanda-ros2";
    case "mqtt":
      return "spanda-mqtt";
    case "dds":
      return "spanda-dds";
    case "websocket":
      return "spanda-ble";
    default:
      return null;
  }
}

function registerTransportStub(
  registry: ProviderRegistry,
  packageName: string,
  kind: TransportKind,
): void {
  // Description:
  //     RegisterTransportStub.
  //
  // Inputs:
  //     registry: ProviderRegistry
  //         Caller-supplied registry.
  //     packageName: string
  //         Caller-supplied packageName.
  //     kind: TransportKind
  //         Caller-supplied kind.
  //
  // Outputs:
  //     None.
  //
  // Example:

  //     const result = registerTransportStub(registry, packageName, kind);

  registry.registerTransport(transportRegistryKey(packageName), createTransportStub(kind));
}

/** Build a provider registry from installed official package names. */
export function bootstrapProvidersForPackages(packageNames: readonly string[]): ProviderRegistry {
  // Description:
  //     BootstrapProvidersForPackages.
  //
  // Inputs:
  //     packageNames: readonly string[]
  //         Caller-supplied packageNames.
  //
  // Outputs:
  //     result: ProviderRegistry
  //         Return value from `bootstrapProvidersForPackages`.
  //
  // Example:

  //     const result = bootstrapProvidersForPackages(packageNames);

  const registry = new ProviderRegistry();
  registry.setOfficialPackages([...packageNames]);
  registry.grantCapability("mqtt.publish");
  registry.grantCapability("mqtt.subscribe");
  registry.grantCapability("comm.ros2.publish");
  registry.grantCapability("comm.ros2.subscribe");

  const names = new Set(packageNames);
  const includeAll = names.size === 0;

  if (includeAll || names.has("spanda-mqtt")) {
    registerTransportStub(registry, "spanda-mqtt", "mqtt");
  }
  if (includeAll || names.has("spanda-ros2")) {
    registerTransportStub(registry, "spanda-ros2", "ros2");
  }
  if (names.has("spanda-dds")) {
    registry.grantCapability("dds.publish");
    registry.grantCapability("dds.subscribe");
    registerTransportStub(registry, "spanda-dds", "dds");
  }
  if (names.has("spanda-ble") || names.has("spanda-wifi")) {
    registry.grantCapability("connectivity.wifi");
    registry.grantCapability("connectivity.ble");
    registerTransportStub(registry, "spanda-ble", "websocket");
  }
  if (names.has("spanda-gps")) {
    registry.grantCapability("positioning.read");
  }
  if (names.has("spanda-nav") || names.has("spanda-nav2")) {
    registry.grantCapability("navigation.plan");
  }
  if (names.has("spanda-slam")) {
    registry.grantCapability("slam.localize");
    registry.grantCapability("slam.map");
  }
  if (names.has("spanda-fleet")) {
    registry.grantCapability("fleet.orchestrate");
  }
  if (names.has("spanda-ota")) {
    registry.grantCapability("deploy.rollout");
  }
  if (names.has("spanda-opencv") || names.has("spanda-yolo")) {
    registry.grantCapability("vision.detect");
  }
  if (names.has("spanda-gazebo") || names.has("spanda-webots")) {
    registry.grantCapability("simulation.step");
  }
  if (names.has("spanda-ledger")) {
    registry.grantCapability("audit.append");
  }
  if (names.has("spanda-cloud")) {
    registry.grantCapability("cloud.invoke");
  }

  return registry;
}

/** Register default compatibility-shim providers when no project manifest is available. */
export function bootstrapDefaultProviders(): ProviderRegistry {
  // Description:
  //     BootstrapDefaultProviders.
  //
  // Inputs:
  //     None.
  //
  // Outputs:
  //     result: ProviderRegistry
  //         Return value from `bootstrapDefaultProviders`.
  //
  // Example:

  //     const result = bootstrapDefaultProviders();

  return bootstrapProvidersForPackages([]);
}

function connectRegistryTransport(
  commBus: RoutingCommBus,
  registry: ProviderRegistry,
  kind: TransportKind,
  packageName: string,
  config: TransportConfig,
): void {
  // Description:
  //     ConnectRegistryTransport.
  //
  // Inputs:
  //     commBus: RoutingCommBus
  //         Caller-supplied commBus.
  //     registry: ProviderRegistry
  //         Caller-supplied registry.
  //     kind: TransportKind
  //         Caller-supplied kind.
  //     packageName: string
  //         Caller-supplied packageName.
  //     config: TransportConfig
  //         Caller-supplied config.
  //
  // Outputs:
  //     None.
  //
  // Example:

  //     const result = connectRegistryTransport(commBus, registry, kind, packageName, config);

  const key = transportRegistryKey(packageName);
  const connected = registry.withTransport(key, (provider) => {
    provider.connect(config);
    return true;
  });
  if (connected) {
    commBus.markRegistryBacked(kind, key);
  }
}

/** Connect comm-bus transports through installed official package providers. */
export function syncCommBusForOfficialPackages(
  commBus: RoutingCommBus,
  registry: ProviderRegistry,
): void {
  // Description:
  //     SyncCommBusForOfficialPackages.
  //
  // Inputs:
  //     commBus: RoutingCommBus
  //         Caller-supplied commBus.
  //     registry: ProviderRegistry
  //         Caller-supplied registry.
  //
  // Outputs:
  //     None.
  //
  // Example:

  //     const result = syncCommBusForOfficialPackages(commBus, registry);

  commBus.clearRegistryBacked();
  const base: TransportConfig = {
    security: defaultTransportSecurity(),
    tls: new TlsTransportSession(),
  };
  for (const name of registry.officialPackages()) {
    switch (name) {
      case "spanda-ros2":
        connectRegistryTransport(commBus, registry, "ros2", name, base);
        break;
      case "spanda-mqtt":
        connectRegistryTransport(commBus, registry, "mqtt", name, {
          ...base,
          brokerUrl: "mqtt://localhost:1883",
          clientId: "spanda",
        });
        break;
      case "spanda-dds":
        connectRegistryTransport(commBus, registry, "dds", name, {
          ...base,
          domainId: 0,
        });
        break;
      case "spanda-ble":
      case "spanda-wifi":
        connectRegistryTransport(commBus, registry, "websocket", "spanda-ble", {
          ...base,
          brokerUrl: "ws://localhost:9090",
        });
        break;
      default:
        break;
    }
  }
}
