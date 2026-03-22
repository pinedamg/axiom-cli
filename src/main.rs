use clap::{Parser, Subcommand};
use std::env;
use axiom::config::AxiomConfig;
use axiom::session::AxiomSession;
use axiom::IntentContext;
use axiom::gateway::execute_command;
use axiom::engine::intent_discovery::IntentDiscoverer;

#[derive(Parser, Debug)]
#[command(author, version, about = "AXIOM: The Semantic Token Streamer", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// The command to execute (proxy mode)
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    proxy_args: Vec<String>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Install Axiom shell integration
    Install,
    /// Show token savings analytics
    Gain,
    /// List currently learned structural templates
    Discovery,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // 1. Setup Session
    let config = AxiomConfig::default();
    let mut session = AxiomSession::new(config)?;

    // 2. Handle Subcommands
    if let Some(cmd) = cli.command {
        match cmd {
            Commands::Install => {
                install_integration()?;
                return Ok(());
            }
            Commands::Gain => {
                show_savings(&session)?;
                return Ok(());
            }
            Commands::Discovery => {
                show_discovery(&session)?;
                return Ok(());
            }
        }
    }

    // 3. Fallback to Proxy Mode
    if cli.proxy_args.is_empty() {
        println!("Axiom: No command provided. Usage: axiom <cmd> [args] or axiom install");
        return Ok(());
    }

    let intent = env::var("AXIOM_CONTEXT")
        .ok()
        .or_else(|| IntentDiscoverer::discover(&session.config.intent_sources))
        .unwrap_or_else(|| "Automated Session".to_string());

    let context = IntentContext {
        last_message: intent,
        command: cli.proxy_args.join(" "),
        keywords: session.config.intent_keywords.clone(),
    };

    let program = &cli.proxy_args[0];
    let cmd_args = &cli.proxy_args[1..];
    execute_command(program, cmd_args, &context, &mut session).await?;

    Ok(())
}

fn install_integration() -> anyhow::Result<()> {
    println!("\x1b[1mAXIOM Shell Integration\x1b[0m");
    println!("------------------------");
    println!("To enable Axiom automatically for common commands, add this to your .bashrc or .zshrc:\n");
    println!("alias git='axiom git'");
    println!("alias docker='axiom docker'");
    println!("alias npm='axiom npm'");
    println!("alias cargo='axiom cargo'");
    println!("\nThen restart your terminal or run: source ~/.bashrc");
    Ok(())
}

fn show_savings(session: &AxiomSession) -> anyhow::Result<()> {
    let (original, compressed) = session.persistence.get_total_savings()?;
    let saved = original.saturating_sub(compressed);
    let ratio = if original > 0 { (saved as f64 / original as f64) * 100.0 } else { 0.0 };
    
    // Estimate tokens (avg 4 chars per token)
    let tokens_saved = saved / 4;

    println!("\x1b[1mAXIOM Token Savings Analytics\x1b[0m");
    println!("------------------------------");
    println!("Total Streamed:    {:>10} chars", original);
    println!("Total Compressed:  {:>10} chars", compressed);
    println!("\x1b[32;1mTotal Saved:       {:>10} chars ({:.1}%)\x1b[0m", saved, ratio);
    println!("------------------------------");
    println!("Estimated Tokens Saved: \x1b[36;1m{}\x1b[0m", tokens_saved);
    println!("Estimated USD Saved:    \x1b[33;1m${:.4}\x1b[0m (avg $0.01 per 1k tokens)", tokens_saved as f64 * 0.00001);
    Ok(())
}

fn show_discovery(session: &AxiomSession) -> anyhow::Result<()> {
    println!("\x1b[1mAXIOM Learned Structures\x1b[0m");
    println!("-------------------------");
    let templates = session.engine.get_learned_templates();
    if templates.is_empty() {
        println!("No structural patterns learned yet. Run some commands first!");
    } else {
        for (template, frequency) in templates {
            println!("[{}] {}", frequency, template);
        }
    }
    Ok(())
}
