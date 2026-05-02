use crate::engine::discovery::LineMetadata;
use super::{CommandHandler, DiscoveryBuffer};

pub struct TerraformHandler;

impl CommandHandler for TerraformHandler {
    fn matches(&self, command: &str) -> bool {
        command.starts_with("terraform") || command.starts_with("tf")
    }

    fn parse_line(&self, line: &str) -> Option<LineMetadata> {
        let trimmed = line.trim();
        if trimmed.is_empty() { return None; }

        // 1. Detect Resource Plan (will be created, etc.)
        if line.contains("will be created") || line.contains("will be updated in-place") || line.contains("will be destroyed") {
            let action = if line.contains("created") { "CREATE" } 
                        else if line.contains("destroyed") { "DESTROY" } 
                        else { "UPDATE" };
            
            return Some(LineMetadata {
                perms: "PLAN".to_string(),
                size: action.to_string(),
                name: trimmed.to_string(),
                is_dir: false,
            });
        }

        // 2. Detect Attribute changes (the +/- lines)
        if (trimmed.starts_with('+') || trimmed.starts_with('~') || trimmed.starts_with('-')) && trimmed.len() > 2 {
             return Some(LineMetadata {
                perms: "ATTRIBUTE".to_string(),
                size: "change".to_string(),
                name: "attr".to_string(),
                is_dir: false,
            });
        }

        // 3. Detect State Refreshing/Reading
        if line.contains("Refreshing state...") || line.contains("Reading...") || line.contains("Read complete") {
            return Some(LineMetadata {
                perms: "STATE".to_string(),
                size: "io".to_string(),
                name: "resource".to_string(),
                is_dir: false,
            });
        }

        None
    }

    fn get_category(&self, _meta: &LineMetadata) -> String {
        "TF".to_string()
    }

    fn generate_insight(&self, command: &str, buffer: &DiscoveryBuffer) -> Option<String> {
        let mut creates = 0;
        let mut destroys = 0;
        let mut updates = 0;

        for (key, items) in buffer {
            if key.starts_with("TF:PLAN") {
                if key.contains("CREATE") { creates += items.len(); }
                else if key.contains("DESTROY") { destroys += items.len(); }
                else if key.contains("UPDATE") { updates += items.len(); }
            }
        }

        if (creates > 0 || destroys > 0 || updates > 0) && (command.contains("plan") || command.contains("apply")) {
            Some(format!("Terraform Plan Summary: {} to add, {} to change, {} to destroy. Verify critical resources before apply.", creates, updates, destroys))
        } else {
            None
        }
    }

    fn format_summary(&self, key: &str, items: &[LineMetadata]) -> Option<String> {
        let parts: Vec<&str> = key.split(':').collect();
        if parts[0] != "TF" { return None; }

        let type_label = parts.get(1).unwrap_or(&"Unknown");
        let count = items.len();

        match *type_label {
            "PLAN" => {
                let action = parts.get(2).unwrap_or(&"Change");
                Some(format!("• Terraform {}: {} resources targeted.", action, count))
            },
            "ATTRIBUTE" => Some(format!("• Hidden {} planned attribute changes.", count)),
            "STATE" => Some(format!("• Collapsed {} state/reading operations.", count)),
            _ => None
        }
    }
}
