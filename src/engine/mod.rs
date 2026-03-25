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
        let mut summaries = self.discovery.flush_variable_summary();
        
        if let Some(text) = insight {
            summaries.insert(0, format!("Semantic Insight: {}", text));
        }

        summaries
    }

    fn generate_semantic_insight(&self) -> Option<String> {
        if self.last_command.starts_with("ls") {
            return self.generate_ls_insight();
        } else if self.last_command.starts_with("ps") {
            return self.generate_ps_insight();
        }
        None
    }

    fn generate_ls_insight(&self) -> Option<String> {
        let mut project_type = None;
        for template in self.discovery.templates.keys() {
            let lower = template.to_lowercase();
            if lower.contains("cargo.toml") { project_type = Some("Detected Rust Project Workspace."); break; }
            if lower.contains("package.json") { project_type = Some("Detected Node.js Project."); break; }
            if lower.contains("go.mod") { project_type = Some("Detected Go Module."); break; }
        }
        if project_type.is_none() {
            for items in self.discovery.synthesis_buffer.values() {
                for item in items {
                    let lower_name = item.name.to_lowercase();
                    if lower_name == "cargo.toml" { project_type = Some("Detected Rust Project Workspace."); break; }
                    if lower_name == "package.json" { project_type = Some("Detected Node.js Project."); break; }
                }
            }
        }
        project_type.map(|s| s.to_string())
    }

    fn generate_ps_insight(&self) -> Option<String> {
        let mut max_cpu = 0.0;
        let mut top_proc = String::new();
        let mut total_procs = 0;

        for (key, items) in &self.discovery.synthesis_buffer {
            if key.starts_with("PROC:") {
                total_procs += items.len();
                for item in items {
                    if let Ok(cpu) = item.size.parse::<f64>() {
                        if cpu > max_cpu {
                            max_cpu = cpu;
                            top_proc = item.name.clone();
                        }
                    }
                }
            }
        }

        if total_procs > 0 {
            if max_cpu > 10.0 {
                Some(format!("High CPU load detected: {} is using {}% CPU. Total active processes: {}.", top_proc, max_cpu, total_procs))
            } else {
                Some(format!("System health stable. Total active processes: {}. No single process exceeding 10% CPU.", total_procs))
            }
        } else {
            None
        }
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

        if self.is_semantically_relevant(&redacted, context) {
            return Some(redacted);
        }

        let final_line = self.apply_compression(&redacted, command);

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
        if ContentTransformer::should_guard(command, self.line_counter, context) {
            if self.line_counter == 101 {
                return Some(Some("(Guardian Mode: File too long. Summary follows...)".to_string()));
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
                    Action::Synthesize => {
                        self.discovery.process_and_check_noise(line);
                        None
                    },
                };
            }
        }

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
