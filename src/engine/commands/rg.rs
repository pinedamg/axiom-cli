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

        // Standard grep/rg output usually has a colon.
        // Formats:
        // 1. file:line:content (with -n)
        // 2. file:content (without -n)
        // 3. line:content (single file with -n)
        let parts: Vec<&str> = trimmed.splitn(3, ':').collect();
        
        if parts.len() == 3 {
            let file = parts[0];
            let line_num = parts[1];
            if line_num.chars().all(|c| c.is_ascii_digit()) && !file.contains(' ') {
                return Some(LineMetadata {
                    perms: "MATCH".to_string(),
                    size: file.to_string(),
                    name: line_num.to_string(),
                    is_dir: false,
                });
            }
        }

        if parts.len() >= 2 {
            let first = parts[0];
            // If it's a number, it's format #3 (local file)
            if first.chars().all(|c| c.is_ascii_digit()) {
                return Some(LineMetadata {
                    perms: "MATCH".to_string(),
                    size: "local".to_string(),
                    name: first.to_string(),
                    is_dir: false,
                });
            }
            
            // If it's a valid looking filename (no spaces, has extensions or paths), it's format #2
            if !first.contains(' ') && (first.contains('.') || first.contains('/')) {
                return Some(LineMetadata {
                    perms: "MATCH".to_string(),
                    size: first.to_string(),
                    name: "0".to_string(), // Unknown line
                    is_dir: false,
                });
            }
        }

        None
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
