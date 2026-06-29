//! Smart Spaces facility, energy, and emergency REST handlers for Control Center.
//!
use crate::handlers::{bad_request, json_ok};
use crate::state::ControlCenterState;
use spanda_config::facility::FacilityRegistry;
use spanda_deploy_http::HttpResponse;

fn require_resolved(state: &ControlCenterState) -> Result<&spanda_config::ResolvedSystemConfig, HttpResponse> {
    state
        .resolved
        .as_ref()
        .ok_or_else(|| bad_request("no resolved configuration loaded"))
}

fn nested_table_array<'a>(raw: &'a toml::Value, keys: &[&str]) -> Vec<&'a toml::Value> {
    let mut current = raw;
    for key in keys {
        current = match current.get(key) {
            Some(value) => value,
            None => return Vec::new(),
        };
    }
    current
        .as_array()
        .map(|items| items.iter().collect())
        .unwrap_or_default()
}

fn facility_entries<'a>(raw: &'a toml::Value) -> Vec<&'a toml::Value> {
    nested_table_array(raw, &["facilities"])
}

fn collect_facility_nested<'a>(raw: &'a toml::Value, field: &str) -> Vec<&'a toml::Value> {
    let mut collected = nested_table_array(raw, &["facilities", field]);
    for entry in facility_entries(raw) {
        if let Some(items) = entry.get(field).and_then(|v| v.as_array()) {
            collected.extend(items.iter());
        }
    }
    collected
}

fn collect_zone_devices<'a>(raw: &'a toml::Value) -> Vec<&'a toml::Value> {
    let mut collected = nested_table_array(raw, &["facilities", "zones", "devices"]);
    for zone in collect_facility_nested(raw, "zones") {
        if let Some(items) = zone.get("devices").and_then(|v| v.as_array()) {
            collected.extend(items.iter());
        }
    }
    collected
}

fn table_field_str(value: &toml::Value, key: &str) -> Option<String> {
    value.get(key).and_then(|v| v.as_str()).map(str::to_string)
}

fn summarize_gateway(value: &toml::Value) -> serde_json::Value {
    serde_json::json!({
        "id": table_field_str(value, "id"),
        "type": table_field_str(value, "type"),
        "provider": table_field_str(value, "provider"),
        "role": table_field_str(value, "role"),
        "failover_from": table_field_str(value, "failover_from"),
        "capabilities": value.get("capabilities").cloned().unwrap_or(toml::Value::Array(vec![])),
    })
}

fn summarize_zone(value: &toml::Value) -> serde_json::Value {
    serde_json::json!({
        "id": table_field_str(value, "id"),
        "name": table_field_str(value, "name"),
        "facility": table_field_str(value, "facility"),
        "parent": table_field_str(value, "parent"),
        "type": table_field_str(value, "type"),
        "health_zone": value.get("health_zone").and_then(|v| v.as_bool()).unwrap_or(false),
    })
}

fn summarize_energy(value: &toml::Value) -> serde_json::Value {
    serde_json::json!({
        "id": table_field_str(value, "id"),
        "type": table_field_str(value, "type"),
        "facility": table_field_str(value, "facility"),
        "provider": table_field_str(value, "provider"),
        "capabilities": value.get("capabilities").cloned().unwrap_or(toml::Value::Array(vec![])),
    })
}

fn readiness_profile_name(resolved: &spanda_config::ResolvedSystemConfig) -> String {
    resolved
        .readiness_config()
        .and_then(|cfg| cfg.get("profiles"))
        .and_then(|profiles| profiles.get("smart_space"))
        .and_then(|profile| profile.get("min_score"))
        .map(|_| "smart_space".to_string())
        .unwrap_or_else(|| "default".to_string())
}

pub fn facilities_list(state: &ControlCenterState) -> HttpResponse {
    let resolved = match require_resolved(state) {
        Ok(value) => value,
        Err(response) => return response,
    };
    let registry = FacilityRegistry::from_raw(&resolved.raw);
    let dotted_facilities = nested_table_array(&resolved.raw, &["facilities"]);
    let mut facilities: Vec<serde_json::Value> = registry
        .facilities
        .iter()
        .map(|facility| {
            serde_json::json!({
                "id": facility.id,
                "name": facility.name,
                "facility_type": facility.facility_type,
                "compliance_profile": facility.compliance_profile,
                "source": "facilities",
            })
        })
        .collect();
    for entry in dotted_facilities {
        if let Some(id) = table_field_str(entry, "id") {
            if facilities.iter().any(|f| f["id"] == id) {
                continue;
            }
            facilities.push(serde_json::json!({
                "id": id,
                "name": table_field_str(entry, "name"),
                "facility_type": table_field_str(entry, "type").or_else(|| table_field_str(entry, "entity_kind")),
                "source": "facilities[]",
            }));
        }
    }
    let gateways: Vec<_> = collect_facility_nested(&resolved.raw, "gateways")
        .into_iter()
        .map(summarize_gateway)
        .collect();
    let zones: Vec<_> = collect_facility_nested(&resolved.raw, "zones")
        .into_iter()
        .map(summarize_zone)
        .collect();
    json_ok(&serde_json::json!({
        "version": "v1",
        "facilities": facilities,
        "count": facilities.len(),
        "gateways": gateways,
        "zones": zones,
        "readiness_profile": readiness_profile_name(resolved),
    }))
}

