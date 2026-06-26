fn main() {
  inject_updater_config_from_env();
  tauri_build::build();
}

fn inject_updater_config_from_env() {
  let Ok(pubkey) = std::env::var("TAURI_UPDATER_PUBKEY") else {
    return;
  };
  if pubkey.trim().is_empty() {
    return;
  }
  let active = std::env::var("TAURI_UPDATER_ACTIVE")
    .map(|value| value == "1" || value.eq_ignore_ascii_case("true"))
    .unwrap_or(true);
  let merge = serde_json::json!({
      "plugins": {
          "updater": {
              "pubkey": pubkey,
              "active": active
          }
      }
  });
  std::env::set_var("TAURI_CONFIG", merge.to_string());
  println!("cargo:rerun-if-env-changed=TAURI_UPDATER_PUBKEY");
  println!("cargo:rerun-if-env-changed=TAURI_UPDATER_ACTIVE");
}
