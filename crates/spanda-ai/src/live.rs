//! Live AI provider backends (OpenAI via Python bridge or native HTTP).

use crate::{
    action_proposal, build_prompt, scan_distance, AiProvider, CompletionRequest, DetectionRequest,
    EmbedRequest, MockAiProvider,
};
use spanda_runtime::value::RuntimeValue;
use std::io::{Read, Write};
use std::process::{Command, Stdio};

/// Return true when live AI providers should be used instead of mock-only mode.
pub fn live_ai_enabled() -> bool {
    std::env::var("SPANDA_LIVE_AI").ok().as_deref() != Some("0")
        && std::env::var("OPENAI_API_KEY")
            .ok()
            .is_some_and(|key| !key.is_empty())
}

/// Select a runtime AI provider for the configured provider name.
pub fn resolve_ai_provider(provider: &str) -> Box<dyn AiProvider> {
    // Select a runtime AI provider for the configured provider name.
    //
    // Parameters:
    // - `provider` — configured provider string from ai_model block
    //
    // Returns:
    // Boxed provider implementation.
    //
    // Options:
    // - `OPENAI_API_KEY` + `SPANDA_LIVE_AI` enable live OpenAI path
    //
    // Example:
    // let backend = resolve_ai_provider("openai");

    match provider.to_ascii_lowercase().as_str() {
        "openai" if live_ai_enabled() => Box::new(OpenAiProvider),
        _ => Box::new(MockAiProvider),
    }
}

pub struct OpenAiProvider;

impl AiProvider for OpenAiProvider {
    fn complete(&self, request: &CompletionRequest) -> RuntimeValue {
        // Complete an LLM prompt through the live OpenAI bridge.
        //
        // Parameters:
        // - `self` — method receiver
        // - `request` — completion request
        //
        // Returns:
        // ActionProposal derived from model output.
        //
        // Options:
        // None.
        //
        // Example:
        // let proposal = provider.complete(request);

        let prompt = build_prompt(&request.prompt, request.input.as_ref(), None);
        if let Some(text) = call_openai_complete(&prompt) {
            return proposal_from_completion(&text, request);
        }
        MockAiProvider.complete(request)
    }

    fn detect(&self, request: &DetectionRequest) -> RuntimeValue {
        MockAiProvider.detect(request)
    }

    fn embed(&self, request: &EmbedRequest) -> RuntimeValue {
        MockAiProvider.embed(request)
    }
}

fn proposal_from_completion(text: &str, request: &CompletionRequest) -> RuntimeValue {
    // Map OpenAI text output into an ActionProposal using safety heuristics.
    //
    // Parameters:
    // - `text` — model completion text
    // - `request` — original completion request
    //
    // Returns:
    // ActionProposal runtime value.
    //
    // Options:
    // None.
    //
    // Example:
    // let proposal = proposal_from_completion("stop", request);

    let dist = scan_distance(request.input.as_ref());
    let lower = text.to_ascii_lowercase();
    if lower.contains("stop") || lower.contains("halt") || lower.contains("wait") {
        return action_proposal(
            0.0,
            0.0,
            &request.model,
            vec![
                format!("model={}", request.model),
                format!("provider={}", request.provider),
                format!("completion={text}"),
                "decision=stop".into(),
            ],
        );
    }
    if lower.contains("turn") || lower.contains("avoid") || dist < 0.8 {
        let angular = if dist < 0.4 { 0.6 } else { 0.25 };
        let linear = if dist < 0.4 { 0.0 } else { (0.4_f64).min(dist * 0.3) };
        return action_proposal(
            linear,
            angular,
            &request.model,
            vec![
                format!("model={}", request.model),
                format!("provider={}", request.provider),
                format!("completion={text}"),
                "decision=avoid_obstacle".into(),
            ],
        );
    }
    let linear = (0.8_f64).min(dist * 0.45);
    action_proposal(
        linear,
        0.0,
        &request.model,
        vec![
            format!("model={}", request.model),
            format!("provider={}", request.provider),
            format!("completion={text}"),
            "decision=forward".into(),
        ],
    )
}

fn call_openai_complete(prompt: &str) -> Option<String> {
    // Invoke the Python bridge openai_complete handler.
    //
    // Parameters:
    // - `prompt` — user prompt text
    //
    // Returns:
    // Completion text when the bridge succeeds, otherwise none.
    //
    // Options:
    // None.
    //
    // Example:
    // let text = call_openai_complete("Plan a safe stop");

    let response = call_python_bridge(
        "openai_complete",
        vec![serde_json::Value::String(prompt.to_string())],
    )?;
    match response.get("result") {
        Some(serde_json::Value::String(text)) if !text.is_empty() => Some(text.clone()),
        _ => None,
    }
}

fn call_python_bridge(fn_name: &str, args: Vec<serde_json::Value>) -> Option<serde_json::Value> {
    // Invoke `scripts/spanda_python_bridge.py` with a JSON request.
    //
    // Parameters:
    // - `fn_name` — bridge handler name
    // - `args` — handler arguments
    //
    // Returns:
    // Parsed JSON response when the bridge succeeds, otherwise none.
    //
    // Options:
    // None.
    //
    // Example:
    // let response = call_python_bridge("openai_complete", vec![]);

    let script = bridge_script_path()?;
    let python = std::env::var("SPANDA_PYTHON").unwrap_or_else(|_| "python3".into());
    let request = serde_json::json!({ "fn": fn_name, "args": args });
    let mut child = Command::new(python)
        .arg(script)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .ok()?;
    {
        let stdin = child.stdin.as_mut()?;
        let payload = serde_json::to_string(&request).ok()?;
        stdin.write_all(payload.as_bytes()).ok()?;
    }
    let mut stdout = String::new();
    child
        .stdout
        .as_mut()?
        .read_to_string(&mut stdout)
        .ok()?;
    let _ = child.wait();
    let parsed: serde_json::Value = serde_json::from_str(stdout.trim()).ok()?;
    if parsed.get("ok") == Some(&serde_json::Value::Bool(true)) {
        Some(parsed)
    } else {
        None
    }
}

fn bridge_script_path() -> Option<String> {
    // Resolve the Python bridge script path from env or repo layout.
    //
    // Parameters:
    // None.
    //
    // Returns:
    // Script path when found, otherwise none.
    //
    // Options:
    // None.
    //
    // Example:
    // let path = bridge_script_path();

    if let Ok(path) = std::env::var("SPANDA_PYTHON_BRIDGE") {
        if std::path::Path::new(&path).is_file() {
            return Some(path);
        }
    }
    let candidates = [
        "scripts/spanda_python_bridge.py".to_string(),
        format!(
            "{}/../../scripts/spanda_python_bridge.py",
            env!("CARGO_MANIFEST_DIR")
        ),
    ];
    for candidate in candidates {
        if std::path::Path::new(&candidate).is_file() {
            return Some(candidate);
        }
    }
    std::env::current_dir()
        .ok()
        .map(|cwd| cwd.join("scripts/spanda_python_bridge.py"))
        .filter(|p| p.is_file())
        .map(|p| p.to_string_lossy().into_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn live_ai_disabled_without_api_key() {
        std::env::remove_var("OPENAI_API_KEY");
        std::env::remove_var("SPANDA_LIVE_AI");
        assert!(!live_ai_enabled());
    }
}
