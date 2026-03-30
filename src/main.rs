use clap::{Parser, Subcommand};
use std::env;
use std::process::exit;
use std::path::Path;
use std::io::{self, Write};
use axiom::config::{AxiomConfig, IntelligenceMode};
use axiom::session::AxiomSession;
use axiom::IntentContext;
use axiom::gateway::execute_command;
use axiom::gateway::detective::ProcessDetective;
use axiom::engine::intent_discovery::IntentDiscoverer;
use axiom::engine::installer::AxiomInstaller;

use axiom::engine::updater::AxiomUpdater;

use axiom::engine::doctor::AxiomDoctor;

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

    /// Show raw output, bypassing Axiom synthesis
    #[arg(short, long, global = true)]
    raw: bool,

    /// Automatically answer yes to all prompts
    #[arg(short, long, global = true)]
    yes: bool,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Install Axiom shell integration and AI context
    Install {
        /// Project path to sync AI context (default: current dir)
        #[arg(short, long, default_value = ".")]
        path: String,
        /// Only install AI context, skip shell aliases
        #[arg(long)]
        context_only: bool,
    },
    /// Remove all Axiom traces from the system
    Uninstall {
        /// Project path to remove AI context (default: current dir)
        #[arg(short, long, default_value = ".")]
        path: String,
    },
    /// Run system health check and diagnostics
    Doctor {
        /// Project path to check AI context (default: current dir)
        #[arg(short, long, default_value = ".")]
        path: String,
        /// Attempt to automatically fix detected issues
        #[arg(short, long)]
        fix: bool,
    },
    /// Update Axiom to the latest version from GitHub
    SelfUpdate,
    /// Show the raw output of the last executed command
    Last {
        /// Number of lines to show from the end
        #[arg(short, long)]
        tail: Option<usize>,
        /// Filter lines by a keyword
        #[arg(short, long)]
        grep: Option<String>,
    },
    /// Show token savings analytics
    Gain {
        /// Show detailed savings history
        #[arg(short = 's', long)]
        history: bool,
    },
    /// List or manage currently learned structural templates
    Discovery {
        #[command(subcommand)]
        action: Option<DiscoveryAction>,
    },
    /// Check if current process was called by an AI agent
    CheckAi,
    /// Configuration management
    Config {
        #[command(subcommand)]
        action: Option<ConfigAction>,
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
    /// Show current configuration
    Show,
    /// Set a configuration value (e.g. config set intelligence neural)
    Set {
        key: String,
        value: String,
    },
}

