use std::fs;
use crate::config::{IntentSource, IntentStrategy};

pub struct IntentDiscoverer;

impl IntentDiscoverer {
    /// Tries to discover the last user intent from a list of sources
    pub fn discover(sources: &[IntentSource]) -> Option<String> {
        for source in sources {
            if source.path.exists() {
                if let Some(intent) = Self::extract_intent(source) {
                    return Some(intent);
                }
            }
        }
        None
    }

    fn extract_intent(source: &IntentSource) -> Option<String> {
        match source.strategy {
            IntentStrategy::LastLine => {
                let content = fs::read_to_string(&source.path).ok()?;
                content.lines().last().map(|s| s.trim().to_string())
            }
            IntentStrategy::TailJSON => {
                // Placeholder for real JSON parsing logic
                fs::read_to_string(&source.path).ok()
            }
            IntentStrategy::SQLiteHistory => {
                // Future implementation for SQLite-based tools
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;
    use crate::config::IntentStrategy;
    use std::path::PathBuf;

    #[test]
    fn test_discover_last_line() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "First prompt").unwrap();
        writeln!(file, "  Second prompt with spaces  ").unwrap();
        
        let source = IntentSource {
            name: "Test AI".to_string(),
            path: file.path().to_path_buf(),
            strategy: IntentStrategy::LastLine,
        };
        
        let discovered = IntentDiscoverer::discover(&[source]);
        assert_eq!(discovered, Some("Second prompt with spaces".to_string()));
    }

    #[test]
    fn test_discover_missing_file() {
        let source = IntentSource {
            name: "Nonexistent".to_string(),
            path: PathBuf::from("/tmp/should_not_exist_12345"),
            strategy: IntentStrategy::LastLine,
        };
        
        let discovered = IntentDiscoverer::discover(&[source]);
        assert_eq!(discovered, None);
    }
}
