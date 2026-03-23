use clap::{Parser, Subcommand};
use std::env;
use std::process::exit;
use axiom::config::{AxiomConfig, TelemetryLevel};
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
    /// Show current configuration and telemetry status
    Status,
    /// Configuration management
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
}

#[derive(Subcommand, Debug)]
enum ConfigAction {
    /// Set telemetry level (anonymous, discovery, full, off)
    Telemetry { level: String },
    /// Register Pro license key
    License { key: String },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // 1. Setup Session (Persistent)
    let is_first_run = !AxiomConfig::config_path().exists();
    let mut config = AxiomConfig::load();
    
    // 2. Handle Subcommands
    if let Some(cmd) = cli.command {
        match cmd {
            Commands::Install => {
                install_integration()?;
                return Ok(());
            }
            Commands::Gain { history: _ } => {
                let session = AxiomSession::new(config)?;
                show_savings(&session)?;
                return Ok(());
            }
            Commands::Discovery => {
                let session = AxiomSession::new(config)?;
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
            Commands::Status => {
                let session = AxiomSession::new(config)?;
                show_status(&session)?;
                return Ok(());
            }
            Commands::Config { action } => {
                match action {
                    ConfigAction::Telemetry { level } => {
                        let new_level = match level.to_lowercase().as_str() {
                            "off" => {
                                if !config.is_pro {
                                    println!("\x1b[31;1mERROR: Telemetry 'OFF' is a Pro feature.\x1b[0m");
                                    println!("Axiom Community requires at least 'anonymous' telemetry to improve the tool.");
                                    println!("Get a Pro license at: \x1b[34mhttps://axiom.mpineda.com.ar/pro\x1b[0m");
                                    None
                                } else {
                                    println!("Telemetry set to OFF. We will miss you!");
                                    Some(TelemetryLevel::Off)
                                }
                            }
                            "anonymous" => Some(TelemetryLevel::Anonymous),
                            "discovery" => Some(TelemetryLevel::Discovery),
                            "full" => Some(TelemetryLevel::Full),
                            _ => {
                                println!("Unknown telemetry level. Use: anonymous, discovery, full, off");
                                None
                            }
                        };
                        
                        if let Some(l) = new_level {
                            config.telemetry_level = l;
                            config.save()?;
                            println!("Telemetry level saved: {:?}", l);
                        }
                    }
                    ConfigAction::License { key } => {
                        println!("Validating license key: {}...", key);
                        // Future: Real validation
                        config.is_pro = true; // Placeholder for alpha
                        config.license_key = Some(key);
                        config.save()?;
                        println!("\x1b[32;1mLicense activated! You are now an Axiom Pro user.\x1b[0m");
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

    // Show first-run informational message (Transparently)
    if is_first_run {
        println!("\x1b[34m[AXIOM] Telemetry is active (Full) to improve the tool. \x1b[0m");
        println!("\x1b[34m[AXIOM] Check 'axiom status' or 'axiom config' to learn more.\x1b[0m\n");
    }

    let mut session = AxiomSession::new(config)?;
    if cli.markdown {
        session.engine.set_markdown_mode(true);
    }

    // Auto-detect intent
    let intent = env::var("AXIOM_CONTEXT")
        .ok()
        .or_else(|| IntentDiscoverer::discover(&session.config.intent_sources))
        .unwrap_or_else(|| "Automated Session".to_string());

    let mut keywords = session.config.intent_keywords.clone();
    keywords.extend(IntentDiscoverer::get_git_context());

    let context = IntentContext {
        last_message: intent,
        command: cli.proxy_args.join(" "),
        keywords,
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
    println!("alias git='if axiom check-ai > /dev/null; then axiom git; else git; fi'");
    println!("alias npm='if axiom check-ai > /dev/null; then axiom npm; else npm; fi'");
    println!("\nThen restart your terminal or run: source ~/.bashrc");
    Ok(())
}

fn show_savings(session: &AxiomSession) -> anyhow::Result<()> {
    let (original, compressed) = session.persistence.get_total_savings()?;
    let saved = original.saturating_sub(compressed);
    let ratio = if original > 0 { (saved as f64 / original as f64) * 100.0 } else { 0.0 };
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

fn show_status(session: &AxiomSession) -> anyhow::Result<()> {
    let config = &session.config;
    let version = env!("CARGO_PKG_VERSION");
    let edition = if config.is_pro { "\x1b[35;1mPRO EDITION\x1b[0m" } else { "\x1b[32;1mCOMMUNITY EDITION\x1b[0m" };

    println!("\x1b[1mAXIOM System Status\x1b[0m");
    println!("-------------------");
    println!("Version:    {} ({})", version, edition);
    println!("Inst. ID:   {}", config.installation_id);
    println!("Telemetry:  {:?}", config.telemetry_level);
    
    println!("\n\x1b[1mTelemetry Transparency\x1b[0m");
    println!("----------------------");
    match config.telemetry_level {
        TelemetryLevel::Off => {
            println!("Status: \x1b[31mREDACTED\x1b[0m (No data is leaving this machine)");
        }
        _ => {
            println!("Sharing:    Anonymous savings, OS, and Architecture");
            if config.telemetry_level as u8 >= TelemetryLevel::Discovery as u8 {
                println!("Discovery:  Binary names (e.g., 'git', 'npm', 'docker')");
                println!("Sanitize:   \x1b[32mENABLED\x1b[0m (Arguments like 'commit -m \"secret\"' are REMOVED)");
            }
            if config.telemetry_level == TelemetryLevel::Full {
                println!("Metrics:    Processing time and rule match IDs");
            }
        }
    }

    println!("\n\x1b[1mLocal Persistence\x1b[0m");
    println!("-----------------");
    println!("Database:   {}", config.db_path.display());
    let (orig, comp) = session.persistence.get_total_savings()?;
    println!("Savings:    {} chars saved locally", orig.saturating_sub(comp));

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
