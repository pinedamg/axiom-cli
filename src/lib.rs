pub mod gateway;
pub mod privacy;
pub mod schema;
pub mod engine;
pub mod persistence;
pub mod config;
pub mod session;
pub mod error;

pub use error::{AxiomError, Result};
use serde::{Deserialize, Serialize};

/// Represents the context of the user message (the "intent")
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntentContext {
    pub last_message: String,
    pub command: String,
    pub keywords: Vec<String>,
}

impl Default for IntentContext {
    fn default() -> Self {
        Self {
            last_message: "Automated Session".to_string(),
            command: "unknown".to_string(),
            keywords: vec![],
        }
    }
}

impl IntentContext {
    /// Determines if a keyword or concept is relevant to the user's intent
    pub fn is_relevant(&self, text: &str) -> bool {
        let msg = self.last_message.to_lowercase();
        let target = text.to_lowercase();

        // Check against injected keywords
        for kw in &self.keywords {
            if msg.contains(kw) && target.contains(kw) {
                return true;
            }
        }
        
        // If the user's message contains specific words from the text
        for word in msg.split_whitespace() {
            if word.len() > 3 && target.contains(word) {
                return true;
            }
        }

        false
    }
}

pub trait SemanticStreamer {
    fn process(&self, input: &str, context: &IntentContext) -> String;
}
