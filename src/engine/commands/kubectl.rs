use crate::engine::discovery::LineMetadata;
use super::{CommandHandler, DiscoveryBuffer};

pub struct KubectlHandler;

impl CommandHandler for KubectlHandler {
    fn matches(&self, command: &str) -> bool {
        command.starts_with("kubectl") || command.starts_with("k ")
    }

    fn parse_line(&self, line: &str) -> Option<LineMetadata> {
        let trimmed = line.trim();
        if trimmed.is_empty() { return None; }

        // 1. Detect Resource Listing (get pods, etc.)
        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        if parts.len() >= 3 {
            let status = parts.iter().find(|&&s| 
                matches!(s, "Running" | "Completed" | "Terminating" | "Pending" | "CrashLoopBackOff" | "Error" | "Bound")
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

        // 2. Detect Metadata (Labels, Annotations)
        if line.starts_with("Labels:") || line.starts_with("Annotations:") || line.starts_with("Selector:") {
            let parts: Vec<&str> = trimmed.split(':').collect();
            return Some(LineMetadata {
                perms: "METADATA".to_string(),
                size: parts[0].to_string(),
                name: "field".to_string(),
                is_dir: false,
            });
        }

        None
    }

    fn get_category(&self, _meta: &LineMetadata) -> String {
        "K8S".to_string()
    }

    fn generate_insight(&self, _command: &str, buffer: &DiscoveryBuffer) -> Option<String> {
        let mut running = 0;
        let mut critical = 0;
        let mut total = 0;

        for (key, items) in buffer {
            if key.starts_with("K8S:RESOURCE") {
                total += items.len();
                if key.contains("Running") || key.contains("Completed") { running += items.len(); }
                else if key.contains("Error") || key.contains("CrashLoopBackOff") { critical += items.len(); }
            }
        }

        if critical > 0 {
            Some(format!("Kubernetes Warning: Detected {} unhealthy resources. Cluster might require immediate attention.", critical))
        } else if total > 0 {
            Some(format!("Kubernetes Health: {} resources are healthy/stable. Total resources in this view: {}.", running, total))
        } else {
            None
        }
    }

    fn format_summary(&self, key: &str, items: &[LineMetadata]) -> Option<String> {
        let parts: Vec<&str> = key.split(':').collect();
        if parts[0] != "K8S" { return None; }

        let type_label = parts.get(1).unwrap_or(&"Unknown");
        let count = items.len();

        match *type_label {
            "RESOURCE" => {
                let status = parts.get(2).unwrap_or(&"Stable");
                let names: Vec<String> = items.iter().take(5).map(|m| m.name.clone()).collect();
                let suffix = if count > 5 { format!(" and {} more...", count - 5) } else { "".to_string() };
                Some(format!("• {} [{}]: {}{}", status, count, names.join(", "), suffix))
            },
            "METADATA" => {
                let fields: Vec<String> = items.iter().take(3).map(|m| m.size.clone()).collect();
                let suffix = if count > 3 { format!(" and {} more...", count - 3) } else { "".to_string() };
                Some(format!("• Collapsed {} metadata fields ({}{})", count, fields.join(", "), suffix))
            },
            _ => None
        }
    }

    fn is_outlier(&self, line: &str, meta: &LineMetadata) -> bool {
        if meta.perms == "RESOURCE" {
            // Outlier if state is not Running/Completed or if restarts > 0
            if meta.size != "Running" && meta.size != "Completed" {
                return true;
            }
            
            // Check for restarts in the line (usually column 4 in kubectl get pods)
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 4 {
                if let Ok(restarts) = parts[3].parse::<u32>() {
                    if restarts > 0 { return true; }
                }
            }
        }
        false
    }
}
