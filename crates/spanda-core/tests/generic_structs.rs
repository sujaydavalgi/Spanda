use spanda_core::{check, run, RunOptions};

#[test]
fn generic_struct_type_params_type_check() {
    let source = r#"
struct Box<T> {
  value: T;
}

robot R {
  actuator wheels: DifferentialDrive;
}
"#;
    check(source).expect("generic struct declaration should type-check");
}

#[test]
fn generic_struct_literal_instantiates_fields() {
    let source = r#"
struct Box<T> {
  value: T;
}

robot R {
  actuator wheels: DifferentialDrive;
  behavior run() {
    let b = Box<Int> { value: 42 };
    let _v = b.value;
    wheels.stop();
  }
}
"#;
    check(source).expect("generic struct literal should type-check");
    run(source, RunOptions::default()).expect("generic struct literal should run");
}

#[test]
fn generic_struct_literal_arity_mismatch_rejected() {
    let source = r#"
struct Box<T> {
  value: T;
}
robot R {
  actuator wheels: DifferentialDrive;
  behavior run() { let b = Box<Int, Float> { value: 1 }; }
}
"#;
    let err = check(source).expect_err("wrong generic arity should fail");
    assert!(
        err.diagnostics()
            .iter()
            .any(|d| d.message.contains("type argument")),
        "got {:?}",
        err.diagnostics()
    );
}
