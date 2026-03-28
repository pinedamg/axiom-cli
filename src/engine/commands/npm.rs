use crate::engine::discovery::LineMetadata;
use super::{CommandHandler, DiscoveryBuffer};

pub struct NpmHandler;

impl CommandHandler for NpmHandler {
    fn matches(&self, command: &str) -> bool {
        command.starts_with("npm") || command.starts_with("pnpm") || command.starts_with("yarn")
    }

    fn parse_line(&self, line: &str) -> Option<LineMetadata> {
        let trimmed = line.trim();
        if trimmed.is_empty() { return None; }

        // 1. Detect deprecation warnings
        if line.contains("deprecated") || line.contains("npm WARN") {
            return Some(LineMetadata {
                perms: "WARN".to_string(),
                size: "deprecation".to_string(),
                name: trimmed.to_string(),
                is_dir: false,
            });
        }

        // 2. Detect package additions (npm/pnpm/yarn styles)
        if trimmed.starts_with("+ ") || trimmed.starts_with("added ") || trimmed.starts_with("installed ") {
            let pkg = trimmed.split_whitespace().nth(1).unwrap_or("package");
            return Some(LineMetadata {
                perms: "ADD".to_string(),
                size: "new".to_string(),
                name: pkg.to_string(),
                is_dir: false,
            });
        }

        // 3. Detect audit results
        if line.contains("vulnerability") || line.contains("vulnerabilities") {
            return Some(LineMetadata {
                perms: "AUDIT".to_string(),
                size: "security".to_string(),
                name: trimmed.to_string(),
                is_dir: false,
            });
        }

        None
    }

    fn generate_insight(&self, _command: &str, buffer: &DiscoveryBuffer) -> Option<String> {
        let mut warnings = 0;
        let mut added = 0;
        let mut audit_issues = 0;

        for (key, items) in buffer {
            if key.starts_with("NPM:") {
                if key.contains("WARN") { warnings += items.len(); }
                else if key.contains("ADD") { added += items.len(); }
                else if key.contains("AUDIT") { audit_issues += items.len(); }
            }
        }

        if warnings > 10 || audit_issues > 0 {
            Some(format!("Node.js Environment: Hidden {} deprecation warnings. Detected {} security audit issues. Recommend 'npm audit fix'.", warnings, audit_issues))
        } else if added > 0 {
            Some(format!("Node.js Installation: Successfully added {} new packages to the project.", added))
        } else {
            None
        }
    }

    fn format_summary(&self, key: &str, items: &[LineMetadata]) -> Option<String> {
        let parts: Vec<&str> = key.split(':').collect();
        if parts[0] != "NPM" { return None; }

        let type_label = parts.get(1).unwrap_or(&"Unknown");
        let count = items.len();
        
        match *type_label {
            "WARN" => {
                let examples: Vec<String> = items.iter().take(2).map(|m| m.name.clone()).collect();
                let suffix = if count > 2 { format!(" and {} more...", count - 2) } else { "".to_string() };
                Some(format!("• Hidden {} deprecation warnings (e.g. {}{})", count, examples.join(", "), suffix))
            },
            "ADD" => {
                let names: Vec<String> = items.iter().take(5).map(|m| m.name.clone()).collect();
                let suffix = if count > 5 { format!(" and {} more...", count - 5) } else { "".to_string() };
                Some(format!("• Added {} packages ({}{})", count, names.join(", "), suffix))
            },
            "AUDIT" => Some(format!("• Security Audit: {} vulnerabilities found and collapsed.", count)),
            _ => None
        }
    }
}
