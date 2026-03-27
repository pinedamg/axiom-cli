use crate::engine::discovery::LineMetadata;
use super::{CommandHandler, DiscoveryBuffer};

pub struct CargoHandler;

impl CommandHandler for CargoHandler {
    fn matches(&self, command: &str) -> bool {
        command.starts_with("cargo")
    }

    fn parse_line(&self, line: &str) -> Option<LineMetadata> {
        let trimmed = line.trim();
        if trimmed.is_empty() { return None; }

        // Cargo lines usually start with 4-12 spaces followed by a keyword (Checking, Compiling, Downloading, etc.)
        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        if parts.len() < 2 { return None; }

        let keyword = parts[0];
        let package = parts[1];
        let version = parts.get(2).unwrap_or(&"v0.0.0");

        let status = match keyword {
            "Checking" | "Compiling" | "Downloading" | "Downloaded" | "Finished" | "Processing" => keyword,
            _ => return None,
        };

        Some(LineMetadata {
            perms: status.to_string(),
            size: version.to_string(),
            name: package.to_string(),
            is_dir: false,
        })
    }

    fn generate_insight(&self, _command: &str, buffer: &DiscoveryBuffer) -> Option<String> {
        let mut checking = 0;
        let mut compiling = 0;
        let mut total = 0;

        for (key, items) in buffer {
            if key.starts_with("CARGO:") {
                total += items.len();
                if key.contains("Checking") { checking += items.len(); }
                else if key.contains("Compiling") { compiling += items.len(); }
            }
        }

        if total > 0 {
            if compiling > 0 {
                Some(format!("Rust Build in progress: Compiling {} crates. Total cargo events: {}.", compiling, total))
            } else if checking > 0 {
                Some(format!("Rust Check in progress: Checking {} crates. Environment is being analyzed.", checking))
            } else {
                Some(format!("Cargo activity detected: {} total events processed.", total))
            }
        } else {
            None
        }
    }

    fn format_summary(&self, key: &str, items: &[LineMetadata]) -> Option<String> {
        let parts: Vec<&str> = key.split(':').collect();
        if parts[0] != "CARGO" { return None; }

        let status = parts.get(1).unwrap_or(&"Unknown");
        let count = items.len();
        
        if count > 5 {
            Some(format!("• Cargo {}: {} crates processed (including {})", status, count, items[0].name))
        } else {
            let names: Vec<String> = items.iter().map(|m| m.name.clone()).collect();
            Some(format!("• Cargo {}: {}", status, names.join(", ")))
        }
    }
}
