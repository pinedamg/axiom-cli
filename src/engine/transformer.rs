use crate::IntentContext;

pub struct ContentTransformer;

impl ContentTransformer {
    /// Detects if a line looks like it belongs to a terminal table
    pub fn looks_like_table(line: &str) -> bool {
        let line = line.trim();
        if line.len() < 10 { return false; }
        
        // Typical terminal table: multiple blocks separated by 2+ spaces
        let parts: Vec<&str> = line.split("  ").filter(|s| !s.trim().is_empty()).collect();
        parts.len() >= 3
    }

    /// Converts a space-aligned terminal line into a Markdown table row
    pub fn to_markdown(line: &str) -> String {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 2 { return line.to_string(); }
        
        format!("| {} |", parts.join(" | "))
    }

    /// Determines if Volume Guardian should intervene
    pub fn should_guard(command: &str, line_count: usize, context: &IntentContext) -> bool {
        command.starts_with("cat") && line_count > 100 && !context.is_relevant("full file")
    }
}
