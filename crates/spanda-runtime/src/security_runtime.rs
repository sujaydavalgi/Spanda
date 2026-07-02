//! Injectable security runtime boundary for interpreter identity and comm enforcement.
//!
use crate::security_types::{
    AuthenticationMode, EncryptionMode, IntegrityMode, RobotIdentity, SecretHandle, SecurePolicy,
    TrustBoundaryKind, TrustLevel,
};
use spanda_audit::AuditRuntime;
use std::collections::{HashMap, HashSet};

/// Extension points for security enforcement at runtime.
pub trait SecurityRuntime {
    fn enable_strict_permissions(&mut self);

    fn inject_security_fault(&mut self, fault: &str);

    fn declare_trust_boundary(&mut self, boundary: TrustBoundaryKind);

    fn configure_wire_session(&mut self, cert_path: Option<String>, key_secret: Option<String>);

    fn set_transport_context(
        &mut self,
        boundary: Option<TrustBoundaryKind>,
        encryption: EncryptionMode,
        authentication: AuthenticationMode,
        integrity: IntegrityMode,
    );

    fn grant_capability(&mut self, capability: &str);

    fn grant_capabilities(&mut self, capabilities: &[&str]);

    fn granted_capability_count(&self) -> usize;

    fn set_trust_level(&mut self, trust: TrustLevel);

    fn trust_level(&self) -> TrustLevel;

    fn grant_if_not_strict(&mut self, capability: &str);

    fn require_operation(&self, operation: &str) -> Result<(), String>;

    fn register_secret(&mut self, handle: SecretHandle);

    fn secret_exists(&self, name: &str) -> bool;

    fn resolve_secret(&self, name: &str) -> Result<String, String>;

    fn set_identity(&mut self, identity: RobotIdentity);

    fn identity_id(&self) -> Option<String>;

    fn register_secure_endpoint(&mut self, path: &str, policy: SecurePolicy);

    fn audit_event(
        &self,
        audit: &mut AuditRuntime,
        event_type: &str,
        detail: &str,
    ) -> Result<(), String>;

    fn audit_security_event(
        &self,
        audit: &mut AuditRuntime,
        event_type: &str,
        detail: &str,
    ) -> Result<(), String>;

    fn prepare_publish(
        &mut self,
        path: &str,
        payload: &str,
        source_id: &str,
        message_type: &str,
    ) -> Result<(), String>;

    fn verify_inbound(
        &self,
        path: &str,
        signed_json: Option<&str>,
        source_id: Option<&str>,
    ) -> Result<(), String>;

    fn sign_outbound(&self, path: &str, payload: &str) -> Result<(), String>;

    fn authorize_subscribe(&self, path: &str) -> Result<(), String>;

    fn verify_inbound_message(
        &mut self,
        path: &str,
        payload: &str,
        source_id: Option<&str>,
        signed_json: Option<&str>,
        message_type: &str,
    ) -> Result<(), String>;

    fn verify_remote_signature(&self, signature_json: &str) -> Result<(), String>;
}

/// Permissive built-in security runtime for simulation without the security crate.
#[derive(Debug, Default)]
pub struct BuiltinSecurityRuntime {
    identity: Option<RobotIdentity>,
    trust: TrustLevel,
    secrets: HashMap<String, SecretHandle>,
    capabilities: HashSet<String>,
    strict_permissions: bool,
    security_faults_active: HashSet<String>,
    secure_endpoints: HashMap<String, SecurePolicy>,
    trust_boundaries: HashSet<TrustBoundaryKind>,
    transport_boundary: Option<TrustBoundaryKind>,
    bus_encryption: EncryptionMode,
    bus_authentication: AuthenticationMode,
    bus_integrity: IntegrityMode,
    audit_security_events: bool,
}

impl SecurityRuntime for BuiltinSecurityRuntime {
    fn enable_strict_permissions(&mut self) {
        self.strict_permissions = true;
    }

    fn inject_security_fault(&mut self, fault: &str) {
        self.security_faults_active.insert(fault.to_string());
    }

    fn declare_trust_boundary(&mut self, boundary: TrustBoundaryKind) {
        self.trust_boundaries.insert(boundary);
    }

    fn configure_wire_session(&mut self, _cert_path: Option<String>, _key_secret: Option<String>) {}

    fn set_transport_context(
        &mut self,
        boundary: Option<TrustBoundaryKind>,
        encryption: EncryptionMode,
        authentication: AuthenticationMode,
        integrity: IntegrityMode,
    ) {
        self.transport_boundary = boundary;
        self.bus_encryption = encryption;
        self.bus_authentication = authentication;
        self.bus_integrity = integrity;
    }

    fn grant_capability(&mut self, capability: &str) {
        self.capabilities.insert(capability.to_string());
    }

