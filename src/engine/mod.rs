pub mod discovery;
pub mod intent_discovery;
pub mod plugins;
pub mod intelligence;
pub mod telemetry;
pub mod transformer;

use crate::privacy::PrivacyRedactor;
use crate::schema::{ToolSchema, Action};
use crate::IntentContext;
use crate::engine::discovery::DiscoveryEngine;
use crate::engine::plugins::WasmPluginManager;
use crate::engine::intelligence::IntelligenceProvider;
use crate::engine::transformer::ContentTransformer;

pub struct AxiomEngine {
    pub redactor: PrivacyRedactor,
    pub schemas: Vec<ToolSchema>,
    pub discovery: DiscoveryEngine,
    pub plugins: Option<WasmPluginManager>,
    pub intelligence: Box<dyn IntelligenceProvider>,
    pub markdown_mode: bool,
    line_counter: usize,
}

impl AxiomEngine {
    pub fn new(
        redactor: PrivacyRedactor, 
        schemas: Vec<ToolSchema>,
        intelligence: Box<dyn IntelligenceProvider>
    ) -> Self {
        Self {
            redactor,
            schemas,
            discovery: DiscoveryEngine::default(),
            plugins: None,
            intelligence,
            markdown_mode: false,
            line_counter: 0,
        }
    }

    pub fn with_plugins(mut self, manager: WasmPluginManager) -> Self {
        self.plugins = Some(manager);
        self
    }

    pub fn with_semantic(mut self, engine: Box<dyn IntelligenceProvider>) -> Self {
        self.intelligence = engine;
        self
    }

    pub fn set_markdown_mode(&mut self, enabled: bool) {
        self.markdown_mode = enabled;
    }

    pub fn load_learned_templates(&mut self, templates: Vec<(String, usize)>) {
        self.discovery.load_templates(templates);
    }

    pub fn get_learned_templates(&self) -> Vec<(String, usize)> {
        self.discovery.templates.iter().map(|(k, v)| (k.clone(), *v)).collect()
    }

    pub fn flush_summaries(&mut self) -> Vec<String> {
        self.discovery.flush_variable_summary()
    }

    /// The main pipeline orchestrator. Adheres to a strict stage-based flow.
    pub fn process_line(&mut self, line: &str, command: &str, context: &IntentContext) -> Option<String> {
        self.line_counter += 1;

        // Stage 1: Structural Pre-processing (Markdown tables)
        let working_line = self.apply_structural_transform(line);

        // Stage 2: Resource Guarding (Preventing buffer overflows/token burns)
        if let Some(guard_result) = self.apply_resource_guard(&working_line, command, context) {
            return guard_result;
        }

        // Stage 3: Security & Privacy (Mandatory redaction)
        let redacted = self.redactor.redact(&working_line);

        // Stage 4: Semantic Relevance (Intent Priority Overriding)
        if self.is_semantically_relevant(&redacted, context) {
            return Some(redacted);
        }

        // Stage 5: Pattern-based Compression (YAML Schemas & Auto-Discovery)
        let final_line = self.apply_compression(&redacted, command);

        // Stage 6: External Logic (WASM Plugins)
        self.apply_plugins(final_line)
    }

    // --- Private Stage Helpers (Encapsulation) ---

    fn apply_structural_transform(&self, line: &str) -> String {
        if self.markdown_mode && ContentTransformer::looks_like_table(line) {
            ContentTransformer::to_markdown(line)
        } else {
            line.to_string()
        }
    }

    fn apply_resource_guard(&mut self, line: &str, command: &str, context: &IntentContext) -> Option<Option<String>> {
        if ContentTransformer::should_guard(command, self.line_counter, context) {
            if self.line_counter == 101 {
                return Some(Some("[AXIOM] (Guardian Mode: File too long. Summary follows...)".to_string()));
            }
            if self.discovery.process_and_check_noise(line) {
                return Some(None);
            }
        }
        None
    }

    fn is_semantically_relevant(&mut self, line: &str, context: &IntentContext) -> bool {
        context.is_relevant(line) || self.intelligence.is_relevant(&context.last_message, line, 0.7)
    }

    fn apply_compression(&mut self, line: &str, command: &str) -> Option<String> {
        if let Some(schema) = self.schemas.iter().find(|s| s.matches(command)) {
            if let Some(action) = schema.apply_rules(line) {
                return match action {
                    Action::Keep => Some(line.to_string()),
                    Action::Collapse => {
                        self.discovery.process_and_check_noise(line);
                        None
                    },
                    Action::Redact => Some("[REDACTED_BY_SCHEMA]".to_string()),
                    Action::Hidden => None,
                };
            }
        }
        
        // Fallback to auto-discovery
        if self.discovery.process_and_check_noise(line) {
            None
        } else {
            Some(line.to_string())
        }
    }

    fn apply_plugins(&mut self, line: Option<String>) -> Option<String> {
        match (line, &mut self.plugins) {
            (Some(mut current), Some(manager)) => {
                current = manager.transform(&current);
                if current.is_empty() { None } else { Some(current) }
            }
            (l, _) => l,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::TransformationRule;
    use crate::engine::intelligence::FuzzyIntelligence;

    #[test]
    fn test_markdown_table_conversion() {
        let redactor = PrivacyRedactor::default();
        let mut engine = AxiomEngine::new(redactor, vec![], Box::new(FuzzyIntelligence));
        engine.set_markdown_mode(true);
        
        let table_line = "ID        NAME      STATUS    AGE";
        let result = engine.process_line(table_line, "ps", &IntentContext {
            last_message: "show table".to_string(),
            command: "ps".to_string(),
            keywords: vec![],
        });
        
        assert!(result.is_some());
        assert!(result.unwrap().contains("|"));
    }

    #[test]
    fn test_engine_auto_discovery_with_summary() {
        let redactor = PrivacyRedactor::default();
        let mut engine = AxiomEngine::new(redactor, vec![], Box::new(FuzzyIntelligence));
        let command = "unknown-tool";
        let context = IntentContext {
            last_message: "Run tool".to_string(),
            command: command.to_string(),
            keywords: vec![],
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
        let mut engine = AxiomEngine::new(redactor, vec![schema], Box::new(FuzzyIntelligence));
        let line = "+ lodash";
        let command = "npm install lodash";

        let context_a = IntentContext {
            last_message: "Fix bug".to_string(),
            command: command.to_string(),
            keywords: vec![],
        };
        assert_eq!(engine.process_line(line, command, &context_a), None);

        let context_b = IntentContext {
            last_message: "lodash version".to_string(),
            command: command.to_string(),
            keywords: vec!["lodash".to_string()],
        };
        assert!(engine.process_line(line, command, &context_b).is_some());
    }
}
