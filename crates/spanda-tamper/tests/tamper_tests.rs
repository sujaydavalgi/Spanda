//! Integration tests for verify-time tamper analysis.

use spanda_lexer::tokenize;
use spanda_parser::parse;
use spanda_tamper::{generate_tamper_check, TamperSeverity, TamperStatus};

fn parse_file(relative: &str) -> spanda_ast::nodes::Program {
    let path = format!("{}/{}", env!("CARGO_MANIFEST_DIR"), relative);
    let source = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {path}: {e}"));
    let tokens = tokenize(&source).expect("tokenize");
    parse(tokens).expect("parse")
}

#[test]
fn warehouse_tamper_check_is_suspicious_not_compromised() {
    let program = parse_file("../../examples/showcase/policy/warehouse.sd");
    let report = generate_tamper_check(&program, "warehouse.sd");
    assert!(
        report.status == TamperStatus::Suspicious || report.status == TamperStatus::Tampered,
        "expected suspicious/tampered, got {:?}",
        report.status
    );
    assert!(report.findings.iter().any(|f| f.severity >= TamperSeverity::Medium));
    assert!(report.passed);
}

#[test]
fn secure_boot_import_reports_contract_coverage() {
    let source = r#"
import trust.jetson;
hardware JetsonRover { sensors [ GPS ]; actuators [ DifferentialDrive ]; }
robot Rover {
  uses hardware JetsonRover;
  sensor gps: GPS;
  actuator wheels: DifferentialDrive;
  behavior patrol() { wheels.drive(0.1 m/s); }
}
deploy Rover to JetsonRover;
"#;
    let tokens = tokenize(source).expect("tokenize");
    let program = parse(tokens).expect("parse");
    let report = generate_tamper_check(&program, "jetson.sd");
    assert!(
        report
            .findings
            .iter()
            .any(|finding| finding.category == "secure_boot"),
        "expected secure_boot finding, got {:?}",
        report.findings
    );
    assert!(
        !report
            .findings
            .iter()
            .any(|finding| {
                finding.category == "package"
                    && finding.message.contains("trust.jetson")
            }),
        "secure-boot import should not emit generic third-party package warning"
    );
}

#[test]
fn readiness_rover_missing_kill_switch_fails() {
    let program = parse_file("../../examples/showcase/readiness/rover.sd");
    let report = generate_tamper_check(&program, "rover.sd");
    assert!(!report.passed);
    assert!(
        report.findings.iter().any(|f| f.severity == TamperSeverity::Critical),
        "expected critical finding for missing kill switch"
    );
}
