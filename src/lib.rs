pub mod gateway;
pub mod privacy;
pub mod schema;
pub mod engine;
pub mod persistence;
pub mod config;
pub mod session;

use serde::{Deserialize, Serialize};

/// Represents the context of the user message (the "intent")
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntentContext {
    pub last_message: String,
    pub command: String,
}

impl IntentContext {
    /// Determines if a keyword or concept is relevant to the user's intent
    pub fn is_relevant(&self, text: &str) -> bool {
        let msg = self.last_message.to_lowercase();
        let target = text.to_lowercase();

        // Simple relevance logic for now (Keyword Matching)
        let keywords = vec!["error", "fail", "package", "version", "diff", "log", "debug"];
        
        for kw in keywords {
            if msg.contains(kw) && target.contains(kw) {
                return true;
            }
        }
        
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
