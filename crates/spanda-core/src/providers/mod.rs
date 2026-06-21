//! Lean-core provider contracts and registry for optional domain packages.
//!
//! Spanda Core defines extension traits (sensor, transport, navigation, vision, etc.).
//! Official packages under `packages/registry/` implement these traits and register
//! at runtime. Legacy modules in `spanda-core` remain as compatibility shims until
//! callers migrate to package imports.
//!
pub mod bootstrap;
pub mod classification;
pub mod registry;
pub mod traits;
pub mod types;

pub use bootstrap::{
    bootstrap_default_providers, bootstrap_providers_for_packages, official_package_for_transport,
    sync_comm_bus_for_official_packages,
};
pub use classification::{
    module_classifications, official_package_names, ModuleClassification, ModuleOwnership,
};
pub use registry::ProviderRegistry;
pub use traits::{
    ActuatorProvider, CloudProvider, ConnectivityProvider, CryptoProvider, FleetProvider,
    HalProvider, LedgerProvider, MaintenanceProvider, NavigationProvider, PositioningProvider,
    RosProvider, SensorProvider, SimulationProvider, SlamProvider, TransportAdapterProvider,
    TransportProvider, VisionProvider,
};
pub use types::{
    ProviderCapability, ProviderCapabilitySet, ProviderError, ProviderId, ProviderMetadata,
    ProviderResult, ProviderSafetyLevel,
};
