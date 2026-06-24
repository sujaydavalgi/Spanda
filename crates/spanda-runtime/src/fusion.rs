//! Weighted multi-sensor fusion helpers for runtime reads and assurance previews.
//!

use spanda_ast::nodes::{Program, RobotDecl, SensorDecl};

/// Parsed fusion input path (`gps.fix` → sensor `gps`, field `fix`).
pub fn parse_fusion_input(input: &str) -> (&str, Option<&str>) {
    match input.split_once('.') {
        Some((sensor, field)) => (sensor, Some(field)),
        None => (input, None),
    }
}

/// Relative weight for a sensor type in weighted fusion.
pub fn weight_for_sensor_type(sensor_type: &str) -> f64 {
    match sensor_type {
        "GPS" | "GNSS" => 0.35,
        "Lidar" => 0.25,
        "IMU" => 0.20,
        "Camera" => 0.15,
        "DepthCamera" | "Radar" => 0.20,
        "Encoder" | "WheelOdometry" => 0.15,
        _ => 0.10,
    }
}

/// Compute normalized confidence from participating sensor types.
pub fn weighted_confidence(sensor_types: &[&str]) -> f64 {
    if sensor_types.is_empty() {
        return 0.0;
    }
    let total: f64 = sensor_types
        .iter()
        .map(|t| weight_for_sensor_type(t))
        .sum();
    let max_possible = sensor_types.len() as f64 * 0.35;
    (total / max_possible.max(0.35)).min(1.0)
}

/// Static fusion preview for assurance reports (no live sensor reads).
#[derive(Debug, Clone, PartialEq)]
pub struct FusionPreview {
    pub confidence: f64,
    pub sources: Vec<String>,
    pub summary: String,
}

/// Build a sensor-name → type map from all robots in a program.
pub fn sensor_type_index(program: &Program) -> std::collections::HashMap<String, String> {
    let Program::Program { robots, .. } = program;
    let mut index = std::collections::HashMap::new();
    for robot in robots {
        let RobotDecl::RobotDecl { sensors, .. } = robot;
        for sensor in sensors {
            let SensorDecl::SensorDecl {
                name,
                sensor_type,
                ..
            } = sensor;
            index.insert(name.clone(), sensor_type.clone());
        }
    }
    index
}

/// Preview weighted fusion for declared estimator inputs.
pub fn preview_fusion_inputs(
    program: &Program,
    inputs: &[String],
    output_type: &str,
) -> FusionPreview {
    let index = sensor_type_index(program);
    let mut types = Vec::new();
    for input in inputs {
        let (sensor, _) = parse_fusion_input(input);
        let sensor_type = index
            .get(sensor)
            .map(String::as_str)
            .unwrap_or("Unknown");
        types.push(sensor_type);
    }
    let confidence = weighted_confidence(&types);
    let summary = format!("weighted {output_type} ({})", inputs.join(" + "));
    FusionPreview {
        confidence,
        sources: inputs.to_vec(),
        summary,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn weighted_confidence_favors_diverse_sensors() {
        let gps_lidar = weighted_confidence(&["GPS", "Lidar"]);
        let unknown_only = weighted_confidence(&["Unknown"]);
        assert!(gps_lidar > unknown_only);
        assert!(gps_lidar > 0.5);
    }

    #[test]
    fn parse_fusion_input_splits_field() {
        let (sensor, field) = parse_fusion_input("gps.fix");
        assert_eq!(sensor, "gps");
        assert_eq!(field, Some("fix"));
    }
}
