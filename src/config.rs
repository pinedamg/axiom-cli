use std::path::PathBuf;

pub const DEFAULT_DB_PATH: &str = "axiom.db";
pub const DEFAULT_SCHEMAS_DIR: &str = "config/schemas";
pub const DEFAULT_PLUGINS_DIR: &str = "config/plugins";
pub const DEFAULT_ENTROPY_THRESHOLD: f64 = 4.5;
pub const DEFAULT_SEMANTIC_THRESHOLD: f32 = 0.75;

#[derive(Debug, Clone)]
pub struct IntentSource {
    pub name: String,
    pub path: PathBuf,
    pub strategy: IntentStrategy,
}

#[derive(Debug, Clone)]
pub enum IntentStrategy {
    LastLine,      // Read the last line of the file
    TailJSON,      // Parse as JSON and find the last 'content' or 'prompt'
    SQLiteHistory, // Query a local SQLite DB (common in IDEs like Cursor)
}

#[derive(Debug, Clone)]
pub struct AxiomConfig {
    pub db_path: PathBuf,
    pub schemas_dir: PathBuf,
    pub plugins_dir: PathBuf,
    pub entropy_threshold: f64,
    pub semantic_threshold: f32,
    pub intent_keywords: Vec<String>,
    pub pii_patterns: Vec<String>,
    pub intent_sources: Vec<IntentSource>,
    pub analytics_opt_out: bool,
}

impl Default for AxiomConfig {
    fn default() -> Self {
        Self {
            db_path: PathBuf::from(DEFAULT_DB_PATH),
            schemas_dir: PathBuf::from(DEFAULT_SCHEMAS_DIR),
            plugins_dir: PathBuf::from(DEFAULT_PLUGINS_DIR),
            entropy_threshold: DEFAULT_ENTROPY_THRESHOLD,
            semantic_threshold: DEFAULT_SEMANTIC_THRESHOLD,
            analytics_opt_out: false,
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
