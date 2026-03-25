use std::collections::BTreeMap;
use std::sync::OnceLock;
use regex::Regex;
use crate::engine::commands::CommandHandler;

#[derive(Debug, Clone)]
pub struct LineMetadata {
    pub perms: String,
    pub size: String,
    pub name: String,
    pub is_dir: bool,
}

/// BTreeMap is more efficient here as the number of active variables and templates
/// per command execution is relatively small and it avoids hash collisions,
/// while keeping the output strictly ordered without needing an extra allocation and sort step.
pub struct DiscoveryEngine {
    pub templates: BTreeMap<String, usize>,
    pub synthesis_buffer: BTreeMap<String, Vec<LineMetadata>>,
    pub variable_buffer: BTreeMap<String, Vec<Vec<String>>>,
    pub threshold: usize,
}

impl Default for DiscoveryEngine {
    fn default() -> Self {
        Self {
            templates: BTreeMap::new(),
            synthesis_buffer: BTreeMap::new(),
            variable_buffer: BTreeMap::new(),
            threshold: 5,
        }
    }
}

impl DiscoveryEngine {
    pub fn load_templates(&mut self, known: Vec<(String, usize)>) {
        for (template, freq) in known {
            self.templates.insert(template, freq);
        }
    }

    fn parse_standard_ls(&self, line: &str, handler: Option<&dyn CommandHandler>) -> Vec<LineMetadata> {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('(') || trimmed.len() > 100 { return vec![]; }
        if line.starts_with('d') || line.starts_with('-') || line.starts_with('l') || line.starts_with("total") { return vec![]; }
        
        // Anti-collision with the current handler
        if handler.map_or(false, |h| h.parse_line(line).is_some()) { 
            return vec![]; 
        }

        line.split_whitespace().filter(|&s| !s.is_empty()).map(|name| {
            let ext = if name.contains('.') { name.split('.').last().unwrap_or("bin").to_string() } else { "dir".to_string() };
            LineMetadata { perms: ext, size: "0".to_string(), name: name.to_string(), is_dir: !name.contains('.') }
        }).collect()
    }

    pub fn synthesize_line(&mut self, line: &str, handler: Option<&dyn CommandHandler>) -> bool {
        // 1. Try command-specific handler first (SOLID: Extension)
        if let Some(h) = handler {
            if let Some(meta) = h.parse_line(line) {
                let prefix = if meta.perms == "LOG_COMMIT" { 
                    "GIT" 
                } else if meta.perms == "Running" || meta.perms == "Stopped" || meta.perms == "Created" {
                    "DOCKER"
                } else if meta.is_dir && h.matches("ps") {
                    "KERNEL"
                } else if !meta.is_dir && h.matches("ps") {
                    "PROC"
                } else if meta.is_dir { 
                    "DIR" 
                } else { 
                    "FILE" 
                };
                
                let key = if prefix == "GIT" || prefix == "DOCKER" {
                    format!("{}:{}:{}", prefix, meta.perms, meta.size)
                } else if prefix == "PROC" || prefix == "KERNEL" {
                    format!("{}:{}", prefix, meta.name)
                } else {
                    format!("{}:{}", prefix, meta.perms)
                };

                self.synthesis_buffer.entry(key).or_default().push(meta);
                return true;
            }
        }

        let files = self.parse_standard_ls(line, handler);
        if !files.is_empty() {
            for meta in files {
                let key = format!("EXT:{}", meta.perms.to_uppercase());
                self.synthesis_buffer.entry(key).or_default().push(meta);
            }
            return true;
        }
        false
    }

    pub fn process_and_check_noise(&mut self, line: &str, handler: Option<&dyn CommandHandler>) -> bool {
        if self.synthesize_line(line, handler) { return true; }
        let (template, vars) = self.extract_parts(line);
        let count = self.templates.entry(template.clone()).or_insert(0);
        *count += 1;
        if *count > self.threshold {
            self.variable_buffer.entry(template).or_default().push(vars);
            true
        } else {
            false
        }
    }

