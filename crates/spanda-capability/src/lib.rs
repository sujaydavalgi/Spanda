//! Capability registry, hardware/robot capability analysis, traceability matrices,
//! minimum-hardware safety checks, and health-check verification for Spanda programs.

pub mod diagnostics;
pub mod health;
pub mod minimum;
pub mod registry;
pub mod robot;
pub mod traceability;

pub use diagnostics::{collect_verification_diagnostics, VerificationDiagnostic};
pub use health::{
    apply_fleet_health_checks, evaluate_health_checks, evaluate_runtime_health,
    health_traceability, HealthCheckResult, HealthReport, HealthStatus, HealthTraceRow,
};
pub use minimum::{check_minimum_capabilities, MinimumCapabilityReport, MinimumCapabilityRow};
pub use registry::{
    capability_registry, lookup_capability, CapabilityDefinition, CapabilityRequirement,
    PackageCapabilityContribution,
};
pub use robot::{infer_robot_capabilities, RobotCapabilityReport, RobotCapabilityRow};
pub use traceability::{
    capability_traceability, hardware_traceability, CapabilityTraceRow, HardwareTraceRow,
    TraceabilityReport,
};
