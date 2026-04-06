use std::fs;
use crate::config::{AxiomConfig, IntelligenceMode};
use crate::persistence::PersistenceManager;
use crate::schema::ToolSchema;
use crate::privacy::PrivacyRedactor;
use crate::engine::plugins::WasmPluginManager;
use crate::engine::intelligence::{FuzzyIntelligence, NeuralIntelligence};
use crate::engine::AxiomEngine;

pub struct AxiomSession {
    pub id: String,
    pub config: AxiomConfig,
    pub persistence: PersistenceManager,
    pub engine: AxiomEngine,
}

impl AxiomSession {
    pub fn new(config: AxiomConfig) -> anyhow::Result<Self> {
        let persistence = PersistenceManager::new_with_path(&config.db_path)?;
        
        // Use Parent PID (PPID) as a session identifier for isolation
        // This ensures the session persists across multiple axiom executions in the same shell
        let ppid = sysinfo::Pid::from_u32(std::process::id())
            .as_u32(); // Fallback if we can't get PPID
            
        let mut system = sysinfo::System::new();
        system.refresh_processes();
        
        let session_id = if let Some(process) = system.process(sysinfo::Pid::from_u32(std::process::id())) {
            if let Some(parent_pid) = process.parent() {
                format!("shell-{}", parent_pid)
            } else {
                format!("shell-{}", ppid)
            }
        } else {
            format!("shell-{}", ppid)
        };
        let schemas = Self::load_schemas(&config)?;
        
        let redactor = PrivacyRedactor::new(
            config.entropy_threshold, 
            config.pii_patterns.clone()
        );
        
        // Strategy Selection: Based on consolidated config
        let intelligence: Box<dyn crate::engine::intelligence::IntelligenceProvider> = 
            match config.intelligence_mode {
                IntelligenceMode::Neural => {
                    match NeuralIntelligence::new() {
                        Ok(n) => Box::new(n),
                        Err(e) => {
                            eprintln!("\x1b[31m[AXIOM] Failed to load Neural Engine: {}. Falling back to Fuzzy.\x1b[0m", e);
                            Box::new(FuzzyIntelligence)
                        }
                    }
                }
                IntelligenceMode::Fuzzy => Box::new(FuzzyIntelligence),
                IntelligenceMode::Off => Box::new(FuzzyIntelligence), // Fallback to fuzzy but transformer will skip filtering
            };
        
        let mut engine = AxiomEngine::new(
            redactor, 
            schemas, 
            intelligence,
            config.discovery_threshold
        );
        
        if config.markdown_enabled {
            engine.set_markdown_mode(true);
        }

        // Initialize WASM plugins
        if let Ok(plugin_manager) = WasmPluginManager::new(&config.plugins_dir) {
            engine = engine.with_plugins(plugin_manager);
        }
        
        // Load historical memory
        if let Ok(known) = persistence.get_known_templates() {
            engine.load_learned_templates(known);
        }

        Ok(Self {
            id: session_id,
            config,
            persistence,
            engine,
        })
    }

    fn load_schemas(config: &AxiomConfig) -> anyhow::Result<Vec<ToolSchema>> {
        let mut schemas = Vec::new();
        if let Ok(entries) = fs::read_dir(&config.schemas_dir) {
            for entry in entries.flatten() {
                if entry.path().extension().and_then(|s| s.to_str()) == Some("yaml") {
                    if let Ok(content) = fs::read_to_string(entry.path()) {
                        if let Ok(mut schema) = serde_yaml::from_str::<ToolSchema>(&content) {
                            schema.compile()?; // Pre-compile Regex
                            schemas.push(schema);
                        }
                    }
                }
            }
        }
        Ok(schemas)
    }

    pub fn finalize(&self, command: &str, original: usize, compressed: usize) -> anyhow::Result<()> {
        // Save learned templates
        for (template, freq) in self.engine.get_learned_templates() {
            let _ = self.persistence.upsert_template(&template, freq);
        }
        
        // Log savings locally
        let _ = self.persistence.log_saving(command, original, compressed);

        // Report to Cloud Telemetry (Axiom Pulse)
        crate::engine::telemetry::Telemetry::report_usage(&self.config, command, original, compressed);
        
        Ok(())
    }
}
