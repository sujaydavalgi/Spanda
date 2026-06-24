//! Markdown documentation generator for Spanda programs.
//!
//! Emits module-level API reference from the AST: imports, functions, structs,
//! enums, traits, robots, and test blocks.

use spanda_ast::foundations::Visibility;
use spanda_ast::nodes::*;
use spanda_error::SpandaError;
use std::path::{Path, PathBuf};

pub fn generate_markdown(source: &str) -> Result<String, SpandaError> {
    // Generate Markdown API documentation from Spanda source.
    //
    // Parameters:
    //
    // - `source` — Full program source text.
    //
    // Returns:
    //
    // Markdown document string, or [`SpandaError`] if lexing/parsing fails.
    //
    // Example:
    //
    // use spanda_core::docs::generate_markdown;
    // let source = r#"
    // module nav;
    // export fn plan() -> Path { return trajectory(from: pose(x: 0.0 m, y: 0.0 m), to: pose(x: 1.0 m, y: 0.0 m), steps: 3); }
    // robot R { actuator wheels: DifferentialDrive; behavior run() { wheels.stop(); } }
    // "#;
    // let md = generate_markdown(source).unwrap();
    // assert!(md.contains("# Module `nav`"));
    // assert!(md.contains("### `R`"));
    let tokens = spanda_lexer::tokenize(source)?;
    let program = spanda_parser::parse(tokens)?;
    Ok(render_program_docs(&program))
}

/// Generate HTML API documentation from Spanda source.
pub fn generate_html(source: &str, title: Option<&str>) -> Result<String, SpandaError> {
    let markdown = generate_markdown(source)?;
    let tokens = spanda_lexer::tokenize(source)?;
    let program = spanda_parser::parse(tokens)?;
    let page_title = title.map(str::to_string).unwrap_or_else(|| {
        let Program::Program { module_name, .. } = &program;
        module_name
            .as_deref()
            .unwrap_or("anonymous")
            .replace('.', "/")
    });
    Ok(crate::html_docs::markdown_to_html(&page_title, &markdown))
}

/// JSON payload for `spanda doc --json`.
#[derive(serde::Serialize)]
pub struct DocJson {
    pub ok: bool,
    pub format: &'static str,
    pub content: String,
}

pub fn generate_json_docs(source: &str, html: bool) -> Result<DocJson, SpandaError> {
    if html {
        let content = generate_html(source, None)?;
        Ok(DocJson {
            ok: true,
            format: "html",
            content,
        })
    } else {
        let content = generate_markdown(source)?;
        Ok(DocJson {
            ok: true,
            format: "markdown",
            content,
        })
    }
}

/// Batch result when generating docs for a directory tree.
#[derive(Debug, Clone)]
pub struct DocBatchResult {
    pub outputs: Vec<(PathBuf, String)>,
    pub errors: Vec<(PathBuf, String)>,
}

/// Generate docs for a single `.sd` file or all `.sd` files under a directory.
pub fn generate_docs_for_path(
    path: &Path,
    html: bool,
    out_dir: Option<&Path>,
) -> Result<DocBatchResult, SpandaError> {
    if path.is_dir() {
        let mut result = DocBatchResult {
            outputs: Vec::new(),
            errors: Vec::new(),
        };
        collect_sd_docs(path, html, out_dir, &mut result)?;
        return Ok(result);
    }
    let source = std::fs::read_to_string(path).map_err(|e| SpandaError::Runtime {
        message: format!("read {}: {e}", path.display()),
        line: 0,
    })?;
    let content = if html {
        generate_html(&source, path.file_stem().and_then(|s| s.to_str()))?
    } else {
        generate_markdown(&source)?
    };
    let out_path = if let Some(dir) = out_dir {
        std::fs::create_dir_all(dir).map_err(|e| SpandaError::Runtime {
            message: format!("create {}: {e}", dir.display()),
            line: 0,
        })?;
        let ext = if html { "html" } else { "md" };
        dir.join(format!(
            "{}.{}",
            path.file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("module"),
            ext
        ))
    } else {
        path.to_path_buf()
    };
    if out_dir.is_some() {
        std::fs::write(&out_path, &content).map_err(|e| SpandaError::Runtime {
            message: format!("write {}: {e}", out_path.display()),
            line: 0,
        })?;
    }
    Ok(DocBatchResult {
        outputs: vec![(out_path, content)],
        errors: Vec::new(),
    })
}

