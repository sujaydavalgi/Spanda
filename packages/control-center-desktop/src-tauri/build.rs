fn main() {
    let config_path = patch_tauri_config();
    tauri_build::Builder::default()
        .config_path(config_path)
        .build();
}

fn patch_tauri_config() -> std::path::PathBuf {
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let source = manifest_dir.join("tauri.conf.json");
    let mut config: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&source).expect("read tauri.conf.json"))
            .expect("parse tauri.conf.json");
    if let Ok(pubkey) = std::env::var("TAURI_UPDATER_PUBKEY") {
        if !pubkey.trim().is_empty() {
            config["plugins"]["updater"]["pubkey"] = serde_json::Value::String(pubkey);
            let active = std::env::var("TAURI_UPDATER_ACTIVE")
                .map(|value| value == "1" || value.eq_ignore_ascii_case("true"))
                .unwrap_or(true);
            config["plugins"]["updater"]["active"] = serde_json::Value::Bool(active);
        }
    }
    let out = std::path::Path::new(&std::env::var("OUT_DIR").expect("OUT_DIR"))
        .join("tauri.conf.json");
    std::fs::write(&out, serde_json::to_string_pretty(&config).expect("serialize tauri config"))
        .expect("write patched tauri config");
    out
}
