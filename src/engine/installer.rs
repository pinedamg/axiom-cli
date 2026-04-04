use std::fs;
use std::path::{Path, PathBuf};
use std::io::{self, Write};

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

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
    /// Helper to ask a Yes/No question, considering the auto_yes flag
    fn ask(prompt: &str, default: bool, auto_yes: bool) -> bool {
        if auto_yes { return true; }
        
        let options = if default { "[Y/n]" } else { "[y/N]" };
        print!("{} {} ", prompt, options);
        let _ = io::stdout().flush();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() { return default; }
        let input = input.trim().to_lowercase();

        if input.is_empty() { return default; }
        input.starts_with('y')
    }

    pub fn get_shell_configs() -> Vec<PathBuf> {
        let mut configs = Vec::new();
        let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
        let home_path = Path::new(&home);

        if let Ok(shell) = std::env::var("SHELL") {
            if shell.contains("zsh") { configs.push(home_path.join(".zshrc")); }
            else if shell.contains("bash") { configs.push(home_path.join(".bashrc")); }
        }

        let common = [".zshrc", ".bashrc", ".config/fish/config.fish"];
        for file in common {
            let p = home_path.join(file);
            if p.exists() && !configs.contains(&p) { configs.push(p); }
        }
        configs
    }

    /// Injects or updates a block of text in a file delimited by markers
    fn inject_block(path: &Path, start_marker: &str, end_marker: &str, content: &str, as_prefix: bool) -> anyhow::Result<()> {
        if path.is_dir() { return Ok(()); }
        let file_content = if path.exists() { fs::read_to_string(path)? } else { String::new() };

        let mut new_block = String::from(start_marker);
        new_block.push('\n');
        new_block.push_str(content);
        if !content.ends_with('\n') { new_block.push('\n'); }
        new_block.push_str(end_marker);

        let updated_content = if file_content.contains(start_marker) {
            let start_idx = file_content.find(start_marker).unwrap();
            let end_idx = file_content.find(end_marker).map(|i| i + end_marker.len()).unwrap_or(file_content.len());
            let mut result = file_content[..start_idx].to_string();
            result.push_str(&new_block);
            result.push_str(&file_content[end_idx..]);
            result
        } else if as_prefix {
            let mut result = new_block;
            result.push_str("\n\n");
            result.push_str(&file_content);
            result
        } else {
            let mut result = file_content;
            if !result.is_empty() && !result.ends_with('\n') { result.push('\n'); }
            result.push_str(&new_block);
            result
        };

        fs::write(path, updated_content)?;
        Ok(())
    }

    /// Removes a delimited block from a file
    fn remove_block(path: &Path, start_marker: &str, end_marker: &str) -> anyhow::Result<()> {
        if !path.exists() || path.is_dir() { return Ok(()); }
        let content = fs::read_to_string(path)?;

        if content.contains(start_marker) {
            let start_idx = content.find(start_marker).unwrap();
            let end_idx = content.find(end_marker).map(|i| i + end_marker.len()).unwrap_or(content.len());
            
            let mut updated = content[..start_idx].to_string();
            let mut suffix = content[end_idx..].to_string();
            
            // Clean up extra newlines left behind
            if updated.ends_with('\n') && suffix.starts_with('\n') {
                suffix = suffix[1..].to_string();
            }
            
            updated.push_str(&suffix);
            fs::write(path, updated)?;
        }
        Ok(())
    }

    pub fn install_shell_integration(path: &Path, include_path: bool) -> anyhow::Result<()> {
        let mut content = String::new();
        if include_path {
            let home = std::env::var("HOME")?;
            content.push_str(&format!("export PATH=\"{}/.axiom/bin:$PATH\"\n", home));
        }
        for alias in DEFAULT_ALIASES {
            content.push_str(alias);
            content.push('\n');
        }
        Self::inject_block(path, SHELL_BLOCK_START, SHELL_BLOCK_END, &content, false)
    }

    pub fn install_shims() -> anyhow::Result<PathBuf> {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
        let shim_dir = Path::new(&home).join(".axiom/bin");
        fs::create_dir_all(&shim_dir)?;

        let axiom_path = std::env::current_exe()?;
        let axiom_path_str = axiom_path.to_string_lossy();

        for cmd in SHIM_COMMANDS {
            let shim_path = shim_dir.join(cmd);
            // Use absolute path to axiom to avoid recursive $PATH lookups
            let content = format!("#!/bin/sh\nexec \"{}\" {} \"$@\"\n", axiom_path_str, cmd);
            fs::write(&shim_path, content)?;
            
            #[cfg(unix)]
            {
                let mut perms = fs::metadata(&shim_path)?.permissions();
                perms.set_mode(0o755);
                fs::set_permissions(&shim_path, perms)?;
            }
        }
        Ok(shim_dir)
    }

    pub fn inject_ai_context(path: &Path, as_prefix: bool) -> anyhow::Result<()> {
        Self::inject_block(path, CONTEXT_BLOCK_START, CONTEXT_BLOCK_END, AGENT_RULES, as_prefix)
    }

    /// Surgically removes all Axiom traces
    pub fn run_uninstall(project_path: Option<&Path>, auto_yes: bool) -> anyhow::Result<()> {
        println!("\x1b[1m🗑️ Axiom Industrial Uninstall\x1b[0m");
        println!("---------------------------------------\n");

        if !Self::ask("This will remove all Axiom aliases, shims and context rules. Proceed?", false, auto_yes) {
            println!("Uninstall cancelled.");
            return Ok(());
        }

        // 1. Remove from Shell
        let configs = Self::get_shell_configs();
        for path in configs {
            print!("Cleaning {} ... ", path.display());
            let _ = Self::remove_block(&path, SHELL_BLOCK_START, SHELL_BLOCK_END);
            println!("✅");
        }

        // 2. Remove Shims
        let home = std::env::var("HOME").unwrap_or_default();
        let shim_dir = Path::new(&home).join(".axiom/bin");
        if shim_dir.exists() {
            print!("Removing Shims directory ... ");
            let _ = fs::remove_dir_all(shim_dir);
            println!("✅");
        }

        // 3. Remove AI Context
        if let Some(root) = project_path {
            let context_files = ["GEMINI.md", "AGENTS.md", "CLAUDE.md", ".cursorrules", ".windsurfrules"];
            for file_name in context_files {
                let path = root.join(file_name);
                if path.exists() {
                    print!("Removing Axiom rules from {} ... ", file_name);
                    let _ = Self::remove_block(&path, CONTEXT_BLOCK_START, CONTEXT_BLOCK_END);
                    println!("✅");
                }
            }
        }

        println!("\n\x1b[1m\x1b[32mUninstall Complete. Axiom traces removed.\x1b[0m");
        Ok(())
    }
}
