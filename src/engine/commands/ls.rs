use crate::engine::discovery::LineMetadata;
use super::{CommandHandler, DiscoveryBuffer};

pub struct LsHandler;

impl CommandHandler for LsHandler {
    fn matches(&self, command: &str) -> bool {
        command.starts_with("ls")
    }

    fn parse_line(&self, line: &str) -> Option<LineMetadata> {
        if line.starts_with("total ") {
            return Some(LineMetadata {
                perms: "total".to_string(),
                size: "0".to_string(),
                name: "total".to_string(),
                is_dir: false
            });
        }
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 5 { return None; }
        
        let perms = parts[0].to_string();
        if perms.len() < 10 { return None; }
        
        // Match standard drwxr-xr-x or -rw-r--r--
        if !perms.starts_with('d') && !perms.starts_with('-') && !perms.starts_with('l') {
            return None;
        }

        let is_dir = perms.starts_with('d');
        let name = parts.last()?.to_string();
        
        // Skip current and parent directory
        if name == "." || name == ".." { return None; }
        
        let short_perms = if perms.len() >= 4 { 
            perms[1..4].to_string() 
        } else { 
            perms 
        };

        Some(LineMetadata { 
            perms: short_perms, 
            size: parts[4].to_string(), 
            name, 
            is_dir 
        })
    }

    fn generate_insight(&self, _command: &str, buffer: &DiscoveryBuffer) -> Option<String> {
        let mut project_type = None;
        
        // Look for signature files in the synthesis buffer
        for items in buffer.values() {
            for item in items {
                let lower_name = item.name.to_lowercase();
                if lower_name == "cargo.toml" { 
                    project_type = Some("Detected Rust Project Workspace."); 
                    break; 
                }
                if lower_name == "package.json" { 
                    project_type = Some("Detected Node.js Project."); 
                    break; 
                }
                if lower_name == "go.mod" { 
                    project_type = Some("Detected Go Module."); 
                    break; 
                }
            }
            if project_type.is_some() { break; }
        }
        
        project_type.map(|s| s.to_string())
    }

    fn format_summary(&self, key: &str, items: &[LineMetadata]) -> Option<String> {
        let parts: Vec<&str> = key.split(':').collect();
        let label = parts[0];
        
        if label == "DIR" || label == "FILE" {
            let count = items.len();
            let perms = parts.get(1).unwrap_or(&"---");
            let names: Vec<String> = items.iter().take(5).map(|m| m.name.clone()).collect();
            let suffix = if count > 5 { format!(" and {} more...", count - 5) } else { "".to_string() };
            
            return Some(format!("{} [{}] ({}) | {}{}", label, perms, count, names.join(", "), suffix));
        }
        
        None
    }

    fn is_outlier(&self, line: &str, meta: &LineMetadata) -> bool {
        // Outlier if permissions are dangerous (777)
        if meta.perms.contains("rwxrwxrwx") || line.contains("rwxrwxrwx") {
            return true;
        }
        
        // Outlier if file is very large (e.g. > 1GB)
        if let Ok(size) = meta.size.parse::<u64>() {
            if size > 1_000_000_000 { return true; }
        }
        
        false
    }

    fn get_category(&self, meta: &LineMetadata) -> String {
        if meta.perms.contains('x') || meta.perms == "rwx" { "DIR".to_string() }
        else { "FILE".to_string() }
    }

    fn get_key(&self, prefix: &str, meta: &LineMetadata) -> String {
        // Group by permissions only, ignore size for ls aggregation
        format!("{}:{}", prefix, meta.perms)
    }
}
