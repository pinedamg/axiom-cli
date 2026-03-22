use std::path::PathBuf;

pub const DEFAULT_DB_PATH: &str = "axiom.db";
pub const DEFAULT_SCHEMAS_DIR: &str = "config/schemas";
pub const DEFAULT_ENTROPY_THRESHOLD: f64 = 4.5;

#[derive(Debug, Clone)]
pub struct AxiomConfig {
    pub db_path: PathBuf,
    pub schemas_dir: PathBuf,
    pub entropy_threshold: f64,
}

impl Default for AxiomConfig {
    fn default() -> Self {
        Self {
            db_path: PathBuf::from(DEFAULT_DB_PATH),
            schemas_dir: PathBuf::from(DEFAULT_SCHEMAS_DIR),
            entropy_threshold: DEFAULT_ENTROPY_THRESHOLD,
        }
    }
}
