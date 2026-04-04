use std::io::{self, Write};
use std::path::Path;
use crate::config::AxiomConfig;
use crate::session::AxiomSession;
use crate::gateway::detective::ProcessDetective;
use crate::engine::telemetry::Telemetry;
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
                println!(
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

        if auto_yes {
            return Self::apply_profile(1, project_path, funnel_id);
        }

        println!("\x1b[1mSelect your installation profile:\x1b[0m\n");
        
        println!("  \x1b[32m[1] Full Automation (Recommended)\x1b[0m");
        println!("      • \x1b[1mWhat changes:\x1b[0m Adds aliases to your shell config and installs shims in ~/.axiom/bin.");
        println!("      • \x1b[1mWhy:\x1b[0m You don't need to change your habits. Commands like 'npm' or 'git' will");
        println!("        automatically be filtered to save context and protect secrets.");
        println!();

        println!("  \x1b[33m[2] Manual (Proxy Mode)\x1b[0m");
        println!("      • \x1b[1mWhat changes:\x1b[0m ZERO system-wide changes. Only installs the binary.");
        println!("      • \x1b[1mWhy:\x1b[0m You prefer full control. You only use Axiom when you explicitly type");
        println!("        the 'axiom' prefix (e.g., 'axiom docker logs').");
        println!();

        println!("  \x1b[34m[3] AI Integration Only\x1b[0m");
        println!("      • \x1b[1mWhat changes:\x1b[0m Only injects AI-specific rules (.md files) into this project.");
        println!("      • \x1b[1mWhy:\x1b[0m You want your AI Agent (Cursor/Claude) to know how to optimize its own");
        println!("        context usage without changing your local terminal experience.");
        println!();
        
        println!("\x1b[2m💡 Note: You can always undo these changes by running 'axiom uninstall'.\x1b[0m\n");

        print!("\x1b[1mSelect profile [1-3, Default: 1]: \x1b[0m");
        io::stdout().flush()?;
        
        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;
        let choice_num = choice.trim().parse::<u8>().unwrap_or(1);

        Self::apply_profile(choice_num, project_path, funnel_id)
    }

    pub fn apply_profile(choice: u8, project_path: Option<&Path>, funnel_id: Option<String>) -> anyhow::Result<()> {
        let configs = AxiomInstaller::get_shell_configs();
        
        // Load config (this triggers node registration if first run)
        // If funnel_id is present, it will be used during registration
        if let Some(fid) = &funnel_id {
            std::env::set_var("AXIOM_FUNNEL_ID", fid);
        }
        let axiom_config = AxiomConfig::load();

        let profile_name = match choice {
            1 => "full",
            2 => "manual",
            3 => "ai_only",
            _ => "unknown",
        };
        
        // 1. Report Choice to Telemetry (Event)
        Telemetry::report_event(
            &axiom_config, 
            "install_profile_selected", 
            Some(profile_name), 
            choice as usize, 
            0, 
            None
        );

        // 2. Update Node Profile in Pulse (Permanent Column)
        let endpoint = format!("{}/v1/node/profile", axiom_config.get_pulse_endpoint());
        let payload = serde_json::json!({
            "iid": axiom_config.node_id,
            "profile": profile_name
        });
        let _ = ureq::post(&endpoint).send_json(payload);

        match choice {
            1 => {
                println!("\n\x1b[1mApplying Full Automation...\x1b[0m");
                if !configs.is_empty() {
                    for path in &configs {
                        print!("  - Configuring {} ... ", path.display());
                        let _ = AxiomInstaller::install_shell_integration(path, true);
                        println!("✅");
                    }
                }
                print!("  - Installing Global Shims in ~/.axiom/bin ... ");
                let _ = AxiomInstaller::install_shims()?;
                println!("✅");
                Self::sync_context(project_path)?;
            }
            2 => {
                println!("\n\x1b[1mApplying Manual Mode...\x1b[0m");
                println!("  - No system changes made.");
                println!("  - Use 'axiom <command>' to filter manually.");
                Self::sync_context(project_path)?;
            }
            3 => {
                println!("\n\x1b[1mApplying AI Integration Only...\x1b[0m");
                Self::sync_context(project_path)?;
            }
            _ => return Self::apply_profile(1, project_path, funnel_id),
        }

        println!("\n\x1b[1;32mInstallation Complete!\x1b[0m");
        println!("\x1b[2mTry running 'axiom doctor' to verify your environment status.\x1b[0m");
        Ok(())
    }

    fn sync_context(project_path: Option<&Path>) -> anyhow::Result<()> {
        if let Some(root) = project_path {
            println!("  - Syncing AI Context Rules...");
            let context_files = ["GEMINI.md", "AGENTS.md", "CLAUDE.md", ".cursorrules", ".windsurfrules"];
            for file_name in context_files {
                let path = root.join(file_name);
                if path.exists() || file_name == "CLAUDE.md" || file_name == "AGENTS.md" {
                    print!("    - Updating {} ... ", file_name);
                    let _ = AxiomInstaller::inject_ai_context(&path, true);
                    println!("✅");
                }
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
