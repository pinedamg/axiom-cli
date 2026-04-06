pub mod discovery;
pub mod intent_discovery;
pub mod plugins;
pub mod intelligence;
pub mod telemetry;
pub mod handshake;
pub mod transformer;
pub mod commands;
pub mod installer;
pub mod updater;
pub mod doctor;
pub mod storage;
pub mod reporting;
pub mod ui;

use crate::privacy::PrivacyRedactor;
use crate::schema::{ToolSchema, Action};
use crate::IntentContext;
use crate::engine::discovery::DiscoveryEngine;
use crate::engine::plugins::WasmPluginManager;
use crate::engine::intelligence::IntelligenceProvider;
use crate::engine::transformer::ContentTransformer;
use crate::engine::commands::{CommandHandler, get_all_handlers};
use crate::engine::storage::LogManager;
use std::borrow::Cow;

use crate::gateway::core::TerminalEvent;

pub struct AxiomEngine {
    pub redactor: PrivacyRedactor,
    pub schemas: Vec<ToolSchema>,
    pub discovery: DiscoveryEngine,
    pub storage: LogManager,
    pub plugins: Option<WasmPluginManager>,
    pub intelligence: Box<dyn IntelligenceProvider>,
    pub handlers: Vec<Box<dyn CommandHandler>>,
    pub markdown_mode: bool,
    pub last_command: String,
    line_counter: usize,
}

/// Governs the flow of a line through the Axiom pipeline.
enum PipelineAction {
    /// Continue to the next stage with the (possibly transformed) line.
    Continue(String),
    /// Stop processing and output this line immediately (bypass remaining stages).
    ShortCircuit(String),
    /// Stop processing and output nothing (swallow the line).
    Swallow,
}

impl AxiomEngine {
    pub fn new(
        redactor: PrivacyRedactor, 
        schemas: Vec<ToolSchema>,
        intelligence: Box<dyn IntelligenceProvider>,
        discovery_threshold: usize,
    ) -> Self {
        let mut discovery = DiscoveryEngine::default();
        discovery.threshold = discovery_threshold;
        
        Self {
            redactor,
            schemas,
            discovery,
            storage: LogManager::default(),
            plugins: None,
            intelligence,
            handlers: get_all_handlers(),
            markdown_mode: false,
            last_command: String::new(),
            line_counter: 0,
        }
    }

    pub fn prepare_session(&mut self, _intent: &str) -> anyhow::Result<()> {
        self.line_counter = 0;
        self.storage.reset_log()?;
        Ok(())
    }

    /// The main pipeline orchestrator (The Recipe).
    pub fn process_line(&mut self, event: TerminalEvent, command: &str, context: &IntentContext) -> Option<String> {
        let (line, is_progress) = match event {
            TerminalEvent::StaticLine(l) => (l, false),
            TerminalEvent::ProgressUpdate(l) => (l, true),
            TerminalEvent::StreamEnd => return None,
        };

        if is_progress {
            // Transient progress bar: we swallow it completely to save tokens and avoid
            // spamming the terminal with `writeln!`. Progress bars are pure noise for AI.
            return None;
        }

        self.line_counter += 1;
        self.last_command = command.to_string();
        let _ = self.storage.append_line(&line);

        // 1. Deduplicate (Stateful)
        let (prefix, action) = self.stage_deduplicate(&line);
        if matches!(action, PipelineAction::Swallow) { return None; }

        // 2. Transform (Pure)
        let working_line = self.stage_transform(&line);

        // 3. Guard (Stateful / Thresholds)
        let action = self.stage_guard(&working_line, command, context);
        
        let final_output = match action {
            PipelineAction::Swallow => None,
            PipelineAction::ShortCircuit(out) => Some(out),
            PipelineAction::Continue(line) => {
                // 4. Redact (Pure / Privacy)
                let redacted = self.stage_redact(&line);

                // 5. Analyze (Schema -> Structural -> Semantic)
                match self.stage_analyze(&redacted, command, context) {
                    PipelineAction::Swallow => None,
                    PipelineAction::ShortCircuit(out) | PipelineAction::Continue(out) => {
                        // 6. Plugins (External WASM)
                        self.stage_plugins(Some(out))
                    }
                }
            }
        };

        self.assemble_output(prefix, final_output)
    }

    // --- Pipeline Stages ---

    fn stage_deduplicate(&mut self, line: &str) -> (Option<String>, PipelineAction) {
        if self.discovery.last_line.as_deref() == Some(line) {
            self.discovery.repeat_count += 1;
            (None, PipelineAction::Swallow)
        } else {
            let prefix = if self.discovery.repeat_count > 0 {
                Some(format!("... (previous line repeated {} more times)", self.discovery.repeat_count))
            } else { None };
            self.discovery.last_line = Some(line.to_string());
            self.discovery.repeat_count = 0;
            (prefix, PipelineAction::Continue(line.to_string()))
        }
    }