fn should_skip_path(path: &Path) -> bool {
    if path
        .file_name()
        .and_then(|n| n.to_str())
        .is_some_and(|n| n.starts_with("._"))
    {
        return true;
    }
    path.components().any(|c| {
        c.as_os_str()
            .to_str()
            .is_some_and(|s| s.starts_with('.') && s != ".")
    })
}

fn collect_sd_docs(
    dir: &Path,
    html: bool,
    out_dir: Option<&Path>,
    result: &mut DocBatchResult,
) -> Result<(), SpandaError> {
    for entry in std::fs::read_dir(dir).map_err(|e| SpandaError::Runtime {
        message: format!("read dir {}: {e}", dir.display()),
        line: 0,
    })? {
        let entry = entry.map_err(|e| SpandaError::Runtime {
            message: format!("read dir entry: {e}"),
            line: 0,
        })?;
        let path = entry.path();
        if should_skip_path(&path) {
            continue;
        }
        if path.is_dir() {
            collect_sd_docs(&path, html, out_dir, result)?;
        } else if path.extension().and_then(|e| e.to_str()) == Some("sd") {
            match generate_docs_for_path(&path, html, out_dir) {
                Ok(single) => result.outputs.extend(single.outputs),
                Err(e) => result.errors.push((path, e.to_string())),
            }
        }
    }
    Ok(())
}

fn render_doc_block(doc: &Option<String>) -> String {
    let Some(text) = doc else {
        return String::new();
    };
    let mut out = String::new();
    for line in text.lines() {
        out.push_str(line);
        out.push_str("\n\n");
    }
    out
}

fn render_program_docs(program: &Program) -> String {
    // Render program docs.
    //
    // Parameters:
    // - `program` — input value
    //
    // Returns:
    // Text result.
    //
    // Options:
    // None.
    //
    // Example:
    // let result = spanda_core::docs::render_program_docs(program);

    // Destructure the program into its top-level sections.
    let Program::Program {
        module_name,
        imports,
        functions,
        tests,
        structs,
        enums,
        traits,
        robots,
        ..
    } = program;
    let mut out = String::new();
    let title = module_name
        .as_deref()
        .unwrap_or("anonymous")
        .replace('.', "/");
    out.push_str(&format!("# Module `{title}`\n\n"));

    // Skip further work when !imports is empty.
    if !imports.is_empty() {
        out.push_str("## Imports\n\n");

        // Emit codegen metadata for each import.
        for import in imports {
            let ImportDecl::ImportDecl { path, .. } = import;
            out.push_str(&format!("- `{path}`\n"));
        }
        out.push('\n');
    }

    // Skip further work when !functions is empty.
    if !functions.is_empty() {
        out.push_str("## Functions\n\n");

        // Generate code for each module function.
        for func in functions {
            out.push_str(&render_module_fn(func));
            out.push('\n');
        }
    }

    // Skip further work when !structs is empty.
    if !structs.is_empty() {
        out.push_str("## Structs\n\n");

        // Process each struct.
        for s in structs {
            out.push_str(&render_struct(s));
            out.push('\n');
        }
    }

    // Skip further work when !enums is empty.
    if !enums.is_empty() {
        out.push_str("## Enums\n\n");

        // Process each enum.
        for e in enums {
            out.push_str(&render_enum(e));
            out.push('\n');
        }
    }

    // Skip further work when !traits is empty.
    if !traits.is_empty() {
        out.push_str("## Traits\n\n");

        // Process each trait.
        for t in traits {
            out.push_str(&render_trait(t));
            out.push('\n');
        }
    }

    // Skip further work when !robots is empty.
    if !robots.is_empty() {
        out.push_str("## Robots\n\n");

        // Handle each robot declared in the program.
        for robot in robots {
            out.push_str(&render_robot(robot));
            out.push('\n');
        }
    }

    // Skip further work when !tests is empty.
    if !tests.is_empty() {
        out.push_str("## Tests\n\n");

        // Run each test block in program order.
        for test in tests {
            out.push_str(&format!(
                "- `\"{}\"` ({} statements)\n",
                test.name,
                test.body.len()
            ));
        }
        out.push('\n');
    }
    out
}

