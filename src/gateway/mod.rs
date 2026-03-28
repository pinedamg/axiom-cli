pub mod detective;
use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use crate::IntentContext;
use crate::session::AxiomSession;

/// Executes a command under Axiom's supervision.
pub async fn execute_command(
    program: &str,
    args: &[String],
    context: &IntentContext,
    session: &mut AxiomSession,
    raw_mode: bool,
) -> anyhow::Result<()> {
    let mut child = Command::new(program)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let stdout = child.stdout.take().ok_or_else(|| anyhow::anyhow!("Failed to capture stdout"))?;
    let stderr = child.stderr.take().ok_or_else(|| anyhow::anyhow!("Failed to capture stderr"))?;

    let command_str = format!("{} {}", program, args.join(" "));

    let mut stdout_lines = BufReader::new(stdout).lines();
    let mut stderr_lines = BufReader::new(stderr).lines();

    let mut total_original = 0;
    let mut total_compressed = 0;

    loop {
        tokio::select! {
            line = stdout_lines.next_line() => {
                match line? {
                    Some(l) => {
                        if raw_mode {
                            println!("{}", l);
                            total_original += l.len();
                            total_compressed += l.len();
                        } else {
                            process_line_output(&l, &command_str, context, session, &mut total_original, &mut total_compressed, false);
                        }
                    },
                    None => break,
                }
            }
            line = stderr_lines.next_line() => {
                match line? {
                    Some(l) => {
                        if raw_mode {
                            eprintln!("{}", l);
                            total_original += l.len();
                            total_compressed += l.len();
                        } else {
                            process_line_output(&l, &command_str, context, session, &mut total_original, &mut total_compressed, true);
                        }
                    },
                    None => {}, 
                }
            }
        }
    }

    if !raw_mode {
        // Flush final summaries and insights once the command execution is finished
        flush_and_print_summaries(session, false);
    }

    session.finalize(&command_str, total_original, total_compressed)?;
    child.wait().await?;
    Ok(())
}

use std::io::Write;

fn safe_print(msg: &str, is_stderr: bool) {
    let result = if is_stderr {
        let mut stderr = std::io::stderr().lock();
        writeln!(stderr, "{}", msg)
    } else {
        let mut stdout = std::io::stdout().lock();
        writeln!(stdout, "{}", msg)
    };

    if let Err(e) = result {
        if e.kind() == std::io::ErrorKind::BrokenPipe {
            std::process::exit(0);
        }
    }
}

fn process_line_output(
    line: &str,
    command: &str,
    context: &IntentContext,
    session: &mut AxiomSession,
    total_original: &mut usize,
    total_compressed: &mut usize,
    is_stderr: bool,
) {
    *total_original += line.len();
    if let Some(processed) = session.engine.process_line(line, command, context) {
        *total_compressed += processed.len();
        safe_print(&processed, is_stderr);
    }
}

fn flush_and_print_summaries(session: &mut AxiomSession, is_stderr: bool) {
    let summaries = session.engine.flush_summaries();
    if summaries.is_empty() { return; }

    // Compact Header for token efficiency
    let header = "\x1b[1;33m[AXIOM]\x1b[0m";
    safe_print(header, is_stderr);

    for summary in summaries {
        let msg = format!("\x1b[33m• {}\x1b[0m", summary);
        safe_print(&msg, is_stderr);
    }
}
