use crate::engine::discovery::LineMetadata;
use super::{CommandHandler, DiscoveryBuffer};

pub struct GoHandler;

impl CommandHandler for GoHandler {
    fn matches(&self, command: &str) -> bool {
        command.starts_with("go")
    }

    fn parse_line(&self, line: &str) -> Option<LineMetadata> {
        let trimmed = line.trim();
        if trimmed.is_empty() { return None; }

        // 1. Detect test status
        if line.starts_with("ok  ") || line.starts_with("FAIL ") || line.starts_with("?   ") {
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            let status = parts[0];
            let package = parts.get(1).unwrap_or(&"unknown");
            
            return Some(LineMetadata {
                perms: "TEST_RESULT".to_string(),
                size: status.to_string(),
                name: package.to_string(),
                is_dir: false,
            });
        }

        // 2. Detect build progress
        if line.starts_with("github.com/") || line.starts_with("google.golang.org/") {
            return Some(LineMetadata {
                perms: "COMPILING".to_string(),
                size: "go-pkg".to_string(),
                name: trimmed.to_string(),
                is_dir: false,
            });
        }

        None
    }

    fn generate_insight(&self, _command: &str, buffer: &DiscoveryBuffer) -> Option<String> {
        let mut ok = 0;
        let mut fail = 0;
        let mut compiling = 0;

        for (key, items) in buffer {
            if key.starts_with("GO:") {
                if key.contains("ok") { ok += items.len(); }
                else if key.contains("FAIL") { fail += items.len(); }
                else if key.contains("COMPILING") { compiling += items.len(); }
            }
        }

        if fail > 0 {
            Some(format!("Go Test failure: Detected {} failed test packages. Suggesting individual package investigation.", fail))
        } else if ok > 0 || compiling > 0 {
            Some(format!("Go environment: {} packages successfully tested. {} packages compiled in this run.", ok, compiling))
        } else {
            None
        }
    }

    fn format_summary(&self, key: &str, items: &[LineMetadata]) -> Option<String> {
        let parts: Vec<&str> = key.split(':').collect();
        if parts[0] != "GO" { return None; }

        let type_label = parts.get(1).unwrap_or(&"Unknown");
        let count = items.len();
        
        match *type_label {
            "TEST_RESULT" => {
                let status = parts.get(2).unwrap_or(&"ok");
                Some(format!("• Go Tests ({}): {} packages processed.", status, count))
            },
            "COMPILING" => Some(format!("• Go Build: {} packages compiled into memory.", count)),
            _ => None
        }
    }
}