fn render_module_fn(func: &spanda_ast::foundations::ModuleFnDecl) -> String {
    // Render module fn.
    //
    // Parameters:
    // - `func` — input value
    //
    // Returns:
    // Text result.
    //
    // Options:
    // None.
    //
    // Example:
    // let result = spanda_core::docs::render_module_fn(func);

    // Compute visibility for the following logic.
    let visibility = match func.visibility {
        Visibility::Export => "export ",
        Visibility::Public => "public ",
        Visibility::Private => "private ",
    };
    let async_kw = if func.is_async { "async " } else { "" };
    let type_params = if func.type_params.is_empty() {
        String::new()
    } else {
        format!("<{}>", func.type_params.join(", "))
    };
    let params = func
        .params
        .iter()
        .map(|p| format!("{}: {}", p.name, type_name(&p.type_ann)))
        .collect::<Vec<_>>()
        .join(", ");
    format!(
        "{doc}### {visibility}{async_kw}fn `{name}{type_params}({params}) -> {ret}`\n",
        doc = render_doc_block(&func.doc),
        name = func.name,
        ret = type_name(&func.return_type),
    )
}

fn render_struct(decl: &spanda_ast::foundations::StructDecl) -> String {
    // Render struct.
    //
    // Parameters:
    // - `decl` — input value
    //
    // Returns:
    // Text result.
    //
    // Options:
    // None.
    //
    // Example:
    // let result = spanda_core::docs::render_struct(decl);

    // Compute crate for the following logic.
    let spanda_ast::foundations::StructDecl::StructDecl { name, fields, doc, .. } = decl;
    let mut out = format!("{}### `{name}`\n\n", render_doc_block(doc));

    // Check each struct field.
    for field in fields {
        out.push_str(&format!("- `{}`: `{}`\n", field.name, field.type_name));
    }
    out
}

fn render_enum(decl: &spanda_ast::foundations::EnumDecl) -> String {
    // Render enum.
    //
    // Parameters:
    // - `decl` — input value
    //
    // Returns:
    // Text result.
    //
    // Options:
    // None.
    //
    // Example:
    // let result = spanda_core::docs::render_enum(decl);

    // Compute crate for the following logic.
    let spanda_ast::foundations::EnumDecl::EnumDecl { name, variants, doc, .. } = decl;
    let mut out = format!("{}### `{name}`\n\n", render_doc_block(doc));

    // Handle each enum variant arm.
    for variant in variants {
        // Skip further work when field types is empty.
        if variant.field_types.is_empty() {
            out.push_str(&format!("- `{}`\n", variant.name));
        } else {
            out.push_str(&format!(
                "- `{}({})`\n",
                variant.name,
                variant.field_types.join(", ")
            ));
        }
    }
    out
}

fn render_trait(decl: &spanda_ast::foundations::TraitDecl) -> String {
    // Render trait.
    //
    // Parameters:
    // - `decl` — input value
    //
    // Returns:
    // Text result.
    //
    // Options:
    // None.
    //
    // Example:
    // let result = spanda_core::docs::render_trait(decl);

    // Compute crate for the following logic.
    let spanda_ast::foundations::TraitDecl::TraitDecl { name, methods, doc, .. } = decl;
    let mut out = format!("{}### `{name}`\n\n", render_doc_block(doc));

    // Process each method.
    for method in methods {
        let params = method
            .params
            .iter()
            .map(|p| format!("{}: {}", p.name, p.type_name))
            .collect::<Vec<_>>()
            .join(", ");
        out.push_str(&format!(
            "- `fn {}({}) -> {}`\n",
            method.name, params, method.return_type
        ));
    }
    out
}