    /// ⚡ Bolt: Return std::borrow::Cow instead of String to avoid allocating for every line in the hot path.
    fn stage_transform<'a>(&self, line: &'a str) -> Cow<'a, str> {
        if self.markdown_mode && ContentTransformer::looks_like_table(line) {
            Cow::Owned(ContentTransformer::to_markdown(line))
        } else {
            Cow::Borrowed(line)
        }
    }

    fn stage_guard(&mut self, line: &str, command: &str, context: &IntentContext) -> PipelineAction {
        let handler = self.handlers.iter().find(|h| h.matches(command)).map(|h| h.as_ref());
        
        let is_outlier = handler.map_or(false, |h| {
            h.parse_line(line).map_or(false, |m| h.is_outlier(line, &m))
        });

        if ContentTransformer::should_guard(command, self.line_counter, context) {
            if self.line_counter == 101 {
                return PipelineAction::ShortCircuit("[AXIOM] High noise detected. Activating Guardian Mode...".to_string());
            }
            if self.line_counter > 100 && !is_outlier {
                return PipelineAction::Swallow;
            }
        }
        PipelineAction::Continue(line.to_string())
    }

    fn stage_redact(&self, line: &str) -> String {
        self.redactor.redact(line)
    }

    fn stage_analyze(&mut self, line: &str, command: &str, context: &IntentContext) -> PipelineAction {
        let handler_idx = self.handlers.iter().position(|h| h.matches(command));
        
        // 5a. Schema Check
        if let Some(schema) = self.schemas.iter().find(|s| s.matches(command)) {
            if let Some(action) = schema.apply_rules(line) {
                match action {
                    Action::Keep => return PipelineAction::Continue(line.to_string()),
                    Action::Redact => return PipelineAction::ShortCircuit("[REDACTED_BY_SCHEMA]".to_string()),
                    Action::Hidden => return PipelineAction::Swallow,
                    Action::Collapse | Action::Synthesize => {
                        let handler = handler_idx.map(|idx| self.handlers[idx].as_ref());
                        if self.discovery.process_and_check_noise(line, handler, command) {
                            return PipelineAction::Swallow;
                        } else {
                            return PipelineAction::Continue(line.to_string());
                        }
                    }
                }
            }
        }

        // 5b. Semantic Relevance (AI Intent)
        if context.is_relevant(line) || self.intelligence.is_relevant(&context.last_message, line, 0.7) {
            return PipelineAction::Continue(line.to_string());
        }

        // 5c. General Synthesis (Discovery)
        let handler = handler_idx.map(|idx| self.handlers[idx].as_ref());
        if self.discovery.process_and_check_noise(line, handler, command) {
            PipelineAction::Swallow
        } else {
            PipelineAction::Continue(line.to_string())
        }
    }

    fn stage_plugins(&mut self, line: Option<String>) -> Option<String> {
        match (line, &mut self.plugins) {
            (Some(mut current), Some(manager)) => {
                current = manager.transform(&current);
                if current.is_empty() { None } else { Some(current) }
            }
            (l, _) => l,
        }
    }

    fn assemble_output(&self, prefix: Option<String>, main: Option<String>) -> Option<String> {
        match (prefix, main) {
            (Some(p), Some(m)) => Some(format!("{}\n{}", p, m)),
            (None, Some(m)) => Some(m),
            (Some(p), None) => Some(p),
            (None, None) => None,
        }
    }

    pub fn with_plugins(mut self, manager: WasmPluginManager) -> Self {
        self.plugins = Some(manager);
        self
    }

    pub fn get_session_stats(&self) -> Option<crate::engine::reporting::SessionStats> {
        let raw = self.storage.get_total_bytes();
        let saved = self.discovery.get_saved_bytes();
        
        if raw == 0 { return None; }
        
        Some(crate::engine::reporting::SessionStats {
            raw_bytes: raw,
            saved_bytes: raw.saturating_sub(saved),
        })
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

    pub fn set_markdown_mode(&mut self, enabled: bool) {
        self.markdown_mode = enabled;
    }

    pub fn load_learned_templates(&mut self, templates: Vec<(String, usize)>) {
        self.discovery.load_templates(templates);
    }

    pub fn get_learned_templates(&self) -> Vec<(String, usize)> {
        self.discovery.templates.iter().map(|(k, v)| (k.clone(), *v)).collect()
    }
}
