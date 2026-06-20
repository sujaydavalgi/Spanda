//! Evaluate serialized `SirCondition` JSON for LLVM runtime `if` branches.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum RtCompareOp {
    Lt,
    Lte,
    Gt,
    Gte,
    Eq,
    Neq,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "snake_case")]
enum RtCondition {
    Bool {
        value: bool,
    },
    Ident {
        name: String,
    },
    Not {
        operand: Box<RtCondition>,
    },
    And {
        left: Box<RtCondition>,
        right: Box<RtCondition>,
    },
    Or {
        left: Box<RtCondition>,
        right: Box<RtCondition>,
    },
    EqBool {
        name: String,
        value: bool,
    },
    NeqBool {
        name: String,
        value: bool,
    },
    EqString {
        name: String,
        value: String,
    },
    CompareDouble {
        name: String,
        cmp: RtCompareOp,
        value: f64,
    },
    ScanDistance {
        scan_var: String,
        cmp: RtCompareOp,
        threshold: f64,
    },
    Unsupported,
}

fn bool_bindings() -> &'static Mutex<HashMap<String, bool>> {
    // Bool bindings.
    //
    // Parameters:
    // None.
    //
    // Returns:
    // &'static Mutex<HashMap<String, bool>>.
    //
    // Options:
    // None.
    //
    // Example:
    // let result = spanda_rt::condition::bool_bindings();

    // Produce new as the result.
    static STORE: OnceLock<Mutex<HashMap<String, bool>>> = OnceLock::new();
    STORE.get_or_init(|| Mutex::new(HashMap::new()))
}

fn double_bindings() -> &'static Mutex<HashMap<String, f64>> {
    // Double bindings.
    //
    // Parameters:
    // None.
    //
    // Returns:
    // &'static Mutex<HashMap<String, f64>>.
    //
    // Options:
    // None.
    //
    // Example:
    // let result = spanda_rt::condition::double_bindings();

    // Produce new as the result.
    static STORE: OnceLock<Mutex<HashMap<String, f64>>> = OnceLock::new();
    STORE.get_or_init(|| Mutex::new(HashMap::new()))
}

fn scan_distances() -> &'static Mutex<HashMap<String, f64>> {
    // Scan distances.
    //
    // Parameters:
    // None.
    //
    // Returns:
    // &'static Mutex<HashMap<String, f64>>.
    //
    // Options:
    // None.
    //
    // Example:
    // let result = spanda_rt::condition::scan_distances();

    // Produce new as the result.
    static STORE: OnceLock<Mutex<HashMap<String, f64>>> = OnceLock::new();
    STORE.get_or_init(|| Mutex::new(HashMap::new()))
}

pub fn store_bool(name: &str, value: bool) {
    // Store bool.
    //
    // Parameters:
    // - `name` — input value
    // - `value` — input value
    //
    // Returns:
    // Nothing.
    //
    // Options:
    // None.
    //
    // Example:
    // let result = spanda_rt::condition::store_bool(name, value);

    // Produce bool bindings as the result.
    bool_bindings()
        .lock()
        .unwrap()
        .insert(name.to_string(), value);
}

pub fn load_bool(name: &str) -> bool {
    // Load bool.
    //
    // Parameters:
    // - `name` — input value
    //
    // Returns:
    // true or false.
    //
    // Options:
    // None.
    //
    // Example:
    // let result = spanda_rt::condition::load_bool(name);

    // Produce bool bindings as the result.
    bool_bindings()
        .lock()
        .unwrap()
        .get(name)
        .copied()
        .unwrap_or(false)
}

pub fn store_double(name: &str, value: f64) {
    // Store double.
    //
    // Parameters:
    // - `name` — input value
    // - `value` — input value
    //
    // Returns:
    // Nothing.
    //
    // Options:
    // None.
    //
    // Example:
    // let result = spanda_rt::condition::store_double(name, value);

    // Produce double bindings as the result.
    double_bindings()
        .lock()
        .unwrap()
        .insert(name.to_string(), value);
}

pub fn load_double(name: &str) -> f64 {
    // Load double.
    //
    // Parameters:
    // - `name` — input value
    //
    // Returns:
    // Numeric result.
    //
    // Options:
    // None.
    //
    // Example:
    // let result = spanda_rt::condition::load_double(name);

    // Produce double bindings as the result.
    double_bindings()
        .lock()
        .unwrap()
        .get(name)
        .copied()
        .unwrap_or(0.0)
}

pub fn scan_nearest(name: &str) -> f64 {
    // Scan nearest.
    //
    // Parameters:
    // - `name` — input value
    //
    // Returns:
    // Numeric result.
    //
    // Options:
    // None.
    //
    // Example:
    // let result = spanda_rt::condition::scan_nearest(name);

    // Produce scan distances as the result.
    scan_distances()
        .lock()
        .unwrap()
        .get(name)
        .copied()
        .unwrap_or(2.0)
}

