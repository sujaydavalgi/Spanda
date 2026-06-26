//! Readiness history storage, trend analysis, and forecasting.

use crate::types::{ReadinessFactorScore, ReadinessReport};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::io;
use std::path::{Path, PathBuf};

/// Single readiness evaluation snapshot in local history.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReadinessHistoryEntry {
    pub recorded_at: String,
    pub program: String,
    pub mission_ready: bool,
    pub total_score: u32,
    pub maximum_score: u32,
    pub factors: Vec<ReadinessFactorScore>,
}

/// Append-only readiness history file.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ReadinessHistory {
    pub version: u32,
    pub entries: Vec<ReadinessHistoryEntry>,
}

/// Trend slope for one readiness factor or overall score.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReadinessTrend {
    pub factor: String,
    pub slope_per_day: f64,
    pub volatility: f64,
    pub latest_score: u32,
}

/// Forecasted readiness at a future horizon.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReadinessForecast {
    pub horizon_days: u32,
    pub predicted_score: u32,
    pub risk_warning: bool,
    pub message: String,
}

/// Trend analysis report for one program.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReadinessTrendReport {
    pub program: String,
    pub sample_count: usize,
    pub overall_trend: Option<ReadinessTrend>,
    pub factor_trends: Vec<ReadinessTrend>,
    pub forecast: Option<ReadinessForecast>,
    pub warnings: Vec<String>,
}

/// Default on-disk path for readiness history.
pub fn default_readiness_history_path() -> PathBuf {
    // Resolve the standard local readiness history file path.
    //
    // Parameters:
    // None.
    //
    // Returns:
    // Path to `.spanda/readiness-history.json`.
    //
    // Options:
    // None.
    //
    // Example:
    // let path = default_readiness_history_path();

    PathBuf::from(".spanda/readiness-history.json")
}

/// Load readiness history from disk.
pub fn load_readiness_history(path: &Path) -> ReadinessHistory {
    // Deserialize readiness history or return an empty store when missing.
    //
    // Parameters:
    // - `path` — history file path
    //
    // Returns:
    // Parsed history, or empty history on missing/invalid files.
    //
    // Options:
    // None.
    //
    // Example:
    // let history = load_readiness_history(&default_readiness_history_path());

    std::fs::read_to_string(path)
        .ok()
        .and_then(|content| serde_json::from_str(&content).ok())
        .unwrap_or_default()
}

/// Persist readiness history to disk.
pub fn save_readiness_history(path: &Path, history: &ReadinessHistory) -> io::Result<()> {
    // Write readiness history JSON, creating parent directories when needed.
    //
    // Parameters:
    // - `path` — destination file
    // - `history` — history payload
    //
    // Returns:
    // `Ok(())` on success.
    //
    // Options:
    // None.
    //
    // Example:
    // save_readiness_history(&path, &history)?;

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let content = serde_json::to_string_pretty(history)?;
    std::fs::write(path, content)
}

/// Append a readiness report snapshot to local history.
pub fn record_readiness_snapshot(
    report: &ReadinessReport,
    program_label: &str,
    path: &Path,
) -> io::Result<ReadinessHistoryEntry> {
    // Append the latest readiness evaluation to the history file.
    //
    // Parameters:
    // - `report` — readiness evaluation result
    // - `program_label` — source file label
    // - `path` — history file path
    //
    // Returns:
    // The recorded history entry.
    //
    // Options:
    // None.
    //
    // Example:
    // let entry = record_readiness_snapshot(&report, "rover.sd", &path)?;

    let mut history = load_readiness_history(path);
    if history.version == 0 {
        history.version = 1;
    }
    let entry = ReadinessHistoryEntry {
        recorded_at: Utc::now().to_rfc3339(),
        program: program_label.into(),
        mission_ready: report.mission_ready,
        total_score: report.score.total,
        maximum_score: report.score.maximum,
        factors: report.score.factors.clone(),
    };
    history.entries.push(entry.clone());
    save_readiness_history(path, &history)?;
    Ok(entry)
}

/// Parse forecast horizons such as `7d` or plain day counts.
pub fn parse_forecast_horizon(raw: &str) -> Option<u32> {
    // Convert CLI forecast strings into day counts.
    //
    // Parameters:
    // - `raw` — horizon text (`7`, `7d`, `14d`)
    //
    // Returns:
    // Day count when parseable.
    //
    // Options:
    // None.
    //
    // Example:
    // let days = parse_forecast_horizon("7d");

    let trimmed = raw.trim().to_ascii_lowercase();
    let digits = trimmed.trim_end_matches('d');
    digits.parse::<u32>().ok().filter(|days| *days > 0)
}

