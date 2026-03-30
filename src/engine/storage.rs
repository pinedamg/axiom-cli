use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::PathBuf;

pub struct LogManager {
    log_path: PathBuf,
}

impl Default for LogManager {
    fn default() -> Self {
        Self {
            log_path: PathBuf::from("/tmp/axiom/last_run.log"),
        }
    }
}

impl LogManager {
    pub fn new(path: PathBuf) -> Self {
        Self { log_path: path }
    }

    /// Appends a line to the raw log (The Tee System)
    pub fn append_line(&self, line: &str) -> io::Result<()> {
        if let Some(parent) = self.log_path.parent() {
            let _ = fs::create_dir_all(parent);
        }

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.log_path)?;

        writeln!(file, "{}", line)?;
        Ok(())
    }

    /// Resets the log for a new command run
    pub fn reset_log(&self) -> io::Result<()> {
        if self.log_path.exists() {
            fs::remove_file(&self.log_path)?;
        }
        Ok(())
    }

    /// Retrieves lines with optional tail and grep filtering (KISS logic)
    pub fn get_last_logs(&self, tail: Option<usize>, grep: Option<&str>) -> anyhow::Result<Vec<String>> {
        if !self.log_path.exists() {
            anyhow::bail!("Log file not found at {}", self.log_path.display());
        }

        let content = fs::read_to_string(&self.log_path)?;
        let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();

        // 1. Grep filtering
        if let Some(pattern) = grep {
            let pattern_lower = pattern.to_lowercase();
            lines.retain(|l| l.to_lowercase().contains(&pattern_lower));
        }

        // 2. Tail filtering
        if let Some(n) = tail {
            let start = lines.len().saturating_sub(n);
            lines = lines[start..].to_vec();
        }

        Ok(lines)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_log_filtering() {
        let file = NamedTempFile::new().unwrap();
        let manager = LogManager::new(file.path().to_path_buf());

        manager.append_line("Error: disk full").unwrap();
        manager.append_line("Success: operation ok").unwrap();
        manager.append_line("Error: connection lost").unwrap();

        // Test Grep
        let errors = manager.get_last_logs(None, Some("error")).unwrap();
        assert_eq!(errors.len(), 2);
        assert!(errors[0].contains("disk full"));

        // Test Tail
        let last_one = manager.get_last_logs(Some(1), None).unwrap();
        assert_eq!(last_one.len(), 1);
        assert!(last_one[0].contains("connection lost"));

        // Test Grep + Tail
        let filtered = manager.get_last_logs(Some(1), Some("error")).unwrap();
        assert_eq!(filtered.len(), 1);
        assert!(filtered[0].contains("connection lost"));
    }
}
