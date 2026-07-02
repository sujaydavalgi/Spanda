//! OTA platform event emission tests.

use spanda_audit::platform_event::names;
use spanda_ota::{plan_rollout, DeployAssignment, DeployPlan, RolloutOptions, RolloutStrategy};
use spanda_runtime::platform_event_runtime::{set_platform_event_runtime, PlatformEventRuntime};
use std::sync::{Arc, Mutex};

struct CapturePlatformEvents {
    types: Mutex<Vec<String>>,
}

impl PlatformEventRuntime for CapturePlatformEvents {
    fn record_platform_event(&self, event: &spanda_audit::PlatformEvent) {
        self.types
            .lock()
            .unwrap()
            .push(event.event_type.as_str().to_string());
    }
}

#[test]
fn plan_rollout_emits_ota_platform_events() {
    let capture = Arc::new(CapturePlatformEvents {
        types: Mutex::new(Vec::new()),
    });
    let _ = set_platform_event_runtime(Arc::clone(&capture) as Arc<dyn PlatformEventRuntime>);

    let plan = DeployPlan {
        program: "fleet_demo.sd".into(),
        version: "2.0.0".into(),
        program_hash: None,
        assignments: vec![DeployAssignment {
            robot_name: "Rover".into(),
            hardware: "Jetson".into(),
        }],
        certifications: vec![],
        certification_proof: None,
    };
    let options = RolloutOptions {
        strategy: RolloutStrategy::All,
        version: "2.0.0".into(),
        dry_run: true,
        ..RolloutOptions::default()
    };
    let _result = plan_rollout(&plan, &options);

    let types = capture.types.lock().unwrap();
    assert!(types.iter().any(|t| t == names::OTA_ROLLOUT_STARTED));
    assert!(types.iter().any(|t| t == names::OTA_ROLLOUT_COMPLETED));
}
