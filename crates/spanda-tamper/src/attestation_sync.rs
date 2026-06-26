//! Serialize attestation environment mutations across unit and integration tests.

use std::sync::{Mutex, MutexGuard};

static ENV_LOCK: Mutex<()> = Mutex::new(());

/// Serialize tests that mutate attestation-related environment variables.
pub fn attestation_env_lock() -> MutexGuard<'static, ()> {
    ENV_LOCK.lock().unwrap_or_else(|error| error.into_inner())
}

/// Clear attestation env vars that affect live TPM/HTTP verification.
pub fn clear_attestation_env() {
    for key in [
        "SPANDA_ATTESTATION_ENDPOINT",
        "SPANDA_ATTESTATION_TRUST_STORE",
        "SPANDA_ATTESTATION_AK_CHAIN_OPTIONAL",
        "SPANDA_ATTESTATION_AK_EXPECT_FINGERPRINT",
        "SPANDA_ATTESTATION_OPENSSL_VERIFY",
        "SPANDA_TPM_BACKEND",
        "SPANDA_TPM_VENDOR_SDK",
    ] {
        std::env::remove_var(key);
    }
}
