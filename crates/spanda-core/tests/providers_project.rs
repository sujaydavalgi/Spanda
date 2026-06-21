//! Project-scoped official package provider wiring tests.
//!
use spanda_core::providers::bootstrap_providers_for_packages;
use spanda_core::runtime::{Interpreter, InterpreterOptions};
use spanda_core::simulator::{create_default_simulator, SimulatorConfig};
use spanda_package::load_official_packages_for_project;
use std::path::Path;

#[test]
fn ros2_adapter_project_loads_official_packages() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../examples/packages/ros2_adapter_package");
    let packages = load_official_packages_for_project(&root).expect("manifest");
    assert!(packages.iter().any(|p| p == "spanda-ros2"));
}

#[test]
fn interpreter_logs_official_packages_from_project_deps() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../examples/packages/ros2_adapter_package");
    let packages = load_official_packages_for_project(&root).expect("manifest");
    let registry = bootstrap_providers_for_packages(
        &packages.iter().map(String::as_str).collect::<Vec<_>>(),
    );
    assert!(registry.has_official_package("spanda-ros2"));

    let sim = create_default_simulator(SimulatorConfig::default());
    let interp = Interpreter::new(
        sim,
        InterpreterOptions {
            official_packages: packages,
            ..Default::default()
        },
    );
    assert!(interp.provider_registry().has_official_package("spanda-ros2"));
}
