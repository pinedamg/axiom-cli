use crate::engine::discovery::LineMetadata;
use super::{CommandHandler, DiscoveryBuffer};

pub struct IoHandler;

impl CommandHandler for IoHandler {
    fn matches(&self, command: &str) -> bool {
        command.starts_with("curl") || command.starts_with("wget") || command.starts_with("cat")
    }

    fn parse_line(&self, line: &str) -> Option<LineMetadata> {
        let trimmed = line.trim();
        if trimmed.is_empty() { return None; }

        // 1. Detect curl progress bar headers
        if line.contains("% Total") && line.contains("% Received") {
            return Some(LineMetadata {
                perms: "PROGRESS".to_string(),
                size: "header".to_string(),
                name: "curl".to_string(),
                is_dir: false,
            });
        }

        // 2. Detect curl progress data lines (usually space-separated numbers)
        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        if parts.len() >= 10 && parts.iter().take(10).all(|s| s.chars().all(|c| c.is_ascii_digit() || c == ':' || c == '-' || c == '.' || c == '*')) {
             return Some(LineMetadata {
                perms: "PROGRESS".to_string(),
                size: parts.get(1).unwrap_or(&"0").to_string(), // Received
                name: "curl".to_string(),
                is_dir: false,
            });
        }

        // 3. Detect wget progress bar
        if line.contains("%[") && line.contains("]") {
            return Some(LineMetadata {
                perms: "PROGRESS".to_string(),
                size: "data".to_string(),
                name: "wget".to_string(),
                is_dir: false,
            });
        }

        // 4. Detect informational network lines or TLS Handshake noise
        if line.trim_start().starts_with('*') || line.contains("TLS handshake") || line.contains("TLS header") || line.contains("bytes data") {
            return Some(LineMetadata {
                perms: "NETWORK_NOISE".to_string(),
                size: "tls".to_string(),
                name: "handshake".to_string(),
                is_dir: false,
            });
        }

        None
    }

    fn generate_insight(&self, _command: &str, buffer: &DiscoveryBuffer) -> Option<String> {
        let mut progress_count = 0;
        let mut noise_count = 0;

        for (key, items) in buffer {
            if key.contains("PROGRESS") { progress_count += items.len(); }
            if key.contains("NETWORK_NOISE") { noise_count += items.len(); }
        }

        if progress_count > 0 || noise_count > 0 {
            Some(format!("Network I/O: Collapsed {} progress updates and {} TLS handshake logs.", progress_count, noise_count))
        } else {
            None
        }
    }

    fn format_summary(&self, key: &str, items: &[LineMetadata]) -> Option<String> {
        let parts: Vec<&str> = key.split(':').collect();
        if parts[0] != "IO" { return None; }

        let type_label = parts.get(1).unwrap_or(&"Unknown");
        
        match *type_label {
            "PROGRESS" => Some(format!("• Hidden {} download progress updates.", items.len())),
            "NETWORK_NOISE" => Some(format!("• Collapsed {} lines of network protocol handshakes.", items.len())),
            _ => None
        }
    }
}
