use std::collections::BTreeMap;
use std::sync::OnceLock;
use regex::Regex;

pub struct DiscoveryEngine {
    /// Map of Template -> Frequency of appearance
    pub templates: BTreeMap<String, usize>,
    /// Buffer for variables captured from collapsed lines: Template -> Vec<VariableList>
    pub variable_buffer: BTreeMap<String, Vec<Vec<String>>>,
    pub threshold: usize,
}

impl Default for DiscoveryEngine {
    fn default() -> Self {
        Self {
            templates: BTreeMap::new(),
            variable_buffer: BTreeMap::new(),
            threshold: 5,
        }
    }
}

impl DiscoveryEngine {
    /// Load previously learned templates (Persistence)
    pub fn load_templates(&mut self, known: Vec<(String, usize)>) {
        for (template, freq) in known {
            self.templates.insert(template, freq);
        }
    }

    /// Extracts the structural "Skeleton" and the dynamic "Variables" from a line
    pub fn extract_parts(&self, line: &str) -> (String, Vec<String>) {
        // Bolt: Vec::with_capacity(4) is used here because lines typically have fewer than 4 variables
        // pre-allocation reduces heap re-allocation during hot paths.
        let mut variables = Vec::with_capacity(4);
        
        static RE_UUID: OnceLock<Regex> = OnceLock::new();
        static RE_PATH: OnceLock<Regex> = OnceLock::new();
        static RE_TIME: OnceLock<Regex> = OnceLock::new();
        static RE_HEX: OnceLock<Regex> = OnceLock::new();
        static RE_NUM: OnceLock<Regex> = OnceLock::new();

        // 1. Handle UUIDs
        // Bolt: Using statically initialized Regexes prevents compiling them on every single line processed
        // drastically reducing memory and CPU overhead.
        let re_uuid = RE_UUID.get_or_init(|| Regex::new(r"[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}").unwrap());
        let s = re_uuid.replace_all(line, |caps: &regex::Captures| {
            variables.push(caps[0].to_string());
            "<UUID>"
        });

        // 2. Handle File Paths (Basic detection)
        let re_path = RE_PATH.get_or_init(|| Regex::new(r"(/[a-zA-Z0-9\._-]+)+").unwrap());
        let s = re_path.replace_all(&s, |caps: &regex::Captures| {
            variables.push(caps[0].to_string());
            "<PATH>"
        });

        // 3. Handle Timestamps
        let re_time = RE_TIME.get_or_init(|| Regex::new(r"\d{2}:\d{2}:\d{2}").unwrap());
        let s = re_time.replace_all(&s, |caps: &regex::Captures| {
            variables.push(caps[0].to_string());
            "<TIME>"
        });

        // 4. Handle Hashes/Hexadecimal
        let re_hex = RE_HEX.get_or_init(|| Regex::new(r"0x[0-9a-fA-F]+|[0-9a-f]{8,}").unwrap());
        let s = re_hex.replace_all(&s, |caps: &regex::Captures| {
            variables.push(caps[0].to_string());
            "<HEX>"
        });
        
        // 5. Handle remaining numbers
        let re_num = RE_NUM.get_or_init(|| Regex::new(r"\d+").unwrap());
        let s = re_num.replace_all(&s, |caps: &regex::Captures| {
            variables.push(caps[0].to_string());
            "<NUM>"
        });

        (s.to_string(), variables)
    }

    /// Records a line, captures variables if it's noise, and returns if it should be collapsed
    pub fn process_and_check_noise(&mut self, line: &str) -> bool {
        let (template, vars) = self.extract_parts(line);
        let count = self.templates.entry(template.clone()).or_insert(0);
        *count += 1;
        
        let is_noise = *count > self.threshold;
        
        if is_noise {
            // Store variables for the summary
            self.variable_buffer.entry(template).or_default().push(vars);
        }
        
        is_noise
    }

    /// Generates a summary of all buffered variables and clears the buffer
    pub fn flush_variable_summary(&mut self) -> Vec<String> {
        let mut summaries = Vec::with_capacity(self.variable_buffer.len());
        
        // BTreeMap doesn't have .drain() that returns an iterator in stable Rust in the same way,
        // so we swap it with a new empty map and iterate over the old one.
        let mut buffer_to_drain = BTreeMap::new();
        std::mem::swap(&mut self.variable_buffer, &mut buffer_to_drain);

        for (template, var_sets) in buffer_to_drain.into_iter() {
            if var_sets.is_empty() { continue; }
            
            // Collect unique variables across all sets to avoid spamming
            let mut unique_vars = std::collections::HashSet::new();
            for set in &var_sets {
                for var in set {
                    unique_vars.insert(var.clone());
                }
            }
            
            let var_list: Vec<String> = unique_vars.into_iter().collect();
            let summary = if var_list.len() > 10 {
                format!("Template '{}' matched {} more times. Sample variables: [{}, ...]", 
                    template, var_sets.len(), var_list[..5].join(", "))
            } else {
                format!("Template '{}' matched {} more times. Variables: [{}]", 
                    template, var_sets.len(), var_list.join(", "))
            };
            summaries.push(summary);
        }
        
        summaries
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variable_extraction() {
        let engine = DiscoveryEngine::default();
        let (template, vars) = engine.extract_parts("Task #1: Processing item 0xABC... [Done]");
        
        assert_eq!(template, "Task #<NUM>: Processing item <HEX>... [Done]");
        assert!(vars.contains(&"1".to_string()));
        assert!(vars.contains(&"0xABC".to_string()));
    }

    #[test]
    fn test_noise_and_summary() {
        let mut engine = DiscoveryEngine::default();
        
        for i in 0..10 {
            let line = format!("Log event at 10:00:0{} with ID 0x12{}", i, i);
            engine.process_and_check_noise(&line);
        }
        
        let summary = engine.flush_variable_summary();
        assert!(!summary.is_empty());
        assert!(summary[0].contains("matched 5 more times"));
    }
}
