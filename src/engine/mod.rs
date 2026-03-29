pub mod discovery;
pub mod intent_discovery;
pub mod plugins;
pub mod intelligence;
pub mod telemetry;
pub mod transformer;
pub mod commands;
pub mod installer;
pub mod updater;
pub mod doctor;

use std::io::{self, Write};
use std::fs::OpenOptions;
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

        // 0. The Tee System (Raw Backup)
        let _ = Self::backup_raw_line(line);

        // 0.5 Aggressive Vertical Deduplication
        let mut dedup_prefix = None;
        if self.discovery.last_line.as_deref() == Some(line) {
            self.discovery.repeat_count += 1;
            return None; // Swallowed
        } else {
            if self.discovery.repeat_count > 0 {
                dedup_prefix = Some(format!("... (previous line repeated {} more times)", self.discovery.repeat_count));
            }
            self.discovery.last_line = Some(line.to_string());
            self.discovery.repeat_count = 0;
        }

        let working_line = self.apply_structural_transform(line);

        if let Some(guard_result) = self.apply_resource_guard(&working_line, command, context) {
            return self.wrap_with_prefix(dedup_prefix, guard_result);
        }

        let redacted = self.redactor.redact(&working_line);
        let mut processed_by_discovery = false;

        // 1. Schema Check: Does the YAML have an explicit instruction for this line?
        let handler_idx = self.handlers.iter().position(|h| h.matches(command));

        if let Some(schema) = self.schemas.iter().find(|s| s.matches(command)) {
            if let Some(action) = schema.apply_rules(&redacted) {
                match action {
                    Action::Keep => return self.wrap_with_prefix(dedup_prefix, Some(redacted)),
                    Action::Redact => return self.wrap_with_prefix(dedup_prefix, Some("[REDACTED_BY_SCHEMA]".to_string())),
                    Action::Hidden => return self.wrap_with_prefix(dedup_prefix, None),
                    Action::Collapse | Action::Synthesize => {
                        let handler = handler_idx.map(|idx| self.handlers[idx].as_ref());
                        processed_by_discovery = true;
                        if self.discovery.process_and_check_noise(&redacted, handler, command) {
                            return self.wrap_with_prefix(dedup_prefix, None);
                        }
                    }
                }
            }
        }

        // 2. Structural Check: If no schema rule, does a handler know this?
        let is_known_by_handler = if let Some(idx) = handler_idx {
            self.handlers[idx].parse_line(&redacted).is_some()
        } else {
            false
        };

        if is_known_by_handler && !processed_by_discovery {
            let handler = self.handlers[handler_idx.unwrap()].as_ref();
            processed_by_discovery = true;
            if self.discovery.process_and_check_noise(&redacted, Some(handler), command) {
                return self.wrap_with_prefix(dedup_prefix, None);
            }
        }

        // 3. Semantic Check: If the IA says this is important
        if self.is_semantically_relevant(&redacted, context) {
            return self.wrap_with_prefix(dedup_prefix, Some(redacted));
        }

        // 4. Fallback: Generic pattern discovery
        if !processed_by_discovery {
            let handler = handler_idx.map(|idx| self.handlers[idx].as_ref());
            if self.discovery.process_and_check_noise(&redacted, handler, command) {
                return self.wrap_with_prefix(dedup_prefix, None);
            }
        }

        let final_output = self.apply_plugins(Some(redacted));
        self.wrap_with_prefix(dedup_prefix, final_output)
    }

    fn wrap_with_prefix(&self, prefix: Option<String>, output: Option<String>) -> Option<String> {
        match (prefix, output) {
            (Some(p), Some(o)) => Some(format!("{}\n{}", p, o)),
            (Some(p), None) => Some(p),
            (None, o) => o,
        }
    }

    // ⚡ BOLT MEMORY OPTIMIZATION: Returns `Cow<str>` to avoid unnecessary heap allocations
    // when the line doesn't need to be structurally transformed (the common path).
    fn apply_structural_transform<'a>(&self, line: &'a str) -> std::borrow::Cow<'a, str> {
        if self.markdown_mode && ContentTransformer::looks_like_table(line) {
            std::borrow::Cow::Owned(ContentTransformer::to_markdown(line))
        } else {
            std::borrow::Cow::Borrowed(line)
        }
    }

    fn apply_resource_guard(&mut self, line: &str, command: &str, context: &IntentContext) -> Option<Option<String>> {
        let handler = self.handlers.iter().find(|h| h.matches(command)).map(|h| h.as_ref());
        if ContentTransformer::should_guard(command, self.line_counter, context) {
            if self.line_counter == 101 {
                return Some(Some("(Guardian Mode: File too long. Summary follows...)".to_string()));
            }
            if self.discovery.process_and_check_noise(line, handler, command) {
                return Some(None);
            }
        }
        None
    }

    fn is_semantically_relevant(&mut self, line: &str, context: &IntentContext) -> bool {
        context.is_relevant(line) || self.intelligence.is_relevant(&context.last_message, line, 0.7)
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

    fn backup_raw_line(line: &str) -> io::Result<()> {
        let path = std::path::Path::new("/tmp/axiom/last_run.log");
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)?;

        writeln!(file, "{}", line)?;
        Ok(())
    }
}
