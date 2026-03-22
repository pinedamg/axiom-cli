use std::fs;
use crate::config::AxiomConfig;
use crate::persistence::PersistenceManager;
use crate::schema::ToolSchema;
use crate::privacy::PrivacyRedactor;
use crate::engine::AxiomEngine;

pub struct AxiomSession {
    pub config: AxiomConfig,
    pub persistence: PersistenceManager,
    pub engine: AxiomEngine,
}

impl AxiomSession {
    pub fn new(config: AxiomConfig) -> anyhow::Result<Self> {
        let persistence = PersistenceManager::new_with_path(&config.db_path)?;
        let schemas = Self::load_schemas(&config)?;
        
        // Use patterns and threshold from config
        let redactor = PrivacyRedactor::new(
            config.entropy_threshold, 
            config.pii_patterns.clone()
        );
        
        let mut engine = AxiomEngine::new(redactor, schemas);
        
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
        
        // Log savings
        let _ = self.persistence.log_saving(command, original, compressed);
        
        Ok(())
    }
}
