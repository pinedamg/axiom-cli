use std::collections::HashMap;

pub struct SessionStats {
    pub raw_bytes: usize,
    pub saved_bytes: usize,
}

pub struct EfficiencyReport {
    pub total_original: usize,
    pub total_compressed: usize,
    pub tool_savings: HashMap<String, (usize, usize)>,
}

impl EfficiencyReport {
    pub fn new(raw_data: Vec<(String, usize, usize)>) -> Self {
        let mut total_original = 0;
        let mut total_compressed = 0;
        let mut tool_savings = HashMap::new();

        for (full_cmd, orig, comp) in raw_data {
            total_original += orig;
            total_compressed += comp;

            let tool = Self::canonicalize_tool(&full_cmd);
            let entry = tool_savings.entry(tool).or_insert((0, 0));
            entry.0 += orig;
            entry.1 += comp;
        }

        Self {
            total_original,
            total_compressed,
            tool_savings,
        }
    }

    fn canonicalize_tool(cmd: &str) -> String {
        let cmd = cmd.to_lowercase();
        if cmd.starts_with("git") { "git".to_string() }
        else if cmd.starts_with("docker") { "docker".to_string() }
        else if cmd.starts_with("npm") { "npm".to_string() }
        else if cmd.starts_with("cargo") { "cargo".to_string() }
        else if cmd.starts_with("kubectl") { "k8s".to_string() }
        else if cmd.starts_with("terraform") { "tf".to_string() }
        else { "other".to_string() }
    }

    pub fn saved_chars(&self) -> usize {
        self.total_original.saturating_sub(self.total_compressed)
    }

    pub fn ratio(&self) -> f64 {
        if self.total_original == 0 { 0.0 } 
        else { (self.saved_chars() as f64 / self.total_original as f64) * 100.0 }
    }

    pub fn estimated_usd_saved(&self) -> f64 {
        let tokens = self.saved_chars() / 4;
        (tokens as f64 / 1_000_000.0) * 15.0
    }
}

pub struct ReportRenderer;

impl ReportRenderer {
    pub fn render_dashboard(report: &EfficiencyReport) {
        println!("\x1b[1m📊 Axiom Efficiency Dashboard (SOLID v1.0)\x1b[0m");
        println!("-------------------------------------------\n");

        println!("\x1b[1mTop Savings by Tool:\x1b[0m");
        println!("{:<12} | {:<10} | {:<10} | {:<8}", "Tool", "Original", "Saved", "Efficiency");
        println!("{}", "-".repeat(50));

        let mut tools: Vec<_> = report.tool_savings.iter().collect();
        tools.sort_by(|a, b| (b.1.0 - b.1.1).cmp(&(a.1.0 - a.1.1)));

        for (tool, (orig, comp)) in tools {
            let saved = orig.saturating_sub(*comp);
            let r = if *orig > 0 { (saved as f64 / *orig as f64) * 100.0 } else { 0.0 };
            println!("{:<12} | {:<10} | {:<10} | {:.1}%", 
                tool, Self::format_bytes(*orig), Self::format_bytes(saved), r
            );
        }

        println!("\n\x1b[1mAggregate Impact:\x1b[0m");
        println!("  Tokens Avoided:    \x1b[33m~{}\x1b[0m", report.saved_chars() / 4);
        println!("  Credits Saved:     \x1b[32m~${:.2} USD\x1b[0m (Premium Estimate)", report.estimated_usd_saved());
        println!("  Total Efficiency:  \x1b[36m{:.1}%\x1b[0m", report.ratio());
        
        println!("\n\x1b[2mKeep it clean. Keep it fast. Axiom.\x1b[0m");
    }

    fn format_bytes(n: usize) -> String {
        if n >= 1_000_000 { format!("{:.1}M", n as f64 / 1_000_000.0) }
        else if n >= 1_000 { format!("{:.1}k", n as f64 / 1_000.0) }
        else { n.to_string() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_report_calculations() {
        let data = vec![
            ("git status".to_string(), 1000, 200), // 800 saved
            ("cargo check".to_string(), 2000, 100), // 1900 saved
        ];
        let report = EfficiencyReport::new(data);
        
        assert_eq!(report.saved_chars(), 2700);
        assert_eq!(report.total_original, 3000);
        assert!(report.ratio() > 80.0);
        assert!(report.tool_savings.contains_key("git"));
        assert!(report.tool_savings.contains_key("cargo"));
    }

    #[test]
    fn test_canonicalization() {
        assert_eq!(EfficiencyReport::canonicalize_tool("GIT checkout"), "git");
        assert_eq!(EfficiencyReport::canonicalize_tool("sudo docker ps"), "other"); // Note: future improvement for 'sudo'
    }
}
