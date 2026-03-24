use regex::Regex;
use crate::privacy::entropy::calculate_entropy;

pub struct PrivacyRedactor {
    entropy_threshold: f64,
    pii_patterns: Vec<Regex>,
    word_regex: Regex,
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
            word_regex: Regex::new(r"\S+").unwrap(),
        }
    }
}

impl Default for PrivacyRedactor {
    fn default() -> Self {
        Self::new(4.5, vec![
            r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}".to_string(), // Email
            r"\b\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}\b".to_string(),         // IP Address
            r"\b(?:AKIA|ABIA|ACCA|ASIA)[A-Z0-9]{16}\b".to_string(),        // AWS
            r"\bsk-ant-api03-[a-zA-Z0-9_-]{20,}\b".to_string(),            // Anthropic
            r"\bgsk_[a-zA-Z0-9]{32,}\b".to_string(),                       // Groq
            r"\b(?:sk|pk)_(?:test|live)_[0-9a-zA-Z]{10,}\b".to_string(),   // Stripe
            r"\bya29\.[a-zA-Z0-9_-]{20,}\b".to_string(),                   // Google OAuth
            r"\bsk-proj-[a-zA-Z0-9_-]{20,}\b".to_string(),                 // OpenAI Project
            r"\bsk-[a-zA-Z0-9]{48}\b".to_string(),                         // OpenAI Legacy
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

        // 2. Entropy Redaction (Secrets) - Optimized single-pass
        // We use Cow::Owned to avoid lifetime issues with the captures reference
        self.word_regex.replace_all(&output, |caps: &regex::Captures| {
            let word = &caps[0];

            // Skip entropy redaction for common hex strings (like Git SHAs or Docker IDs)
            // to reduce false positives. Git SHAs are 40 chars, Docker IDs are 64 chars.
            let is_hex_hash = (word.len() == 40 || word.len() == 64)
                && word.chars().all(|c| c.is_ascii_hexdigit());

            if !is_hex_hash && word.len() > 10 && calculate_entropy(word) > self.entropy_threshold {
                std::borrow::Cow::Owned("[REDACTED_SECRET]".to_string())
            } else {
                std::borrow::Cow::Owned(word.to_string())
            }
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