#[derive(Subcommand, Debug)]
enum DiscoveryAction {
    /// List all learned templates (default)
    List,
    /// Clear all learned patterns
    Clear,
    /// Forget a specific template pattern
    Forget {
        /// The template pattern to remove
        pattern: String,
    },
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
            Commands::Install { path, context_only } => {
                let project_path = Path::new(&path);
                if context_only {
                    let context_files = ["GEMINI.md", "AGENTS.md", "CLAUDE.md", ".cursorrules", ".windsurfrules"];
                    for file_name in context_files {
                        let path = project_path.join(file_name);
                        if path.exists() {
                            AxiomInstaller::inject_ai_context(&path, true)?;
                            println!("Synced AI Context in {}", file_name);
                        }
                    }
                } else {
                    AxiomInstaller::run_full_install(Some(project_path), cli.yes)?;
                }
                return Ok(());
            }
            Commands::Uninstall { path } => {
                let project_path = Path::new(&path);
                AxiomInstaller::run_uninstall(Some(project_path), cli.yes)?;
                return Ok(());
            }
            Commands::Doctor { path, fix } => {
                AxiomDoctor::run_diagnostic(Some(Path::new(&path)), fix)?;
                return Ok(());
            }
            Commands::SelfUpdate => {
                println!("Checking for updates...");
                match AxiomUpdater::check_latest() {
                    Ok(Some((tag, url))) => {
                        println!("A new version is available: \x1b[32m{}\x1b[0m", tag);
                        // Using a simple interactive confirmation
                        print!("Do you want to update Axiom? [Y/n] ");
                        io::stdout().flush().unwrap();
                        let mut input = String::new();
                        io::stdin().read_line(&mut input)?;
                        if input.trim().to_lowercase().starts_with('n') {
                            println!("Update cancelled.");
                        } else {
                            AxiomUpdater::run_self_update(&url)?;
                        }
                    }
                    Ok(None) => println!("Axiom is already up to date."),
                    Err(e) => println!("Error checking for updates: {}", e),
                }
                return Ok(());
            }
            Commands::Last { tail, grep } => {
                match session.engine.storage.get_last_logs(tail, grep.as_deref()) {
                    Ok(lines) => {
                        for line in lines {
                            println!("{}", line);
                        }
                    }
                    Err(e) => println!("Error retrieving logs: {}", e),
                }
                return Ok(());
            }
            Commands::Gain { history: _ } => {
                show_savings(&session)?;
                return Ok(());
            }
            Commands::Discovery { action } => {
                match action.unwrap_or(DiscoveryAction::List) {
                    DiscoveryAction::List => {
                        show_discovery(&session)?;
                    }
                    DiscoveryAction::Clear => {
                        session.persistence.clear_templates()?;
                        println!("✅ All learned patterns cleared.");
                    }
                    DiscoveryAction::Forget { pattern } => {
                        session.persistence.delete_template(&pattern)?;
                        println!("✅ Forgot pattern: {}", pattern);
                    }
                }
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
                    Some(ConfigAction::Init) => {
                        let config = AxiomConfig::default();
                        let yaml = serde_yaml::to_string(&config)?;
                        std::fs::write(".axiom.yaml", yaml)?;
                        println!("Created local configuration file: .axiom.yaml");
                    }
                    Some(ConfigAction::Show) => {
                        let yaml = serde_yaml::to_string(&session.config)?;
                        println!("\x1b[1mCurrent Axiom Configuration:\x1b[0m\n");
                        println!("{}", yaml);
                    }
                    Some(ConfigAction::Set { key, value }) => {
                        let mut config = session.config.clone();
                        match key.as_str() {
                            "intelligence" => {
                                config.intelligence_mode = match value.as_str() {
                                    "fuzzy" => IntelligenceMode::Fuzzy,
                                    "neural" => IntelligenceMode::Neural,
                                    "off" => IntelligenceMode::Off,
                                    _ => anyhow::bail!("Invalid mode. Use: fuzzy, neural, off"),
                                };
                            }
                            "markdown" => {
                                config.markdown_enabled = value.parse::<bool>()?;
                            }
                            _ => anyhow::bail!("Key not supported yet via CLI. Edit .axiom.yaml manually."),
                        }
                        let yaml = serde_yaml::to_string(&config)?;
                        std::fs::write(".axiom.yaml", yaml)?;
                        println!("✅ Config updated: {} = {}", key, value);
                    }
                    None => {
                        // Interactive Menu
                        run_interactive_config(&session.config)?;
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
    execute_command(program, cmd_args, &context, &mut session, cli.raw).await?;

    Ok(())
}

fn run_interactive_config(current: &AxiomConfig) -> anyhow::Result<()> {
    loop {
        println!("\n\x1b[1m⚙️ Axiom Interactive Configuration\x1b[0m");
        println!("--------------------------------\n");
        
        println!("1. Intelligence Mode (Current: {:?})", current.intelligence_mode);
        println!("2. Markdown Table Support (Current: {})", current.markdown_enabled);
        println!("3. Telemetry Level (Current: {:?})", current.telemetry_level);
        println!("4. Privacy Patterns (PII)");
        println!("5. Intent Context Sources");
        println!("6. Exit");
        
        print!("\nSelect an option [1-6]: ");
        io::stdout().flush()?;
        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;
        
        let mut new_config = current.clone();
        
        match choice.trim() {
            "1" => {
                println!("\nSelect Intelligence Mode:");
                println!("  a. Fuzzy (Keywords, fast, no downloads)");
                println!("  b. Neural (Embeddings, accurate, requires local model)");
                println!("  c. Off (No semantic filtering)");
                print!("Choice [a/b/c]: ");
                io::stdout().flush()?;
                let mut sub = String::new();
                io::stdin().read_line(&mut sub)?;
                new_config.intelligence_mode = match sub.trim().to_lowercase().as_str() {
                    "a" => IntelligenceMode::Fuzzy,
                    "b" => IntelligenceMode::Neural,
                    "c" => IntelligenceMode::Off,
                    _ => continue,
                };
            }
            "2" => {
                print!("Enable Markdown Tables? [y/n]: ");
                io::stdout().flush()?;
                let mut sub = String::new();
                io::stdin().read_line(&mut sub)?;
                new_config.markdown_enabled = sub.trim().to_lowercase().starts_with('y');
            }
            "3" => {
                println!("\nSelect Telemetry Level:");
                println!("  a. Off (Privacy first)");
                println!("  b. Basic (Only total savings)");
                println!("  c. Discovery (New command patterns)");
                print!("Choice [a/b/c]: ");
                io::stdout().flush()?;
                let mut sub = String::new();
                io::stdin().read_line(&mut sub)?;
                new_config.telemetry_level = match sub.trim().to_lowercase().as_str() {
                    "a" => axiom::config::TelemetryLevel::Off,
                    "b" => axiom::config::TelemetryLevel::Basic,
                    "c" => axiom::config::TelemetryLevel::Discovery,
                    _ => continue,
                };
            }
            "4" => {
                println!("\n\x1b[1mPrivacy Patterns (PII Redaction):\x1b[0m");
                for (i, p) in current.pii_patterns.iter().enumerate() {
                    println!("  {}. {}", i + 1, p);
                }
                println!("\n(To add/remove patterns, please edit .axiom.yaml directly for now.)");
                print!("Press Enter to return...");
                io::stdout().flush()?;
                let _ = io::stdin().read_line(&mut String::new());
                continue;
            }
            "5" => {
                println!("\n\x1b[1mIntent Sources (Context):\x1b[0m");
                for s in &current.intent_sources {
                    println!("  - {}: {:?} ({:?})", s.name, s.path, s.strategy);
                }
                print!("\nPress Enter to return...");
                io::stdout().flush()?;
                let _ = io::stdin().read_line(&mut String::new());
                continue;
            }
            "6" => break,
            _ => continue,
        }
        
        let yaml = serde_yaml::to_string(&new_config)?;
        std::fs::write(".axiom.yaml", yaml)?;
        println!("\n✅ Configuration saved to .axiom.yaml");
        return Ok(());
    }
    Ok(())
}

use axiom::engine::reporting::{EfficiencyReport, ReportRenderer};

fn show_savings(session: &AxiomSession) -> anyhow::Result<()> {
    // 1. Get raw data from persistence (unbounded for total dashboard)
    let raw_data = session.persistence.get_recent_history(10000)?; 
    
    // 2. Process via SOLID reporting module
    let report = EfficiencyReport::new(raw_data);
    
    // 3. Render via renderer
    ReportRenderer::render_dashboard(&report);
    
    Ok(())
}

fn show_discovery(session: &AxiomSession) -> anyhow::Result<()> {
    println!("\x1b[1mLearned Structures (Axiom Discovery):\x1b[0m\n");
    let templates = session.engine.get_learned_templates();
    if templates.is_empty() {
        println!("No structural patterns learned yet.");
    } else {
        for (template, freq) in templates {
            println!("[{}] {}", freq, template);
        }
    }
    Ok(())
}
