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
            Regex::new(r"AKIA[0-9A-Z]{16}").unwrap(), // AWS Access Key // axiom-scan:ignore
            Regex::new(r"gh[pousr]_[A-Za-z0-9_]{36,255}").unwrap(), // GitHub Tokens // axiom-scan:ignore
            Regex::new(r"eyJ[a-zA-Z0-9_-]+\.[a-zA-Z0-9_-]+\.[a-zA-Z0-9_-]+").unwrap(), // JWT // axiom-scan:ignore
            Regex::new(r"\bsk-ant-api03-[a-zA-Z0-9_-]{20,}\b").unwrap(), // Anthropic // axiom-scan:ignore
            Regex::new(r"\bgsk_[a-zA-Z0-9]{32,}\b").unwrap(), // Groq // axiom-scan:ignore
            Regex::new(r"\b(?:sk|pk)_(?:test|live)_[0-9a-zA-Z]{10,}\b").unwrap(), // Stripe // axiom-scan:ignore
            Regex::new(r"\bya29\.[a-zA-Z0-9_-]{20,}\b").unwrap(), // Google OAuth // axiom-scan:ignore
            Regex::new(r"\bsk-proj-[a-zA-Z0-9_-]{20,}\b").unwrap(), // OpenAI Project // axiom-scan:ignore
            Regex::new(r"\bsk-[a-zA-Z0-9]{48}\b").unwrap(), // OpenAI Legacy // axiom-scan:ignore
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
        Self::new(4.5, vec![
            r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}".to_string(), // Email
            r"\b\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}\b".to_string(), // IP Address
        ])
    }
}

impl PrivacyRedactor {
    pub fn redact<'a>(&self, input: &'a str) -> std::borrow::Cow<'a, str> {
        let mut current = std::borrow::Cow::Borrowed(input);

        // 1. Exact Pattern Redaction (High Confidence Secrets)
        for pattern in &self.secret_patterns {
            if let std::borrow::Cow::Owned(s) = pattern.replace_all(&current, "[REDACTED_SECRET]") {
                current = std::borrow::Cow::Owned(s);
            }
        }

        // 2. Regex Redaction (PII)
        for pattern in &self.pii_patterns {
            if let std::borrow::Cow::Owned(s) = pattern.replace_all(&current, "[REDACTED_PII]") {
                current = std::borrow::Cow::Owned(s);
            }
        }

        // 3. Entropy Redaction (Generic Secrets fallback)
        if let std::borrow::Cow::Owned(s) = self.word_regex.replace_all(&current, |caps: &regex::Captures| {
            let word = &caps[0];
            
            // Skip entropy redaction for common hex strings (like Git SHAs or Docker IDs)
            // to reduce false positives. Git SHAs are 40 chars, Docker IDs are 64 chars.
            let is_hex_hash = (word.len() == 40 || word.len() == 64)
                && word.chars().all(|c| c.is_ascii_hexdigit());

            // Only check entropy for words longer than 15 chars that aren't already redacted or hex hashes
            if !is_hex_hash && word.len() > 15 && !word.starts_with("REDACTED") {
                if calculate_entropy(word) > self.entropy_threshold {
                    return "[REDACTED_SECRET]".to_string();
                }
            }
            word.to_string()
        }) {
            current = std::borrow::Cow::Owned(s);
        }

        current
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
