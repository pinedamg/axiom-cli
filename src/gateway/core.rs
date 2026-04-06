pub enum TerminalEvent {
    StaticLine(String),
    ProgressUpdate(String),
    StreamEnd,
}

pub trait StreamFilter {
    fn process(&mut self, chunk: &[u8]) -> Vec<TerminalEvent>;
}

pub trait OutputRenderer {
    fn render_line(&mut self, text: &str, is_stderr: bool);
    fn render_summary(&mut self, summaries: &[String], is_stderr: bool);
}
