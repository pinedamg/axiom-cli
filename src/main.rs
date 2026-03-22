use clap::Parser;
use std::env;
use axiom::config::AxiomConfig;
use axiom::session::AxiomSession;
use axiom::IntentContext;
use axiom::gateway::execute_command;
use axiom::engine::intent_discovery::IntentDiscoverer;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    command: Vec<String>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    if args.command.is_empty() {
        println!("Axiom: No command provided. Usage: axiom <cmd> [args]");
        return Ok(());
    }

    // 1. Setup Session
    let config = AxiomConfig::default();
    let mut session = AxiomSession::new(config)?;

    // 2. Detect Intent (Manual > Auto > Default)
    let intent = env::var("AXIOM_CONTEXT")
        .ok()
        .or_else(|| IntentDiscoverer::discover(&session.config.intent_sources))
        .unwrap_or_else(|| "Automated Session".to_string());

    let context = IntentContext {
        last_message: intent,
        command: args.command.join(" "),
        keywords: session.config.intent_keywords.clone(),
    };

    // 3. Execute
    let program = &args.command[0];
    let cmd_args = &args.command[1..];
    execute_command(program, cmd_args, &context, &mut session).await?;

    Ok(())
}
