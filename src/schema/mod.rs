use serde::{Deserialize, Serialize};
use regex::Regex;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ToolSchema {
    pub name: String,
    pub command_pattern: String,
    pub rules: Vec<TransformationRule>,
    
    #[serde(skip)]
    pub(crate) compiled_command_re: Option<Regex>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransformationRule {
    pub name: String,
    pub pattern: String,
    pub action: Action,
    pub priority: i32,

    #[serde(skip)]
    pub(crate) compiled_re: Option<Regex>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Action {
    Keep,
    Collapse,
    Redact,
    Hidden,
    Synthesize, // Nueva acción para agrupamiento inteligente
}

impl ToolSchema {
    /// Compiles all regexes in the schema for efficient matching
    pub fn compile(&mut self) -> anyhow::Result<()> {
        self.compiled_command_re = Some(Regex::new(&self.command_pattern)?);
        for rule in &mut self.rules {
            rule.compiled_re = Some(Regex::new(&rule.pattern)?);
        }
        Ok(())
    }

    pub fn matches(&self, command: &str) -> bool {
        self.compiled_command_re.as_ref().map_or(false, |re| re.is_match(command))
    }

    /// Applies rules to a line and returns the action to take
    pub fn apply_rules(&self, line: &str) -> Option<Action> {
        // Find the matching rule with the highest priority
        self.rules.iter()
            .filter(|r| r.compiled_re.as_ref().map_or(false, |re| re.is_match(line)))
            .max_by_key(|r| r.priority)
            .map(|r| r.action)
    }
}
