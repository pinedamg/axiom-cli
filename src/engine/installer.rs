use std::fs;
use std::path::{Path, PathBuf};
use std::os::unix::fs::PermissionsExt;
use std::io::{self, Write};

pub struct AxiomInstaller;

const SHELL_BLOCK_START: &str = "# >>> axiom initialize >>>";
const SHELL_BLOCK_END: &str = "# <<< axiom initialize <<<";

const CONTEXT_BLOCK_START: &str = "<!-- BEGIN AXIOM INSTRUCTIONS -->";
const CONTEXT_BLOCK_END: &str = "<!-- END AXIOM INSTRUCTIONS -->";

const DEFAULT_ALIASES: &[&str] = &[
    "alias git='axiom git'",
    "alias docker='axiom docker'",
    "alias ls='axiom ls'",
    "alias npm='axiom npm'",
    "alias cargo='axiom cargo'",
    "alias kubectl='axiom kubectl'",
];

const SHIM_COMMANDS: &[&str] = &["git", "docker", "ls", "npm", "cargo", "kubectl", "rg", "ps", "go"];

const AGENT_RULES: &str = r#"
### 🤖 Axiom: Agent Execution Protocol
> **CRITICAL**: This project uses **Axiom** to optimize terminal streams. 
> As an AI Agent, use `axiom` for noisy commands (git, npm, docker, etc.) to preserve context and redact secrets.

- **ALWAYS** prefix with `axiom` for: `npm`, `cargo`, `docker`, `git`, `kubectl`, `terraform`.
- **REDACTION**: Axiom handles secrets locally. Do not leak credentials into context.
- **RAW MODE**: Use `axiom --raw <cmd>` if you need 100% metadata precision (timestamps, etc.).
"#;

impl AxiomInstaller {
    /// Helper to ask a Yes/No question
    fn ask(prompt: &str, default: bool) -> bool {
        let options = if default { "[Y/n]" } else { "[y/N]" };
        print!("{} {} ", prompt, options);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().to_lowercase();

        if input.is_empty() {
            return default;
        }

        input.starts_with('y')
    }

    /// Detects potential shell config files
    pub fn get_shell_configs() -> Vec<PathBuf> {
        let mut configs = Vec::new();
        let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
        let home_path = Path::new(&home);

        if let Ok(shell) = std::env::var("SHELL") {
            if shell.contains("zsh") {
                configs.push(home_path.join(".zshrc"));
            } else if shell.contains("bash") {
                configs.push(home_path.join(".bashrc"));
            }
        }

        let common = [".zshrc", ".bashrc", ".config/fish/config.fish"];
        for file in common {
            let p = home_path.join(file);
            if p.exists() && !configs.contains(&p) {
                configs.push(p);
            }
        }
        configs
    }

    /// Creates "Shims" in ~/.axiom/bin to intercept commands globally
    pub fn install_shims() -> anyhow::Result<PathBuf> {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
        let shim_dir = Path::new(&home).join(".axiom/bin");
        fs::create_dir_all(&shim_dir)?;

        let axiom_path = std::env::current_exe()?;

        for cmd in SHIM_COMMANDS {
            let shim_path = shim_dir.join(cmd);
            let content = format!("#!/bin/sh\nexec {} {} \"$@\"\n", axiom_path.display(), cmd);
            fs::write(&shim_path, content)?;
            
            let mut perms = fs::metadata(&shim_path)?.permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&shim_path, perms)?;
        }

