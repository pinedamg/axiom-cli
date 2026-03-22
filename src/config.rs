use std::path::PathBuf;

pub const DEFAULT_DB_PATH: &str = "axiom.db";
pub const DEFAULT_SCHEMAS_DIR: &str = "config/schemas";
pub const DEFAULT_ENTROPY_THRESHOLD: f64 = 4.5;

#[derive(Debug, Clone)]
pub struct AxiomConfig {
    pub db_path: PathBuf,
    pub schemas_dir: PathBuf,
    pub entropy_threshold: f64,
    pub intent_keywords: Vec<String>,
    pub pii_patterns: Vec<String>,
}

impl Default for AxiomConfig {
    fn default() -> Self {
        Self {
            db_path: PathBuf::from(DEFAULT_DB_PATH),
            schemas_dir: PathBuf::from(DEFAULT_SCHEMAS_DIR),
            entropy_threshold: DEFAULT_ENTROPY_THRESHOLD,
            intent_keywords: vec![
                "error".to_string(),
                "fail".to_string(),
                "package".to_string(),
                "version".to_string(),
                "diff".to_string(),
                "log".to_string(),
                "debug".to_string(),
                "trace".to_string(),
                "crash".to_string(),
            ],
            pii_patterns: vec![
                r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}".to_string(), // Email
                r"\b\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}\b".to_string(),         // IPv4
            ],
        }
    }
}
