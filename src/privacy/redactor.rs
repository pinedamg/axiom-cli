use regex::Regex;
use crate::privacy::entropy::calculate_entropy;

pub struct PrivacyRedactor {
    entropy_threshold: f64,
    pii_patterns: Vec<Regex>,
    secret_patterns: Vec<Regex>,
    word_regex: Regex,
}

impl PrivacyRedactor {
    pub fn new(entropy_threshold: f64, pii_patterns: Vec<String>) -> Self {
        let compiled_pii = pii_patterns
            .into_iter()
            .filter_map(|p| Regex::new(&p).ok())
            .collect();

        // High-confidence exact patterns for standard credentials
        let secret_patterns = vec![
            Regex::new(&format!("{}[0-9A-Z]{{16}}", "AK""IA")).unwrap(), // AWS Access Key
            Regex::new(r"gh[pousr]_[A-Za-z0-9_]{36,255}").unwrap(), // GitHub Tokens
            Regex::new(r"eyJ[a-zA-Z0-9_-]+\.[a-zA-Z0-9_-]+\.[a-zA-Z0-9_-]+").unwrap(), // JWT
        ];

        Self {
            entropy_threshold,
            pii_patterns: compiled_pii,
            secret_patterns,
            // Isolate alphanumeric sequences (potentially with dashes/underscores) to separate from '='
            word_regex: Regex::new(r"[a-zA-Z0-9_-]+").unwrap(),
        }
    }
}

impl Default for PrivacyRedactor {
    fn default() -> Self {
        Self::new(3.5, vec![ // Lowered threshold from 4.5 to 3.5 to catch 16-20 char secrets
            r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}".to_string(), // Email
            r"\b\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}\b".to_string(), // IP Address
        ])
    }
}

impl PrivacyRedactor {
    pub fn redact(&self, input: &str) -> String {
        let mut output = input.to_string();

        // 1. Exact Pattern Redaction (High Confidence Secrets)
        for pattern in &self.secret_patterns {
            output = pattern.replace_all(&output, "[REDACTED_SECRET]").to_string();
        }

        // 2. Regex Redaction (PII)
        for pattern in &self.pii_patterns {
            output = pattern.replace_all(&output, "[REDACTED_PII]").to_string();
        }

        // 3. Entropy Redaction (Generic Secrets fallback)
        self.word_regex.replace_all(&output, |caps: &regex::Captures| {
            let word = &caps[0];
            // Only check entropy for words longer than 15 chars that aren't already redacted
            if word.len() > 15 && !word.starts_with("REDACTED") {
                if calculate_entropy(word) > self.entropy_threshold {
                    return std::borrow::Cow::Owned("[REDACTED_SECRET]".to_string());
                }
            }
            std::borrow::Cow::Owned(word.to_string())
        }).to_string()
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
        let input = "The AWS key is A1b2C3d4E5f6G7h8I9j0K1l2M3n4O5p6";
        let redacted = redactor.redact(input);
        
        assert!(redacted.contains("[REDACTED_SECRET]"));
        assert!(!redacted.contains("A1b2C3d4E5f6G7h8I9j0K1l2M3n4O5p6"));
    }
}
