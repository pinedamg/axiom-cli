use std::path::PathBuf;
use std::fs;

pub const DEFAULT_DB_PATH: &str = "axiom.db";
pub const DEFAULT_SCHEMAS_DIR: &str = "config/schemas";
pub const DEFAULT_PLUGINS_DIR: &str = "config/plugins";
pub const DEFAULT_ENTROPY_THRESHOLD: f64 = 4.5;
pub const DEFAULT_SEMANTIC_THRESHOLD: f32 = 0.75;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IntentSource {
    pub name: String,
    pub path: PathBuf,
    pub strategy: IntentStrategy,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum IntentStrategy {
    LastLine,      // Read the last line of the file
    TailJSON,      // Parse as JSON and find the last 'content' or 'prompt'
    SQLiteHistory, // Query a local SQLite DB (common in IDEs like Cursor)
}

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum TelemetryLevel {
    /// No telemetry is sent. (PRO ONLY)
    Off,
    /// Send only anonymous aggregate savings, OS, and version. (Privacy-First)
    Anonymous,
    /// Level 1 + Command binary names (no arguments) to discover missing schemas.
    Discovery,
    /// Level 2 + Internal performance metrics and matched rule IDs. (DEFAULT)
    Full,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AxiomConfig {
    pub installation_id: String,
    pub db_path: PathBuf,
    pub schemas_dir: PathBuf,
    pub plugins_dir: PathBuf,
    pub entropy_threshold: f64,
    pub semantic_threshold: f32,
    pub intent_keywords: Vec<String>,
    pub pii_patterns: Vec<String>,
    pub intent_sources: Vec<IntentSource>,
    pub telemetry_level: TelemetryLevel,
    pub is_pro: bool,
    pub license_key: Option<String>,
}

impl AxiomConfig {
    pub fn config_path() -> PathBuf {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        PathBuf::from(home).join(".axiom").join("config.yaml")
    }

    pub fn load() -> Self {
        let path = Self::config_path();
        if path.exists() {
            if let Ok(content) = fs::read_to_string(&path) {
                if let Ok(config) = serde_yaml::from_str::<Self>(&content) {
                    return config;
                }
            }
        }
        
        // If not found or invalid, create a new default and save it
        let config = Self::default();
        let _ = config.save();
        config
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let path = Self::config_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let yaml = serde_yaml::to_string(self)?;
        fs::write(path, yaml)?;
        Ok(())
    }
}

impl Default for AxiomConfig {
    fn default() -> Self {
        // Generate a simple random installation ID (nanos to hex)
        let id = format!("{:x}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos());
        
        Self {
            installation_id: id,
            db_path: PathBuf::from(DEFAULT_DB_PATH),
            schemas_dir: PathBuf::from(DEFAULT_SCHEMAS_DIR),
            plugins_dir: PathBuf::from(DEFAULT_PLUGINS_DIR),
            entropy_threshold: DEFAULT_ENTROPY_THRESHOLD,
            semantic_threshold: DEFAULT_SEMANTIC_THRESHOLD,
            telemetry_level: TelemetryLevel::Full,
            is_pro: false,
            license_key: None,
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
