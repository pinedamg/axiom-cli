use crate::engine::discovery::LineMetadata;
use super::{CommandHandler, DiscoveryBuffer};

pub struct JournalHandler;

impl CommandHandler for JournalHandler {
    fn matches(&self, command: &str) -> bool {
        command.starts_with("journalctl")
    }

    fn parse_line(&self, line: &str) -> Option<LineMetadata> {
        let trimmed = line.trim();
        if trimmed.is_empty() { return None; }

        // Standard journalctl line: Mar 27 18:50 machine process[pid]: message
        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        if parts.len() < 5 { return None; }

        let process_with_pid = parts[4];
        let process = process_with_pid.split('[').next().unwrap_or(process_with_pid).trim_matches(':');

        // Noise patterns
        if line.contains("systemd") || line.contains("session opened") || line.contains("kernel:") {
            return Some(LineMetadata {
                perms: "NOISE".to_string(),
                size: process.to_string(),
                name: "system".to_string(),
                is_dir: false,
            });
        }

        // Generic log line
        Some(LineMetadata {
            perms: "LOG".to_string(),
            size: process.to_string(),
            name: "event".to_string(),
            is_dir: false,
        })
    }

    fn get_category(&self, _meta: &LineMetadata) -> String {
        "SYS".to_string()
    }

    fn generate_insight(&self, _command: &str, buffer: &DiscoveryBuffer) -> Option<String> {
        let mut noise = 0;
        let mut logs = 0;

        for (key, items) in buffer {
            if key.starts_with("SYS:") {
                if key.contains("NOISE") { noise += items.len(); }
                else if key.contains("LOG") { logs += items.len(); }
            }
        }

        if noise > 0 || logs > 0 {
            Some(format!("System Logs: Collapsed {} noise lines and {} generic logs. Focus on application-specific errors.", noise, logs))
        } else {
            None
        }
    }

    fn format_summary(&self, key: &str, items: &[LineMetadata]) -> Option<String> {
        let parts: Vec<&str> = key.split(':').collect();
        if parts[0] != "SYS" { return None; }

        let type_label = parts.get(1).unwrap_or(&"Unknown");
        let process = parts.get(2).unwrap_or(&"unknown");
        let count = items.len();

        match *type_label {
            "NOISE" => Some(format!("• Hidden {} noise lines from system service [{}].", count, process)),
            "LOG" => Some(format!("• Synthesized {} log entries for process [{}].", count, process)),
            _ => None
        }
    }
}
