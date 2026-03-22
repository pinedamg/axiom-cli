use std::collections::HashMap;
use regex::Regex;

pub struct DiscoveryEngine {
    /// Map of Template -> Frequency of appearance
    pub templates: HashMap<String, usize>,
    /// Buffer for variables captured from collapsed lines: Template -> Vec<VariableList>
    pub variable_buffer: HashMap<String, Vec<Vec<String>>>,
    pub threshold: usize,
}

impl Default for DiscoveryEngine {
    fn default() -> Self {
        Self {
            templates: HashMap::new(),
            variable_buffer: HashMap::new(),
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
        let mut variables = Vec::new();
        
        // 1. Handle UUIDs
        let re_uuid = Regex::new(r"[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}").unwrap();
        let s = re_uuid.replace_all(line, |caps: &regex::Captures| {
            variables.push(caps[0].to_string());
            "<UUID>"
        });

        // 2. Handle File Paths (Basic detection)
        let re_path = Regex::new(r"(/[a-zA-Z0-9\._-]+)+").unwrap();
        let s = re_path.replace_all(&s, |caps: &regex::Captures| {
            variables.push(caps[0].to_string());
            "<PATH>"
        });

        // 3. Handle Timestamps
        let re_time = Regex::new(r"\d{2}:\d{2}:\d{2}").unwrap();
        let s = re_time.replace_all(&s, |caps: &regex::Captures| {
            variables.push(caps[0].to_string());
            "<TIME>"
        });

        // 4. Handle Hashes/Hexadecimal
        let re_hex = Regex::new(r"0x[0-9a-fA-F]+|[0-9a-f]{8,}").unwrap();
        let s = re_hex.replace_all(&s, |caps: &regex::Captures| {
            variables.push(caps[0].to_string());
            "<HEX>"
        });
        
        // 5. Handle remaining numbers
        let re_num = Regex::new(r"\d+").unwrap();
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
        let mut summaries = Vec::new();
        
        for (template, var_sets) in self.variable_buffer.drain() {
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
