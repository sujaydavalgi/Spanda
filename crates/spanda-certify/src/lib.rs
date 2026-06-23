//! Spanda certification runtime gate, proof checklist, and audit artifacts.
//!
pub mod artifact;
pub mod prover;
pub mod runtime;
pub mod verify;

pub use artifact::hash_program_artifact;

pub use prover::{
    build_certification_proof, build_certification_proof_summary, CertificationEntry,
    CertificationProofReport, CertificationProofSummary, DeployTargetEntry,
};
pub use runtime::{certification_runtime_enabled_from_env, enforce_certification_runtime};
pub use verify::verify_certification_proof;
