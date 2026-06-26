//! Remote attestation AK certificate chain validation for secure-boot quotes.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};

/// Result of validating an attestation key certificate chain.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AkCertChainValidation {
    pub verified: bool,
    pub chain_length: usize,
    pub anchor_matched: bool,
    pub detail: String,
}

/// Validate an AK certificate chain against an optional trust store directory.
pub fn validate_ak_cert_chain(
    pem_chain: &[String],
    trust_store_dir: Option<&Path>,
) -> AkCertChainValidation {
    // Verify attestation key PEM chain against trusted CA fingerprints.
    //
    // Parameters:
    // - `pem_chain` — ordered PEM certificates (leaf AK first)
    // - `trust_store_dir` — directory of trusted CA/intermediate PEM files
    //
    // Returns:
    // Chain validation outcome with anchor match detail.
    //
    // Options:
    // `SPANDA_ATTESTATION_AK_CHAIN_MIN` — minimum chain length (default 1)
    // `SPANDA_ATTESTATION_AK_EXPECT_FINGERPRINT` — optional leaf SHA-256 fingerprint
    // `SPANDA_ATTESTATION_OPENSSL_VERIFY` — run `openssl verify` when set to `1`
    //
    // Example:
    // let result = validate_ak_cert_chain(&chain, Some(Path::new("trust-store")));

    let min_chain = std::env::var("SPANDA_ATTESTATION_AK_CHAIN_MIN")
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(1);

    let parsed = pem_chain
        .iter()
        .filter_map(|pem| parse_pem_certificate(pem))
        .collect::<Vec<_>>();

    if parsed.is_empty() {
        return AkCertChainValidation {
            verified: false,
            chain_length: 0,
            anchor_matched: false,
            detail: "ak cert chain empty or unparsable".into(),
        };
    }

    if parsed.len() < min_chain {
        return AkCertChainValidation {
            verified: false,
            chain_length: parsed.len(),
            anchor_matched: false,
            detail: format!(
                "ak cert chain length {} below minimum {}",
                parsed.len(),
                min_chain
            ),
        };
    }

    let fingerprints: Vec<String> = parsed.iter().map(|der| cert_fingerprint(der)).collect();
    let mut anchor_matched = false;

    if let Some(expected_leaf) = std::env::var("SPANDA_ATTESTATION_AK_EXPECT_FINGERPRINT")
        .ok()
        .filter(|value| !value.trim().is_empty())
    {
        let expected = normalize_fingerprint(&expected_leaf);
        if fingerprints
            .first()
            .map(|fp| fp == &expected)
            .unwrap_or(false)
        {
            anchor_matched = true;
        } else {
            return AkCertChainValidation {
                verified: false,
                chain_length: parsed.len(),
                anchor_matched: false,
                detail: format!(
                    "leaf ak fingerprint mismatch: expected {expected} got {}",
                    fingerprints.first().cloned().unwrap_or_default()
                ),
            };
        }
    }

    if let Some(store_dir) = trust_store_dir {
        let trusted = load_trust_store_fingerprints(store_dir);
        if trusted.is_empty() {
            return AkCertChainValidation {
                verified: false,
                chain_length: parsed.len(),
                anchor_matched: false,
                detail: format!(
                    "trust store {} contains no PEM certificates",
                    store_dir.display()
                ),
            };
        }

        anchor_matched = fingerprints
            .iter()
            .any(|fingerprint| trusted.iter().any(|trusted_fp| trusted_fp == fingerprint));
        if !anchor_matched {
            return AkCertChainValidation {
                verified: false,
                chain_length: parsed.len(),
                anchor_matched: false,
                detail: "no chain certificate matched trust store anchor fingerprints".into(),
            };
        }
    } else if !anchor_matched {
        return AkCertChainValidation {
            verified: false,
            chain_length: parsed.len(),
            anchor_matched: false,
            detail:
                "ak cert chain present but no trust store or leaf fingerprint policy configured"
                    .into(),
        };
    }

    if openssl_verify_enabled() {
        if let Err(error) = verify_with_openssl(pem_chain, trust_store_dir) {
            return AkCertChainValidation {
                verified: false,
                chain_length: parsed.len(),
                anchor_matched,
                detail: error,
            };
        }
    }

    AkCertChainValidation {
        verified: true,
        chain_length: parsed.len(),
        anchor_matched,
        detail: if anchor_matched {
            format!(
                "ak cert chain verified ({} certs, anchor matched)",
                parsed.len()
            )
        } else {
            format!("ak cert chain verified ({} certs)", parsed.len())
        },
    }
}

/// Resolve the configured attestation trust store directory when present.
pub fn attestation_trust_store_dir() -> Option<PathBuf> {
    std::env::var("SPANDA_ATTESTATION_TRUST_STORE")
        .ok()
        .filter(|value| !value.trim().is_empty())
        .map(PathBuf::from)
        .filter(|path| path.is_dir())
}

fn parse_pem_certificate(pem: &str) -> Option<Vec<u8>> {
    let body = pem
        .lines()
        .filter(|line| !line.starts_with("-----"))
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect::<String>();
    if body.is_empty() {
        return None;
    }
    decode_base64(&body).ok()
}

