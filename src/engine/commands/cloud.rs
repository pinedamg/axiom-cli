use crate::engine::discovery::LineMetadata;
use super::{CommandHandler, DiscoveryBuffer};

pub struct CloudHandler;

impl CommandHandler for CloudHandler {
    fn matches(&self, command: &str) -> bool {
        command.starts_with("gcloud") || command.starts_with("aws") || command.starts_with("az")
    }

    fn parse_line(&self, line: &str) -> Option<LineMetadata> {
        let trimmed = line.trim();
        if trimmed.is_empty() { return None; }

        // 1. Detect Resource Listing Rows (NAME STATUS ...)
        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        if parts.len() >= 2 {
            let status = parts.iter().find(|&&s| 
                matches!(s, "RUNNING" | "READY" | "PROVISIONING" | "STOPPED" | "DELETING" | "ACTIVE" | "INACTIVE" | "PENDING")
            );

            if let Some(&s) = status {
                return Some(LineMetadata {
                    perms: "RESOURCE".to_string(),
                    size: s.to_string(),
                    name: parts[0].to_string(),
                    is_dir: false,
                });
            }
        }

        // 2. Generic table row fallback
        if parts.len() >= 3 && !trimmed.starts_with('-') {
             return Some(LineMetadata {
                perms: "ROW".to_string(),
                size: "table".to_string(),
                name: parts[0].to_string(),
                is_dir: false,
            });
        }

        None
    }

    fn generate_insight(&self, _command: &str, buffer: &DiscoveryBuffer) -> Option<String> {
        let mut running = 0;
        let mut stopped = 0;
        let mut total = 0;

        for (key, items) in buffer {
            if key.starts_with("CLOUD:RESOURCE") {
                total += items.len();
                if key.contains("RUNNING") || key.contains("READY") || key.contains("ACTIVE") { running += items.len(); }
                else { stopped += items.len(); }
            }
        }

        if total > 0 {
            Some(format!("Cloud Resources: {} active/running, {} other states. Found {} total resources.", running, stopped, total))
        } else {
            None
        }
    }

    fn format_summary(&self, key: &str, items: &[LineMetadata]) -> Option<String> {
        let parts: Vec<&str> = key.split(':').collect();
        if parts[0] != "CLOUD" { return None; }

        let type_label = parts.get(1).unwrap_or(&"Unknown");
        let count = items.len();

        match *type_label {
            "RESOURCE" => {
                let status = parts.get(2).unwrap_or(&"Status");
                Some(format!("• Cloud {} [{}]: {} items summarized.", status, count, count))
            },
            "ROW" => Some(format!("• Collapsed {} generic table rows.", count)),
            _ => None
        }
    }
}