        Ok(shim_dir)
    }

    /// Injects aliases into the shell config file safely
    pub fn install_shell_integration(path: &Path) -> anyhow::Result<()> {
        if path.is_dir() { return Ok(()); }
        let content = if path.exists() { fs::read_to_string(path)? } else { String::new() };

        let mut new_block = String::from(SHELL_BLOCK_START);
        new_block.push('\n');
        for alias in DEFAULT_ALIASES {
            new_block.push_str(alias);
            new_block.push('\n');
        }
        new_block.push_str(SHELL_BLOCK_END);

        let updated_content = if content.contains(SHELL_BLOCK_START) {
            let start_idx = content.find(SHELL_BLOCK_START).unwrap();
            let end_idx = content.find(SHELL_BLOCK_END).map(|i| i + SHELL_BLOCK_END.len()).unwrap_or(content.len());
            let mut result = content[..start_idx].to_string();
            result.push_str(&new_block);
            result.push_str(&content[end_idx..]);
            result
        } else {
            let mut result = content;
            if !result.is_empty() && !result.ends_with('\n') { result.push('\n'); }
            result.push_str(&new_block);
            result.push('\n');
            result
        };

        fs::write(path, updated_content)?;
        Ok(())
    }

    /// Injects AI Agent instructions into a markdown file
    pub fn inject_ai_context(path: &Path, prefix: bool) -> anyhow::Result<()> {
        if path.is_dir() { return Ok(()); }
        let content = if path.exists() { fs::read_to_string(path)? } else { String::new() };

        let mut new_block = String::from(CONTEXT_BLOCK_START);
        new_block.push('\n');
        new_block.push_str(AGENT_RULES);
        new_block.push_str(CONTEXT_BLOCK_END);

        let updated_content = if content.contains(CONTEXT_BLOCK_START) {
            let start_idx = content.find(CONTEXT_BLOCK_START).unwrap();
            let end_idx = content.find(CONTEXT_BLOCK_END).map(|i| i + CONTEXT_BLOCK_END.len()).unwrap_or(content.len());
            let mut result = content[..start_idx].to_string();
            result.push_str(&new_block);
            result.push_str(&content[end_idx..]);
            result
        } else if prefix {
            let mut result = new_block;
            result.push_str("\n\n");
            result.push_str(&content);
            result
        } else {
            let mut result = content;
            if !result.is_empty() && !result.ends_with('\n') { result.push('\n'); }
            result.push('\n');
            result.push_str(&new_block);
            result
        };

        fs::write(path, updated_content)?;
        Ok(())
    }

    /// Performs an interactive industrial installation
    pub fn run_full_install(project_path: Option<&Path>) -> anyhow::Result<()> {
        println!("\x1b[1m🚀 Axiom Interactive Installation\x1b[0m");
        println!("---------------------------------------\n");

        // 1. Shell Integration
        let configs = Self::get_shell_configs();
        if !configs.is_empty() {
            println!("\x1b[1m[Shell Integration]\x1b[0m");
            for path in configs {
                let prompt = format!("Found {}. Add Axiom aliases?", path.display());
                if Self::ask(&prompt, true) {
                    print!("Configuring {} ... ", path.display());
                    let _ = Self::install_shell_integration(&path);
                    println!("✅");
                }
            }
            println!();
        }

        // 2. Global Shims
        println!("\x1b[1m[IDE & Agent Shims]\x1b[0m");
        println!("Shims allow Axiom to work inside Cursor, Claude Code, and VS Code automatically.");
        if Self::ask("Install Global Shims in ~/.axiom/bin?", true) {
            print!("Installing Shims ... ");
            let shim_dir = Self::install_shims()?;
            println!("✅");
            println!("   \x1b[33mNote:\x1b[0m Remember to add \x1b[36m{}:$PATH\x1b[0m to your shell config.", shim_dir.display());
            println!();
        }

        // 3. AI Context Sync
        if let Some(root) = project_path {
            println!("\x1b[1m[AI Context Sync]\x1b[0m");
            let context_files = [
                "GEMINI.md", 
                "AGENTS.md", 
                "CLAUDE.md", 
                ".cursorrules", 
                ".windsurfrules",
                ".github/copilot-instructions.md",
                ".cursor/rules/axiom.md"
            ];
            
            for file_name in context_files {
                let path = root.join(file_name);
                if path.exists() || file_name == "CLAUDE.md" || file_name == "AGENTS.md" {
                    let prompt = format!("Sync Axiom instructions in {}?", file_name);
                    if Self::ask(&prompt, true) {
                        // Create parent dir if needed
                        if let Some(parent) = path.parent() {
                            if !parent.exists() { let _ = fs::create_dir_all(parent); }
                        }
                        print!("Syncing {} ... ", file_name);
                        let _ = Self::inject_ai_context(&path, true);
                        println!("✅");
                    }
                }
            }
            println!();
        }

        // 4. Persistence Setup
        let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
        let axiom_dir = Path::new(&home).join(".axiom");
        if !axiom_dir.exists() {
            fs::create_dir_all(&axiom_dir)?;
        }

        println!("\x1b[1m\x1b[32mInstallation Complete!\x1b[0m");
        Ok(())
    }
}
