use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use crate::engine::installer::AxiomInstaller;

pub struct AxiomDoctor;

impl AxiomDoctor {
    /// Runs a full diagnostic report without aborting on individual check failures
    pub fn run_diagnostic(project_path: Option<&Path>) -> anyhow::Result<()> {
        println!("\x1b[1m🩺 Axiom System Health Check (v1.1)\x1b[0m");
        println!("------------------------------------\n");

        // 1. Binary & Path
        let _ = Self::check_binary_path();

        // 2. Shims Check
        let _ = Self::check_shims();

        // 3. Shell Integration
        let _ = Self::check_shell_integration();

        // 4. Persistence & Permissions
        let _ = Self::check_persistence();

        // 5. AI Context Sync (Local)
        if let Some(root) = project_path {
            let _ = Self::check_ai_context(root);
        }

        println!("\n\x1b[1m\x1b[32mDiagnostic Complete.\x1b[0m");
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
            println!("   \x1b[33mFix:\x1b[0m Add 'export PATH=\"$HOME/.axiom/bin:$PATH\"' to your shell config.");
        }
        Ok(())
    }

    fn check_shims() -> anyhow::Result<()> {
        let home = env::var("HOME").unwrap_or_default();
        let shim_dir = Path::new(&home).join(".axiom/bin");
        
        if !shim_dir.exists() {
            println!("⚠️ [Shims] Directory not found. Run 'axiom install' to create shims.");
            return Ok(());
        }

        match fs::read_dir(&shim_dir) {
            Ok(entries) => {
                let mut count = 0;
                let mut errors = 0;
                for entry in entries {
                    if let Ok(e) = entry {
                        if let Ok(metadata) = fs::metadata(e.path()) {
                            #[cfg(unix)]
                            {
                                use std::os::unix::fs::PermissionsExt;
                                if metadata.permissions().mode() & 0o111 == 0 {
                                    println!("❌ [Shims] {} is NOT executable.", e.path().display());
                                    errors += 1;
                                }
                            }
                            count += 1;
                        }
                    }
                }
                if errors == 0 {
                    println!("✅ [Shims] {} shims found and verified.", count);
                } else {
                    println!("❌ [Shims] Found {} shims, but {} have permission issues.", count, errors);
                }
            }
            Err(e) => println!("❌ [Shims] Could not read shim directory: {}", e),
        }
        Ok(())
    }

    fn check_shell_integration() -> anyhow::Result<()> {
        let configs = AxiomInstaller::get_shell_configs();
        if configs.is_empty() {
            println!("⚠️ [Shell] No common shell configuration files detected.");
            return Ok(());
        }

        for path in configs {
            if !path.exists() {
                println!("❌ [Shell] {} was detected but no longer exists.", path.display());
                continue;
            }

            match fs::read_to_string(&path) {
                Ok(content) => {
                    if content.contains("axiom initialize") {
                        println!("✅ [Shell] Integration found in {}.", path.display());
                    } else {
                        println!("❌ [Shell] \x1b[31mIntegration MISSING in {}.\x1b[0m", path.display());
                        println!("   \x1b[33mFix:\x1b[0m Run 'axiom install' to configure aliases.");
                    }
                }
                Err(e) => println!("❌ [Shell] Could not read {}: {}", path.display(), e),
            }
        }
        Ok(())
    }

    fn check_persistence() -> anyhow::Result<()> {
        let home = env::var("HOME").unwrap_or_default();
        let axiom_dir = Path::new(&home).join(".axiom");
        
        if axiom_dir.exists() {
            let test_file = axiom_dir.join(".doctor_test");
            match fs::write(&test_file, "test") {
                Ok(_) => {
                    println!("✅ [Persistence] ~/.axiom is writable.");
                    let _ = fs::remove_file(test_file);
                }
                Err(e) => println!("❌ [Persistence] \x1b[31m~/.axiom is NOT writable: {}\x1b[0m", e),
            }
        } else {
            println!("❌ [Persistence] ~/.axiom directory not found. Run 'axiom install'.");
        }
        Ok(())
    }

    fn check_ai_context(root: &Path) -> anyhow::Result<()> {
        let context_files = ["GEMINI.md", "AGENTS.md", "CLAUDE.md", ".cursorrules"];
        let mut found = 0;
        for file in context_files {
            let path = root.join(file);
            if path.exists() {
                match fs::read_to_string(&path) {
                    Ok(content) => {
                        if content.contains("BEGIN AXIOM INSTRUCTIONS") {
                            println!("✅ [AI Context] {} is synced with Axiom.", file);
                            found += 1;
                        } else {
                            println!("⚠️ [AI Context] {} exists but is NOT synced.", file);
                        }
                    }
                    Err(e) => println!("❌ [AI Context] Could not read {}: {}", file, e),
                }
            }
        }
        if found == 0 {
            println!("⚠️ [AI Context] No synced AI instructions found in current project.");
        }
        Ok(())
    }
}
