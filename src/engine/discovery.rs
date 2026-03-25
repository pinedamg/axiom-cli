use std::collections::HashMap;
use regex::Regex;

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

    fn parse_ls_line(&self, line: &str) -> Option<LineMetadata> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 5 { return None; }

        let perms = parts[0].to_string();
        if perms.len() < 10 { return None; }
        if !perms.starts_with('d') && !perms.starts_with('-') && !perms.starts_with('l') { return None; }

        let is_dir = perms.starts_with('d');
        let name = parts.last()?.to_string();

        if name == "." || name == ".." { return None; }

        let short_perms = if perms.len() >= 4 { perms[1..4].to_string() } else { perms };

        Some(LineMetadata {
            perms: short_perms,
            size: parts[4].to_string(), 
            name,
            is_dir,
        })
    }

    fn parse_ps_line(&self, line: &str) -> Option<LineMetadata> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 11 { return None; }

        let user = parts[0];
        if line.contains("modified:") || line.contains("new file:") || line.starts_with("On branch") {
            return None;
        }

        let cpu = parts[2];
        let command = parts[10..].join(" ");

        let is_kernel = command.starts_with('[') && command.ends_with(']');
        let clean_cmd = if is_kernel {
            let base = command.trim_matches(|c| c == '[' || c == ']');
            let normalized = base.split('/').next().unwrap()
                                .split(':').next().unwrap()
                                .split('-').next().unwrap();
            format!("[{}]", normalized)
        } else {
            command.split('/').last().unwrap_or(&command)
                   .split_whitespace().next().unwrap_or(&command).to_string()
        };

        Some(LineMetadata {
            perms: user.to_string(),
            size: cpu.to_string(),
            name: clean_cmd,
            is_dir: is_kernel,
        })
    }

    fn parse_git_line(&self, line: &str) -> Option<LineMetadata> {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('(') || trimmed.contains("files:") { return None; }

        let (state, path) = if line.contains("modified:") {
            ("MODIFIED", trimmed.trim_start_matches("modified:").trim())
        } else if line.contains("new file:") {
            ("NEW", trimmed.trim_start_matches("new file:").trim())
        } else if line.contains("deleted:") {
            ("DELETED", trimmed.trim_start_matches("deleted:").trim())
        } else if line.starts_with("\t") || line.starts_with("    ") {
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
            is_dir: path.contains('/'),
        })
    }

    fn parse_standard_ls(&self, line: &str) -> Vec<LineMetadata> {
        let trimmed = line.trim();
        // Stricter check for standard LS output
        if trimmed.is_empty() || trimmed.starts_with('(') || trimmed.len() > 100 {
            return vec![];
        }

        if line.starts_with('d') || line.starts_with('-') || line.starts_with('l') || line.starts_with("total") {
            return vec![];
        }
        if self.parse_ps_line(line).is_some() || self.parse_git_line(line).is_some() {
            return vec![];
        }

        line.split_whitespace()
            .filter(|&s| !s.is_empty())
            .map(|name| {
                let ext = if name.contains('.') {
                    name.split('.').last().unwrap_or("bin").to_string()
                } else {
                    "dir".to_string()
                };
                LineMetadata {
                    perms: ext,
                    size: "0".to_string(),
                    name: name.to_string(),
                    is_dir: !name.contains('.'),
                }
            })
            .collect()
    }

    pub fn synthesize_line(&mut self, line: &str) -> bool {
        if let Some(meta) = self.parse_git_line(line) {
            let key = format!("GIT:{}:{}", meta.perms, meta.size);
            self.synthesis_buffer.entry(key).or_default().push(meta);
            return true;
        }
        if let Some(meta) = self.parse_ps_line(line) {
            let label = if meta.is_dir { "KERNEL" } else { "PROC" };
            let key = format!("{}:{}", label, meta.name);
            self.synthesis_buffer.entry(key).or_default().push(meta);
            return true;
        }
        if let Some(meta) = self.parse_ls_line(line) {
            let key = format!("{}:{}", if meta.is_dir { "DIR" } else { "FILE" }, meta.perms);
            self.synthesis_buffer.entry(key).or_default().push(meta);
            return true;
        }
        let files = self.parse_standard_ls(line);
        if !files.is_empty() {
            for meta in files {
                let key = format!("EXT:{}", meta.perms.to_uppercase());
                self.synthesis_buffer.entry(key).or_default().push(meta);
            }
            return true;
        }
        false
    }

    pub fn process_and_check_noise(&mut self, line: &str) -> bool {
        if self.synthesize_line(line) {
            return true;
        }
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
        let s = re_months.replace_all(line, |caps: &regex::Captures| {
            variables.push(caps[0].to_string());
            "<MONTH>"
        });
        let re_time = Regex::new(r"\d{1,2}:\d{2}").unwrap();
        let s = re_time.replace_all(&s, |caps: &regex::Captures| {
            variables.push(caps[0].to_string());
            "<TIME>"
        });
        let re_num = Regex::new(r"\d+").unwrap();
        let s = re_num.replace_all(&s, |caps: &regex::Captures| {
            variables.push(caps[0].to_string());
            "<NUM>"
        });
        (s.to_string(), variables)
    }

    pub fn flush_variable_summary(&mut self) -> Vec<String> {
        let mut summaries = Vec::new();
        let mut keys: Vec<_> = self.synthesis_buffer.keys().cloned().collect();
        keys.sort();

        for key in keys {
            if let Some(items) = self.synthesis_buffer.remove(&key) {
                let parts: Vec<&str> = key.split(':').collect();
                let label = parts[0];

                let summary = match label {
                    "GIT" => {
                        let state = parts[1];
                        let folder = parts[2];
                        let names: Vec<String> = items.iter().map(|m| m.name.clone()).collect();
                        format!("Git {}: {} files in [{}] | {}", state, items.len(), folder, names.join(", "))
                    },
                    "PROC" => {
                        let cmd = parts[1];
                        let users: std::collections::HashSet<_> = items.iter().map(|m| &m.perms).collect();
                        let mut user_list: Vec<_> = users.into_iter().cloned().collect();
                        user_list.sort();
                        format!("Active processes: {} (count: {}) | Owners: {}", cmd, items.len(), user_list.join(", "))
                    },
                    "KERNEL" => format!("Kernel Workers: {} (count: {})", parts[1], items.len()),
                    "DIR" | "FILE" => {
                        let names: Vec<String> = items.iter().map(|m| m.name.clone()).collect();
                        format!("{} [{}] | {}", label, parts[1], names.join(", "))
                    },
                    "EXT" => {
                        let names: Vec<String> = items.iter().map(|m| m.name.clone()).collect();
                        format!("Grouped {} files by extension [{}] | {}", items.len(), parts[1], names.join(", "))
                    },
                    _ => format!("Summary for {}: {} items", label, items.len())
                };
                summaries.push(summary);
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
