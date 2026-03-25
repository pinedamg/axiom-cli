use crate::engine::discovery::LineMetadata;
use std::collections::HashMap;

pub mod git;
pub mod docker;
pub mod ls;
// pub mod ps;

pub type DiscoveryBuffer = HashMap<String, Vec<LineMetadata>>;

pub trait CommandHandler: Send + Sync {
    fn matches(&self, command: &str) -> bool;
    fn parse_line(&self, line: &str) -> Option<LineMetadata>;
    fn generate_insight(&self, command: &str, buffer: &DiscoveryBuffer) -> Option<String>;
    fn format_summary(&self, _key: &str, _items: &[LineMetadata]) -> Option<String> {
        None 
    }
}

pub fn get_all_handlers() -> Vec<Box<dyn CommandHandler>> {
    vec![
        Box::new(git::GitHandler),
        Box::new(docker::DockerHandler),
        Box::new(ls::LsHandler),
    ]
}
