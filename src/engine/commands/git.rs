use crate::engine::discovery::LineMetadata;
use super::{CommandHandler, DiscoveryBuffer};

pub struct GitHandler;

impl GitHandler {
    fn is_oneline_hash(s: &str) -> bool {
        s.len() >= 7 && s.len() <= 10 && s.chars().all(|c| c.is_ascii_hexdigit())
    }
}

impl CommandHandler for GitHandler {
    fn matches(&self, command: &str) -> bool {
        command.starts_with("git")
    }

    fn parse_line(&self, line: &str) -> Option<LineMetadata> {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('(') || trimmed.contains("files:") { return None; }
        
        // 1. Detect standard commit log (commit <hash>)
        if line.starts_with("commit ") {
            let hash = trimmed.split_whitespace().nth(1).unwrap_or("unknown");
            return Some(LineMetadata { 
                perms: "LOG_COMMIT".to_string(), 
                size: hash[..7.min(hash.len())].to_string(), 
                name: "commit".to_string(), 
                is_dir: false 
            });
        }

        // 2. Detect git log --oneline (hash at start)
        let first_word = trimmed.split_whitespace().next().unwrap_or("");
        if Self::is_oneline_hash(first_word) {
            return Some(LineMetadata { 
                perms: "LOG_COMMIT".to_string(), 
                size: first_word.to_string(), 
                name: "commit".to_string(), 
                is_dir: false 
            });
        }

        // 3. Detect file status (modified, new, etc)
        let (state, path) = if line.contains("modified:") {
            ("MODIFIED", trimmed.trim_start_matches("modified:").trim())
        } else if line.contains("new file:") {
            ("NEW", trimmed.trim_start_matches("new file:").trim())
        } else if line.contains("deleted:") {
            ("DELETED", trimmed.trim_start_matches("deleted:").trim())
        } else if line.starts_with("\t") || (line.starts_with("    ") && !line.contains(':')) {
            ("UNTRACKED", trimmed)
        } else {
            return None;
        };

        if path.is_empty() || path.contains("nothing to commit") { return None; }
        
        let folder = if path.contains('/') { 
            path.split('/').next().unwrap_or("root").to_string() 
        } else { 
            "root".to_string() 
        };

        Some(LineMetadata { 
            perms: state.to_string(), 
            size: folder, 
            name: path.to_string(), 
            is_dir: path.contains('/') 
        })
    }

    fn generate_insight(&self, command: &str, buffer: &DiscoveryBuffer) -> Option<String> {
        let mut modified = 0;
        let mut untracked = 0;
        let mut log_commits = 0;

        for (key, items) in buffer {
            if key.contains("MODIFIED") { modified += items.len(); }
            else if key.contains("UNTRACKED") { untracked += items.len(); }
            else if key.contains("LOG_COMMIT") { log_commits += items.len(); }
        }

        if command.contains("status") {
            if modified > 0 || untracked > 0 {
                Some(format!("Repository has pending changes: {} modified, {} untracked. Recommend 'git commit'.", modified, untracked))
            } else {
                Some("Repository clean. No pending changes detected.".to_string())
            }
        } else if command.contains("log") {
            Some(format!("Detected active history with {} commits in this view. Use 'git show' for details.", log_commits))
        } else {
            None
        }
    }

    fn format_summary(&self, key: &str, items: &[LineMetadata]) -> Option<String> {
        let parts: Vec<&str> = key.split(':').collect();
        if parts[0] != "GIT" { return None; }
        
        let state = parts.get(1).unwrap_or(&"UNKNOWN");
        
        if *state == "LOG_COMMIT" {
            let hashes: Vec<String> = items.iter().map(|m| m.size.clone()).collect();
            return Some(format!("Git History: {} recent commits | Hashes: {}...", items.len(), hashes.join(", ")));
        } else {
            let folder = parts.get(2).unwrap_or(&"root");
            let names: Vec<String> = items.iter().map(|m| m.name.clone()).collect();
            return Some(format!("Git {}: {} files in [{}] | {}", state, items.len(), folder, names.join(", ")));
        }
    }
}
