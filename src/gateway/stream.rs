use tokio::io::AsyncReadExt;
use tokio::process::Child;
use crate::IntentContext;
use crate::session::AxiomSession;
use crate::gateway::core::{TerminalEvent, OutputRenderer, StreamFilter};
use crate::gateway::filters::StreamPipeline;

pub async fn stream_io(
    child: &mut Child,
    command_str: &str,
    context: &IntentContext,
    session: &mut AxiomSession,
    renderer: &mut dyn OutputRenderer,
    raw_mode: bool,
) -> anyhow::Result<(usize, usize)> {
    let mut stdout = child.stdout.take().ok_or_else(|| anyhow::anyhow!("Failed to capture stdout"))?;
    let mut stderr = child.stderr.take().ok_or_else(|| anyhow::anyhow!("Failed to capture stderr"))?;

    let mut total_original = 0;
    let mut total_compressed = 0;

    let mut stdout_pipeline = StreamPipeline::default();
    let mut stderr_pipeline = StreamPipeline::default();

    let mut out_buf = [0u8; 4096];
    let mut err_buf = [0u8; 4096];

    loop {
        tokio::select! {
            n = stdout.read(&mut out_buf) => {
                match n? {
                    0 => {
                        let events = stdout_pipeline.process(&[]);
                        process_events(events, command_str, context, session, renderer, &mut total_original, &mut total_compressed, false, raw_mode);
                        break; // EOF reached on stdout (and likely stderr soon after or child exited)
                    },
                    n => {
                        total_original += n;
                        if raw_mode {
                            let text = String::from_utf8_lossy(&out_buf[..n]);
                            let redacted = session.engine.redactor.redact(&text);
                            renderer.render_line(&redacted, false);
                            total_compressed += redacted.len();
                        } else {
                            let events = stdout_pipeline.process(&out_buf[..n]);
                            process_events(events, command_str, context, session, renderer, &mut total_original, &mut total_compressed, false, false);
                        }
                    }
                }
            }
            n = stderr.read(&mut err_buf) => {
                match n? {
                    0 => {
                        let events = stderr_pipeline.process(&[]);
                        process_events(events, command_str, context, session, renderer, &mut total_original, &mut total_compressed, true, raw_mode);
                        break; 
                    },
                    n => {
                        total_original += n;
                        if raw_mode {
                            let text = String::from_utf8_lossy(&err_buf[..n]);
                            let redacted = session.engine.redactor.redact(&text);
                            renderer.render_line(&redacted, true);
                            total_compressed += redacted.len();
                        } else {
                            let events = stderr_pipeline.process(&err_buf[..n]);
                            process_events(events, command_str, context, session, renderer, &mut total_original, &mut total_compressed, true, false);
                        }
                    }
                }
            }
        }
    }

    Ok((total_original, total_compressed))
}

#[allow(clippy::too_many_arguments)]
fn process_events(
    events: Vec<TerminalEvent>,
    command: &str,
    context: &IntentContext,
    session: &mut AxiomSession,
    renderer: &mut dyn OutputRenderer,
    _total_original: &mut usize,
    total_compressed: &mut usize,
    is_stderr: bool,
    _raw_mode: bool,
) {
    for event in events {
        if let Some(processed) = session.engine.process_line(event, command, context) {
            *total_compressed += processed.len();
            renderer.render_line(&processed, is_stderr);
        }
    }
}
