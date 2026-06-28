//! SDK program-operation contract tests (no live server required for parsing).
use spanda_api::sdk_ops::ProgramRequest;

#[test]
fn program_request_parses_defaults() {
    let req: ProgramRequest = serde_json::from_str("{}").unwrap();
    assert!(!req.include_runtime);
}