fn eval_compare(op: RtCompareOp, left: f64, right: f64) -> bool {
    // Eval compare.
    //
    // Parameters:
    // - `op` — input value
    // - `left` — input value
    // - `right` — input value
    //
    // Returns:
    // true or false.
    //
    // Options:
    // None.
    //
    // Example:
    // let result = spanda_rt::condition::eval_compare(op, left, right);

    // Match on op and handle each case.
    match op {
        RtCompareOp::Lt => left < right,
        RtCompareOp::Lte => left <= right,
        RtCompareOp::Gt => left > right,
        RtCompareOp::Gte => left >= right,
        RtCompareOp::Eq => (left - right).abs() < f64::EPSILON,
        RtCompareOp::Neq => (left - right).abs() >= f64::EPSILON,
    }
}

pub fn eval_condition_json(json: &str) -> bool {
    // Eval condition json.
    //
    // Parameters:
    // - `json` — input value
    //
    // Returns:
    // true or false.
    //
    // Options:
    // None.
    //
    // Example:
    // let result = spanda_rt::condition::eval_condition_json(json);

    // Compute condition for the following logic.
    let condition: RtCondition = match serde_json::from_str(json) {
        Ok(value) => value,
        Err(_) => return false,
    };
    eval_condition(&condition)
}

fn eval_condition(condition: &RtCondition) -> bool {
    // Eval condition.
    //
    // Parameters:
    // - `condition` — input value
    //
    // Returns:
    // true or false.
    //
    // Options:
    // None.
    //
    // Example:
    // let result = spanda_rt::condition::eval_condition(condition);

    // Match on condition and handle each case.
    match condition {
        RtCondition::Bool { value } => *value,
        RtCondition::Ident { name } => load_bool(name),
        RtCondition::Not { operand } => !eval_condition(operand),
        RtCondition::And { left, right } => eval_condition(left) && eval_condition(right),
        RtCondition::Or { left, right } => eval_condition(left) || eval_condition(right),
        RtCondition::EqBool { name, value } => load_bool(name) == *value,
        RtCondition::NeqBool { name, value } => load_bool(name) != *value,
        RtCondition::EqString { name, value } => string_bindings()
            .lock()
            .unwrap()
            .get(name)
            .map(|bound| bound == value)
            .unwrap_or(false),
        RtCondition::CompareDouble { name, cmp, value } => {
            eval_compare(*cmp, load_double(name), *value)
        }
        RtCondition::ScanDistance {
            scan_var,
            cmp,
            threshold,
        } => eval_compare(*cmp, scan_nearest(scan_var), *threshold),
        RtCondition::Unsupported => false,
    }
}

fn string_bindings() -> &'static Mutex<HashMap<String, String>> {
    // String bindings.
    //
    // Parameters:
    // None.
    //
    // Returns:
    // &'static Mutex<HashMap<String, String>>.
    //
    // Options:
    // None.
    //
    // Example:
    // let result = spanda_rt::condition::string_bindings();

    // Produce new as the result.
    static STORE: OnceLock<Mutex<HashMap<String, String>>> = OnceLock::new();
    STORE.get_or_init(|| Mutex::new(HashMap::new()))
}

pub fn store_string(name: &str, value: &str) {
    // Store string.
    //
    // Parameters:
    // - `name` — input value
    // - `value` — input value
    //
    // Returns:
    // Nothing.
    //
    // Options:
    // None.
    //
    // Example:
    // let result = spanda_rt::condition::store_string(name, value);

    // Produce string bindings as the result.
    string_bindings()
        .lock()
        .unwrap()
        .insert(name.to_string(), value.to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evals_and_or_bool_tree() {
        // Evals and or bool tree.
        //
        // Parameters:
        // None.
        //
        // Returns:
        // Nothing.
        //
        // Options:
        // None.
        //
        // Example:
        // let result = spanda_rt::condition::evals_and_or_bool_tree();

        store_bool("flag", true);
        store_bool("other", false);
        let json = r#"{"op":"and","left":{"op":"ident","name":"flag"},"right":{"op":"not","operand":{"op":"ident","name":"other"}}}"#;
        assert!(eval_condition_json(json));
    }

    #[test]
    fn evals_double_compare() {
        // Evals double compare.
        //
        // Parameters:
        // None.
        //
        // Returns:
        // Nothing.
        //
        // Options:
        // None.
        //
        // Example:
        // let result = spanda_rt::condition::evals_double_compare();

        store_double("speed", 0.5);
        let json = r#"{"op":"compare_double","name":"speed","cmp":"lt","value":1.0}"#;
        assert!(eval_condition_json(json));
    }

    #[test]
    fn evals_string_compare() {
        // Evals string compare.
        //
        // Parameters:
        // None.
        //
        // Returns:
        // Nothing.
        //
        // Options:
        // None.
        //
        // Example:
        // let result = spanda_rt::condition::evals_string_compare();

        store_string("mode", "auto");
        let json = r#"{"op":"eq_string","name":"mode","value":"auto"}"#;
        assert!(eval_condition_json(json));
    }
}
