use crate::engine::discovery::LineMetadata;
use std::collections::BTreeMap;

pub mod git;
pub mod docker;
pub mod ls;
pub mod ps;
pub mod rg;
pub mod cargo;
pub mod go;
pub mod io;
pub mod npm;
pub mod kubectl;
pub mod terraform;
pub mod cloud;
pub mod jq;
pub mod journalctl;

pub type DiscoveryBuffer = BTreeMap<String, Vec<LineMetadata>>;

pub trait CommandHandler: Send + Sync {
    fn matches(&self, command: &str) -> bool;
    fn parse_line(&self, line: &str) -> Option<LineMetadata>;
    fn generate_insight(&self, command: &str, buffer: &DiscoveryBuffer) -> Option<String>;
    fn format_summary(&self, _key: &str, _items: &[LineMetadata]) -> Option<String> {
        None 
    }
    
    /// Detects if a line contains critical information that should NOT be synthesized
    fn is_outlier(&self, _line: &str, _meta: &LineMetadata) -> bool {
        false
    }

    /// Returns the semantic category for this line (e.g. GIT, DOCKER, CARGO)
    fn get_category(&self, _meta: &LineMetadata) -> String {
        "FILE".to_string()
    }

    /// Returns the grouping key for the synthesis buffer
    fn get_key(&self, prefix: &str, meta: &LineMetadata) -> String {
        format!("{}:{}:{}", prefix, meta.perms, meta.size)
    }
}

pub fn get_all_handlers() -> Vec<Box<dyn CommandHandler>> {
    vec![
        Box::new(git::GitHandler),
        Box::new(docker::DockerHandler),
        Box::new(ls::LsHandler),
        Box::new(ps::PsHandler),
        Box::new(rg::RgHandler),
        Box::new(cargo::CargoHandler),
        Box::new(io::IoHandler),
        Box::new(npm::NpmHandler),
        Box::new(go::GoHandler),
        Box::new(kubectl::KubectlHandler),
        Box::new(terraform::TerraformHandler),
        Box::new(cloud::CloudHandler),
        Box::new(jq::JqHandler),
        Box::new(journalctl::JournalHandler),
    ]
}
