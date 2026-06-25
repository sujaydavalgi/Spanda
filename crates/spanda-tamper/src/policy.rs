//! Tamper policy extraction and runtime matching.

use crate::detect::TamperSeverity;
use serde::{Deserialize, Serialize};
use spanda_ast::assurance_decl::{TamperPolicyBranch, TamperPolicyDecl};
use spanda_ast::nodes::Program;

/// Declarative tamper response policy extracted from a program.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TamperPolicySpec {
    pub name: String,
    pub triggers: Vec<(String, Vec<String>)>,
}

/// Extract tamper policies declared in a parsed program.
pub fn extract_tamper_policies(program: &Program) -> Vec<TamperPolicySpec> {
    // Collect tamper_policy branches into trigger/action pairs for matching.
    //
    // Parameters:
    // - `program` — parsed Spanda program
    //
    // Returns:
    // Tamper policy specs with normalized condition strings.
    //
    // Options:
    // None.
    //
    // Example:
    // let policies = extract_tamper_policies(&program);

    let Program::Program { tamper_policies, .. } = program;
    tamper_policies
        .iter()
        .map(|decl| {
            let TamperPolicyDecl::TamperPolicyDecl { name, branches, .. } = decl;
            TamperPolicySpec {
                name: name.clone(),
                triggers: branches
                    .iter()
                    .map(|TamperPolicyBranch { condition, actions, .. }| {
                        (condition.clone(), actions.clone())
                    })
                    .collect(),
            }
        })
        .collect()
}

/// Resolve tamper policy actions for a runtime signal and severity.
pub fn actions_for_tamper_event(
    policies: &[TamperPolicySpec],
    signal: &str,
    severity: TamperSeverity,
) -> Vec<String> {
    // Match declared tamper policy branches against a runtime signal.
    //
    // Parameters:
    // - `policies` — extracted tamper policies
    // - `signal` — runtime tamper signal label
    // - `severity` — tamper severity tier
    //
    // Returns:
    // Ordered action strings to dispatch.
    //
    // Options:
    // None.
    //
    // Example:
    // let actions = actions_for_tamper_event(&policies, "capability_denied", TamperSeverity::High);

    let mut actions = Vec::new();
    for policy in policies {
        for (condition, branch_actions) in &policy.triggers {
            if tamper_condition_matches(condition, signal, severity) {
                actions.extend(branch_actions.iter().cloned());
            }
        }
    }
    actions
}

/// Report whether a program declares tamper response policies.
pub fn tamper_policy_coverage(program: &Program) -> (bool, usize) {
    let policies = extract_tamper_policies(program);
    let branch_count = policies
        .iter()
        .map(|policy| policy.triggers.len())
        .sum::<usize>();
    (!policies.is_empty(), branch_count)
}

fn tamper_condition_matches(
    condition: &str,
    signal: &str,
    severity: TamperSeverity,
) -> bool {
    let normalized = condition.to_lowercase();
    let signal_lower = signal.to_lowercase();
    let severity_label = format!("{:?}", severity).to_lowercase();

    if normalized.starts_with("tamper.severity.") {
        let expected = normalized.trim_start_matches("tamper.severity.");
        return expected == severity_label || expected == "any";
    }

    if normalized.starts_with("tamper.signal.") {
        let expected = normalized.trim_start_matches("tamper.signal.");
        return signal_lower.contains(expected) || expected == "any";
    }

    if normalized == "gps.spoofed" {
        return signal_lower.contains("gps.spoofed") || signal_lower.contains("spoof");
    }

    signal_lower.contains(&normalized) || normalized.split('.').all(|part| signal_lower.contains(part))
}

#[cfg(test)]
mod tests {
    use super::*;
    use spanda_lexer::tokenize;
    use spanda_parser::parse;

    fn parse_source(source: &str) -> Program {
        parse(tokenize(source).unwrap()).unwrap()
    }

    #[test]
    fn extracts_tamper_policy_branches() {
        let program = parse_source(
            r#"
tamper_policy CriticalResponse {
    on tamper severity Critical {
        enter SafeMode;
        audit.record("critical_tamper_detected");
    }
}
robot Rover { }
"#,
        );
        let policies = extract_tamper_policies(&program);
        assert_eq!(policies.len(), 1);
        assert_eq!(policies[0].triggers[0].0, "tamper.severity.Critical");
        assert_eq!(policies[0].triggers[0].1.len(), 2);
    }

    #[test]
    fn matches_capability_denied_signal() {
        let program = parse_source(
            r#"
tamper_policy DenyResponse {
    on tamper signal capability_denied {
        stop_all_actuators();
    }
}
robot Rover { }
"#,
        );
        let policies = extract_tamper_policies(&program);
        let actions =
            actions_for_tamper_event(&policies, "agent_capability_denied", TamperSeverity::High);
        assert_eq!(actions, vec!["stop_all_actuators()"]);
    }
}