    fn grant_capabilities(&mut self, capabilities: &[&str]) {
        for cap in capabilities {
            self.grant_capability(cap);
        }
    }

    fn granted_capability_count(&self) -> usize {
        self.capabilities.len()
    }

    fn set_trust_level(&mut self, trust: TrustLevel) {
        self.trust = trust;
    }

    fn trust_level(&self) -> TrustLevel {
        self.trust
    }

    fn grant_if_not_strict(&mut self, capability: &str) {
        if !self.strict_permissions {
            self.grant_capability(capability);
        }
    }

    fn require_operation(&self, operation: &str) -> Result<(), String> {
        let _ = operation;
        Ok(())
    }

    fn register_secret(&mut self, handle: SecretHandle) {
        self.secrets.insert(handle.name.clone(), handle);
    }

    fn secret_exists(&self, name: &str) -> bool {
        self.secrets.contains_key(name)
    }

    fn resolve_secret(&self, name: &str) -> Result<String, String> {
        let handle = self
            .secrets
            .get(name)
            .ok_or_else(|| format!("secret '{name}' not found"))?;
        match &handle.source {
            crate::security_types::SecretSource::Env { var } => {
                std::env::var(var).map_err(|_| format!("environment variable '{var}' not found"))
            }
            crate::security_types::SecretSource::File { path } => {
                std::fs::read_to_string(path).map_err(|_| format!("secret file '{path}' not found"))
            }
            crate::security_types::SecretSource::Literal { value } => Ok(value.clone()),
        }
    }

    fn set_identity(&mut self, identity: RobotIdentity) {
        self.trust = identity.trust;
        self.identity = Some(identity);
    }

    fn identity_id(&self) -> Option<String> {
        self.identity.as_ref().map(|id| id.id.clone())
    }

    fn register_secure_endpoint(&mut self, path: &str, policy: SecurePolicy) {
        self.secure_endpoints.insert(path.to_string(), policy);
    }

    fn audit_event(
        &self,
        audit: &mut AuditRuntime,
        event_type: &str,
        detail: &str,
    ) -> Result<(), String> {
        if !self.audit_security_events {
            return Ok(());
        }
        audit
            .record_event(event_type, detail)
            .map(|_| ())
            .map_err(|e| format!("audit failed: {e}"))
    }

    fn audit_security_event(
        &self,
        audit: &mut AuditRuntime,
        event_type: &str,
        detail: &str,
    ) -> Result<(), String> {
        self.audit_event(audit, &format!("security.{event_type}"), detail)
    }

    fn prepare_publish(
        &mut self,
        path: &str,
        payload: &str,
        _source_id: &str,
        _message_type: &str,
    ) -> Result<(), String> {
        self.check_security_faults(path, payload)
    }

    fn verify_inbound(
        &self,
        _path: &str,
        _signed_json: Option<&str>,
        _source_id: Option<&str>,
    ) -> Result<(), String> {
        Ok(())
    }

    fn sign_outbound(&self, _path: &str, _payload: &str) -> Result<(), String> {
        Ok(())
    }

    fn authorize_subscribe(&self, _path: &str) -> Result<(), String> {
        Ok(())
    }

    fn verify_inbound_message(
        &mut self,
        path: &str,
        payload: &str,
        _source_id: Option<&str>,
        _signed_json: Option<&str>,
        _message_type: &str,
    ) -> Result<(), String> {
        self.check_security_faults(path, payload)
    }

    fn verify_remote_signature(&self, _signature_json: &str) -> Result<(), String> {
        if self.identity.is_none() {
            return Err("robot identity required for signature verification".into());
        }
        Ok(())
    }
}

impl BuiltinSecurityRuntime {
    fn check_security_faults(&self, path: &str, _payload: &str) -> Result<(), String> {
        if self.security_faults_active.contains("InvalidSignature") {
            return Err("signature invalid".into());
        }
        if self.security_faults_active.contains("ExpiredCertificate") {
            return Err("certificate expired".into());
        }
        if self.security_faults_active.contains("ManInTheMiddle") {
            return Err("authentication failed: man-in-the-middle detected".into());
        }
        if self
            .security_faults_active
            .contains("SecureHandshakeDropped")
        {
            return Err(format!(
                "secure endpoint '{path}': secure handshake dropped"
            ));
        }
        Ok(())
    }
}

/// Factory for default built-in security runtime instances.
pub type SecurityRuntimeFactory = fn() -> Box<dyn SecurityRuntime>;

/// Default built-in security runtime factory for direct interpreter use.
pub fn default_security_runtime() -> Box<dyn SecurityRuntime> {
    Box::new(BuiltinSecurityRuntime::default())
}

/// Default factory pointer for option wiring.
pub fn default_security_runtime_factory() -> SecurityRuntimeFactory {
    default_security_runtime
}
