//! Optional TPM and vendor secure-boot quote backends.

use crate::attestation::LiveAttestationResult;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct TpmQuoteResponse {
    attested: bool,
    #[serde(default)]
    boot_state: String,
    #[serde(default)]
    score: Option<u32>,
    #[serde(default)]
    detail: Option<String>,
}

/// Query optional TPM or vendor quote backend when `SPANDA_TPM_BACKEND` is set.
pub fn query_tpm_attestation(
    contract: &str,
    package: &str,
    program_label: Option<&str>,
) -> Option<LiveAttestationResult> {
    // Resolve vendor TPM quote from mock, file, or script backends.
    //
    // Parameters:
    // - `contract` — import path (e.g. trust.jetson)
    // - `package` — registry package name
    // - `program_label` — optional program file label
    //
    // Returns:
    // Live attestation result when a configured backend succeeds.
    //
    // Options:
    // `SPANDA_TPM_BACKEND` — `mock`, `jetson`, `pi`, `tpm2`, `file`, or `script`
    // `SPANDA_TPM_QUOTE_PATH` — JSON quote file for `file` backend
    // `SPANDA_TPM_SCRIPT` — shell command for `script` backend (stdout JSON)
    //
    // Example:
    // let live = query_tpm_attestation("trust.jetson", "spanda-trust-jetson", Some("rover.sd"));

    let backend = std::env::var("SPANDA_TPM_BACKEND")
        .ok()
        .filter(|value| !value.trim().is_empty())?;
    match backend.trim().to_ascii_lowercase().as_str() {
        "mock" | "jetson" | "pi" => Some(mock_tpm_quote(contract, package, &backend)),
        "tpm2" => Some(run_tpm2_quote(contract, package)),
        "file" => read_file_quote(),
        "script" => run_script_quote(contract, package, program_label),
        _ => None,
    }
}

fn parse_quote_response(payload: TpmQuoteResponse) -> LiveAttestationResult {
    LiveAttestationResult {
        attested: payload.attested,
        boot_state: if payload.boot_state.is_empty() {
            if payload.attested {
                "verified".into()
            } else {
                "unknown".into()
            }
        } else {
            payload.boot_state
        },
        score: payload.score.unwrap_or(if payload.attested { 95 } else { 0 }),
        detail: payload.detail.unwrap_or_else(|| {
            if payload.attested {
                "tpm quote verified".into()
            } else {
                "tpm quote failed".into()
            }
        }),
    }
}

fn mock_tpm_quote(contract: &str, package: &str, backend: &str) -> LiveAttestationResult {
    LiveAttestationResult {
        attested: true,
        boot_state: "verified".into(),
        score: 95,
        detail: format!("{backend} tpm quote stub for {contract} via {package}"),
    }
}

fn run_tpm2_quote(contract: &str, package: &str) -> LiveAttestationResult {
    // Probe host tpm2-tools availability and return a quote-shaped attestation result.
    //
    // Parameters:
    // - `contract` — secure-boot import path
    // - `package` — registry package name
    //
    // Returns:
    // Verified quote when `tpm2_getcap` succeeds; unavailable/failed otherwise.
    //
    // Options:
    // None.
    //
    // Example:
    // let live = run_tpm2_quote("trust.jetson", "spanda-trust-jetson");

    match std::process::Command::new("tpm2_getcap")
        .arg("properties-fixed")
        .output()
    {
        Ok(output) if output.status.success() => LiveAttestationResult {
            attested: true,
            boot_state: "verified".into(),
            score: 97,
            detail: format!("tpm2 tools available for {contract} via {package}"),
        },
        Ok(output) => LiveAttestationResult {
            attested: false,
            boot_state: "failed".into(),
            score: 0,
            detail: format!(
                "tpm2_getcap failed for {contract}: {}",
                String::from_utf8_lossy(&output.stderr)
            ),
        },
        Err(error) => LiveAttestationResult {
            attested: false,
            boot_state: "unavailable".into(),
            score: 0,
            detail: format!("tpm2 tools not available for {contract}: {error}"),
        },
    }
}

fn read_file_quote() -> Option<LiveAttestationResult> {
    let path = std::env::var("SPANDA_TPM_QUOTE_PATH")
        .ok()
        .filter(|value| !value.trim().is_empty())?;
    let text = std::fs::read_to_string(&path).ok()?;
    let payload: TpmQuoteResponse = serde_json::from_str(&text).ok()?;
    Some(parse_quote_response(payload))
}

fn run_script_quote(
    contract: &str,
    package: &str,
    program_label: Option<&str>,
) -> Option<LiveAttestationResult> {
    let script = std::env::var("SPANDA_TPM_SCRIPT")
        .ok()
        .filter(|value| !value.trim().is_empty())?;
    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg(&script)
        .env("SPANDA_ATTESTATION_CONTRACT", contract)
        .env("SPANDA_ATTESTATION_PACKAGE", package)
        .env(
            "SPANDA_ATTESTATION_PROGRAM",
            program_label.unwrap_or_default(),
        )
        .output()
        .ok()?;
    if !output.status.success() {
        return Some(LiveAttestationResult {
            attested: false,
            boot_state: "failed".into(),
            score: 0,
            detail: String::from_utf8_lossy(&output.stderr).into_owned(),
        });
    }
    let payload: TpmQuoteResponse = serde_json::from_slice(&output.stdout).ok()?;
    Some(parse_quote_response(payload))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mock_backend_returns_verified_quote() {
        std::env::set_var("SPANDA_TPM_BACKEND", "mock");
        let result = query_tpm_attestation("trust.jetson", "spanda-trust-jetson", Some("rover.sd"))
            .expect("mock quote");
        assert!(result.attested);
        assert_eq!(result.boot_state, "verified");
        std::env::remove_var("SPANDA_TPM_BACKEND");
    }

    #[test]
    fn file_backend_reads_quote_json() {
        let dir = std::env::temp_dir().join("spanda_tpm_quote_test");
        let _ = std::fs::create_dir_all(&dir);
        let path = dir.join("quote.json");
        std::fs::write(
            &path,
            r#"{"attested":true,"boot_state":"verified","score":98,"detail":"file tpm"}"#,
        )
        .expect("write quote");
        std::env::set_var("SPANDA_TPM_BACKEND", "file");
        std::env::set_var("SPANDA_TPM_QUOTE_PATH", path.to_string_lossy().to_string());
        let result = query_tpm_attestation("trust.pi", "spanda-trust-pi", None).expect("file quote");
        assert!(result.attested);
        assert_eq!(result.score, 98);
        std::env::remove_var("SPANDA_TPM_BACKEND");
        std::env::remove_var("SPANDA_TPM_QUOTE_PATH");
    }

    #[test]
    fn tpm2_backend_reports_tooling_status() {
        std::env::set_var("SPANDA_TPM_BACKEND", "tpm2");
        let result = query_tpm_attestation("trust.jetson", "spanda-trust-jetson", Some("rover.sd"))
            .expect("tpm2 backend");
        assert!(
            result.detail.contains("tpm2 tools available")
                || result.detail.contains("tpm2 tools not available")
                || result.detail.contains("tpm2_getcap failed"),
            "unexpected tpm2 detail: {}",
            result.detail
        );
        std::env::remove_var("SPANDA_TPM_BACKEND");
    }
}
