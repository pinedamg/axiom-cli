use std::path::PathBuf;
use std::env;
use std::fs;
use serde::{Serialize, Deserialize};

pub const DEFAULT_DB_PATH: &str = "axiom.db";
pub const DEFAULT_SCHEMAS_DIR: &str = "config/schemas";
pub const DEFAULT_PLUGINS_DIR: &str = "config/plugins";
pub const DEFAULT_ENTROPY_THRESHOLD: f64 = 4.5;
pub const DEFAULT_SEMANTIC_THRESHOLD: f32 = 0.75;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TelemetryLevel {
    Off,
    Basic,
    Discovery,
    Full,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntentSource {
    pub name: String,
    pub path: PathBuf,
    pub strategy: IntentStrategy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum IntentStrategy {
    LastLine,
    TailJSON,
    SQLiteHistory,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum IntelligenceMode {
    Fuzzy,
    Neural,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AxiomConfig {
    pub db_path: PathBuf,
    pub schemas_dir: PathBuf,
    pub plugins_dir: PathBuf,
    pub entropy_threshold: f64,
    pub semantic_threshold: f32,
    pub intelligence_mode: IntelligenceMode,
    pub markdown_enabled: bool,
    pub telemetry_level: TelemetryLevel,
    pub installation_id: String,
    pub intent_keywords: Vec<String>,
    pub pii_patterns: Vec<String>,
    pub intent_sources: Vec<IntentSource>,
}

impl Default for AxiomConfig {
    fn default() -> Self {
        Self {
            db_path: PathBuf::from(DEFAULT_DB_PATH),
            schemas_dir: PathBuf::from(DEFAULT_SCHEMAS_DIR),
            plugins_dir: PathBuf::from(DEFAULT_PLUGINS_DIR),
            entropy_threshold: DEFAULT_ENTROPY_THRESHOLD,
            semantic_threshold: DEFAULT_SEMANTIC_THRESHOLD,
            intelligence_mode: IntelligenceMode::Fuzzy,
            markdown_enabled: false,
            telemetry_level: TelemetryLevel::Off,
            installation_id: "local-dev".to_string(),
            intent_keywords: vec![
                "error".to_string(), "fail".to_string(), "package".to_string(),
                "version".to_string(), "diff".to_string(), "log".to_string(),
                "debug".to_string(), "trace".to_string(), "crash".to_string(),
            ],
            pii_patterns: vec![
                r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}".to_string(),
                r"\b\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}\b".to_string(),
            ],
            intent_sources: vec![
                IntentSource {
                    name: "Claude Code".to_string(),
                    path: PathBuf::from(".claude/history.json"),
                    strategy: IntentStrategy::TailJSON,
                },
                IntentSource {
                    name: "Gemini CLI".to_string(),
                    path: PathBuf::from(".gemini/last_prompt.log"),
                    strategy: IntentStrategy::LastLine,
                },
                IntentSource {
                    name: "Axiom Local Context".to_string(),
                    path: PathBuf::from(".axiom_context"),
                    strategy: IntentStrategy::LastLine,
                },
            ],
        }
    }
}

impl AxiomConfig {
    /// Layered loading logic: Defaults -> Project File -> Env Vars
    pub fn load() -> Self {
        let mut config = Self::default();

        // 1. Try loading from local project file (.axiom.yaml)
        if let Ok(content) = fs::read_to_string(".axiom.yaml") {
            if let Ok(local_config) = serde_yaml::from_str::<AxiomConfig>(&content) {
                config = local_config; // Simple override for now
            }
        }

        // 2. Overrides from Environment Variables
        if env::var("AXIOM_FORCE_NEURAL").is_ok() {
            config.intelligence_mode = IntelligenceMode::Neural;
        }

        if env::var("AXIOM_MARKDOWN").is_ok() {
            config.markdown_enabled = true;
        }

        if let Ok(level) = env::var("AXIOM_TELEMETRY") {
            config.telemetry_level = match level.to_lowercase().as_str() {
                "full" => TelemetryLevel::Full,
                "discovery" => TelemetryLevel::Discovery,
                "basic" => TelemetryLevel::Basic,
                _ => TelemetryLevel::Off,
            };
        }

        if let Ok(path) = env::var("AXIOM_DB_PATH") {
            config.db_path = PathBuf::from(path);
        }

        config
    }
}
