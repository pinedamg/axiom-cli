use std::collections::BTreeMap;
use regex::Regex;
use std::sync::OnceLock;
use crate::engine::commands::CommandHandler;

#[derive(Debug, Clone)]
pub struct LineMetadata {
    pub perms: String,
    pub size: String,
    pub name: String,
    pub is_dir: bool,
}

pub struct DiscoveryEngine {
    // Memory Efficiency: Replaced HashMap with BTreeMap to reduce hashing overhead for string keys
    // and inherently sort keys, preventing intermediate allocations during flush.
    pub templates: BTreeMap<String, usize>,
    pub synthesis_buffer: BTreeMap<String, Vec<LineMetadata>>,
    pub variable_buffer: BTreeMap<String, Vec<Vec<String>>>,
    pub threshold: usize,
    pub last_line: Option<String>,
    pub repeat_count: usize,
    pub last_category: Option<String>,
    pub category_count: usize,
}

impl Default for DiscoveryEngine {
    fn default() -> Self {
        Self {
            templates: BTreeMap::new(),
            synthesis_buffer: BTreeMap::new(),
            variable_buffer: BTreeMap::new(),
            threshold: 5,
            last_line: None,
            repeat_count: 0,
            last_category: None,
            category_count: 0,
        }
    }
}

impl DiscoveryEngine {
    pub fn load_templates(&mut self, known: Vec<(String, usize)>) {
        for (template, freq) in known {
            self.templates.insert(template, freq);
        }
    }

    fn parse_standard_ls(&self, line: &str, handler: Option<&dyn CommandHandler>, command: &str) -> Vec<LineMetadata> {
        if !command.starts_with("ls") { return vec![]; }
        
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('(') || trimmed.len() > 100 { return vec![]; }
        if line.starts_with('d') || line.starts_with('-') || line.starts_with('l') || line.starts_with("total") { return vec![]; }
        
        // Anti-collision: skip lines that look like logs or structured data
        if trimmed.starts_with('[') || trimmed.contains(": ") || trimmed.contains(" = ") {
            return vec![];
        }

        // Anti-collision with the current handler
        if handler.map_or(false, |h| h.parse_line(line).is_some()) { 
            return vec![]; 
        }

        line.split_whitespace().filter(|&s| !s.is_empty()).map(|name| {
            let ext = if name.contains('.') { name.split('.').last().unwrap_or("bin").to_string() } else { "dir".to_string() };
            LineMetadata { perms: ext, size: "0".to_string(), name: name.to_string(), is_dir: !name.contains('.') }
        }).collect()
    }

    pub fn synthesize_line(&mut self, line: &str, handler: Option<&dyn CommandHandler>, command: &str) -> bool {
        // 1. Try command-specific handler first (SOLID: Extension)
        if let Some(h) = handler {
            if let Some(meta) = h.parse_line(line) {
                let is_outlier = h.is_outlier(line, &meta);
                
                let prefix = if ["LOG_COMMIT", "MODIFIED", "UNTRACKED", "DELETED", "NEW", "RENAMED", "STAGED"].contains(&meta.perms.as_str()) { 
                    "GIT" 
                } else if ["Running", "Stopped", "Created", "LAYER", "BUILD", "COMPOSE"].contains(&meta.perms.as_str()) {
                    "DOCKER"
                } else if meta.perms == "MATCH" {
                    "SEARCH"
                } else if ["Checking", "Compiling", "Downloading", "Downloaded", "Finished", "Processing"].contains(&meta.perms.as_str()) {
                    "CARGO"
                } else if meta.perms == "PROGRESS" || meta.perms == "NETWORK_NOISE" {
                    "IO"
                } else if ["WARN", "ADD", "AUDIT"].contains(&meta.perms.as_str()) {
                    "NPM"
                } else if ["TEST_RESULT", "COMPILING"].contains(&meta.perms.as_str()) {
                    "GO"
                } else if ["RESOURCE", "METADATA"].contains(&meta.perms.as_str()) {
                    "K8S"
                } else if ["PLAN", "ATTRIBUTE", "STATE"].contains(&meta.perms.as_str()) {
                    "TF"
                } else if ["RESOURCE", "ROW"].contains(&meta.perms.as_str()) && (h.matches("gcloud") || h.matches("aws") || h.matches("az")) {
                    "CLOUD"
                } else if ["STRUCT", "KEY"].contains(&meta.perms.as_str()) {
                    "DATA"
                } else if ["NOISE", "LOG"].contains(&meta.perms.as_str()) {
                    "SYS"
                } else if meta.is_dir && h.matches("ps") {
                    "KERNEL"
                } else if !meta.is_dir && h.matches("ps") {
                    "PROC"
                } else if meta.is_dir { 
                    "DIR" 
                } else { 
                    "FILE" 
                };

                let key = if prefix == "GIT" {
                    if meta.perms == "LOG_COMMIT" {
                        format!("{}:{}:ALL", prefix, meta.perms)
                    } else {
                        format!("{}:{}:{}", prefix, meta.perms, meta.size)
                    }
                } else if prefix == "CARGO" || prefix == "NPM" || (prefix == "DOCKER" && (meta.perms == "LAYER" || meta.perms == "BUILD")) {
                    format!("{}:{}", prefix, meta.perms)
                } else if prefix == "GO" || prefix == "K8S" || prefix == "TF" || prefix == "CLOUD" || prefix == "DATA" || prefix == "SYS" {
                    format!("{}:{}:{}", prefix, meta.perms, meta.size)
                } else if prefix == "DOCKER" || prefix == "SEARCH" || prefix == "IO" {
                    format!("{}:{}:{}", prefix, meta.perms, meta.size)
                } else if prefix == "PROC" || prefix == "KERNEL" {
                    format!("{}:{}", prefix, meta.name)
                } else {
                    format!("{}:{}", prefix, meta.perms)
                };

                self.synthesis_buffer.entry(key).or_default().push(meta);
                
                // If it's an outlier, we return false so the line is printed, 
                // but it's already in the buffer for the final insight.
                return !is_outlier;
            }
        }

        let files = self.parse_standard_ls(line, handler, command);
        if !files.is_empty() {
            for meta in files {
                let key = format!("EXT:{}", meta.perms.to_uppercase());
                self.synthesis_buffer.entry(key).or_default().push(meta);
            }
            return true;
        }
        false
    }

