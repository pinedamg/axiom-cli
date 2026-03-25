use clap::{Parser, Subcommand};
use std::env;
use std::process::exit;
use axiom::config::{AxiomConfig, IntelligenceMode};
use axiom::session::AxiomSession;
use axiom::IntentContext;
use axiom::gateway::execute_command;
use axiom::gateway::detective::ProcessDetective;
use axiom::engine::intent_discovery::IntentDiscoverer;

#[derive(Parser, Debug)]
#[command(author, version, about = "AXIOM: The Semantic Token Streamer", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// The command to execute (proxy mode)
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    proxy_args: Vec<String>,

    /// Enable Markdown table transformation
    #[arg(short, long, global = true)]
    markdown: bool,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Install Axiom shell integration
    Install,
    /// Show token savings analytics
    Gain {
        /// Show detailed savings history
        #[arg(short = 's', long)]
        history: bool,
    },
    /// List currently learned structural templates
    Discovery,
    /// Check if current process was called by an AI agent
    CheckAi,
    /// Configuration management
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
    /// Manage Intent Discovery and Intelligence Levels
    Intent {
        #[command(subcommand)]
        action: IntentAction,
    },
}

#[derive(Subcommand, Debug)]
enum IntentAction {
    /// Enable intent intelligence (fuzzy or neural)
    Enable {
        /// Intelligence mode: fuzzy (keywords) or neural (AI embeddings)
        #[arg(default_value = "fuzzy")]
        mode: String,
    },
    /// Disable intent intelligence (maintain formatting but show all files)
    Disable,
    /// Show current intent discovery status and relevant files
    Status,
}

#[derive(Subcommand, Debug)]
enum ConfigAction {
    /// Initialize a local .axiom.yaml with default values
    Init,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // 1. Setup Session with consolidated config
    let mut config = AxiomConfig::load();
    if cli.markdown {
        config.markdown_enabled = true;
    }
    
    let mut session = AxiomSession::new(config)?;

    // 1.5 Override config with session-specific settings
    if let Ok(Some(mode_str)) = session.persistence.get_session_intelligence(&session.id) {
        session.config.intelligence_mode = match mode_str.as_str() {
            "off" => IntelligenceMode::Off,
            "fuzzy" => IntelligenceMode::Fuzzy,
            "neural" => IntelligenceMode::Neural,
            _ => session.config.intelligence_mode,
        };
    }

    // 2. Handle Subcommands
    if let Some(cmd) = cli.command {
        match cmd {
            Commands::Install => {
                install_integration()?;
                return Ok(());
            }
            Commands::Gain { history: _ } => {
                show_savings(&session)?;
                return Ok(());
            }
            Commands::Discovery => {
                show_discovery(&session)?;
                return Ok(());
            }
            Commands::CheckAi => {
                if ProcessDetective::is_called_by_ai() {
                    println!("DETECTED: AI Agent ({})", ProcessDetective::get_parent_name());
                    exit(0);
                } else {
                    println!("DETECTED: Human Shell ({})", ProcessDetective::get_parent_name());
                    exit(1);
                }
            }
            Commands::Intent { action } => {
                match action {
                    IntentAction::Enable { mode } => {
                        let normalized_mode = mode.to_lowercase();
                        if normalized_mode != "fuzzy" && normalized_mode != "neural" {
                            println!("Error: Invalid mode '{}'. Use 'fuzzy' or 'neural'.", mode);
                            exit(1);
                        }
                        session.persistence.set_session_intelligence(&session.id, &normalized_mode)?;
                        println!("Intent Discovery ENABLED (Mode: {})", normalized_mode);
                    }
                    IntentAction::Disable => {
                        session.persistence.set_session_intelligence(&session.id, "off")?;
                        println!("Intent Discovery DISABLED (Mode: off)");
                    }
                    IntentAction::Status => {
                        let mode = session.config.intelligence_mode;
                        println!("\x1b[1mAXIOM Intent Status\x1b[0m");
                        println!("---------------------");
                        println!("Session ID:        {}", session.id);
                        println!("Intelligence Mode: {:?}", mode);
                        println!("Parent Process:    {}", ProcessDetective::get_parent_name());
                        
                        if mode != IntelligenceMode::Off {
                            let intent = IntentDiscoverer::discover(&session.config.intent_sources).unwrap_or_default();
                            println!("Last Intent:       \"{}\"", intent);
                        }
                    }
                }
                return Ok(());
            }
            Commands::Config { action } => {
                match action {
                    ConfigAction::Init => {
                        let config = AxiomConfig::default();
                        let yaml = serde_yaml::to_string(&config)?;
                        std::fs::write(".axiom.yaml", yaml)?;
                        println!("Created local configuration file: .axiom.yaml");
                    }
                }
                return Ok(());
            }
        }
    }

    // 3. Fallback to Proxy Mode
    if cli.proxy_args.is_empty() {
        println!("Axiom: No command provided. Usage: axiom <cmd> [args] or axiom install");
        return Ok(());
    }

    // Auto-detect intent
    let intent = env::var("AXIOM_CONTEXT")
        .ok()
        .or_else(|| IntentDiscoverer::discover(&session.config.intent_sources))
        .unwrap_or_else(|| "Automated Session".to_string());

    // Enrich keywords with Git context
    let mut keywords = session.config.intent_keywords.clone();
    keywords.extend(IntentDiscoverer::get_git_context());

    let context = IntentContext {
        last_message: intent.clone(),
        command: cli.proxy_args.join(" "),
        keywords,
    };

    // 4. Prepare Intelligence Engine
    let _ = session.engine.prepare_session(&intent);

    // 5. Execute
    let program = &cli.proxy_args[0];
    let cmd_args = &cli.proxy_args[1..];
    execute_command(program, cmd_args, &context, &mut session).await?;

    Ok(())
}

fn install_integration() -> anyhow::Result<()> {
    println!("\x1b[1mAXIOM Shell Integration\x1b[0m");
    println!("------------------------");
    println!("To enable Axiom automatically, add this to your shell config:\n");
    println!("alias git='axiom git'");
    println!("alias ls='axiom ls'");
    Ok(())
}

fn show_savings(session: &AxiomSession) -> anyhow::Result<()> {
    let (original, compressed) = session.persistence.get_total_savings()?;
    let saved = original.saturating_sub(compressed);
    let ratio = if original > 0 { (saved as f64 / original as f64) * 100.0 } else { 0.0 };
    println!("Total Saved: {} chars ({:.1}%)", saved, ratio);
    Ok(())
}

fn show_discovery(session: &AxiomSession) -> anyhow::Result<()> {
    println!("Learned Structures:");
    for (template, freq) in session.engine.get_learned_templates() {
        println!("[{}] {}", freq, template);
    }
    Ok(())
}
