//! Multi-tenant scoping for Control Center API keys and deployments.
//!

/// Default tenant when `SPANDA_TENANT_ID` is unset.
pub fn default_tenant_id() -> String {
    std::env::var("SPANDA_TENANT_ID")
        .ok()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| "default".to_string())
}

/// True when the authenticated key belongs to the active Control Center tenant.
pub fn tenant_matches(server_tenant: &str, key_tenant: &str) -> bool {
    server_tenant == key_tenant
}
