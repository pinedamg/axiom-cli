use crate::gateway::core::{StreamFilter, TerminalEvent};
use regex::Regex;

pub struct StreamPipeline {
    buffer: String,
    ansi_regex: Regex,
    last_was_cr: bool,
}

impl Default for StreamPipeline {
    fn default() -> Self {
        Self {
            buffer: String::with_capacity(1024),
            ansi_regex: Regex::new(r"[\u001b\u009b][\[()#;?]*(?:[0-9]{1,4}(?:;[0-9]{0,4})*)?[0-9A-ORZcf-nqry=><]").unwrap(),
            last_was_cr: false,
        }
    }
}

impl StreamFilter for StreamPipeline {
    fn process(&mut self, chunk: &[u8]) -> Vec<TerminalEvent> {
        let mut events = Vec::with_capacity(16);
        let text = String::from_utf8_lossy(chunk);
        let stripped = self.ansi_regex.replace_all(&text, "");

        for c in stripped.chars() {
            if c == '\n' {
                // Line feed: emit what we have as a static line
                // ⚡ Bolt: Use .clone() and .clear() to retain pre-allocated capacity
                let line = self.buffer.clone();
                self.buffer.clear();
                events.push(TerminalEvent::StaticLine(line));
                self.last_was_cr = false;
            } else if c == '\r' {
                // Carriage return: emit current buffer as progress, then clear buffer
                if !self.buffer.is_empty() {
                    events.push(TerminalEvent::ProgressUpdate(self.buffer.clone()));
                    self.buffer.clear();
                }
                self.last_was_cr = true;
            } else {
                self.buffer.push(c);
                self.last_was_cr = false;
            }
        }
        events
    }
}
