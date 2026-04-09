pub mod core;
pub mod process;
pub mod render;
pub mod stream;
pub mod detective;
pub mod filters;

use crate::IntentContext;
use crate::session::AxiomSession;
use crate::gateway::core::OutputRenderer;
use crate::gateway::render::TtyRenderer;

/// Executes a command under Axiom's supervision.
pub async fn execute_command(
    program: &str,
    args: &[String],
    context: &IntentContext,
    session: &mut AxiomSession,
    raw_mode: bool,
) -> anyhow::Result<()> {
    let mut child = process::spawn_child(program, args)?;
    let command_str = format!("{} {}", program, args.join(" "));
    let mut renderer = TtyRenderer;

    let (total_original, total_compressed) = stream::stream_io(
        &mut child, 
        &command_str, 
        context, 
        session, 
        &mut renderer,
        raw_mode
    ).await?;

    if !raw_mode {
        let summaries = session.engine.flush_summaries();
        for s in &summaries {
            session.engine.trace_summary(s);
        }
        renderer.render_summary(&summaries, false);
    }

    session.finalize(&command_str, total_original, total_compressed)?;
    child.wait().await?;
    Ok(())
}
