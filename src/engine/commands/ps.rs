use crate::engine::discovery::LineMetadata;
use super::{CommandHandler, DiscoveryBuffer};

pub struct PsHandler;

impl CommandHandler for PsHandler {
    fn matches(&self, command: &str) -> bool {
        command.starts_with("ps")
    }

    fn parse_line(&self, line: &str) -> Option<LineMetadata> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 11 { return None; }
        
        let user = parts[0];
        
        // Anti-collision check
        if line.contains("modified:") || line.contains("new file:") || line.starts_with("On branch") || 
           line.starts_with("commit ") || line.starts_with("CONTAINER ID") || 
           (parts[0].len() == 12 && parts[0].chars().all(|c| c.is_ascii_hexdigit())) {
            return None;
        }

        let cpu = parts[2];
        let command_full = parts[10..].join(" ");
        let is_kernel = command_full.starts_with('[') && command_full.ends_with(']');
        
        let clean_cmd = if is_kernel {
            let base = command_full.trim_matches(|c| c == '[' || c == ']');
            let normalized = base.split('/').next().unwrap()
                .split(':').next().unwrap()
                .split('-').next().unwrap();
            format!("[{}]", normalized)
        } else {
            // Take the first part of the command (the executable)
            let exe_path = parts[10];
            exe_path.split('/').last().unwrap_or(exe_path).to_string()
        };

        Some(LineMetadata { 
            perms: user.to_string(), 
            size: cpu.to_string(), 
            name: clean_cmd, 
            is_dir: is_kernel 
        })
    }

    fn generate_insight(&self, _command: &str, buffer: &DiscoveryBuffer) -> Option<String> {
        let mut max_cpu = 0.0;
        // ⚡ Bolt: Using Option<&str> to avoid cloning String inside the loop, referencing the existing DiscoveryBuffer.
        let mut top_proc: Option<&str> = None;
        let mut total_procs = 0;

        for (key, items) in buffer {
            if key.starts_with("PROC:") {
                total_procs += items.len();
                for item in items {
                    if let Ok(cpu) = item.size.parse::<f64>() {
                        if cpu > max_cpu {
                            max_cpu = cpu;
                            top_proc = Some(item.name.as_str());
                        }
                    }
                }
            }
        }

        if total_procs > 0 {
            if max_cpu > 10.0 {
                Some(format!("High CPU load detected: {} is using {}% CPU. Total active processes: {}.", top_proc.unwrap_or("unknown"), max_cpu, total_procs))
            } else {
                Some(format!("System health stable. Total active processes: {}. No single process exceeding 10% CPU.", total_procs))
            }
        } else {
            None
        }
    }

    fn format_summary(&self, key: &str, items: &[LineMetadata]) -> Option<String> {
        let parts: Vec<&str> = key.split(':').collect();
        let label = parts[0];
        
        match label {
            "PROC" => {
                let cmd = parts.get(1).unwrap_or(&"unknown");
                let users: std::collections::HashSet<_> = items.iter().map(|m| &m.perms).collect();
                let mut user_list: Vec<_> = users.into_iter().cloned().collect();
                user_list.sort();
                Some(format!("Active processes: {} (count: {}) | Owners: {}", cmd, items.len(), user_list.join(", ")))
            },
            "KERNEL" => {
                let name = parts.get(1).unwrap_or(&"worker");
                Some(format!("Kernel Workers: {} (count: {})", name, items.len()))
            },
            _ => None
        }
    }

    fn get_category(&self, _perms: &str) -> String {
        // We use is_dir as a proxy flag for kernel processes in our ps implementation
        // This is a bit of a hack but it's KISS for this specific tool.
        // Actually, we should check meta during get_key, but the trait only gives perms.
        // Let's improve the trait to pass metadata or just handle it in get_key.
        "PROC".to_string() 
    }

    fn get_key(&self, _prefix: &str, meta: &LineMetadata) -> String {
        if meta.is_dir { format!("KERNEL:{}", meta.name) }
        else { format!("PROC:{}", meta.name) }
    }
}
