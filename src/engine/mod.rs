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
    pub fn process_line(&mut self, line: &str, command: &str, context: &IntentContext) -> Option<String> {
        self.line_counter += 1;
        self.last_command = command.to_string();
        let _ = self.storage.append_line(line);

        // 1. Deduplicate (Stateful)
        let (prefix, action) = self.stage_deduplicate(line);
        if matches!(action, PipelineAction::Swallow) { return None; }

        // 2. Transform (Pure)
        let working_line = self.stage_transform(line);

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

    fn stage_transform(&self, line: &str) -> String {
        if self.markdown_mode && ContentTransformer::looks_like_table(line) {
            ContentTransformer::to_markdown(line)
        } else {
            line.to_string()
        }
    }

    fn stage_guard(&mut self, line: &str, command: &str, context: &IntentContext) -> PipelineAction {
        let handler = self.handlers.iter().find(|h| h.matches(command)).map(|h| h.as_ref());
        
        let is_outlier = handler.map_or(false, |h| {
            h.parse_line(line).map_or(false, |m| h.is_outlier(line, &m))
        });

        if ContentTransformer::should_guard(command, self.line_counter, context) {
            if self.line_counter == 101 {
                return PipelineAction::ShortCircuit("(Guardian Mode: File too long. Switched to smart filtering...)".to_string());
            }
            if is_outlier { return PipelineAction::Continue(line.to_string()); }
            if self.discovery.process_and_check_noise(line, handler, command) {
                return PipelineAction::Swallow;
            }
        }
        PipelineAction::Continue(line.to_string())
    }

    fn stage_redact(&self, line: &str) -> String {
        self.redactor.redact(line)
    }

    fn stage_analyze(&mut self, line: &str, command: &str, context: &IntentContext) -> PipelineAction {
        // Step 1: Explicit YAML Schema Rules
        if let Some(action) = self.analyze_schema(line, command) {
            return action;
        }

        // Step 2: Structural Handler (Tool-specific logic)
        if let Some(action) = self.analyze_structural(line, command) {
            return action;
        }

        // Step 3: Semantic Relevance (AI Intent)
        if self.is_semantically_relevant(line, context) {
            return PipelineAction::Continue(line.to_string());
        }

        // Step 4: Generic Noise Discovery (Fallback)
        let handler = self.handlers.iter().find(|h| h.matches(command)).map(|h| h.as_ref());
        if self.discovery.process_and_check_noise(line, handler, command) {
            PipelineAction::Swallow
        } else {
            PipelineAction::Continue(line.to_string())
        }
    }

    fn analyze_schema(&mut self, line: &str, command: &str) -> Option<PipelineAction> {
        let schema = self.schemas.iter().find(|s| s.matches(command))?;
        let action = schema.apply_rules(line)?;
        
        match action {
            Action::Keep => Some(PipelineAction::Continue(line.to_string())),
            Action::Redact => Some(PipelineAction::Continue("[REDACTED_BY_SCHEMA]".to_string())),
            Action::Hidden => Some(PipelineAction::Swallow),
            Action::Collapse | Action::Synthesize => {
                let handler = self.handlers.iter().find(|h| h.matches(command)).map(|h| h.as_ref());
                if self.discovery.process_and_check_noise(line, handler, command) {
                    Some(PipelineAction::Swallow)
                } else {
                    Some(PipelineAction::Continue(line.to_string()))
                }
            }
        }
    }

    fn analyze_structural(&mut self, line: &str, command: &str) -> Option<PipelineAction> {
        let idx = self.handlers.iter().position(|h| h.matches(command))?;
        let meta = self.handlers[idx].parse_line(line)?;
        
        let _category = self.handlers[idx].get_category(&meta.perms);
        
        if self.discovery.process_and_check_noise(line, Some(self.handlers[idx].as_ref()), command) {
            Some(PipelineAction::Swallow)
        } else {
            Some(PipelineAction::Continue(line.to_string()))
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

    fn assemble_output(&self, prefix: Option<String>, output: Option<String>) -> Option<String> {
        match (prefix, output) {
            (Some(p), Some(o)) => Some(format!("{}\n{}", p, o)),
            (Some(p), None) => Some(p),
            (None, o) => o,
        }
    }

    fn is_semantically_relevant(&mut self, line: &str, context: &IntentContext) -> bool {
        context.is_relevant(line) || self.intelligence.is_relevant(&context.last_message, line, 0.7)
    }

    pub fn set_markdown_mode(&mut self, enabled: bool) {
        self.markdown_mode = enabled;
    }

    pub fn with_plugins(mut self, manager: WasmPluginManager) -> Self {
        self.plugins = Some(manager);
        self
    }

    pub fn load_learned_templates(&mut self, templates: Vec<(String, usize)>) {
        self.discovery.load_templates(templates);
    }

    pub fn get_learned_templates(&self) -> Vec<(String, usize)> {
        self.discovery.get_templates()
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
        let mut summaries = Vec::new();
        
        // 1. Get semantic insights from matcheable handlers (The high-level intelligence)
        for handler in &self.handlers {
            if handler.matches(&self.last_command) {
                if let Some(insight) = handler.generate_insight(&self.last_command, &self.discovery.synthesis_buffer) {
                    summaries.push(format!("\x1b[1;34m💡 Insight: {}\x1b[0m", insight));
                }
            }
        }

        // 2. Get structural summaries (e.g. Grouped 42 files)
        summaries.extend(self.discovery.flush_variable_summary(&self.handlers));

        // 3. Get repetition summaries
        if self.discovery.repeat_count > 0 {
            summaries.push(format!("... (previous line repeated {} more times)", self.discovery.repeat_count));
        }
        
        summaries
    }
}
