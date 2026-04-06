use std::io::Write;
use crate::gateway::core::OutputRenderer;

pub struct TtyRenderer;

impl OutputRenderer for TtyRenderer {
    fn render_line(&mut self, text: &str, is_stderr: bool) {
        let result = if is_stderr {
            let mut stderr = std::io::stderr().lock();
            writeln!(stderr, "{}", text)
        } else {
            let mut stdout = std::io::stdout().lock();
            writeln!(stdout, "{}", text)
        };

        if let Err(e) = result {
            if e.kind() == std::io::ErrorKind::BrokenPipe {
                std::process::exit(0);
            }
        }
    }

    fn render_summary(&mut self, summaries: &[String], is_stderr: bool) {
        if summaries.is_empty() { return; }

        let header = "\x1b[1;33m[AXIOM]\x1b[0m";
        self.render_line(header, is_stderr);

        for summary in summaries {
            let msg = format!("\x1b[33m• {}\x1b[0m", summary);
            self.render_line(&msg, is_stderr);
        }
    }
}