/// Analyze readiness trends and optional forecast for one program label.
pub fn analyze_readiness_trends(
    history: &ReadinessHistory,
    program_label: &str,
    forecast_days: Option<u32>,
    minimum_score: u32,
) -> ReadinessTrendReport {
    // Compute slopes, volatility, and optional forecast from stored snapshots.
    //
    // Parameters:
    // - `history` — loaded readiness history
    // - `program_label` — program file label to analyze
    // - `forecast_days` — optional forecast horizon in days
    // - `minimum_score` — policy threshold for risk warnings
    //
    // Returns:
    // Trend report with factor breakdown and warnings.
    //
    // Options:
    // `forecast_days` enables score projection.
    //
    // Example:
    // let report = analyze_readiness_trends(&history, "rover.sd", Some(7), 80);

    let entries: Vec<&ReadinessHistoryEntry> = history
        .entries
        .iter()
        .filter(|entry| entry.program == program_label)
        .collect();

    let mut warnings = Vec::new();
    if entries.is_empty() {
        warnings.push(format!(
            "no readiness history recorded for {program_label}; run `spanda readiness {program_label} --record`"
        ));
        return ReadinessTrendReport {
            program: program_label.into(),
            sample_count: 0,
            overall_trend: None,
            factor_trends: Vec::new(),
            forecast: None,
            warnings,
        };
    }

    if entries.len() < 2 {
        warnings.push("need at least two recorded readiness evaluations to compute trends".into());
    }

    let overall_trend = trend_for_series(
        "overall",
        &entries
            .iter()
            .map(|entry| entry.total_score)
            .collect::<Vec<_>>(),
        &entries,
    );

    let factor_names: Vec<String> = entries
        .last()
        .map(|entry| {
            entry
                .factors
                .iter()
                .map(|factor| factor.factor.clone())
                .collect()
        })
        .unwrap_or_default();

    let factor_trends = factor_names
        .into_iter()
        .filter_map(|name| {
            let values = entries
                .iter()
                .map(|entry| {
                    entry
                        .factors
                        .iter()
                        .find(|factor| factor.factor == name)
                        .map(|factor| factor.score)
                        .unwrap_or(0)
                })
                .collect::<Vec<_>>();
            trend_for_series(&name, &values, &entries)
        })
        .collect::<Vec<_>>();

    let forecast = forecast_days.and_then(|days| {
        overall_trend.as_ref().map(|trend| {
            let predicted = (trend.latest_score as f64 + trend.slope_per_day * days as f64)
                .round()
                .clamp(0.0, 100.0) as u32;
            let risk_warning = predicted < minimum_score;
            let message = if risk_warning {
                format!("forecast {days}d score {predicted} below policy minimum {minimum_score}")
            } else {
                format!("forecast {days}d score {predicted} remains above minimum {minimum_score}")
            };
            if risk_warning {
                warnings.push(message.clone());
            }
            ReadinessForecast {
                horizon_days: days,
                predicted_score: predicted,
                risk_warning,
                message,
            }
        })
    });

    ReadinessTrendReport {
        program: program_label.into(),
        sample_count: entries.len(),
        overall_trend,
        factor_trends,
        forecast,
        warnings,
    }
}

/// Format readiness trend report for CLI output.
pub fn format_readiness_trends(report: &ReadinessTrendReport, json: bool) -> String {
    // Render trend analysis as JSON or human-readable text.
    //
    // Parameters:
    // - `report` — trend analysis report
    // - `json` — JSON output when true
    //
    // Returns:
    // Formatted report string.
    //
    // Options:
    // None.
    //
    // Example:
    // println!("{}", format_readiness_trends(&report, false));

    if json {
        return serde_json::to_string_pretty(report).unwrap_or_else(|e| e.to_string());
    }

    let mut lines = vec![
        format!("Readiness trends: {}", report.program),
        format!("Samples: {}", report.sample_count),
    ];

    if let Some(trend) = &report.overall_trend {
        lines.push(format!(
            "Overall: latest={} slope={:.2}/day volatility={:.2}",
            trend.latest_score, trend.slope_per_day, trend.volatility
        ));
    }

    for trend in &report.factor_trends {
        lines.push(format!(
            "  {}: latest={} slope={:.2}/day volatility={:.2}",
            trend.factor, trend.latest_score, trend.slope_per_day, trend.volatility
        ));
    }

    if let Some(forecast) = &report.forecast {
        lines.push(format!(
            "Forecast {}d: score={} risk={}",
            forecast.horizon_days, forecast.predicted_score, forecast.risk_warning
        ));
        lines.push(format!("  {}", forecast.message));
    }

    for warning in &report.warnings {
        lines.push(format!("Warning: {warning}"));
    }

    lines.join("\n")
}

fn trend_for_series(
    factor: &str,
    values: &[u32],
    entries: &[&ReadinessHistoryEntry],
) -> Option<ReadinessTrend> {
    if values.is_empty() {
        return None;
    }
    let latest_score = *values.last()?;
    if values.len() < 2 || entries.len() < 2 {
        return Some(ReadinessTrend {
            factor: factor.into(),
            slope_per_day: 0.0,
            volatility: 0.0,
            latest_score,
        });
    }

    let first_time = parse_timestamp(&entries.first()?.recorded_at)?;
    let last_time = parse_timestamp(&entries.last()?.recorded_at)?;
    let elapsed_days = ((last_time - first_time).num_seconds() as f64 / 86_400.0).max(0.01);
    let slope_per_day = (values.last()? - values.first()?) as f64 / elapsed_days;
    let mean = values.iter().map(|v| *v as f64).sum::<f64>() / values.len() as f64;
    let variance = values
        .iter()
        .map(|value| {
            let delta = *value as f64 - mean;
            delta * delta
        })
        .sum::<f64>()
        / values.len() as f64;
    Some(ReadinessTrend {
        factor: factor.into(),
        slope_per_day,
        volatility: variance.sqrt(),
        latest_score,
    })
}

fn parse_timestamp(raw: &str) -> Option<DateTime<Utc>> {
    DateTime::parse_from_rfc3339(raw)
        .ok()
        .map(|value| value.with_timezone(&Utc))
}
