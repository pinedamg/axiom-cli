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
pub mod storage;
pub mod reporting;

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
            storage: LogManager::default(),
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
        let _ = self.storage.reset_log();
        self.intelligence.pre_compute_intent(intent)
    }

    /// The main pipeline orchestrator. Adheres to a strict stage-based flow.
    pub fn process_line(&mut self, line: &str, command: &str, context: &IntentContext) -> Option<String> {
        self.line_counter += 1;
        self.last_command = command.to_string();

        // 0. The Tee System (Raw Backup)
        let _ = self.storage.append_line(line);

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
        let mut semantic_swallow = false;
        let mut semantic_msg = None;

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
            if let Some(meta) = self.handlers[idx].parse_line(&redacted) {
                // Get the category (prefix) for semantic deduplication
                let category = self.get_category(&meta.perms, self.handlers[idx].as_ref());
                
                // Semantic Deduplication Check
                if let Some(msg) = self.handle_semantic_deduplication(&category) {
                    if msg.is_empty() {
                        semantic_swallow = true;
                    } else {
                        semantic_msg = Some(msg);
                    }
                }
                true
            } else {
                false
            }
        } else {
            false
        };

        if is_known_by_handler && !processed_by_discovery {
            let handler = self.handlers[handler_idx.unwrap()].as_ref();
            processed_by_discovery = true;
            let is_noise = self.discovery.process_and_check_noise(&redacted, Some(handler), command);
            
            // If semantic deduplication says swallow, we do it BUT after discovery process
            if semantic_swallow {
                return self.wrap_with_prefix(dedup_prefix, None);
            }
            if let Some(msg) = semantic_msg {
                return self.wrap_with_prefix(dedup_prefix, Some(msg));
            }

            if is_noise {
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

    fn apply_structural_transform(&self, line: &str) -> String {
        if self.markdown_mode && ContentTransformer::looks_like_table(line) {
            ContentTransformer::to_markdown(line)
        } else {
            line.to_string()
        }
    }

    fn apply_resource_guard(&mut self, line: &str, command: &str, context: &IntentContext) -> Option<Option<String>> {
        let handler = self.handlers.iter().find(|h| h.matches(command)).map(|h| h.as_ref());
        
        // 1. Check if it's an outlier (ERROR, critical message, etc.)
        let is_outlier = if let Some(h) = handler {
            if let Some(meta) = h.parse_line(line) {
                h.is_outlier(line, &meta)
            } else { false }
        } else { false };

        // 2. Guardian Mode: If line count > threshold, only allow outliers
        if ContentTransformer::should_guard(command, self.line_counter, context) {
            if self.line_counter == 101 {
                return Some(Some("(Guardian Mode: File too long. Switched to smart filtering...)".to_string()));
            }
            
            if is_outlier {
                return None; // Allow outlier to pass through Guardian Mode
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

    fn handle_semantic_deduplication(&mut self, category: &str) -> Option<String> {
        // Protected categories that should NOT be collapsed by semantic burst
        if ["SEARCH", "GIT"].contains(&category) {
            return None;
        }

        if self.discovery.last_category.as_deref() == Some(category) {
            self.discovery.category_count += 1;
            if self.discovery.category_count == 3 {
                return Some(format!("[AXIOM] Continuous burst of {} detected. Collapsing...", category));
            } else if self.discovery.category_count > 3 {
                return Some("".to_string()); // Signal to swallow
            }
        } else {
            self.discovery.last_category = Some(category.to_string());
            self.discovery.category_count = 0;
        }
        None
    }

    /// Helper to get the category name based on perms and handler (DRY from DiscoveryEngine)
    fn get_category(&self, perms: &str, handler: &dyn CommandHandler) -> String {
        if ["LOG_COMMIT", "MODIFIED", "UNTRACKED", "DELETED", "NEW", "RENAMED", "STAGED"].contains(&perms) { 
            "GIT".to_string() 
        } else if ["Running", "Stopped", "Created", "LAYER", "BUILD", "COMPOSE"].contains(&perms) {
            "DOCKER".to_string()
        } else if perms == "MATCH" {
            "SEARCH".to_string()
        } else if ["Checking", "Compiling", "Downloading", "Downloaded", "Finished", "Processing"].contains(&perms) {
            "CARGO".to_string()
        } else if perms == "PROGRESS" || perms == "NETWORK_NOISE" {
            "IO".to_string()
        } else if ["WARN", "ADD", "AUDIT"].contains(&perms) {
            "NPM".to_string()
        } else if ["TEST_RESULT", "COMPILING"].contains(&perms) {
            "GO".to_string()
        } else if ["RESOURCE", "METADATA"].contains(&perms) {
            "K8S".to_string()
        } else if ["PLAN", "ATTRIBUTE", "STATE"].contains(&perms) {
            "TF".to_string()
        } else if ["RESOURCE", "ROW"].contains(&perms) && (handler.matches("gcloud") || handler.matches("aws") || handler.matches("az")) {
            "CLOUD".to_string()
        } else if ["STRUCT", "KEY"].contains(&perms) {
            "DATA".to_string()
        } else if ["NOISE", "LOG"].contains(&perms) {
            "SYS".to_string()
        } else if perms.contains("kernel") && handler.matches("ps") {
            "KERNEL".to_string()
        } else if handler.matches("ps") {
            "PROC".to_string()
        } else {
            "FILE".to_string()
        }
    }
}
