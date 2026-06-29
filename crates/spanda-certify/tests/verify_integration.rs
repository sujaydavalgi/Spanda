//! Certification proof checklist tests for strict verify.

use spanda_certify::verify_certification_proof;
use spanda_config::verify_with_system_config;
use spanda_driver::compile;
use spanda_hardware::{CompatSeverity, VerifyOptions};

#[test]
fn deploy_without_certify_warns_by_default() {
    // Description:
    //     Deploy without certify warns by default.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     None.
    //
    // Example:

    //     let result = spanda_certify::verify_integration::deploy_without_certify_warns_by_default();

    let source = r#"
hardware Tiny {
  actuators [ DifferentialDrive ];
}

robot Rover {
  actuator wheels: DifferentialDrive;
  behavior run() { wheels.stop(); }
}

deploy Rover to Tiny;
"#;
    let program = compile(source).expect("compile").program;
    let items = verify_certification_proof(&program, false);
    assert!(
        items
            .iter()
            .any(|i| i.category == "certify" && i.severity == CompatSeverity::Warning),
        "expected warning for deploy without certify"
    );
}

#[test]
fn deploy_without_certify_errors_under_strict() {
    // Description:
    //     Deploy without certify errors under strict.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     None.
    //
    // Example:

    //     let result = spanda_certify::verify_integration::deploy_without_certify_errors_under_strict();

    let source = r#"
hardware Tiny {
  actuators [ DifferentialDrive ];
}

robot Rover {
  actuator wheels: DifferentialDrive;
  behavior run() { wheels.stop(); }
}

deploy Rover to Tiny;
"#;
    let program = compile(source).expect("compile").program;
    let items = verify_certification_proof(&program, true);
    assert!(
        items
            .iter()
            .any(|i| i.category == "certify" && i.severity == CompatSeverity::Error),
        "expected error for deploy without certify under strict"
    );
}

#[test]
fn iso13849_without_level_errors_under_strict() {
    // Description:
    //     Iso13849 without level errors under strict.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     None.
    //
    // Example:

    //     let result = spanda_certify::verify_integration::iso13849_without_level_errors_under_strict();

    let source = r#"
certify ISO13849;

hardware Tiny {
  actuators [ DifferentialDrive ];
}

robot Rover {
  actuator wheels: DifferentialDrive;
  safety { max_speed = 0.5 m/s; }
  mission Patrol { navigate; }
  behavior run() { wheels.stop(); }
}

deploy Rover to Tiny;
"#;
    let program = compile(source).expect("compile").program;
    let items = verify_certification_proof(&program, true);
    assert!(
        items.iter().any(|i| {
            i.category == "certify"
                && i.severity == CompatSeverity::Error
                && i.message.contains("performance level")
        }),
        "expected ISO13849 level error under strict"
    );
}

#[test]
fn strict_certify_flag_surfaces_in_verify_report() {
    // Description:
    //     Strict certify flag surfaces in verify report.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     None.
    //
    // Example:

    //     let result = spanda_certify::verify_integration::strict_certify_flag_surfaces_in_verify_report();

    let source = include_str!("../../../examples/robotics/ota_deployment.sd");
    let program = compile(source).expect("compile").program;
    let report = verify_with_system_config(
        &program,
        None,
        VerifyOptions {
            all_targets: true,
            strict_certify: true,
            ..Default::default()
        },
    );
    assert!(
        report
            .items
            .iter()
            .any(|i| i.category == "certify" && i.severity == CompatSeverity::Error),
        "OTA example without certify should fail strict verify"
    );
}
