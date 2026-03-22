pub mod discovery;

use crate::privacy::PrivacyRedactor;
use crate::schema::{ToolSchema, Action};
use crate::IntentContext;
use crate::engine::discovery::DiscoveryEngine;

pub struct AxiomEngine {
    pub redactor: PrivacyRedactor,
    pub schemas: Vec<ToolSchema>,
    pub discovery: DiscoveryEngine,
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

    /// Flushes all buffered variables into a list of human-readable summaries
    pub fn flush_summaries(&mut self) -> Vec<String> {
        self.discovery.flush_variable_summary()
    }

    /// Processes an output line. Uses pre-compiled Regex for maximum efficiency.
    pub fn process_line(&mut self, line: &str, command: &str, context: &IntentContext) -> Option<String> {
        // 1. Privacy Redaction
        let redacted = self.redactor.redact(line);

        // 2. Intent Priority (Force show if relevant)
        if context.is_relevant(&redacted) {
            return Some(redacted);
        }

        // 3. Apply static schemas
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
            // 4. Auto-discovery (Phase 3 + 3.4 Aggregation)
            if self.discovery.process_and_check_noise(&redacted) {
                return None;
            }
        }

        Some(redacted)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::TransformationRule;

    #[test]
    fn test_engine_auto_discovery_with_summary() {
        let redactor = PrivacyRedactor::default();
        let mut engine = AxiomEngine::new(redactor, vec![]);
        let command = "unknown-tool";
        let context = IntentContext {
            last_message: "Run tool".to_string(),
            command: command.to_string(),
            keywords: vec!["run".to_string()],
        };

        for i in 0..10 {
            let line = format!("Event #{} occurred", i);
            engine.process_line(&line, command, &context);
        }
        
        let summaries = engine.flush_summaries();
        assert!(!summaries.is_empty());
        assert!(summaries[0].contains("Event #<NUM> occurred"));
    }

    #[test]
    fn test_engine_intent_aware_overriding() {
        let mut schema = ToolSchema {
            name: "npm".to_string(),
            command_pattern: "npm.*install".to_string(),
            rules: vec![
                TransformationRule {
                    name: "collapse_plus".to_string(),
                    pattern: r"^\+ [a-z]+".to_string(),
                    action: Action::Collapse,
                    priority: 1,
                    compiled_re: None,
                }
            ],
            compiled_command_re: None,
        };
        schema.compile().unwrap();

        let redactor = PrivacyRedactor::default();
        let mut engine = AxiomEngine::new(redactor, vec![schema]);
        let line = "+ lodash";
        let command = "npm install lodash";

        // Scenario A: User DOES NOT ask about packages -> Collapse
        let context_a = IntentContext {
            last_message: "Fix some bug".to_string(),
            command: command.to_string(),
            keywords: vec!["bug".to_string()],
        };
        assert_eq!(engine.process_line(line, command, &context_a), None);

        // Scenario B: User asks about the package -> Show (Intent Override)
        let context_b = IntentContext {
            last_message: "What version of lodash did you install?".to_string(),
            command: command.to_string(),
            keywords: vec!["version".to_string()],
        };
        assert!(engine.process_line(line, command, &context_b).is_some());
    }
}