fn decode_base64(input: &str) -> Result<Vec<u8>, String> {
    const TABLE: &[u8; 256] = &{
        let mut table = [255u8; 256];
        let mut index = 0u8;
        while index < 26 {
            table[(b'A' + index) as usize] = index;
            table[(b'a' + index) as usize] = index + 26;
            index += 1;
        }
        let mut digit = 0u8;
        while digit < 10 {
            table[(b'0' + digit) as usize] = digit + 52;
            digit += 1;
        }
        table[b'+' as usize] = 62;
        table[b'/' as usize] = 63;
        table
    };

    let mut output = Vec::with_capacity(input.len() * 3 / 4);
    let mut buffer = 0u32;
    let mut bits = 0u32;

    for byte in input.bytes().filter(|byte| !byte.is_ascii_whitespace()) {
        if byte == b'=' {
            break;
        }
        let value = TABLE[byte as usize];
        if value == 255 {
            return Err(format!("invalid base64 byte {byte}"));
        }
        buffer = (buffer << 6) | u32::from(value);
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            output.push((buffer >> bits) as u8);
            buffer &= (1 << bits) - 1;
        }
    }
    Ok(output)
}

fn cert_fingerprint(der: &[u8]) -> String {
    let digest = Sha256::digest(der);
    hex::encode(digest)
}

fn normalize_fingerprint(value: &str) -> String {
    value
        .replace(':', "")
        .chars()
        .filter(|ch| ch.is_ascii_hexdigit())
        .map(|ch| ch.to_ascii_lowercase())
        .collect()
}

fn load_trust_store_fingerprints(store_dir: &Path) -> Vec<String> {
    let mut fingerprints = Vec::new();
    let entries = std::fs::read_dir(store_dir).ok();
    let Some(entries) = entries else {
        return fingerprints;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("pem") {
            continue;
        }
        let Ok(text) = std::fs::read_to_string(&path) else {
            continue;
        };
        if let Some(der) = parse_pem_certificate(&text) {
            fingerprints.push(cert_fingerprint(&der));
        }
    }
    fingerprints
}

fn openssl_verify_enabled() -> bool {
    std::env::var("SPANDA_ATTESTATION_OPENSSL_VERIFY")
        .ok()
        .map(|value| value == "1" || value.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

fn verify_with_openssl(pem_chain: &[String], trust_store_dir: Option<&Path>) -> Result<(), String> {
    let temp_dir = std::env::temp_dir().join(format!("spanda_ak_chain_{}", std::process::id()));
    std::fs::create_dir_all(&temp_dir).map_err(|error| error.to_string())?;
    let chain_path = temp_dir.join("ak-chain.pem");
    std::fs::write(&chain_path, pem_chain.join("\n")).map_err(|error| error.to_string())?;

    let mut command = std::process::Command::new("openssl");
    command.arg("verify");
    if let Some(store_dir) = trust_store_dir {
        for entry in std::fs::read_dir(store_dir)
            .map_err(|error| error.to_string())?
            .flatten()
        {
            let path = entry.path();
            if path.extension().and_then(|ext| ext.to_str()) == Some("pem") {
                command.arg("-CAfile").arg(path);
                break;
            }
        }
    }
    command.arg(&chain_path);
    let output = command
        .output()
        .map_err(|error| format!("openssl unavailable: {error}"))?;
    let _ = std::fs::remove_dir_all(&temp_dir);
    if output.status.success() {
        Ok(())
    } else {
        Err(format!(
            "openssl verify failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::attestation_sync::attestation_env_lock;

    const SAMPLE_PEM: &str = "-----BEGIN CERTIFICATE-----\nQUJDRA==\n-----END CERTIFICATE-----";

    #[test]
    fn validates_chain_against_trust_store_fingerprint() {
        let _guard = attestation_env_lock();
        std::env::remove_var("SPANDA_ATTESTATION_AK_EXPECT_FINGERPRINT");
        std::env::remove_var("SPANDA_ATTESTATION_OPENSSL_VERIFY");
        let store = std::env::temp_dir().join(format!("spanda_trust_store_{}", std::process::id()));
        let _ = std::fs::create_dir_all(&store);
        std::fs::write(store.join("anchor.pem"), SAMPLE_PEM).expect("write anchor");
        let chain = vec![SAMPLE_PEM.into()];
        let result = validate_ak_cert_chain(&chain, Some(&store));
        assert!(result.verified, "detail={}", result.detail);
        assert!(result.anchor_matched);
        let _ = std::fs::remove_dir_all(&store);
    }

    #[test]
    fn rejects_chain_without_trust_anchor() {
        let _guard = attestation_env_lock();
        std::env::remove_var("SPANDA_ATTESTATION_AK_EXPECT_FINGERPRINT");
        let chain = vec![SAMPLE_PEM.into()];
        let result = validate_ak_cert_chain(&chain, None);
        assert!(!result.verified);
    }

    #[test]
    fn leaf_fingerprint_policy_verifies_chain() {
        let _guard = attestation_env_lock();
        std::env::remove_var("SPANDA_ATTESTATION_OPENSSL_VERIFY");
        let der = decode_base64("QUJDRA==").expect("decode");
        let fingerprint = cert_fingerprint(&der);
        std::env::set_var("SPANDA_ATTESTATION_AK_EXPECT_FINGERPRINT", &fingerprint);
        let chain = vec![SAMPLE_PEM.into()];
        let result = validate_ak_cert_chain(&chain, None);
        assert!(result.verified, "detail={}", result.detail);
        std::env::remove_var("SPANDA_ATTESTATION_AK_EXPECT_FINGERPRINT");
    }
}
