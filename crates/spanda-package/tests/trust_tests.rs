//! Integration tests for package trust scoring.

use spanda_package::evaluate_package_trust;

#[test]
fn official_registry_package_scores_acceptably() {
    let report = evaluate_package_trust("spanda-mqtt", Some("0.1.0"), None);
    assert!(report.score >= 45);
    assert!(report.factors.iter().any(|f| f.name == "registry_listed" && f.passed));
    assert!(report.factors.iter().any(|f| f.name == "official_framework" && f.passed));
}

#[test]
fn unknown_package_scores_low() {
    let report = evaluate_package_trust("not-a-real-package", None, None);
    assert!(!report.passed);
    assert!(report.score < 60);
}
