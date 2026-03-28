use crate::engine::discovery::LineMetadata;
use super::{CommandHandler, DiscoveryBuffer};

pub struct DockerHandler;

impl CommandHandler for DockerHandler {
    fn matches(&self, command: &str) -> bool {
        command.starts_with("docker")
    }

    fn parse_line(&self, line: &str) -> Option<LineMetadata> {
        let trimmed = line.trim();
        if trimmed.is_empty() { return None; }

        // 1. Detect Standard Container Listing (docker ps)
        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        if parts.len() >= 7 {
            let id = parts[0];
            if id.len() == 12 && id.chars().all(|c| c.is_ascii_hexdigit()) {
                let image = parts[1];
                let status = if line.contains("Up ") { "Running" } else if line.contains("Exited") { "Stopped" } else { "Created" };
                return Some(LineMetadata {
                    perms: status.to_string(),
                    size: image.to_string(),
                    name: parts.last().unwrap_or(&"unknown").to_string(),
                    is_dir: false,
                });
            }
        }

        // 2. Detect Layer Progress (docker pull/push)
        if line.contains(':') && (line.contains("Pulling") || line.contains("Waiting") || line.contains("Download") || line.contains("Extracting")) {
            let parts: Vec<&str> = trimmed.split(':').collect();
            let layer_id = parts[0];
            let status = parts.get(1).unwrap_or(&"Processing").trim();
            return Some(LineMetadata {
                perms: "LAYER".to_string(),
                size: status.to_string(),
                name: layer_id.to_string(),
                is_dir: false,
            });
        }

        // 3. Detect Build Steps (docker build)
        if line.starts_with("Step ") && line.contains('/') {
            return Some(LineMetadata {
                perms: "BUILD".to_string(),
                size: "step".to_string(),
                name: trimmed.to_string(),
                is_dir: false,
            });
        }

        // 4. Detect Docker Compose logs
        if line.contains('|') {
            let parts: Vec<&str> = trimmed.split('|').collect();
            let service = parts[0].trim();
            if !service.contains(' ') && service.len() > 1 {
                return Some(LineMetadata {
                    perms: "COMPOSE".to_string(),
                    size: service.to_string(),
                    name: "log".to_string(),
                    is_dir: false,
                });
            }
        }

        None
    }

    fn generate_insight(&self, _command: &str, buffer: &DiscoveryBuffer) -> Option<String> {
        let mut running = 0;
        let mut stopped = 0;
        let mut layers = 0;
        let mut build_steps = 0;

        for (key, items) in buffer {
            if key.starts_with("DOCKER:") {
                if key.contains("Running") { running += items.len(); }
                else if key.contains("Stopped") { stopped += items.len(); }
                else if key.contains("LAYER") { layers += items.len(); }
                else if key.contains("BUILD") { build_steps += items.len(); }
            }
        }

        if layers > 0 {
            Some(format!("Docker Transfer: Processing {} image layers. Stream is compressed for token efficiency.", layers))
        } else if build_steps > 0 {
            Some(format!("Docker Build: Executing {} build steps. Analyzing environment layers.", build_steps))
        } else if running > 0 || stopped > 0 {
            if stopped > 5 {
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
        
        let type_label = parts.get(1).unwrap_or(&"Unknown");
        let count = items.len();

        match *type_label {
            "Running" | "Stopped" | "Created" => {
                let image = parts.get(2).unwrap_or(&"unknown-image");
                let names: Vec<String> = items.iter().take(5).map(|m| m.name.clone()).collect();
                let suffix = if count > 5 { format!(" and {} more...", count - 5) } else { "".to_string() };
                Some(format!("Docker {}: {} containers from [{}] | {}{}", type_label, count, image, names.join(", "), suffix))
            },
            "LAYER" => {
                let examples: Vec<String> = items.iter().take(3).map(|m| m.name.clone()).collect();
                Some(format!("• Hidden {} layer progress updates (e.g. {})", count, examples.join(", ")))
            },
            "BUILD" => Some(format!("• Collapsed {} build steps.", count)),
            "COMPOSE" => {
                let service = parts.get(2).unwrap_or(&"service");
                Some(format!("• {} service logs: {} lines synthesized.", service, count))
            },
            _ => None
        }
    }
}
