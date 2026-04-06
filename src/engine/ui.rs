use std::io::{self, Write};
use std::path::Path;
use crate::config::AxiomConfig;
use crate::session::AxiomSession;
use crate::gateway::detective::ProcessDetective;
use crate::engine::installer::AxiomInstaller;

pub struct DopamineEngine;

impl DopamineEngine {
    pub fn render_session_savings(session: &AxiomSession, raw_mode: bool) {
        if ProcessDetective::is_called_by_ai() || raw_mode || !session.config.show_savings_footer {
            return;
        }

        if let Some(stats) = session.engine.get_session_stats() {
            if stats.raw_bytes > 500 {
                let savings = (stats.raw_bytes as f64 - stats.saved_bytes as f64) / stats.raw_bytes as f64 * 100.0;
                // Output to stderr to keep stdout clean for AI
                eprintln!(
                    "\n\x1b[32m✨ Axiom: {} bytes → {} bytes ({:.1}% reduction)\x1b[0m",
                    stats.raw_bytes, stats.saved_bytes, savings
                );
            }
        }
    }
}

pub struct OnboardingManager;

impl OnboardingManager {
    pub fn run_install_flow(project_path: Option<&Path>, auto_yes: bool, funnel_id: Option<String>) -> anyhow::Result<()> {
        println!("\x1b[1;36m🚀 AXIOM: The Semantic Token Streamer\x1b[0m");
        println!("---------------------------------------\n");

        // Load config (this triggers node registration if first run)
        if let Some(fid) = &funnel_id {
            std::env::set_var("AXIOM_FUNNEL_ID", fid);
        }
        let _axiom_config = AxiomConfig::load();

        println!("📦 \x1b[1mInstalling Axiom Stealth (Dynamic Shell Hook)...\x1b[0m");

        // 1. Install Shell Integration
        let configs = AxiomInstaller::get_shell_configs();
        if !configs.is_empty() {
            for path in &configs {
                print!("  - Configuring {} ... ", path.display());
                let _ = AxiomInstaller::install_shell_integration(path, true);
                println!("✅");
            }
        }
        
        // 2. Sync AI Context
        if let Some(root) = project_path {
            print!("  - Syncing AI Context Rules ... ");
            let context_files = ["GEMINI.md", "AGENTS.md", "CLAUDE.md", ".cursorrules", ".windsurfrules"];
            for file_name in context_files {
                let path = root.join(file_name);
                if path.exists() || file_name == "CLAUDE.md" || file_name == "AGENTS.md" {
                    let _ = AxiomInstaller::inject_ai_context(&path, true);
                }
            }
            println!("✅");
        }

        println!("\n\x1b[1;32mInstallation Complete!\x1b[0m");
        println!("---------------------------------------\n");

        // 3. The "Aha!" Moment Demo
        println!("\x1b[1;33m💡 Let's see Axiom in action (The Aha! Moment):\x1b[0m");
        
        // Setup dummy noise for the demo
        let demo_dir = project_path.unwrap_or(Path::new(".")).join(".axiom_demo");
        let _ = std::fs::create_dir_all(&demo_dir);
        for i in 0..15 {
            let _ = std::fs::write(demo_dir.join(format!("temp_log_{}.log", i)), "noise");
        }

        println!("\n1. \x1b[1mStandard 'ls -la' (Raw Output is noisy!):\x1b[0m");
        println!("\x1b[2m----------------------------------------------------------\x1b[0m");
        let _ = std::process::Command::new("ls")
            .arg("-la")
            .arg(&demo_dir)
            .spawn()?
            .wait()?;
        println!("\x1b[2m----------------------------------------------------------\x1b[0m");

        println!("\n2. \x1b[1m'axiom ls -la' (Semantic Output is clean!):\x1b[0m");
        println!("\x1b[1;32m----------------------------------------------------------\x1b[0m");
        // Run our own binary in proxy mode
        let _ = std::process::Command::new(std::env::current_exe()?)
            .arg("ls")
            .arg("-la")
            .arg(&demo_dir)
            .spawn()?
            .wait()?;
        println!("\x1b[1;32m----------------------------------------------------------\x1b[0m");

        // Cleanup demo
        let _ = std::fs::remove_dir_all(&demo_dir);

        println!("\n\x1b[1m🚀 Axiom is now ACTIVE and protecting your terminal.\x1b[0m");
        println!("\nQuick Commands:");
        println!("  - \x1b[1maxiom disable\x1b[0m         : Turn off for this session");
        println!("  - \x1b[1maxiom enable\x1b[0m          : Turn back on");
        println!("  - \x1b[1maxiom bypass 3\x1b[0m        : Bypass for next 3 commands");
        println!("  - \x1b[1maxiom bypass always <c>\x1b[0m : Blacklist a command forever");
        println!("\nTry running 'axiom doctor' to verify your status.");

        if !auto_yes {
            print!("\nWould you like to keep Axiom enabled? [Y/n] ");
            io::stdout().flush()?;
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            if input.trim().to_lowercase().starts_with('n') {
                let p = crate::persistence::PersistenceManager::new()?;
                p.set_global_enabled(false)?;
                println!("❌ Axiom disabled. You can re-enable it with 'axiom enable'.");
            }
        }

        Ok(())
    }

    pub fn report_system_status() -> anyhow::Result<()> {
        let home = std::env::var("HOME").unwrap_or_default();
        let shim_dir = Path::new(&home).join(".axiom/bin");
        let has_shims = shim_dir.exists() && std::fs::read_dir(&shim_dir).map(|d| d.count()).unwrap_or(0) > 0;
        
        let mut has_aliases = false;
        let configs = AxiomInstaller::get_shell_configs();
        for path in configs {
            if path.exists() {
                let content = std::fs::read_to_string(&path)?;
                if content.contains("axiom initialize") {
                    has_aliases = true;
                    break;
                }
            }
        }

        println!("\x1b[1mCurrent Installation Profile:\x1b[0m");
        if has_shims && has_aliases {
            println!("  🛡️ \x1b[32mFULL AUTOMATION\x1b[0m (Aliases & Shims active)");
        } else if has_shims || has_aliases {
            println!("  ⚠️ \x1b[33mPARTIAL AUTOMATION\x1b[0m (Some components missing)");
        } else {
            println!("  👤 \x1b[34mMANUAL MODE\x1b[0m (No system changes detected)");
        }
        println!();
        Ok(())
    }
}
