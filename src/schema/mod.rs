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
}

impl ToolSchema {
    /// Compiles all regexes in the schema
    pub fn compile(&mut self) -> anyhow::Result<()> {
        self.compiled_command_re = Some(Regex::new(&self.command_pattern)?);
        for rule in &mut self.rules {
            rule.compiled_re = Some(Regex::new(&rule.pattern)?);
        }
        Ok(())
    }

    pub fn matches(&self, command: &str) -> bool {
        if let Some(re) = &self.compiled_command_re {
            re.is_match(command)
        } else {
            false
        }
    }
}
