use std::env;
use serde_json::json;
use crate::config::{AxiomConfig, TelemetryLevel};

pub struct Telemetry;

impl Telemetry {
    /// Internal endpoint for telemetry ingestion (PocketBase or Custom API)
    const ENDPOINT: &'static str = "https://subdomain.mpineda.com.ar/api/collections/telemetry/records";

    /// Reports aggregate savings or errors to the Axiom backend.
    pub fn report_event(
        config: &AxiomConfig, 
        event_type: &str, 
        command: Option<&str>,
        raw: usize, 
        processed: usize, 
        error_msg: Option<String>
    ) {
        if config.telemetry_level == TelemetryLevel::Off {
            return;
        }

        // 1. Basic Anonymous Data (Level: Anonymous+)
        let os = env::consts::OS;
        let arch = env::consts::ARCH;
        let version = env!("CARGO_PKG_VERSION");
        
        let saved = raw.saturating_sub(processed);
        let ratio = if raw > 0 { (saved as f64 / raw as f64) * 100.0 } else { 0.0 };

        let mut payload = json!({
            "iid": config.installation_id,
            "event_type": event_type,
            "os": os,
            "arch": arch,
            "version": version,
            "raw_bytes": raw,
            "saved_bytes": saved,
            "saving_ratio": ratio,
            "error_msg": error_msg.unwrap_or_default(),
        });

        // 2. Command Discovery (Level: Discovery+)
        if config.telemetry_level as u8 >= TelemetryLevel::Discovery as u8 {
            if let Some(cmd) = command {
                let binary = cmd.split_whitespace().next().unwrap_or("unknown");
                payload["command_bin"] = json!(binary);
            }
        }

        // 3. Detailed Metrics (Level: Full)
        if config.telemetry_level == TelemetryLevel::Full {
            payload["full_metrics"] = json!(true);
        }

        // Debug mode: show what we are sending
        if env::var("AXIOM_DEBUG_TELEMETRY").is_ok() {
            println!("\x1b[34m[AXIOM TELEMETRY - {:?}] Sending event: {}\x1b[0m", config.telemetry_level, payload);
        }

        // Send non-blocking
        let _ = ureq::post(Self::ENDPOINT)
            .timeout(std::time::Duration::from_millis(200))
            .send_json(payload);
    }

    /// Report savings with optional command discovery
    pub fn report_usage(config: &AxiomConfig, command: &str, raw: usize, processed: usize) {
        Self::report_event(config, "usage", Some(command), raw, processed, None);
    }

    /// Report an internal Axiom error
    pub fn report_error(config: &AxiomConfig, msg: &str) {
        Self::report_event(config, "error", None, 0, 0, Some(msg.to_string()));
    }
}
