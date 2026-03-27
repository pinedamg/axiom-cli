use std::collections::HashMap;
use regex::Regex;
use crate::engine::commands::CommandHandler;

#[derive(Debug, Clone)]
pub struct LineMetadata {
    pub perms: String,
    pub size: String,
    pub name: String,
    pub is_dir: bool,
}

pub struct DiscoveryEngine {
    pub templates: HashMap<String, usize>,
    pub synthesis_buffer: HashMap<String, Vec<LineMetadata>>,
    pub variable_buffer: HashMap<String, Vec<Vec<String>>>,
    pub threshold: usize,
}

impl Default for DiscoveryEngine {
    fn default() -> Self {
        Self {
            templates: HashMap::new(),
            synthesis_buffer: HashMap::new(),
            variable_buffer: HashMap::new(),
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
                } else if prefix == "GO" || prefix == "K8S" || prefix == "TF" || prefix == "CLOUD" {
                    format!("{}:{}:{}", prefix, meta.perms, meta.size)
                } else if prefix == "DOCKER" || prefix == "SEARCH" || prefix == "IO" {
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
        let mut variables = Vec::new();
        let re_months = Regex::new(r"(?i)(Jan|Feb|Mar|Apr|May|Jun|Jul|Aug|Sep|Oct|Nov|Dec)").unwrap();
        let s = re_months.replace_all(line, |caps: &regex::Captures| { variables.push(caps[0].to_string()); "<MONTH>" });
        let re_time = Regex::new(r"\d{1,2}:\d{2}").unwrap();
        let s = re_time.replace_all(&s, |caps: &regex::Captures| { variables.push(caps[0].to_string()); "<TIME>" });
        let re_num = Regex::new(r"\d+").unwrap();
        let s = re_num.replace_all(&s, |caps: &regex::Captures| { variables.push(caps[0].to_string()); "<NUM>" });
        (s.to_string(), variables)
    }

    pub fn flush_variable_summary(&mut self, handlers: &[Box<dyn CommandHandler>]) -> Vec<String> {
        let mut summaries = Vec::new();
        let mut keys: Vec<_> = self.synthesis_buffer.keys().cloned().collect();
        keys.sort();

        for key in keys {
            if let Some(items) = self.synthesis_buffer.remove(&key) {
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
        }

        for (template, var_sets) in self.variable_buffer.drain() {
            if var_sets.len() > 1 {
                summaries.push(format!("Collapsed {} lines of pattern: {}", var_sets.len(), template));
            }
        }
        summaries
    }
}
