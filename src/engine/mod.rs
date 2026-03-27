pub mod discovery;
pub mod intent_discovery;
pub mod plugins;
pub mod intelligence;
pub mod telemetry;
pub mod transformer;
pub mod commands;

use crate::privacy::PrivacyRedactor;
use crate::schema::{ToolSchema, Action};
use crate::IntentContext;
use crate::engine::discovery::DiscoveryEngine;
use crate::engine::plugins::WasmPluginManager;
use crate::engine::intelligence::IntelligenceProvider;
use crate::engine::transformer::ContentTransformer;
use crate::engine::commands::{CommandHandler, get_all_handlers};

pub struct AxiomEngine {
    pub redactor: PrivacyRedactor,
    pub schemas: Vec<ToolSchema>,
    pub discovery: DiscoveryEngine,
    pub plugins: Option<WasmPluginManager>,
    pub intelligence: Box<dyn IntelligenceProvider>,
    pub handlers: Vec<Box<dyn CommandHandler>>,
    pub markdown_mode: bool,
    pub last_command: String,
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
            handlers: get_all_handlers(),
            markdown_mode: false,
            last_command: String::new(),
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
        let insight = self.generate_semantic_insight();
        let mut summaries = self.discovery.flush_variable_summary(&self.handlers);
        
        if let Some(text) = insight {
            summaries.insert(0, format!("Semantic Insight: {}", text));
        }

        summaries
    }

    fn generate_semantic_insight(&self) -> Option<String> {
        if let Some(handler) = self.handlers.iter().find(|h| h.matches(&self.last_command)) {
            if let Some(insight) = handler.generate_insight(&self.last_command, &self.discovery.synthesis_buffer) {
                return Some(insight);
            }
        }
        None
    }

    /// Pre-calculates session data (like embeddings) to improve per-line latency
    pub fn prepare_session(&mut self, intent: &str) -> anyhow::Result<()> {
        self.intelligence.pre_compute_intent(intent)
    }

    /// The main pipeline orchestrator. Adheres to a strict stage-based flow.
    pub fn process_line(&mut self, line: &str, command: &str, context: &IntentContext) -> Option<String> {
        self.line_counter += 1;
        self.last_command = command.to_string();

        let working_line = self.apply_structural_transform(line);

        if let Some(guard_result) = self.apply_resource_guard(&working_line, command, context) {
            return guard_result;
        }

        let redacted = self.redactor.redact(&working_line);

        let final_line = self.apply_compression(&redacted, command);

        if final_line.is_some() && self.is_semantically_relevant(&redacted, context) {
            return Some(redacted);
        }

        self.apply_plugins(final_line)
    }

    fn apply_structural_transform(&self, line: &str) -> String {
        if self.markdown_mode && ContentTransformer::looks_like_table(line) {
            ContentTransformer::to_markdown(line)
        } else {
            line.to_string()
        }
    }

    fn apply_resource_guard(&mut self, line: &str, command: &str, context: &IntentContext) -> Option<Option<String>> {
        let handler = self.handlers.iter().find(|h| h.matches(command)).map(|h| h.as_ref());
        if ContentTransformer::should_guard(command, self.line_counter, context) {
            if self.line_counter == 101 {
                return Some(Some("(Guardian Mode: File too long. Summary follows...)".to_string()));
            }
            if self.discovery.process_and_check_noise(line, handler) {
                return Some(None);
            }
        }
        None
    }

    fn is_semantically_relevant(&mut self, line: &str, context: &IntentContext) -> bool {
        context.is_relevant(line) || self.intelligence.is_relevant(&context.last_message, line, 0.7)
    }

    fn apply_compression(&mut self, line: &str, command: &str) -> Option<String> {
        let handler = self.handlers.iter().find(|h| h.matches(command)).map(|h| h.as_ref());
        if let Some(schema) = self.schemas.iter().find(|s| s.matches(command)) {
            if let Some(action) = schema.apply_rules(line) {
                return match action {
                    Action::Keep => Some(line.to_string()),
                    Action::Collapse => {
                        self.discovery.process_and_check_noise(line, handler);
                        None
                    },
                    Action::Redact => Some("[REDACTED_BY_SCHEMA]".to_string()),
                    Action::Hidden => None,
                    Action::Synthesize => {
                        self.discovery.process_and_check_noise(line, handler);
                        None
                    },
                };
            }
        }

        if self.discovery.process_and_check_noise(line, handler) {
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
