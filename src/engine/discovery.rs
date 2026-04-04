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
    pub last_line: Option<String>,
    pub repeat_count: usize,
    pub last_category: Option<String>,
    pub category_count: usize,
}

impl Default for DiscoveryEngine {
    fn default() -> Self {
        Self {
            templates: HashMap::new(),
            synthesis_buffer: HashMap::new(),
            variable_buffer: HashMap::new(),
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

    pub fn get_templates(&self) -> Vec<(String, usize)> {
        self.templates.iter().map(|(k, v)| (k.clone(), *v)).collect()
    }

    pub fn get_saved_bytes(&self) -> usize {
        // Approximate saved bytes from synthesis and variable buffers
        let mut total = 0;
        for items in self.synthesis_buffer.values() {
            for item in items {
                total += item.name.len() + 20; // Plus overhead
            }
        }
        for (template, var_sets) in &self.variable_buffer {
            total += template.len() * var_sets.len();
        }
        total
    }

    fn parse_standard_ls(&self, line: &str, handler: Option<&dyn CommandHandler>, command: &str) -> Vec<LineMetadata> {
        if !command.starts_with("ls") { return vec![]; }
        
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('(') || trimmed.len() > 100 { return vec![]; }
        
        // Swallow metadata lines (total, current dir, parent dir) by returning a marker or just empty
        // but synthesize_line needs to know we 'handled' it.
        if trimmed.starts_with("total") || trimmed == "." || trimmed == ".." { 
            return vec![LineMetadata { perms: "META".to_string(), size: "0".to_string(), name: "metadata".to_string(), is_dir: false }];
        }

        // Handle 'ls -la' format (permissions, links, owner, group, size, month, day, time, name)
        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        if parts.len() >= 9 && (trimmed.starts_with('d') || trimmed.starts_with('-') || trimmed.starts_with('l')) {
            let name = parts[8..].join(" ");
            let perms = &parts[0][1..4]; // Take first 3 chars of perms after type
            let is_dir = trimmed.starts_with('d');
            return vec![LineMetadata { perms: perms.to_string(), size: parts[4].to_string(), name, is_dir }];
        }

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
                let prefix = h.get_category(&meta.perms);
                let key = h.get_key(&prefix, &meta);

                self.synthesis_buffer.entry(key).or_default().push(meta);
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
        let mut variables = Vec::new();
        
        let re_uuid = Regex::new(r"[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}").unwrap();
        let s = re_uuid.replace_all(line, |caps: &regex::Captures| { variables.push(caps[0].to_string()); "<UUID>" });
        
        let re_hex = Regex::new(r"0x[0-9a-fA-F]+").unwrap();
        let s = re_hex.replace_all(&s, |caps: &regex::Captures| { variables.push(caps[0].to_string()); "<HEX>" });

        let re_path = Regex::new(r"/[a-zA-Z0-9\._\-/]+").unwrap();
        let s = re_path.replace_all(&s, |caps: &regex::Captures| { variables.push(caps[0].to_string()); "<PATH>" });

        let re_months = Regex::new(r"(?i)(Jan|Feb|Mar|Apr|May|Jun|Jul|Aug|Sep|Oct|Nov|Dec)").unwrap();
        let s = re_months.replace_all(&s, |caps: &regex::Captures| { variables.push(caps[0].to_string()); "<MONTH>" });
        
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
                summaries.push(format!("Line matched {} more times: {}", var_sets.len(), template));
            }
        }
        summaries
    }
}
