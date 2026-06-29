//! OTA deployment runtime extracted from Spanda core for lean-core package architecture.
//!
pub mod agent;
pub mod bundle;
pub mod deploy_plan;
pub mod plan;
pub mod remote;
pub mod service;
pub mod types;

pub use agent::*;
pub use bundle::*;
pub use deploy_plan::build_deploy_plan;
pub use plan::build_deploy_plan_from_program;
pub use remote::*;
pub use service::*;
pub use types::*;
