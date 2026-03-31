use std::env;
use serde_json::json;
use crate::config::{AxiomConfig, TelemetryLevel};

use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

pub struct Telemetry;

impl Telemetry {
    /// Axiom Pulse Ingestion Endpoint (TimescaleDB / Redis)
    const ENDPOINT: &'static str = "https://axiom-pulse-api.mpineda.com.ar/v1/report/usage";

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

        // 1. Basic Data
        let saved = raw.saturating_sub(processed);
        let ratio = if raw > 0 { saved as f64 / raw as f64 } else { 0.0 };

        let binary = if let Some(cmd) = command {
            cmd.split_whitespace().next().unwrap_or("unknown")
        } else {
            "unknown"
        };

        // Axiom Pulse Usage Schema (v1)
        let payload = json!({
            "iid": config.node_id, 
            "command_bin": binary,
            "raw_bytes": raw as i64,
            "saved_bytes": saved as i64,
            "saving_ratio": ratio,
            "secrets_redacted": Some(0),
        });

        let body = payload.to_string();

        // 2. Secure Signing (HMAC-SHA256)
        // We use the node_token as the secret key
        let mut mac = HmacSha256::new_from_slice(config.node_token.as_bytes())
            .expect("HMAC can take key of any size");
        mac.update(body.as_bytes());
        let signature = hex::encode(mac.finalize().into_bytes());

        // Debug mode
        if env::var("AXIOM_DEBUG_TELEMETRY").is_ok() {
            println!("\x1b[34m[AXIOM PULSE] Reporting: {}\x1b[0m", body);
            println!("\x1b[34m[AXIOM PULSE] Signature: {}\x1b[0m", signature);
        }

        // Send to Pulse (Wait for response to ensure bytes hit the wire)
        let response = ureq::post(Self::ENDPOINT)
            .timeout(std::time::Duration::from_millis(1000))
            .set("X-Axiom-Node-Id", &config.node_id)
            .set("X-Axiom-Signature", &signature)
            .send_string(&body);

        if env::var("AXIOM_DEBUG_TELEMETRY").is_ok() {
            match response {
                Ok(res) => println!("\x1b[34m[AXIOM PULSE] Success: {}\x1b[0m", res.status()),
                Err(e) => eprintln!("\x1b[31m[AXIOM PULSE] Error: {:?}\x1b[0m", e),
            }
        }
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
