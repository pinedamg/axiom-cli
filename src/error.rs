use thiserror::Error;

#[derive(Error, Debug)]
pub enum AxiomError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error("WASM runtime error: {0}")]
    Wasm(String),

    #[error("Model loading error: {0}")]
    Intelligence(String),

    #[error("Schema parsing error: {0}")]
    Schema(String),

    #[error("Regex error: {0}")]
    Regex(#[from] regex::Error),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

pub type Result<T> = std::result::Result<T, AxiomError>;
