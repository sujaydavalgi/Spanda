//! Injectable provider dispatch boundary for official package runtime wiring.
//!
use crate::providers::ProviderRegistry;
use crate::replay::MissionTrace;
use crate::telemetry::RuntimeTelemetry;
use crate::value::RuntimeValue;
use std::sync::Arc;

/// Optional observability sinks for provider dispatch.
pub struct ProviderDispatchContext<'a> {
    pub telemetry: Option<&'a mut RuntimeTelemetry>,
    pub mission_trace: Option<&'a mut MissionTrace>,
    pub sim_time_ms: f64,
}

/// Extension points for official package provider bootstrap and dispatch.
pub trait ProviderRuntime: Send + Sync {
    fn bootstrap_providers_for_packages(&self, package_names: &[&str]) -> ProviderRegistry;

    fn sync_comm_bus(&self, comm_bus: &mut dyn std::any::Any, registry: &mut ProviderRegistry);

    fn dispatch_official_package_call(
        &self,
        registry: &mut ProviderRegistry,
        module_path: &str,
        function_name: &str,
        args: &[RuntimeValue],
        context: ProviderDispatchContext<'_>,
    ) -> Option<RuntimeValue>;
}

/// Built-in provider runtime with empty bootstrap and no official package dispatch.
#[derive(Debug, Default, Clone, Copy)]
pub struct BuiltinProviderRuntime;

impl ProviderRuntime for BuiltinProviderRuntime {
    fn bootstrap_providers_for_packages(&self, package_names: &[&str]) -> ProviderRegistry {
        let mut registry = ProviderRegistry::new();
        registry.set_official_packages(
            package_names
                .iter()
                .map(|name| (*name).to_string())
                .collect(),
        );
        registry
    }

    fn sync_comm_bus(&self, _comm_bus: &mut dyn std::any::Any, _registry: &mut ProviderRegistry) {}

    fn dispatch_official_package_call(
        &self,
        _registry: &mut ProviderRegistry,
        _module_path: &str,
        _function_name: &str,
        _args: &[RuntimeValue],
        _context: ProviderDispatchContext<'_>,
    ) -> Option<RuntimeValue> {
        None
    }
}

/// Shared provider runtime handle passed through run options at the driver boundary.
pub type SharedProviderRuntime = Arc<dyn ProviderRuntime>;

/// Default built-in provider runtime for direct interpreter use without providers crate.
pub fn default_provider_runtime() -> SharedProviderRuntime {
    Arc::new(BuiltinProviderRuntime)
}
