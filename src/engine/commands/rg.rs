use crate::engine::discovery::LineMetadata;
use super::{CommandHandler, DiscoveryBuffer};

pub struct RgHandler;

impl CommandHandler for RgHandler {
    fn matches(&self, command: &str) -> bool {
        command.starts_with("rg") || command.starts_with("grep")
    }

    fn parse_line(&self, line: &str) -> Option<LineMetadata> {
        let trimmed = line.trim();
        if trimmed.is_empty() { return None; }

        // Standard grep/rg output: file:line:content
        let parts: Vec<&str> = trimmed.splitn(3, ':').collect();
        if parts.len() < 3 { 
            // Handle single-file grep output: line:content
            if parts.len() == 2 && parts[0].chars().all(|c| c.is_ascii_digit()) {
                return Some(LineMetadata {
                    perms: "MATCH".to_string(),
                    size: "current".to_string(),
                    name: "local".to_string(),
                    is_dir: false,
                });
            }
            return None; 
        }

        let file = parts[0];
        let line_num = parts[1];
        
        // Validate it's likely a grep output
        if !line_num.chars().all(|c| c.is_ascii_digit()) || file.contains(' ') {
            return None;
        }

        Some(LineMetadata {
            perms: "MATCH".to_string(),
            size: file.to_string(),
            name: line_num.to_string(),
            is_dir: false,
        })
    }

    fn generate_insight(&self, _command: &str, buffer: &DiscoveryBuffer) -> Option<String> {
        let mut total_matches = 0;
        let mut files = std::collections::HashSet::new();

        for (key, items) in buffer {
            if key.starts_with("SEARCH:MATCH") {
                total_matches += items.len();
                for item in items {
                    files.insert(item.size.clone());
                }
            }
        }

        if total_matches > 0 {
            Some(format!("Search found {} matches across {} unique files/locations.", total_matches, files.len()))
        } else {
            None
        }
    }

    fn format_summary(&self, key: &str, items: &[LineMetadata]) -> Option<String> {
        let parts: Vec<&str> = key.split(':').collect();
        if parts[0] != "SEARCH" { return None; }

        let location = parts.get(2).unwrap_or(&"unknown");
        let count = items.len();
        
        if *location == "current" || *location == "local" {
            let lines: Vec<String> = items.iter().take(3).map(|m| m.name.clone()).collect();
            let suffix = if count > 3 { format!(" and {} more...", count - 3) } else { "".to_string() };
            Some(format!("• {} matches in current context (lines: {}{})", count, lines.join(", "), suffix))
        } else {
            // Group by file
            let lines: Vec<String> = items.iter().take(3).map(|m| m.name.clone()).collect();
            let suffix = if count > 3 { format!(" and {} more...", count - 3) } else { "".to_string() };
            Some(format!("• {}: {} matches (lines: {}{})", location, count, lines.join(", "), suffix))
        }
    }
}
