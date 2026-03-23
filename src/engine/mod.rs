pub mod discovery;
pub mod intent_discovery;
pub mod plugins;
pub mod intelligence;
pub mod telemetry;

use crate::privacy::PrivacyRedactor;
use crate::schema::{ToolSchema, Action};
use crate::IntentContext;
use crate::engine::discovery::DiscoveryEngine;
use crate::engine::plugins::WasmPluginManager;
use crate::engine::intelligence::IntelligenceProvider;

pub struct AxiomEngine {
    pub redactor: PrivacyRedactor,
    pub schemas: Vec<ToolSchema>,
    pub discovery: DiscoveryEngine,
    pub plugins: Option<WasmPluginManager>,
    pub intelligence: Box<dyn IntelligenceProvider>,
    pub markdown_mode: bool,
    /// Counter for volume control
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

    /// Flushes all buffered variables into a list of human-readable summaries
    pub fn flush_summaries(&mut self) -> Vec<String> {
        self.discovery.flush_variable_summary()
    }

    fn looks_like_table(&self, line: &str) -> bool {
        let line = line.trim();
        if line.len() < 10 { return false; }
        
        // Heuristic: Check for multiple blocks of text separated by 2+ spaces
        // This is typical for terminal tables (docker, ps, kubectl)
        let parts: Vec<&str> = line.split("  ").filter(|s| !s.trim().is_empty()).collect();
        parts.len() >= 3
    }

    fn convert_to_markdown(&self, line: &str) -> String {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 2 { return line.to_string(); }
        
        format!("| {} |", parts.join(" | "))
    }

    /// Processes an output line.
    pub fn process_line(&mut self, line: &str, command: &str, context: &IntentContext) -> Option<String> {
        self.line_counter += 1;

        // 1. Markdown Transformation (Pre-processing)
        let working_line = if self.markdown_mode && self.looks_like_table(line) {
            self.convert_to_markdown(line)
        } else {
            line.to_string()
        };

        // 2. Volume Guardian
        if command.starts_with("cat") && self.line_counter > 100 && !context.is_relevant("full file") {
            if self.line_counter == 101 {
                return Some("[AXIOM] (Guardian Mode: File too long. Summary follows...)".to_string());
            }
            if self.discovery.process_and_check_noise(&working_line) {
                return None;
            }
        }

        // 3. Privacy Redaction
        let redacted = self.redactor.redact(&working_line);

        // 4. Intent Priority
        let mut is_relevant = context.is_relevant(&redacted);
        
        if !is_relevant {
            is_relevant = self.intelligence.is_relevant(&context.last_message, &redacted, 0.7);
        }

        if is_relevant {
            return Some(redacted);
        }

        // 5. Apply static schemas
        let mut final_line = Some(redacted.clone());
        if let Some(schema) = self.schemas.iter().find(|s| s.matches(command)) {
            for rule in &schema.rules {
                if let Some(re) = &rule.compiled_re {
                    if re.is_match(&redacted) {
                        match rule.action {
                            Action::Keep => { final_line = Some(redacted.clone()); break; },
                            Action::Collapse => { 
                                self.discovery.process_and_check_noise(&redacted);
                                final_line = None; 
                                break; 
                            },
                            Action::Redact => { final_line = Some("[REDACTED_BY_SCHEMA]".to_string()); break; },
                            Action::Hidden => { final_line = None; break; },
                        }
                    }
                }
            }
        } else {
            // 6. Auto-discovery
            if self.discovery.process_and_check_noise(&redacted) {
                final_line = None;
            }
        }

        // 7. Apply WASM Plugins
        if let Some(mut current_line) = final_line {
            if let Some(plugin_manager) = &mut self.plugins {
                current_line = plugin_manager.transform(&current_line);
                if current_line.is_empty() {
                    return None;
                }
                return Some(current_line);
            }
            return Some(current_line);
        }

        None
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
        assert_eq!(result.unwrap(), "| ID | NAME | STATUS | AGE |");
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
    fn test_engine_with_no_plugins() {
        let redactor = PrivacyRedactor::default();
        let mut engine = AxiomEngine::new(redactor, vec![], Box::new(FuzzyIntelligence));
        let command = "ls";
        let context = IntentContext {
            last_message: "list files".to_string(),
            command: command.to_string(),
            keywords: vec![],
        };

        let line = "file.txt";
        let result = engine.process_line(line, command, &context);
        assert_eq!(result, Some("file.txt".to_string()));
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

        // Scenario A: User DOES NOT ask about packages -> Collapse
        let context_a = IntentContext {
            last_message: "Fix some bug".to_string(),
            command: command.to_string(),
            keywords: vec![],
        };
        assert_eq!(engine.process_line(line, command, &context_a), None);

        // Scenario B: User asks about the package -> Show (Intent Override)
        let context_b = IntentContext {
            last_message: "What version of lodash did you install?".to_string(),
            command: command.to_string(),
            keywords: vec!["lodash".to_string()],
        };
        assert!(engine.process_line(line, command, &context_b).is_some());
    }
}
