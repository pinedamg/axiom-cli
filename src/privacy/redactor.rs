use regex::Regex;
use crate::privacy::entropy::calculate_entropy;

pub struct PrivacyRedactor {
    entropy_threshold: f64,
    pii_patterns: Vec<Regex>,
}

impl PrivacyRedactor {
    pub fn new(entropy_threshold: f64, pii_patterns: Vec<String>) -> Self {
        let compiled_patterns = pii_patterns
            .into_iter()
            .filter_map(|p| Regex::new(&p).ok())
            .collect();

        Self {
            entropy_threshold,
            pii_patterns: compiled_patterns,
        }
    }
}

impl Default for PrivacyRedactor {
    fn default() -> Self {
        Self::new(4.5, vec![
            r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}".to_string(),
            r"\b\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}\b".to_string(),
        ])
    }
}

impl PrivacyRedactor {
    pub fn redact(&self, input: &str) -> String {
        let mut output = input.to_string();

        // 1. Regex Redaction (PII)
        for pattern in &self.pii_patterns {
            output = pattern.replace_all(&output, "[REDACTED_PII]").to_string();
        }

        // 2. Entropy Redaction (Secrets)
        let words: Vec<&str> = output.split_whitespace().collect();
        let mut final_output = output.clone();

        for word in words {
            if word.len() > 10 && calculate_entropy(word) > self.entropy_threshold {
                final_output = final_output.replace(word, "[REDACTED_SECRET]");
            }
        }

        final_output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_redaction_pii() {
        let redactor = PrivacyRedactor::default();
        let input = "Contact me at dev@axiom.ai or visit 192.168.1.1";
        let redacted = redactor.redact(input);
        
        assert!(redacted.contains("[REDACTED_PII]"));
        assert!(!redacted.contains("dev@axiom.ai"));
        assert!(!redacted.contains("192.168.1.1"));
    }

    #[test]
    fn test_redaction_secret() {
        let redactor = PrivacyRedactor::default();
        let input = "The AWS key is AKIA5G4H3J2K1L0M9N8P7Q6R5S4T3U2V1W0X";
        let redacted = redactor.redact(input);
        
        assert!(redacted.contains("[REDACTED_SECRET]"));
        assert!(!redacted.contains("AKIA5G4H3J2K1L0M9N8P7Q6R5S4T3U2V1W0X"));
    }
}
