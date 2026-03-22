use std::collections::HashMap;
use regex::Regex;

pub struct DiscoveryEngine {
    /// Map of Template -> Frequency of occurrence
    pub templates: HashMap<String, usize>,
    pub threshold: usize,
}

impl Default for DiscoveryEngine {
    fn default() -> Self {
        Self {
            templates: HashMap::new(),
            threshold: 5, // After 5 times, it is considered a repetitive/noisy pattern
        }
    }
}

impl DiscoveryEngine {
    /// Loads previously learned templates (Persistence)
    pub fn load_templates(&mut self, known: Vec<(String, usize)>) {
        for (template, freq) in known {
            self.templates.insert(template, freq);
        }
    }

    /// Cleans a line to extract its structural "Skeleton"
    pub fn extract_template(&self, line: &str) -> String {
        // 1. Replace timestamps (High specificity)
        let re_time = Regex::new(r"\d{2}:\d{2}:\d{2}").unwrap();
        let s = re_time.replace_all(line, "<TIME>");

        // 2. Replace hashes/hexadecimal
        let re_hex = Regex::new(r"0x[0-9a-fA-F]+|[0-9a-f]{8,}").unwrap();
        let s = re_hex.replace_all(&s, "<HEX>");
        
        // 3. Replace remaining numbers (Low specificity)
        let re_num = Regex::new(r"\d+").unwrap();
        let s = re_num.replace_all(&s, "<NUM>");

        s.to_string()
    }

    /// Registers a line and determines if it is already a repetitive pattern (noise)
    pub fn is_repetitive_noise(&mut self, line: &str) -> bool {
        let template = self.extract_template(line);
        let count = self.templates.entry(template).or_insert(0);
        *count += 1;
        
        *count > self.threshold
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template_extraction() {
        let engine = DiscoveryEngine::default();
        let t1 = engine.extract_template("Process 1234 started at 10:00:00");
        let t2 = engine.extract_template("Process 5678 started at 11:22:33");
        
        assert_eq!(t1, t2);
        // Now it should match <TIME>
        assert_eq!(t1, "Process <NUM> started at <TIME>");
    }

    #[test]
    fn test_noise_detection() {
        let mut engine = DiscoveryEngine::default();
        let line = "Loading resource 0xABC123...";
        
        for _ in 0..5 {
            assert!(!engine.is_repetitive_noise(line));
        }
        // By the sixth time, it is already noise
        assert!(engine.is_repetitive_noise(line));
    }
}