    pub fn process_and_check_noise(&mut self, line: &str, handler: Option<&dyn CommandHandler>, command: &str) -> bool {
        if self.synthesize_line(line, handler, command) { return true; }
        let (template, vars) = self.extract_parts(line);
        
        let count = self.templates.entry(template.clone()).or_insert(0);
        
        // If we already have high confidence in this pattern (e.g. loaded from DB with high frequency),
        // collapse it immediately. Otherwise, wait for the threshold.
        if *count > self.threshold {
            self.variable_buffer.entry(template).or_default().push(vars);
            return true;
        }

        *count += 1;
        if *count > self.threshold {
            self.variable_buffer.entry(template).or_default().push(vars);
            true
        } else {
            false
        }
    }

    pub fn extract_parts(&self, line: &str) -> (String, Vec<String>) {
        // Memory Efficiency: Pre-allocated vector to prevent re-allocations during capture
        let mut variables = Vec::with_capacity(4);
        
        // Memory Efficiency: Use OnceLock to compile Regex patterns only once,
        // avoiding excessive allocations and CPU overhead in the line-processing hot path.
        static RE_UUID: OnceLock<Regex> = OnceLock::new();
        let re_uuid = RE_UUID.get_or_init(|| Regex::new(r"[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}").unwrap());
        let s = re_uuid.replace_all(line, |caps: &regex::Captures| { variables.push(caps[0].to_string()); "<UUID>" });
        
        static RE_HEX: OnceLock<Regex> = OnceLock::new();
        let re_hex = RE_HEX.get_or_init(|| Regex::new(r"0x[0-9a-fA-F]+").unwrap());
        let s = re_hex.replace_all(&s, |caps: &regex::Captures| { variables.push(caps[0].to_string()); "<HEX>" });

        static RE_PATH: OnceLock<Regex> = OnceLock::new();
        let re_path = RE_PATH.get_or_init(|| Regex::new(r"/[a-zA-Z0-9\._\-/]+").unwrap());
        let s = re_path.replace_all(&s, |caps: &regex::Captures| { variables.push(caps[0].to_string()); "<PATH>" });

        static RE_MONTHS: OnceLock<Regex> = OnceLock::new();
        let re_months = RE_MONTHS.get_or_init(|| Regex::new(r"(?i)(Jan|Feb|Mar|Apr|May|Jun|Jul|Aug|Sep|Oct|Nov|Dec)").unwrap());
        let s = re_months.replace_all(&s, |caps: &regex::Captures| { variables.push(caps[0].to_string()); "<MONTH>" });
        
        static RE_TIME: OnceLock<Regex> = OnceLock::new();
        let re_time = RE_TIME.get_or_init(|| Regex::new(r"\d{1,2}:\d{2}").unwrap());
        let s = re_time.replace_all(&s, |caps: &regex::Captures| { variables.push(caps[0].to_string()); "<TIME>" });
        
        static RE_NUM: OnceLock<Regex> = OnceLock::new();
        let re_num = RE_NUM.get_or_init(|| Regex::new(r"\d+").unwrap());
        let s = re_num.replace_all(&s, |caps: &regex::Captures| { variables.push(caps[0].to_string()); "<NUM>" });
        
        (s.to_string(), variables)
    }

    pub fn flush_variable_summary(&mut self, handlers: &[Box<dyn CommandHandler>]) -> Vec<String> {
        let mut summaries = Vec::new();

        // Memory Efficiency: Utilize std::mem::take to directly drain and iterate over the BTreeMap,
        // which avoids intermediate heap allocations from cloning and sorting keys. BTreeMap is already sorted.
        let synthesis_buffer = std::mem::take(&mut self.synthesis_buffer);

        for (key, items) in synthesis_buffer {
            let mut formatted = false;
            for handler in handlers {
                if let Some(summary) = handler.format_summary(&key, &items) {
                    summaries.push(summary);
                    formatted = true;
                    break;
                }
            }

            if !formatted {
                let parts: Vec<&str> = key.split(':').collect();
                let label = parts[0];
                match label {
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
                }
            }
        }

        // Variable buffer is also drained, which is already optimized.
        for (template, var_sets) in std::mem::take(&mut self.variable_buffer) {
            if var_sets.len() > 1 {
                summaries.push(format!("Line matched {} more times: {}", var_sets.len(), template));
            }
        }
        summaries
    }
}
