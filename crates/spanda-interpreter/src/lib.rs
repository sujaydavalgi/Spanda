//! Spanda interpreter — modular runtime implementation and public run API.
//!
//! The full `Interpreter` implementation is split across `src/runtime/*.rs` in this crate.
//! `spanda-core` compiles those sources via a thin `#[path]` include shim (`spanda_core::runtime`)
//! so auxiliary core modules (`ai`, `safety`, `transport`, …) remain available without a
//! `spanda-core` ↔ `spanda-interpreter` dependency cycle.
//!

pub use spanda_core::replay::{MissionTrace, PlaybackReport, TraceVerification};
pub use spanda_core::runtime::{Interpreter, InterpreterOptions, RobotBackend};
pub use spanda_core::simulator::{
    create_default_simulator, Obstacle, Simulator, SimulatorConfig,
};
pub use spanda_core::telemetry::ExecutionMetrics;
pub use spanda_core::{
    playback_mission, replay_mission, run, run_program, run_tests, run_tests_with_registry,
    ObstacleConfig, PoseState, RobotState, RunOptions, RunResult, SpandaError, TestRunResult,
    VelocityState,
};
pub use spanda_runtime::RuntimeHost;

/// In-process simulator backend implementing [`RobotBackend`].
pub type SimRobotBackend = Simulator;
