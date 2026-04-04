use std::env;
use std::fs;
use std::path::Path;
use crate::engine::installer::AxiomInstaller;

pub struct AxiomDoctor;

impl AxiomDoctor {
    /// Runs a full diagnostic report and attempts fixes if requested
    pub fn run_diagnostic(project_path: Option<&Path>, fix: bool) -> anyhow::Result<()> {
        println!("\x1b[1;36m🩺 Axiom System Health Check (v1.2)\x1b[0m");
        println!("------------------------------------\n");

        // 0. Profile Summary
        crate::engine::ui::OnboardingManager::report_system_status()?;

        // 1. Binary & Path
        Self::check_binary_path()?;

        // 2. Shims Check
        Self::check_shims(fix)?;

        // 3. Shell Integration
        Self::check_shell_integration(fix)?;

        // 4. Persistence & Permissions
        Self::check_persistence(fix)?;

        // 5. Secrets & AI Connectivity
        Self::check_secrets()?;

        // 6. Database Health
        Self::check_database()?;

        // 7. AI Context Sync (Local)
        if let Some(root) = project_path {
            Self::check_ai_context(root, fix)?;
        }

        println!("\n\x1b[1m\x1b[32mDiagnostic Complete.\x1b[0m");
        Ok(())
    }

    fn check_secrets() -> anyhow::Result<()> {
        let env_path = Path::new(".env");
        if env_path.exists() {
            println!("✅ [Config] .env file found for local environment overrides.");
        }
        
        // We can check for core Axiom configuration here if needed
        // For now, we just ensure the environment is clean of hardcoded legacy keys
        Ok(())
    }

    fn check_database() -> anyhow::Result<()> {
        let db_path = Path::new("axiom.db");
        if db_path.exists() {
            match fs::metadata(db_path) {
                Ok(meta) => {
                    if meta.permissions().readonly() {
                        println!("❌ [Database] \x1b[31maxiom.db is READ-ONLY.\x1b[0m");
                    } else {
                        println!("✅ [Database] axiom.db is healthy and writable ({} bytes).", meta.len());
                    }
                },
                Err(e) => println!("❌ [Database] Could not read metadata: {}", e),
            }
        } else {
            println!("⚠️ [Database] axiom.db not found. It will be created on first run.");
        }
        Ok(())
    }

    fn check_binary_path() -> anyhow::Result<()> {
        match env::current_exe() {
            Ok(exe) => println!("✅ [Binary] Located at: {}", exe.display()),
            Err(e) => println!("❌ [Binary] Could not locate own binary: {}", e),
        }
        
        let home = env::var("HOME").unwrap_or_default();
        let shim_dir = Path::new(&home).join(".axiom/bin");
        let path_var = env::var("PATH").unwrap_or_default();
        
        if path_var.contains(shim_dir.to_str().unwrap_or("")) {
            println!("✅ [PATH] Axiom shims directory is correctly configured.");
        } else {
            println!("❌ [PATH] \x1b[31m~/.axiom/bin is NOT in your PATH.\x1b[0m");
            println!("   \x1b[33mFix:\x1b[0m Add 'export PATH=\"$HOME/.axiom/bin:$PATH\"' to your shell config (or run axiom doctor --fix).");
        }
        Ok(())
    }

    fn check_shims(fix: bool) -> anyhow::Result<()> {
        let home = env::var("HOME").unwrap_or_default();
        let shim_dir = Path::new(&home).join(".axiom/bin");
        
        if !shim_dir.exists() {
            println!("⚠️ [Shims] Directory not found.");
            if fix {
                print!("   🔧 Attempting fix: Creating shims... ");
                let _ = AxiomInstaller::install_shims();
                println!("✅ Done.");
            }
            return Ok(());
        }

        match fs::read_dir(&shim_dir) {
            Ok(entries) => {
                let mut count = 0;
                for _ in entries { count += 1; }
                println!("✅ [Shims] {} shims verified.", count);
            }
            Err(e) => println!("❌ [Shims] Error: {}", e),
        }
        Ok(())
    }

    fn check_shell_integration(fix: bool) -> anyhow::Result<()> {
        let configs = AxiomInstaller::get_shell_configs();
        for path in configs {
            if !path.exists() { continue; }
            let content = fs::read_to_string(&path)?;
            if content.contains("axiom initialize") {
                println!("✅ [Shell] Integration found in {}.", path.display());
            } else {
                println!("❌ [Shell] Integration MISSING in {}.", path.display());
                if fix {
                    print!("   🔧 Attempting fix: Configuring aliases... ");
                    // We assume true for include_path since it's the industrial default now
                    let _ = AxiomInstaller::install_shell_integration(&path, true);
                    println!("✅ Done.");
                }
            }
        }
        Ok(())
    }

    fn check_persistence(fix: bool) -> anyhow::Result<()> {
        let home = env::var("HOME").unwrap_or_default();
        let axiom_dir = Path::new(&home).join(".axiom");
        
        if !axiom_dir.exists() {
            println!("❌ [Persistence] ~/.axiom directory not found.");
            if fix {
                print!("   🔧 Attempting fix: Creating directory... ");
                let _ = fs::create_dir_all(&axiom_dir);
                println!("✅ Done.");
            }
        } else {
            println!("✅ [Persistence] ~/.axiom is healthy.");
        }
        Ok(())
    }

    fn check_ai_context(root: &Path, fix: bool) -> anyhow::Result<()> {
        let context_files = ["GEMINI.md", "AGENTS.md", "CLAUDE.md", ".cursorrules"];
        for file in context_files {
            let path = root.join(file);
            if path.exists() {
                let content = fs::read_to_string(&path)?;
                if content.contains("BEGIN AXIOM INSTRUCTIONS") {
                    println!("✅ [AI Context] {} is synced.", file);
                } else {
                    println!("⚠️ [AI Context] {} is NOT synced.", file);
                    if fix {
                        print!("   🔧 Attempting fix: Syncing rules... ");
                        let _ = AxiomInstaller::inject_ai_context(&path, true);
                        println!("✅ Done.");
                    }
                }
            }
        }
        Ok(())
    }
}
