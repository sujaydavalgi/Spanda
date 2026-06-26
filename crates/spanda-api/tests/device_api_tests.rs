//! API endpoint tests for device pool routes.

use spanda_api::handlers::handle_request;
use spanda_api::state::ControlCenterState;
use spanda_config::ConfigResolver;
use spanda_deploy_http::HttpRequest;
use std::path::PathBuf;

fn warehouse_state() -> ControlCenterState {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("spanda-config/tests/fixtures/warehouse");
    let resolved = ConfigResolver::new()
        .resolve_from_dir(&root)
        .expect("resolve warehouse");
    let mut state = ControlCenterState::new();
    state.resolved = Some(resolved);
    state
}

#[test]
fn devices_list_returns_pool() {
    let mut state = warehouse_state();
    let (response, _) = handle_request(
        &mut state,
        &HttpRequest {
            method: "GET".into(),
            path: "/v1/devices".into(),
            body: String::new(),
            authorization: None,
        },
        "",
    );
    assert_eq!(response.status, 200);
    assert!(response.body.contains("devices"));
}

#[test]
fn device_get_by_id() {
    let mut state = warehouse_state();
    let (response, _) = handle_request(
        &mut state,
        &HttpRequest {
            method: "GET".into(),
            path: "/v1/devices/gps-001".into(),
            body: String::new(),
            authorization: None,
        },
        "",
    );
    assert_eq!(response.status, 200);
    assert!(response.body.contains("gps-001"));
}

#[test]
fn robots_and_fleets_endpoints() {
    let mut state = warehouse_state();
    let (robots, _) = handle_request(
        &mut state,
        &HttpRequest {
            method: "GET".into(),
            path: "/v1/robots".into(),
            body: String::new(),
            authorization: None,
        },
        "",
    );
    assert_eq!(robots.status, 200);
    assert!(robots.body.contains("rover-001"));

    let (fleets, _) = handle_request(
        &mut state,
        &HttpRequest {
            method: "GET".into(),
            path: "/v1/fleets".into(),
            body: String::new(),
            authorization: None,
        },
        "",
    );
    assert_eq!(fleets.status, 200);
    assert!(fleets.body.contains("warehouse-fleet"));
}

#[test]
fn readiness_run_endpoint() {
    let mut state = warehouse_state();
    let (response, _) = handle_request(
        &mut state,
        &HttpRequest {
            method: "POST".into(),
            path: "/v1/readiness/run".into(),
            body: "{}".into(),
            authorization: None,
        },
        "",
    );
    assert_eq!(response.status, 200);
    assert!(response.body.contains("mission_ready"));
}

#[test]
fn device_tree_endpoint() {
    let mut state = warehouse_state();
    let (response, _) = handle_request(
        &mut state,
        &HttpRequest {
            method: "GET".into(),
            path: "/v1/device-tree".into(),
            body: String::new(),
            authorization: None,
        },
        "",
    );
    assert_eq!(response.status, 200);
    assert!(response.body.contains("mapping"));
}

#[test]
fn device_trust_gps_endpoint() {
    std::env::set_var("SPANDA_API_KEY", "enterprise-ops-smoke-key");
    let mut state = warehouse_state();
    let (response, _) = handle_request(
        &mut state,
        &HttpRequest {
            method: "POST".into(),
            path: "/v1/devices/gps-001/trust".into(),
            body: String::new(),
            authorization: Some("enterprise-ops-smoke-key".into()),
        },
        "",
    );
    assert_eq!(response.status, 200, "body={}", response.body);
    assert!(response.body.contains("device trusted by operator"));
}

#[test]
fn device_discover_registers_matches() {
    std::env::set_var("SPANDA_API_KEY", "enterprise-ops-smoke-key");
    std::env::set_var("SPANDA_DISCOVERY_MDNS_MATCHES", "smoke@127.0.0.1");
    let mut state = warehouse_state();
    let (response, _) = handle_request(
        &mut state,
        &HttpRequest {
            method: "POST".into(),
            path: "/v1/devices/discover".into(),
            body: r#"{"transports":["mdns"]}"#.into(),
            authorization: Some("enterprise-ops-smoke-key".into()),
        },
        "",
    );
    std::env::remove_var("SPANDA_DISCOVERY_MDNS_MATCHES");
    assert_eq!(response.status, 200);
    assert!(response.body.contains("registered"));
}
