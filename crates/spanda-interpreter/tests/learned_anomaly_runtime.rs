//! Runtime learned anomaly backend polling during health transitions.

use spanda_ast::nodes::UnitKind;
use spanda_interpreter::{run_program, RunOptions};
use spanda_lexer::tokenize;
use spanda_parser::parse;
use spanda_runtime::provider_runtime::{ProviderDispatchContext, ProviderRuntime};
use spanda_runtime::providers::ProviderRegistry;
use spanda_runtime::value::RuntimeValue;
use std::sync::Arc;

struct LearnedAnomalyTestRuntime;

impl ProviderRuntime for LearnedAnomalyTestRuntime {
    fn bootstrap_providers_for_packages(&self, package_names: &[&str]) -> ProviderRegistry {
        let mut registry = ProviderRegistry::new();
        let mut names: Vec<String> = package_names
            .iter()
            .map(|name| (*name).to_string())
            .collect();
        if !names.iter().any(|name| name == "spanda-anomaly") {
            names.push("spanda-anomaly".into());
        }
        registry.set_official_packages(names);
        registry.grant_capability("assurance.anomaly.scan");
        registry
    }

    fn sync_comm_bus(&self, _comm_bus: &mut dyn std::any::Any, _registry: &mut ProviderRegistry) {}

    fn dispatch_official_package_call(
        &self,
        _registry: &mut ProviderRegistry,
        _module_path: &str,
        function_name: &str,
        _args: &[RuntimeValue],
        _context: ProviderDispatchContext<'_>,
    ) -> Option<RuntimeValue> {
        if function_name == "scan_learned" {
            Some(RuntimeValue::Number {
                value: 1.0,
                unit: UnitKind::None,
            })
        } else {
            None
        }
    }
}

#[test]
fn learned_anomaly_backend_triggers_handler_on_health_fault() {
    let source = r#"
hardware H {
    sensors [GPS, Lidar];
    actuators [DifferentialDrive];
}

anomaly_detector NavigationML {
    learned backend assurance.anomaly;
    expected localization.confidence >= 0.80;
}

on anomaly NavigationML severity High {
    enter degraded_mode;
}

robot Rover {
    sensor gps: GPS;
    sensor lidar: Lidar;
    actuator wheels: DifferentialDrive;
    safety { max_speed = 1.0 m/s; }
    behavior patrol() {
        loop every 50ms {
            let _ = gps.read();
        }
    }
}
"#;
    let program = parse(tokenize(source).unwrap()).unwrap();
    let result = run_program(
        &program,
        RunOptions {
            inject_health_faults: true,
            max_loop_iterations: 3,
            provider_runtime: Some(Arc::new(LearnedAnomalyTestRuntime)),
            ..Default::default()
        },
    )
    .expect("run");
    assert!(
        result
            .logs
            .iter()
            .any(|line| line.contains("learned anomaly: NavigationML")),
        "expected learned anomaly scan log, got: {:?}",
        result.logs
    );
    assert!(
        result
            .logs
            .iter()
            .any(|line| line.contains("anomaly: applying handler for NavigationML")),
        "expected anomaly handler log, got: {:?}",
        result.logs
    );
}
