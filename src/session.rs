use std::fs;
use std::env;
use crate::config::AxiomConfig;
use crate::persistence::PersistenceManager;
use crate::schema::ToolSchema;
use crate::privacy::PrivacyRedactor;
use crate::engine::plugins::WasmPluginManager;
use crate::engine::intelligence::{FuzzyIntelligence, NeuralIntelligence};
use crate::engine::AxiomEngine;

pub struct AxiomSession {
    pub config: AxiomConfig,
    pub persistence: PersistenceManager,
    pub engine: AxiomEngine,
}

impl AxiomSession {
    pub fn new(mut config: AxiomConfig) -> anyhow::Result<Self> {
        // Read opt-out from env
        if env::var("AXIOM_ANALYTICS_OPT_OUT").is_ok() {
            config.telemetry_level = crate::config::TelemetryLevel::Off;
        }

        let persistence = PersistenceManager::new_with_path(&config.db_path)?;
        let schemas = Self::load_schemas(&config)?;
        
        let redactor = PrivacyRedactor::new(
            config.entropy_threshold, 
            config.pii_patterns.clone()
        );
        
        // Strategy Selection: Select intelligence provider
        let intelligence: Box<dyn crate::engine::intelligence::IntelligenceProvider> = 
            if env::var("AXIOM_FORCE_NEURAL").is_ok() {
                // If it fails to load, fallback to Fuzzy gracefully
                match NeuralIntelligence::new() {
                    Ok(n) => Box::new(n),
                    Err(e) => {
                        eprintln!("\x1b[31m[AXIOM] Failed to load Neural Engine: {}. Falling back to Fuzzy.\x1b[0m", e);
                        Box::new(FuzzyIntelligence)
                    }
                }
            } else {
                Box::new(FuzzyIntelligence)
            };
        
        let mut engine = AxiomEngine::new(redactor, schemas, intelligence);
        
        // Initialize WASM plugins
        if let Ok(plugin_manager) = WasmPluginManager::new(&config.plugins_dir) {
            engine = engine.with_plugins(plugin_manager);
        }
        
        // Load historical memory
        if let Ok(known) = persistence.get_known_templates() {
            engine.load_learned_templates(known);
        }

        Ok(Self {
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
        
        // Report telemetry based on configured level (Default: Discovery)
        crate::engine::telemetry::Telemetry::report_usage(&self.config, command, original, compressed);

        Ok(())
    }
}
