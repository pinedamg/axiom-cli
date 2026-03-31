use std::env;
use serde_json::json;
use crate::config::{AxiomConfig, TelemetryLevel};

pub struct Telemetry;

impl Telemetry {
    /// Axiom Pulse Ingestion Endpoint (TimescaleDB / Redis)
    const ENDPOINT: &'static str = "https://pulse-api.mpineda.com.ar/v1/report/usage";

    /// Reports aggregate savings or errors to the Axiom Pulse backend.
    pub fn report_event(
        config: &AxiomConfig, 
        _event_type: &str, 
        command: Option<&str>,
        raw: usize, 
        processed: usize, 
        _error_msg: Option<String>
    ) {
        if config.telemetry_level == TelemetryLevel::Off {
            return;
        }

        // 1. Basic Anonymous Data
        let saved = raw.saturating_sub(processed);
        let ratio = if raw > 0 { (saved as f64 / raw as f64) } else { 0.0 };

        let binary = if let Some(cmd) = command {
            cmd.split_whitespace().next().unwrap_or("unknown")
        } else {
            "unknown"
        };

        // Axiom Pulse Usage Schema (v1)
        let payload = json!({
            "iid": config.node_id, // Compatibility with pulse-api
            "node_id": Some(&config.node_id),
            "command_bin": binary,
            "raw_bytes": raw as i64,
            "saved_bytes": saved as i64,
            "saving_ratio": ratio,
            "secrets_redacted": Some(0), // Placeholder for future PII counter
        });

        // Debug mode: show what we are sending
        if env::var("AXIOM_DEBUG_TELEMETRY").is_ok() {
            println!("\x1b[34m[AXIOM PULSE - {:?}] Reporting: {}\x1b[0m", config.telemetry_level, payload);
        }

        // Send to Pulse (Non-blocking timeout)
        let _ = ureq::post(Self::ENDPOINT)
            .timeout(std::time::Duration::from_millis(500))
            .set("X-Node-Token", &config.node_token)
            .send_json(payload);
    }

    /// Report savings with optional command discovery
    pub fn report_usage(config: &AxiomConfig, command: &str, raw: usize, processed: usize) {
        Self::report_event(config, "usage", Some(command), raw, processed, None);
    }

    /// Report an internal Axiom error
    pub fn report_error(config: &AxiomConfig, msg: &str) {
        // Errors follow a different path in Pulse (v1/report/crash)
        let endpoint = "https://pulse-api.mpineda.com.ar/v1/report/crash";
        let payload = json!({
            "iid": config.node_id,
            "error_msg": msg,
            "location": Some("Axiom CLI Runtime")
        });

        let _ = ureq::post(endpoint)
            .timeout(std::time::Duration::from_millis(500))
            .send_json(payload);
    }
}
