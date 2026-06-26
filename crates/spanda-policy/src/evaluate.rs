//! Verify-time operational policy evaluation.

use serde::{Deserialize, Serialize};
use spanda_ast::nodes::{Expr, Program, RobotDecl, SafetyBlock, SafetyRule, UnitKind};
use spanda_ast::policy_decl::{OperationalPolicyDecl, OperationalPolicyRule};
use spanda_readiness::{evaluate_readiness, ReadinessOptions};

/// Severity for a policy violation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PolicySeverity {
    Error,
    Warning,
}

/// Failed policy rule with context.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PolicyViolation {
    pub policy: String,
    pub rule: String,
    pub severity: PolicySeverity,
    pub message: String,
}

/// Policy evaluation report.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PolicyEvaluationReport {
    pub policy: String,
    pub program: String,
    pub passed: bool,
    pub violations: Vec<PolicyViolation>,
}

/// List operational policy names declared in a program.
pub fn list_policies(program: &Program) -> Vec<String> {
    // Collect policy names from program-level declarations.
    //
    // Parameters:
    // - `program` — parsed program
    //
    // Returns:
    // Policy names in source order.
    //
    // Options:
    // None.
    //
    // Example:
    // let names = list_policies(&program);

    let Program::Program {
        operational_policies,
        ..
    } = program;
    operational_policies
        .iter()
        .map(|policy| match policy {
            OperationalPolicyDecl::OperationalPolicyDecl { name, .. } => name.clone(),
        })
        .collect()
}

/// Evaluate a named operational policy against a program at verify time.
pub fn evaluate_policy(
    program: &Program,
    policy_name: &str,
    source_label: &str,
) -> Result<PolicyEvaluationReport, String> {
    // Check declared policy rules against robots, safety, and readiness signals.
    //
    // Parameters:
    // - `program` — parsed program
    // - `policy_name` — policy to evaluate
    // - `source_label` — file label
    //
    // Returns:
    // Policy evaluation report or not-found error.
    //
    // Options:
    // None.
    //
    // Example:
    // let report = evaluate_policy(&program, "WarehousePolicy", "rover.sd")?;

    let policy = find_policy(program, policy_name)?;
    let mut violations = Vec::new();
    for rule in policy_rules(policy) {
        match rule {
            OperationalPolicyRule::MaxSpeed { limit_mps, .. } => {
                check_max_speed(program, policy_name, *limit_mps, &mut violations);
            }
            OperationalPolicyRule::RequiresKillSwitch { .. } => {
                check_requires_kill_switch(program, policy_name, &mut violations);
            }
            OperationalPolicyRule::RequiresCapability { capabilities, .. } => {
                check_requires_capabilities(program, policy_name, capabilities, &mut violations);
            }
            OperationalPolicyRule::MinReadinessScore { score, .. } => {
                check_min_readiness_score(
                    program,
                    policy_name,
                    source_label,
                    *score,
                    &mut violations,
                );
            }
            OperationalPolicyRule::OperationHours { range, .. } => {
                check_operation_hours(policy_name, range, &mut violations);
            }
        }
    }
    let passed = violations
        .iter()
        .all(|violation| violation.severity != PolicySeverity::Error);
    Ok(PolicyEvaluationReport {
        policy: policy_name.into(),
        program: source_label.into(),
        passed,
        violations,
    })
}

fn find_policy<'a>(
    program: &'a Program,
    policy_name: &str,
) -> Result<&'a OperationalPolicyDecl, String> {
    let Program::Program {
        operational_policies,
        ..
    } = program;
    operational_policies
        .iter()
        .find(|policy| policy_decl_name(policy) == policy_name)
        .ok_or_else(|| format!("Policy '{policy_name}' not found in program"))
}

fn policy_decl_name(policy: &OperationalPolicyDecl) -> &str {
    let OperationalPolicyDecl::OperationalPolicyDecl { name, .. } = policy;
    name
}

fn policy_rules(policy: &OperationalPolicyDecl) -> &[OperationalPolicyRule] {
    let OperationalPolicyDecl::OperationalPolicyDecl { rules, .. } = policy;
    rules
}

fn check_max_speed(
    program: &Program,
    policy_name: &str,
    limit_mps: f64,
    violations: &mut Vec<PolicyViolation>,
) {
    let Program::Program { robots, .. } = program;
    for robot in robots {
        let RobotDecl::RobotDecl { name, safety, .. } = robot;
        let Some(safety) = safety else {
            violations.push(violation(
                policy_name,
                "max_speed",
                PolicySeverity::Error,
                format!("robot {name} has no safety block with max_speed"),
            ));
            continue;
        };
        let SafetyBlock::SafetyBlock { rules, .. } = safety;
        let mut robot_limit = None;
        for rule in rules {
            if let SafetyRule::MaxSpeedRule { value, unit, .. } = rule {
                if let Some(mps) = expr_to_mps(value, *unit) {
                    robot_limit = Some(robot_limit.map_or(mps, |current: f64| current.max(mps)));
                }
            }
        }
        let Some(robot_limit) = robot_limit else {
            violations.push(violation(
                policy_name,
                "max_speed",
                PolicySeverity::Error,
                format!("robot {name} missing max_speed safety rule"),
            ));
            continue;
        };
        if robot_limit > limit_mps {
            violations.push(violation(
                policy_name,
                "max_speed",
                PolicySeverity::Error,
                format!(
                    "robot {name} max_speed {robot_limit:.2} m/s exceeds policy limit {limit_mps:.2} m/s"
                ),
            ));
        }
    }
}

