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
    pub dev_mode: bool,
    pub traces: Vec<LineTrace>,
    pub last_command: String,
    line_counter: usize,
}

#[derive(Clone, Debug)]
pub struct LineTrace {
    pub line_number: usize,
    pub original: String,
    pub final_output: Option<String>,
    pub events: Vec<TraceEvent>,
}

#[derive(Clone, Debug)]
pub enum TraceEvent {
    Deduplicated(String),
    Transformed(String),
    Guarded(String),
    Redacted(String),
    Analyzed(String, String), // (Action, Reason)
    Buffered(String),         // For items added to synthesis buffer
    PluginTransformed(String, String), // (PluginName, Result)
}

/// Governs the flow of a line through the Axiom pipeline.
#[derive(Debug)]
enum PipelineAction<'a> {
    /// Continue to the next stage with the (possibly transformed) line.
    Continue(Cow<'a, str>),
    /// Stop processing and output this line immediately (bypass remaining stages).
    ShortCircuit(Cow<'a, str>),
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
            dev_mode: false,
            traces: Vec::new(),
            last_command: String::new(),
            line_counter: 0,
        }
    }

    pub fn prepare_session(&mut self, _intent: &str) -> anyhow::Result<()> {
        self.line_counter = 0;
        self.storage.reset_log()?;
        Ok(())
    }

    pub fn set_dev_mode(&mut self, enabled: bool) {
        self.dev_mode = enabled;
    }

    pub fn get_traces(&self) -> &[LineTrace] {
        &self.traces
    }

    pub fn trace_summary(&mut self, summary: &str) {
        if self.dev_mode {
            self.traces.push(LineTrace {
                line_number: 0, // Summary lines don't have a source line number
                original: "[GENERATED SUMMARY]".to_string(),
                final_output: Some(summary.to_string()),
                events: vec![TraceEvent::Analyzed("Summary".to_string(), "Synthesized from buffer".to_string())],
            });
        }
    }

    /// The main pipeline orchestrator (The Recipe).
    pub fn process_line(&mut self, event: TerminalEvent, command: &str, context: &IntentContext) -> Option<String> {
        let (line, is_progress) = match event {
            TerminalEvent::StaticLine(l) => (l, false),
            TerminalEvent::ProgressUpdate(l) => (l, true),
            TerminalEvent::StreamEnd => {
                if self.dev_mode {
                    self.traces.push(LineTrace {
                        line_number: self.line_counter + 1,
                        original: "[STREAM END]".to_string(),
                        final_output: None,
                        events: vec![],
                    });
                }
                return None;
            }
        };

        if is_progress {
            // Transient progress bar: we swallow it completely to save tokens and avoid
            // spamming the terminal with `writeln!`. Progress bars are pure noise for AI.
            return None;
        }

        self.line_counter += 1;
        self.last_command = command.to_string();
        let _ = self.storage.append_line(&line);

        let mut events = if self.dev_mode { Vec::new() } else { Vec::with_capacity(0) };

        // 1. Deduplicate (Stateful)
        let (prefix, action, reason) = self.stage_deduplicate(&line);
        if self.dev_mode { events.push(TraceEvent::Deduplicated(reason)); }
        
        if matches!(action, PipelineAction::Swallow) { 
            if self.dev_mode {
                self.traces.push(LineTrace {
                    line_number: self.line_counter,
                    original: line.to_string(),
                    final_output: None,
                    events,
                });
            }
            return None; 
        }

        // 2. Transform (Pure)
        let (working_line, reason) = self.stage_transform(&line);
        if self.dev_mode { events.push(TraceEvent::Transformed(reason)); }

        // 3. Guard (Stateful / Thresholds)
        let (action, reason) = self.stage_guard(&working_line, command, context);
        if self.dev_mode { events.push(TraceEvent::Guarded(reason)); }
        
        let final_output = match action {
            PipelineAction::Swallow => None,
            PipelineAction::ShortCircuit(out) => Some(out),
            PipelineAction::Continue(l) => {
                // 4. Redact (Pure / Privacy)
                let redacted = self.stage_redact(&l);
                if self.dev_mode {
                    if redacted != *l {
                        events.push(TraceEvent::Redacted("PII/Secret redacted".to_string()));
                    } else {
                        events.push(TraceEvent::Redacted("No PII detected".to_string()));
                    }
                }

                // 5. Analyze (Schema -> Structural -> Semantic)
                let (analyze_result, reason, is_buffered) = self.stage_analyze(&redacted, command, context);
                if self.dev_mode { 
                    events.push(TraceEvent::Analyzed(format!("{:?}", analyze_result), reason)); 
                    if is_buffered {
                        events.push(TraceEvent::Buffered("Added to synthesis buffer for future summary".to_string()));
                    }
                }

                let analyze_out = match analyze_result {
                    PipelineAction::Swallow => None,
                    PipelineAction::ShortCircuit(out) | PipelineAction::Continue(out) => {
                        Some(Cow::Owned(out.into_owned()))
                    }
                };

                // 6. Plugins (External WASM)
                let (plugin_out, plugin_reasons) = self.stage_plugins(analyze_out);
                if self.dev_mode {
                    for (plugin, r) in plugin_reasons {
                        events.push(TraceEvent::PluginTransformed(plugin, r));
                    }
                }
                plugin_out
            }
        };

        let result = self.assemble_output(prefix, final_output);
        
        if self.dev_mode {
            self.traces.push(LineTrace {
                line_number: self.line_counter,
                original: line.to_string(),
                final_output: result.clone(),
                events,
            });
        }

        result
    }

    // --- Pipeline Stages ---

    fn stage_deduplicate<'a>(&mut self, line: &'a str) -> (Option<String>, PipelineAction<'a>, String) {
        if self.discovery.last_line.as_deref() == Some(line) {
            self.discovery.repeat_count += 1;
            (None, PipelineAction::Swallow, "Identical to previous line".to_string())
        } else {
            let prefix = if self.discovery.repeat_count > 0 {
                Some(format!("... (previous line repeated {} more times)", self.discovery.repeat_count))
            } else { None };

            // ⚡ Bolt: Reuse the existing Option<String> buffer to avoid allocating a new String per line.
            if let Some(mut existing) = self.discovery.last_line.take() {
                existing.clear();
                existing.push_str(line);
                self.discovery.last_line = Some(existing);
            } else {
                self.discovery.last_line = Some(line.to_string());
            }

            self.discovery.repeat_count = 0;
            (prefix, PipelineAction::Continue(Cow::Borrowed(line)), "New line".to_string())
        }
    }

    /// ⚡ Bolt: Return std::borrow::Cow instead of String to avoid allocating for every line in the hot path.
    fn stage_transform<'a>(&self, line: &'a str) -> (Cow<'a, str>, String) {
        if self.markdown_mode && ContentTransformer::looks_like_table(line) {
            (Cow::Owned(ContentTransformer::to_markdown(line)), "Table converted to Markdown".to_string())
        } else {
            (Cow::Borrowed(line), "No transformation needed".to_string())
        }
    }

    fn stage_guard<'a>(&mut self, line: &'a str, command: &str, context: &IntentContext) -> (PipelineAction<'a>, String) {
        let handler = self.handlers.iter().find(|h| h.matches(command)).map(|h| h.as_ref());
        
        let is_outlier = handler.map_or(false, |h| {
            h.parse_line(line).map_or(false, |m| h.is_outlier(line, &m))
        });

        if ContentTransformer::should_guard(command, self.line_counter, context) {
            if self.line_counter == 101 {
                return (PipelineAction::ShortCircuit(Cow::Owned("[AXIOM] High noise detected. Activating Guardian Mode...".to_string())), "High noise threshold reached (line 101)".to_string());
            }
            if self.line_counter > 100 && !is_outlier {
                return (PipelineAction::Swallow, "Dropped by Guardian Mode (noise line > 100)".to_string());
            }
        }
        (PipelineAction::Continue(Cow::Borrowed(line)), "Allowed by Guardian".to_string())
    }

    fn stage_redact(&self, line: &str) -> String {
        self.redactor.redact(line)
    }

    fn stage_analyze<'a>(&mut self, line: &'a str, command: &str, context: &IntentContext) -> (PipelineAction<'a>, String, bool) {
        let handler_idx = self.handlers.iter().position(|h| h.matches(command));
        
        // 5a. Schema Check
        if let Some(schema) = self.schemas.iter().find(|s| s.matches(command)) {
            if let Some(action) = schema.apply_rules(line) {
                match action {
                    Action::Keep => return (PipelineAction::Continue(Cow::Borrowed(line)), format!("Schema match ({}): KEEP", schema.name), false),
                    Action::Redact => return (PipelineAction::ShortCircuit(Cow::Owned("[REDACTED_BY_SCHEMA]".to_string())), format!("Schema match ({}): REDACT", schema.name), false),
                    Action::Hidden => return (PipelineAction::Swallow, format!("Schema match ({}): HIDE", schema.name), false),
                    Action::Collapse | Action::Synthesize => {
                        let handler = handler_idx.map(|idx| self.handlers[idx].as_ref());
                        if self.discovery.process_and_check_noise(line, handler, command) {
                            return (PipelineAction::Swallow, format!("Schema match ({}): COLLAPSE (discovered noise)", schema.name), true);
                        } else {
                            return (PipelineAction::Continue(Cow::Borrowed(line)), format!("Schema match ({}): KEEP (not noise yet)", schema.name), false);
                        }
                    }
                }
            }
        }

        // 5b. Semantic Relevance (AI Intent)
        if context.is_relevant(line) {
            return (PipelineAction::Continue(Cow::Borrowed(line)), "Semantic match (Context keywords)".to_string(), false);
        }
        if self.intelligence.is_relevant(&context.last_message, line, 0.7) {
            return (PipelineAction::Continue(Cow::Borrowed(line)), "Semantic match (LLM Embedding)".to_string(), false);
        }

        // 5c. General Synthesis (Discovery)
        let handler = handler_idx.map(|idx| self.handlers[idx].as_ref());
        if self.discovery.process_and_check_noise(line, handler, command) {
            (PipelineAction::Swallow, "Dropped by General Discovery Engine".to_string(), true)
        } else {
            (PipelineAction::Continue(Cow::Borrowed(line)), "General keep (Low noise)".to_string(), false)
        }
    }

    fn stage_plugins<'a>(&mut self, line: Option<Cow<'a, str>>) -> (Option<Cow<'a, str>>, Vec<(String, String)>) {
        let mut reasons = Vec::new();
        match (line, &mut self.plugins) {
            (Some(current), Some(manager)) => {
                // For now, WasmPluginManager::transform handles all plugins.
                // In the future, we might want to trace each plugin individually here.
                let transformed = manager.transform(&current);
                if transformed != *current {
                    reasons.push(("WASM Plugins".to_string(), "Line modified by external plugin".to_string()));
                }
                if transformed.is_empty() { 
                    (None, reasons) 
                } else { 
                    (Some(Cow::Owned(transformed)), reasons) 
                }
            }
            (l, _) => (l, reasons),
        }
    }

    fn assemble_output(&self, prefix: Option<String>, main: Option<Cow<'_, str>>) -> Option<String> {
        match (prefix, main) {
            (Some(p), Some(m)) => Some(format!("{}\n{}", p, m)),
            (None, Some(m)) => Some(m.into_owned()),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::intelligence::FuzzyIntelligence;
    use crate::privacy::PrivacyRedactor;

    #[test]
    fn test_line_tracing() {
        let redactor = PrivacyRedactor::new(4.0, vec![]);
        let intelligence = Box::new(FuzzyIntelligence);
        let mut engine = AxiomEngine::new(redactor, vec![], intelligence, 5);
        engine.set_dev_mode(true);

        let context = IntentContext::default();
        let event = TerminalEvent::StaticLine("test line".to_string());
        
        engine.process_line(event, "ls", &context);

        let traces = engine.get_traces();
        assert_eq!(traces.len(), 1);
        assert_eq!(traces[0].original, "test line");
        
        // Verify we have events
        assert!(!traces[0].events.is_empty());
        
        // Verify deduplication event exists
        let has_dedup = traces[0].events.iter().any(|e| matches!(e, TraceEvent::Deduplicated(_)));
        assert!(has_dedup);
    }
}
