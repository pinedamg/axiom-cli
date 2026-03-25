use crate::engine::discovery::LineMetadata;
use super::{CommandHandler, DiscoveryBuffer};

pub struct DockerHandler;

impl CommandHandler for DockerHandler {
    fn matches(&self, command: &str) -> bool {
        command.starts_with("docker")
    }

    fn parse_line(&self, line: &str) -> Option<LineMetadata> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 7 { return None; }
        
        // Docker Container ID is usually 12 hex chars
        let id = parts[0];
        if id.len() != 12 || !id.chars().all(|c| c.is_ascii_hexdigit()) {
            return None;
        }

        let image = parts[1];
        let status = if line.contains("Up ") { 
            "Running" 
        } else if line.contains("Exited") { 
            "Stopped" 
        } else { 
            "Created" 
        };

        Some(LineMetadata {
            perms: status.to_string(),
            size: image.to_string(),
            name: parts.last().unwrap_or(&"unknown").to_string(),
            is_dir: false,
        })
    }

    fn generate_insight(&self, _command: &str, buffer: &DiscoveryBuffer) -> Option<String> {
        let mut running = 0;
        let mut stopped = 0;
        let mut total = 0;

        for (key, items) in buffer {
            if key.starts_with("DOCKER:") {
                total += items.len();
                if key.contains("Running") { running += items.len(); }
                else if key.contains("Stopped") { stopped += items.len(); }
            }
        }

        if total > 0 {
            if stopped > 5 { // Refined threshold for suggestion
                Some(format!("Detected {} stopped containers. Suggesting 'docker system prune' to recover space.", stopped))
            } else {
                Some(format!("Docker environment: {} running, {} stopped containers.", running, stopped))
            }
        } else {
            None
        }
    }

    fn format_summary(&self, key: &str, items: &[LineMetadata]) -> Option<String> {
        let parts: Vec<&str> = key.split(':').collect();
        if parts[0] != "DOCKER" { return None; }
        
        let status = parts.get(1).unwrap_or(&"Unknown");
        let image = parts.get(2).unwrap_or(&"unknown-image");
        let names: Vec<String> = items.iter().map(|m| m.name.clone()).collect();
        
        Some(format!("Docker {}: {} containers from image [{}] | {}", status, items.len(), image, names.join(", ")))
    }
}
