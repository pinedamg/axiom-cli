use std::env;
use crate::config::AxiomConfig;

pub struct Telemetry;

impl Telemetry {
    /// Reports aggregate savings to the Axiom backend (Anonymous & Opt-in by default).
    pub fn report_savings(config: &AxiomConfig, raw: usize, processed: usize) {
        if config.analytics_opt_out {
            return;
        }

        // Aggregate statistics (Strictly NO PII, NO Commands, NO IPs)
        let os = env::consts::OS;
        let _arch = env::consts::ARCH;
        let _version = env!("CARGO_PKG_VERSION");
        
        let saved = raw.saturating_sub(processed);
        let ratio = if raw > 0 { (saved as f64 / raw as f64) * 100.0 } else { 0.0 };

        // For MVP: We simulate the network call by printing to a log or stdout in debug
        // In production: This would be a non-blocking async HTTP POST request.
        if env::var("AXIOM_DEBUG_TELEMETRY").is_ok() {
            println!(
                "\x1b[34m[AXIOM TELEMETRY] Sending anonymous report: OS={}, Saved={} bytes ({:.1}%)\x1b[0m", 
                os, saved, ratio
            );
        }

        // Logic for real reporting:
        // let payload = json!({
        //     "os": os,
        //     "arch": arch,
        //     "version": version,
        //     "raw_bytes": raw,
        //     "processed_bytes": processed,
        //     "timestamp": chrono::Utc::now().to_rfc3339(),
        // });
        // let _ = reqwest::blocking::Client::new()
        //     .post("https://api.axiom.ai/v1/telemetry")
        //     .json(&payload)
        //     .send();
    }
}
