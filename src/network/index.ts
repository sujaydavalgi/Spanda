/** std.network — communication and transport domain types for Spanda programs. */

export const STD_NETWORK_TYPES = [
  "Transport",
  "QosProfile",
  "QoS",
  "Bandwidth",
  "Latency",
  "TopicPath",
  "ServiceEndpoint",
  "MessageEnvelope",
  "DiscoveryFilter",
  "NetworkRequirements",
  "Reliability",
  "HistoryPolicy",
  "CommBus",
  "Endpoint",
  "Topic",
  "Message",
  "Service",
  "Action",
] as const;

export type StdNetworkType = (typeof STD_NETWORK_TYPES)[number];

export function isStdNetworkType(name: string): name is StdNetworkType {
  // Description:
  //     IsStdNetworkType.
  //
  // Inputs:
  //     name: string
  //         Caller-supplied name.
  //
  // Outputs:
  //     result: name is StdNetworkType
  //         Return value from `isStdNetworkType`.
  //
  // Example:
  //     const result = isStdNetworkType(name);
  // Description:
  //     IsStdNetworkType.
  //
  // Inputs:
  //     name: string
  //         Caller-supplied name.
  //
  // Outputs:
  //     result: name is StdNetworkType
  //         Return value from `isStdNetworkType`.
  //
  // Example:
  //     const result = isStdNetworkType(name);

  // const result = isStdNetworkType(name);
  return (STD_NETWORK_TYPES as readonly string[]).includes(name);
}

export function resolveStdNetworkImport(path: string): boolean {
  // Description:
  //     ResolveStdNetworkImport.
  //
  // Inputs:
  //     path: string
  //         Caller-supplied path.
  //
  // Outputs:
  //     result: boolean
  //         Return value from `resolveStdNetworkImport`.
  //
  // Example:
  //     const result = resolveStdNetworkImport(path);
  // Description:
  //     ResolveStdNetworkImport.
  //
  // Inputs:
  //     path: string
  //         Caller-supplied path.
  //
  // Outputs:
  //     result: boolean
  //         Return value from `resolveStdNetworkImport`.
  //
  // Example:
  //     const result = resolveStdNetworkImport(path);

  // const result = resolveStdNetworkImport(path);
  return path === "std.network";
}