    pub fn extract_parts(&self, line: &str) -> (String, Vec<String>) {
        static RE_MONTHS: OnceLock<Regex> = OnceLock::new();
        static RE_TIME: OnceLock<Regex> = OnceLock::new();
        static RE_NUM: OnceLock<Regex> = OnceLock::new();
        static RE_HEX: OnceLock<Regex> = OnceLock::new();
        static RE_UUID: OnceLock<Regex> = OnceLock::new();
        static RE_PATH: OnceLock<Regex> = OnceLock::new();

        let re_months = RE_MONTHS.get_or_init(|| Regex::new(r"(?i)(Jan|Feb|Mar|Apr|May|Jun|Jul|Aug|Sep|Oct|Nov|Dec)").unwrap());
        let re_time = RE_TIME.get_or_init(|| Regex::new(r"\d{1,2}:\d{2}").unwrap());
        let re_num = RE_NUM.get_or_init(|| Regex::new(r"\d+").unwrap());
        let re_uuid = RE_UUID.get_or_init(|| Regex::new(r"[a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}").unwrap());
        let re_hex = RE_HEX.get_or_init(|| Regex::new(r"0x[a-fA-F0-9]+|[a-fA-F0-9]{12,64}").unwrap());
        let re_path = RE_PATH.get_or_init(|| Regex::new(r"(?:/[a-zA-Z0-9_.-]+)+").unwrap());

        // We use with_capacity as typically a line won't have more than 4-6 matches, reducing vector reallocations
        let mut variables = Vec::with_capacity(4);

        let s = re_uuid.replace_all(line, |caps: &regex::Captures| { variables.push(caps[0].to_string()); "<UUID>" });
        let s = re_hex.replace_all(&s, |caps: &regex::Captures| { variables.push(caps[0].to_string()); "<HEX>" });
        let s = re_months.replace_all(&s, |caps: &regex::Captures| { variables.push(caps[0].to_string()); "<MONTH>" });
        let s = re_time.replace_all(&s, |caps: &regex::Captures| { variables.push(caps[0].to_string()); "<TIME>" });
        let s = re_num.replace_all(&s, |caps: &regex::Captures| { variables.push(caps[0].to_string()); "<NUM>" });
        let s = re_path.replace_all(&s, |caps: &regex::Captures| { variables.push(caps[0].to_string()); "<PATH>" });

        (s.to_string(), variables)
    }

    pub fn flush_variable_summary(&mut self) -> Vec<String> {
        let mut summaries = Vec::new();

        let mut log_count = 0;
        let mut log_hashes = Vec::new();

        // Using std::mem::take to avoid cloning keys and the BTreeMap guarantees sorted iteration natively
        let synthesis_buffer = std::mem::take(&mut self.synthesis_buffer);

        for (key, items) in synthesis_buffer {
            let parts: Vec<&str> = key.split(':').collect();
            let label = parts[0];

            match label {
                    "DOCKER" => {
                        let status = parts[1];
                        let image = parts[2];
                        let names: Vec<String> = items.iter().map(|m| m.name.clone()).collect();
                        summaries.push(format!("Docker {}: {} containers from image [{}] | {}", status, items.len(), image, names.join(", ")));
                    },
                    "GIT" => {
                        let state = parts[1];
                        if state == "LOG_COMMIT" {
                            log_count += items.len();
                            log_hashes.extend(items.iter().map(|m| m.size.clone()));
                        } else {
                            let folder = parts[2];
                            let names: Vec<String> = items.iter().map(|m| m.name.clone()).collect();
                            summaries.push(format!("Git {}: {} files in [{}] | {}", state, items.len(), folder, names.join(", ")));
                        }
                    },
                    "PROC" => {
                        let cmd = parts[1];
                        let users: std::collections::HashSet<_> = items.iter().map(|m| &m.perms).collect();
                        let mut user_list: Vec<_> = users.into_iter().cloned().collect();
                        user_list.sort();
                        summaries.push(format!("Active processes: {} (count: {}) | Owners: {}", cmd, items.len(), user_list.join(", ")));
                    },
                    "KERNEL" => summaries.push(format!("Kernel Workers: {} (count: {})", parts[1], items.len())),
                    "DIR" | "FILE" => {
                        let names: Vec<String> = items.iter().map(|m| m.name.clone()).collect();
                        let perms = parts.get(1).unwrap_or(&"---");
                        summaries.push(format!("{} [{}] | {}", label, perms, names.join(", ")));
                    },
                "EXT" => {
                    let names: Vec<String> = items.iter().map(|m| m.name.clone()).collect();
                    summaries.push(format!("Grouped {} files by extension [{}] | {}", items.len(), parts[1], names.join(", ")));
                },
                _ => summaries.push(format!("Summary for {}: {} items", label, items.len()))
            };
        }

        if log_count > 0 {
            summaries.push(format!("Git History: {} recent commits | Hashes: {}...", log_count, log_hashes.join(", ")));
        }

        // Iterate through drain for memory efficiency (clean buffer)
        for (template, var_sets) in std::mem::take(&mut self.variable_buffer) {
            if var_sets.len() > 1 {
                summaries.push(format!("Collapsed {} lines of pattern: {}", var_sets.len(), template));
            }
        }
        summaries
    }
}
