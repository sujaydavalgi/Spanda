//! Runtime tamper policy dispatch during simulation and live execution.

use super::{Interpreter, RobotBackend};
use spanda_ast::nodes::Program;
use spanda_tamper::{actions_for_tamper_event, extract_tamper_policies, TamperSeverity};

impl<B: RobotBackend> Interpreter<B> {
    pub(super) fn cache_tamper_policies(&mut self, program: &Program) {
        // Cache tamper policies when declared so runtime signals can dispatch responses.
        //
        // Parameters:
        // - `program` — parsed program AST
        //
        // Returns:
        // None (updates interpreter state).
        //
        // Options:
        // None.
        //
        // Example:
        // self.cache_tamper_policies(&program);

        let Program::Program { tamper_policies, .. } = program;
        self.tamper_policies = if tamper_policies.is_empty() {
            Vec::new()
        } else {
            extract_tamper_policies(program)
        };
        self.applied_tamper_branches.clear();
    }

    pub(super) fn invoke_tamper_policies(
        &mut self,
        signal: &str,
        severity: TamperSeverity,
    ) {
        // Match tamper policies and dispatch declared response actions once per branch.
        //
        // Parameters:
        // - `signal` — runtime tamper signal label
        // - `severity` — tamper severity tier
        //
        // Returns:
        // None (dispatches recovery actions and records audit events).
        //
        // Options:
        // None.
        //
        // Example:
        // self.invoke_tamper_policies("agent_capability_denied", TamperSeverity::High);

        if self.tamper_policies.is_empty() {
            return;
        }

        let actions = actions_for_tamper_event(&self.tamper_policies, signal, severity);
        if actions.is_empty() {
            return;
        }

        self.log(format!(
            "tamper: signal '{signal}' severity {:?} matched {} action(s)",
            severity,
            actions.len()
        ));
        self.record_mission_event(
            "tamper_policy",
            serde_json::json!({
                "signal": signal,
                "severity": format!("{:?}", severity),
            }),
        );

        for action in actions {
            let branch_key = format!("{signal}:{action}");
            if !self.applied_tamper_branches.insert(branch_key) {
                continue;
            }
            self.log(format!("tamper: action {action}"));
            if let Err(error) = self.dispatch_recovery_action(&action) {
                self.log(format!("tamper: action failed: {error}"));
            }
            if action.contains("audit.record") {
                self.record_debug_event(1, "audit_record", &[("event", action.clone())]);
            }
        }
    }
}
