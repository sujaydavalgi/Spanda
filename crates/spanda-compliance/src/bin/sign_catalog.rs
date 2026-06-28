//! Generate signed compliance catalog manifest for commit.
use spanda_audit::crypto::{public_key_from_material, sha256, sign};
use std::fs;
use std::path::PathBuf;

fn main() {
    let material = "spanda-official-catalog-signing-v1";
    let pubkey = public_key_from_material(material);
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let templates = [
        ("defense", "templates/defense.json"),
        ("medical", "templates/medical.json"),
        ("iso26262", "templates/iso26262.json"),
        ("iso13849", "templates/iso13849.json"),
        ("iec61508", "templates/iec61508.json"),
    ];
    let mut entries = Vec::new();
    for (name, path) in templates {
        let full = root.join(path);
        let raw = fs::read_to_string(&full).expect("read template");
        let hash = sha256(&raw);
        let signature = sign(&hash.0, material);
        entries.push(serde_json::json!({
            "name": name,
            "version": "1.0.0",
            "content_sha256": hash.0,
            "signature": signature,
            "template_path": path,
        }));
    }
    let manifest = serde_json::json!({
        "publisher_pubkey": pubkey,
        "entries": entries,
    });
    let out = root.join("templates/catalog.json");
    fs::write(&out, serde_json::to_string_pretty(&manifest).unwrap()).expect("write catalog");
    println!("wrote {}", out.display());
}