fn check_requires_kill_switch(
    program: &Program,
    policy_name: &str,
    violations: &mut Vec<PolicyViolation>,
) {
    let Program::Program {
        kill_switches,
        robots,
        ..
    } = program;
    let robot_switches = robots.iter().any(|robot| {
        let RobotDecl::RobotDecl { kill_switches, .. } = robot;
        !kill_switches.is_empty()
    });
    if kill_switches.is_empty() && !robot_switches {
        violations.push(violation(
            policy_name,
            "requires_kill_switch",
            PolicySeverity::Error,
            "program must declare at least one kill_switch",
        ));
    }
}

fn check_requires_capabilities(
    program: &Program,
    policy_name: &str,
    required: &[String],
    violations: &mut Vec<PolicyViolation>,
) {
    let Program::Program { robots, .. } = program;
    if robots.is_empty() {
        violations.push(violation(
            policy_name,
            "requires_capability",
            PolicySeverity::Error,
            "policy requires robots with declared capabilities",
        ));
        return;
    }
    for robot in robots {
        let RobotDecl::RobotDecl {
            name,
            exposes_capabilities,
            ..
        } = robot;
        for capability in required {
            if !exposes_capabilities.iter().any(|c| c == capability) {
                violations.push(violation(
                    policy_name,
                    "requires_capability",
                    PolicySeverity::Error,
                    format!("robot {name} missing required capability `{capability}`"),
                ));
            }
        }
    }
}

fn check_min_readiness_score(
    program: &Program,
    policy_name: &str,
    source_label: &str,
    minimum: u32,
    violations: &mut Vec<PolicyViolation>,
) {
    let _ = source_label;
    let readiness = evaluate_readiness(program, &ReadinessOptions::default());
    if readiness.score.total < minimum {
        violations.push(violation(
            policy_name,
            "min_readiness_score",
            PolicySeverity::Error,
            format!(
                "readiness score {}/{} below policy minimum {minimum}",
                readiness.score.total, readiness.score.maximum
            ),
        ));
    }
}

fn check_operation_hours(policy_name: &str, range: &str, violations: &mut Vec<PolicyViolation>) {
    let valid = range.contains(':') && range.contains('-');
    if !valid {
        violations.push(violation(
            policy_name,
            "operation_hours",
            PolicySeverity::Warning,
            format!("operation_hours `{range}` is not a valid HH:MM-HH:MM range"),
        ));
    }
}

fn violation(
    policy: &str,
    rule: &str,
    severity: PolicySeverity,
    message: impl Into<String>,
) -> PolicyViolation {
    PolicyViolation {
        policy: policy.into(),
        rule: rule.into(),
        severity,
        message: message.into(),
    }
}

fn expr_to_mps(expr: &Expr, unit: UnitKind) -> Option<f64> {
    let value = match expr {
        Expr::UnitLiteralExpr { value, unit, .. } => {
            return match unit {
                UnitKind::MPerS => Some(*value),
                UnitKind::KmPerH => Some(*value / 3.6),
                UnitKind::Mph => Some(*value * 0.44704),
                _ => None,
            };
        }
        Expr::LiteralExpr {
            value: spanda_ast::nodes::LiteralValue::Number(n),
            ..
        } => *n,
        _ => return None,
    };
    match unit {
        UnitKind::MPerS => Some(value),
        UnitKind::KmPerH => Some(value / 3.6),
        UnitKind::Mph => Some(value * 0.44704),
        _ => None,
    }
}

/// Format a policy evaluation report for CLI output.
pub fn format_policy_report(report: &PolicyEvaluationReport, json: bool) -> String {
    // Serialize or pretty-print a policy evaluation report.
    //
    // Parameters:
    // - `report` — policy evaluation report
    // - `json` — emit JSON when true
    //
    // Returns:
    // Formatted report string.
    //
    // Options:
    // None.
    //
    // Example:
    // let text = format_policy_report(&report, false);

    if json {
        return serde_json::to_string_pretty(report).unwrap_or_else(|e| e.to_string());
    }
    let mut lines = vec![
        format!("Policy evaluation: {} on {}", report.policy, report.program),
        if report.passed {
            "Result: PASS".into()
        } else {
            "Result: FAIL".into()
        },
    ];
    if report.violations.is_empty() {
        lines.push("No violations.".into());
    } else {
        for violation in &report.violations {
            lines.push(format!(
                "  [{:?}] {} — {}",
                violation.severity, violation.rule, violation.message
            ));
        }
    }
    lines.join("\n")
}
