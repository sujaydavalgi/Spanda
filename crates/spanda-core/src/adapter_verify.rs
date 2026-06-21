//! Framework adapter import verification for `spanda verify`.

use crate::ast::ImportDecl;
use crate::hardware::{CompatItem, CompatSeverity};

fn pass(category: &str, message: impl Into<String>, line: u32, column: u32) -> CompatItem {
    CompatItem {
        category: category.into(),
        message: message.into(),
        severity: CompatSeverity::Pass,
        line,
        column,
    }
}

const FRAMEWORK_IMPORT_PACKAGES: &[(&str, &str)] = &[
    ("robotics.ros2", "spanda-ros2"),
    ("communication.mqtt", "spanda-mqtt"),
    ("vision.opencv", "spanda-opencv"),
    ("vision.yolo", "spanda-yolo"),
    ("navigation.slam", "spanda-slam"),
    ("navigation.path_planning", "spanda-nav"),
    ("navigation.nav2", "spanda-nav2"),
    ("navigation.cartographer", "spanda-cartographer"),
    ("navigation.rtabmap", "spanda-rtabmap"),
    ("vision.detectron", "spanda-detectron"),
    ("manipulation.grasp", "spanda-manipulation"),
    ("hri.dialogue", "spanda-hri"),
    ("twin.sync", "spanda-digital-twin"),
    ("sim.gazebo", "spanda-sim-gazebo"),
    ("sim.webots", "spanda-sim-webots"),
    ("connectivity.ble", "spanda-ble"),
    ("positioning.gps", "spanda-gps"),
    ("connectivity.lte", "spanda-lte"),
];

/// Report registry adapter mappings for framework import paths declared in a program.
pub fn verify_framework_imports(imports: &[ImportDecl]) -> Vec<CompatItem> {
    // Match each import path against known framework package stubs.
    let mut items = Vec::new();
    for imp in imports {
        let ImportDecl::ImportDecl { path, span, .. } = imp;
        for (import_path, package_name) in FRAMEWORK_IMPORT_PACKAGES {
            if path == *import_path {
                items.push(pass(
                    "adapter",
                    format!(
                        "Framework import '{path}' maps to {package_name} — stub adapter (orchestration hook only)",
                    ),
                    span.start.line,
                    span.start.column,
                ));
                break;
            }
        }
    }
    items
}