fn render_robot(robot: &RobotDecl) -> String {
    // Render robot.
    //
    // Parameters:
    // - `robot` — input value
    //
    // Returns:
    // Text result.
    //
    // Options:
    // None.
    //
    // Example:
    // let result = spanda_core::docs::render_robot(robot);

    // Compute RobotDecl for the following logic.
    let RobotDecl::RobotDecl {
        name,
        sensors,
        actuators,
        agents,
        behaviors,
        tasks,
        doc,
        ..
    } = robot;
    let mut out = format!("{}### `{name}`\n\n", render_doc_block(doc));

    // Skip further work when !sensors is empty.
    if !sensors.is_empty() {
        out.push_str("**Sensors**\n\n");

        // Process each sensor.
        for sensor in sensors {
            let SensorDecl::SensorDecl {
                name, sensor_type, ..
            } = sensor;
            out.push_str(&format!("- `{name}`: `{sensor_type}`\n"));
        }
        out.push('\n');
    }

    // Skip further work when !actuators is empty.
    if !actuators.is_empty() {
        out.push_str("**Actuators**\n\n");

        // Process each actuator.
        for actuator in actuators {
            let ActuatorDecl::ActuatorDecl {
                name,
                actuator_type,
                ..
            } = actuator;
            out.push_str(&format!("- `{name}`: `{actuator_type}`\n"));
        }
        out.push('\n');
    }

    // Skip further work when !agents is empty.
    if !agents.is_empty() {
        out.push_str("**Agents**\n\n");

        // Process each agent.
        for agent in agents {
            let AgentDecl::AgentDecl { name, goal, .. } = agent;
            out.push_str(&format!("- `{name}` — goal: \"{goal}\"\n"));
        }
        out.push('\n');
    }

    // Skip further work when !behaviors is empty.
    if !behaviors.is_empty() {
        out.push_str("**Behaviors**\n\n");

        // Process each behavior.
        for behavior in behaviors {
            let BehaviorDecl::BehaviorDecl { name, .. } = behavior;
            out.push_str(&format!("- `{name}()`\n"));
        }
        out.push('\n');
    }

    // Skip further work when !tasks is empty.
    if !tasks.is_empty() {
        out.push_str("**Tasks**\n\n");

        // Process each task.
        for task in tasks {
            let spanda_ast::foundations::TaskDecl::TaskDecl {
                name, interval_ms, ..
            } = task;
            out.push_str(&format!("- `{name}` every {interval_ms}ms\n"));
        }
    }
    out
}

fn type_name(ty: &SpandaType) -> String {
    // Type name.
    //
    // Parameters:
    // - `ty` — input value
    //
    // Returns:
    // Text result.
    //
    // Options:
    // None.
    //
    // Example:
    // let result = spanda_core::docs::type_name(ty);

    // Match on ty and handle each case.
    match ty {
        SpandaType::Void => "Void".into(),
        SpandaType::Int => "Int".into(),
        SpandaType::Float => "Float".into(),
        SpandaType::Bool => "Bool".into(),
        SpandaType::String => "String".into(),
        SpandaType::Char => "Char".into(),
        SpandaType::Bytes => "Bytes".into(),
        SpandaType::Null => "Null".into(),
        SpandaType::Number { unit } => {
            // Take the branch when *unit equals None.
            if *unit == UnitKind::None {
                "Number".into()
            } else {
                format!("Number({})", unit.as_str())
            }
        }
        SpandaType::Named { name } => name.clone(),
        SpandaType::Generic { name, type_args } => {
            let args = type_args
                .iter()
                .map(type_name)
                .collect::<Vec<_>>()
                .join(", ");
            format!("{name}<{args}>")
        }
        SpandaType::TypeParam { name } => name.clone(),
        SpandaType::Scan => "Scan".into(),
        SpandaType::Pose => "Pose".into(),
        SpandaType::Velocity => "Velocity".into(),
        SpandaType::Trajectory => "Trajectory".into(),
        SpandaType::Transform => "Transform".into(),
        SpandaType::EnumVariant { enum_name, variant } => format!("{enum_name}.{variant}"),
        SpandaType::TraitObject { trait_name } => format!("dyn {trait_name}"),
        SpandaType::Regex => "Regex".into(),
        SpandaType::Match => "Match".into(),
        SpandaType::Capture => "Capture".into(),
        SpandaType::CaptureGroup => "CaptureGroup".into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_doc_comments_on_functions() {
        let source = r#"
/// Plans a safe path between two poses.
export fn plan_path(start: Pose, goal: Pose) -> Path {
  return trajectory(from: start, to: goal, steps: 3);
}
"#;
        let md = generate_markdown(source).expect("docs");
        assert!(md.contains("Plans a safe path between two poses."));
        assert!(md.contains("export fn `plan_path("));
    }

    #[test]
    fn generates_module_docs() {
        // Generates module docs.
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
        // let result = spanda_core::docs::generates_module_docs();

        let source = r#"
module navigation;

export fn plan() -> Path {
  return trajectory(from: pose(x: 0.0 m, y: 0.0 m), to: pose(x: 1.0 m, y: 0.0 m), steps: 3);
}

robot R {
  actuator wheels: DifferentialDrive;
  behavior run() { wheels.stop(); }
}
"#;
        let md = generate_markdown(source).expect("docs");
        assert!(md.contains("# Module `navigation`"));
        assert!(md.contains("export fn `plan("));
        assert!(md.contains("### `R`"));
    }
}
