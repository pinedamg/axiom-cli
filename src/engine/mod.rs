pub mod discovery;

use crate::privacy::PrivacyRedactor;
use crate::schema::{ToolSchema, Action};
use crate::IntentContext;
use crate::engine::discovery::DiscoveryEngine;

pub struct AxiomEngine {
    redactor: PrivacyRedactor,
    schemas: Vec<ToolSchema>,
    discovery: DiscoveryEngine,
}

impl AxiomEngine {
    pub fn new(redactor: PrivacyRedactor, schemas: Vec<ToolSchema>) -> Self {
        Self {
            redactor,
            schemas,
            discovery: DiscoveryEngine::default(),
        }
    }

    pub fn load_learned_templates(&mut self, templates: Vec<(String, usize)>) {
        self.discovery.load_templates(templates);
    }

    pub fn get_learned_templates(&self) -> Vec<(String, usize)> {
        self.discovery.templates.clone().into_iter().collect()
    }

    /// Processes an output line. Uses pre-compiled Regex for maximum efficiency.
    pub fn process_line(&mut self, line: &str, command: &str, context: &IntentContext) -> Option<String> {
        // 1. Security redaction
        let redacted = self.redactor.redact(line);

        // 2. Intent Priority
        if context.is_relevant(&redacted) {
            return Some(redacted);
        }

        // 3. Schema application (with already compiled Regex)
        if let Some(schema) = self.schemas.iter().find(|s| s.matches(command)) {
            for rule in &schema.rules {
                if let Some(re) = &rule.compiled_re {
                    if re.is_match(&redacted) {
                        match rule.action {
                            Action::Keep => return Some(redacted),
                            Action::Collapse => return None,
                            Action::Redact => return Some("[REDACTED_BY_SCHEMA]".to_string()),
                            Action::Hidden => return None,
                        }
                    }
                }
            }
        } else {
            // 4. Auto-discovery
            if self.discovery.is_repetitive_noise(&redacted) {
                return None;
            }
        }

        Some(redacted)
    }
}
