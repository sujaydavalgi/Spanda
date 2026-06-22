//! Live cloud upload integration when `SPANDA_CLOUD_UPLOAD_URL` is set.

use spanda_providers::package_stubs::CloudPackageStub;
use spanda_runtime::providers::CloudProvider;
use spanda_runtime::value::RuntimeValue;
use std::sync::{Mutex, OnceLock};

static ENV_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

fn env_lock() -> &'static Mutex<()> {
    ENV_LOCK.get_or_init(|| Mutex::new(()))
}

#[test]
fn cloud_upload_posts_when_url_set() {
    let Ok(url) = std::env::var("SPANDA_CLOUD_UPLOAD_URL") else {
        return;
    };
    let _guard = env_lock().lock().expect("env lock");
    assert!(!url.trim().is_empty());

    let mut provider = CloudPackageStub;
    provider
        .upload(
            "twin/replay.json",
            RuntimeValue::String {
                value: "replay-frame".into(),
            },
        )
        .expect("cloud upload should succeed against mock server");
}
