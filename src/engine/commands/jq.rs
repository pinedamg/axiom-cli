use crate::engine::discovery::LineMetadata;
use super::{CommandHandler, DiscoveryBuffer};

pub struct JqHandler;

impl CommandHandler for JqHandler {
    fn matches(&self, command: &str) -> bool {
        command.starts_with("jq") || command.starts_with("yq")
    }

    fn parse_line(&self, line: &str) -> Option<LineMetadata> {
        let trimmed = line.trim();
        if trimmed.is_empty() { return None; }

        // 1. Detect JSON object start in an array
        if trimmed.starts_with('{') || trimmed.starts_with('}') {
            return Some(LineMetadata {
                perms: "STRUCT".to_string(),
                size: "object".to_string(),
                name: "json".to_string(),
                is_dir: false,
            });
        }

        // 2. Detect JSON keys (common in large objects)
        if trimmed.starts_with('"') && trimmed.contains(':') {
            let key = trimmed.split(':').next().unwrap_or("key").trim_matches(|c| c == '"' || c == ' ' || c == ',');
            return Some(LineMetadata {
                perms: "KEY".to_string(),
                size: key.to_string(),
                name: "value".to_string(),
                is_dir: false,
            });
        }

        None
    }

    fn get_category(&self, _perms: &str) -> String {
        "DATA".to_string()
    }

    fn generate_insight(&self, _command: &str, buffer: &DiscoveryBuffer) -> Option<String> {
        let mut objects = 0;
        let mut key_count = 0;

        for (key, items) in buffer {
            if key.starts_with("DATA:") {
                if key.contains("STRUCT") { objects += items.len(); }
                else if key.contains("KEY") { key_count += items.len(); }
            }
        }

        if objects >= 4 {
            Some(format!("Data Stream: Synthesized {} JSON/YAML objects. The structure is repetitive and has been compressed.", objects / 2))
        } else if key_count > 0 {
            Some(format!("Data Stream: Identified {} unique data keys in the stream.", key_count))
        } else {
            None
        }
    }

    fn format_summary(&self, key: &str, items: &[LineMetadata]) -> Option<String> {
        let parts: Vec<&str> = key.split(':').collect();
        if parts[0] != "DATA" { return None; }

        let type_label = parts.get(1).unwrap_or(&"Unknown");
        let count = items.len();

        match *type_label {
            "STRUCT" => Some(format!("• Collapsed {} structural markers (JSON brackets).", count)),
            "KEY" => {
                let key_name = parts.get(2).unwrap_or(&"key");
                Some(format!("• Synthesized {} occurrences of key [{}]", count, key_name))
            },
            _ => None
        }
    }
}