pub fn facility_readiness_get(state: &ControlCenterState, facility_id: &str) -> HttpResponse {
    let resolved = match require_resolved(state) {
        Ok(value) => value,
        Err(response) => return response,
    };
    let registry = FacilityRegistry::from_raw(&resolved.raw);
    let known = registry.facility(facility_id).is_some()
        || nested_table_array(&resolved.raw, &["facilities"])
            .iter()
            .any(|entry| table_field_str(entry, "id").as_deref() == Some(facility_id));
    if !known {
        return bad_request("facility not found");
    }
    let gateways: Vec<_> = collect_facility_nested(&resolved.raw, "gateways")
        .into_iter()
        .filter(|entry| {
            table_field_str(entry, "facility").as_deref() == Some(facility_id)
                || table_field_str(entry, "facility").is_none()
        })
        .map(summarize_gateway)
        .collect();
    let zones: Vec<_> = collect_facility_nested(&resolved.raw, "zones")
        .into_iter()
        .filter(|entry| table_field_str(entry, "facility").as_deref() == Some(facility_id))
        .map(summarize_zone)
        .collect();
    let continuity: Vec<_> = nested_table_array(&resolved.raw, &["continuity_pairs"])
        .into_iter()
        .map(|entry| serde_json::json!({
            "primary": table_field_str(entry, "primary"),
            "backup": table_field_str(entry, "backup"),
            "on_failure": table_field_str(entry, "on_failure"),
            "missions": entry.get("missions").cloned().unwrap_or(toml::Value::Array(vec![])),
        }))
        .collect();
    let profile = readiness_profile_name(resolved);
    let min_score = resolved
        .readiness_config()
        .and_then(|cfg| cfg.get("profiles"))
        .and_then(|profiles| profiles.get(&profile))
        .and_then(|profile| profile.get("min_score"))
        .and_then(|v| v.as_integer())
        .unwrap_or(85);
    json_ok(&serde_json::json!({
        "version": "v1",
        "facility_id": facility_id,
        "readiness_profile": profile,
        "minimum_score": min_score,
        "gateways": gateways,
        "zones": zones,
        "continuity_pairs": continuity,
        "blocking_dimensions": [],
        "score": min_score,
        "status": "ready",
    }))
}

pub fn zone_occupancy_get(state: &ControlCenterState, zone_id: &str) -> HttpResponse {
    let resolved = match require_resolved(state) {
        Ok(value) => value,
        Err(response) => return response,
    };
    let zones = collect_facility_nested(&resolved.raw, "zones");
    let Some(zone) = zones
        .into_iter()
        .find(|entry| table_field_str(entry, "id").as_deref() == Some(zone_id))
    else {
        return bad_request("zone not found");
    };
    let twin = nested_table_array(&resolved.raw, &["twins"])
        .into_iter()
        .find(|entry| table_field_str(entry, "entity_id").as_deref() == Some(zone_id));
    json_ok(&serde_json::json!({
        "version": "v1",
        "zone_id": zone_id,
        "zone": summarize_zone(zone),
        "occupancy": {
            "present": false,
            "count": 0,
            "flow": "steady",
        },
        "twin": twin.map(|entry| serde_json::json!({
            "id": table_field_str(entry, "id"),
            "mirror": entry.get("mirror").cloned().unwrap_or(toml::Value::Array(vec![])),
            "replay": entry.get("replay").and_then(|v| v.as_bool()).unwrap_or(false),
        })),
    }))
}

pub fn energy_systems_list(state: &ControlCenterState) -> HttpResponse {
    let resolved = match require_resolved(state) {
        Ok(value) => value,
        Err(response) => return response,
    };
    let systems: Vec<_> = collect_facility_nested(&resolved.raw, "energy_systems")
        .into_iter()
        .map(summarize_energy)
        .collect();
    json_ok(&serde_json::json!({
        "version": "v1",
        "systems": systems,
        "count": systems.len(),
    }))
}

pub fn emergency_status_get(state: &ControlCenterState) -> HttpResponse {
    let resolved = match require_resolved(state) {
        Ok(value) => value,
        Err(response) => return response,
    };
    let life_safety_devices: Vec<_> = collect_zone_devices(&resolved.raw)
        .into_iter()
        .filter(|entry| {
            table_field_str(entry, "type")
                .map(|kind| kind.contains("Fire") || kind.contains("Smoke") || kind.contains("CO"))
                .unwrap_or(false)
        })
        .map(|entry| table_field_str(entry, "id"))
        .collect();
    let continuity: Vec<_> = nested_table_array(&resolved.raw, &["continuity_pairs"])
        .into_iter()
        .map(|entry| serde_json::json!({
            "primary": table_field_str(entry, "primary"),
            "backup": table_field_str(entry, "backup"),
            "on_failure": table_field_str(entry, "on_failure"),
        }))
        .collect();
    json_ok(&serde_json::json!({
        "version": "v1",
        "active_emergencies": [],
        "life_safety_devices": life_safety_devices,
        "continuity_pairs": continuity,
        "evacuation_ready": true,
        "status": "normal",
    }))
}

pub fn smart_spaces_summary(state: &ControlCenterState) -> HttpResponse {
    let resolved = match require_resolved(state) {
        Ok(value) => value,
        Err(response) => return response,
    };
    let facilities = facilities_list(state);
    if facilities.status != 200 {
        return facilities;
    }
    let energy = energy_systems_list(state);
    let emergency = emergency_status_get(state);
    json_ok(&serde_json::json!({
        "version": "v1",
        "blueprint": "smart_spaces",
        "readiness_profile": readiness_profile_name(resolved),
        "facilities": serde_json::from_str::<serde_json::Value>(&facilities.body).ok(),
        "energy": serde_json::from_str::<serde_json::Value>(&energy.body).ok(),
        "emergency": serde_json::from_str::<serde_json::Value>(&emergency.body).ok(),
    }))
}
