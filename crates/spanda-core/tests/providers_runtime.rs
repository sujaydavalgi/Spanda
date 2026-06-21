//! Runtime provider registry wiring tests.
//!
use spanda_core::runtime::{Interpreter, InterpreterOptions};
use spanda_core::simulator::{create_default_simulator, SimulatorConfig};

#[test]
fn interpreter_bootstraps_provider_registry_by_default() {
    let sim = create_default_simulator(SimulatorConfig::default());
    let interp = Interpreter::new(sim, InterpreterOptions::default());
    assert!(interp.provider_registry().transport_count() >= 2);
}
