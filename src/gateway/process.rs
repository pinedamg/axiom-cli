use std::process::Stdio;
use std::env;
use tokio::process::{Command, Child};

/// Creates a sanitized PATH excluding Axiom's shim directory
pub fn get_sanitized_path() -> anyhow::Result<std::ffi::OsString> {
    let home = env::var("HOME").unwrap_or_default();
    let shim_dir = format!("{}/.axiom/bin", home);
    
    let current_path = env::var_os("PATH").unwrap_or_default();
    let filtered_path = env::join_paths(
        env::split_paths(&current_path)
            .filter(|p| {
                let p_str = p.to_string_lossy();
                !p_str.contains(".axiom/bin") && p_str != shim_dir
            })
    )?;
    
    Ok(filtered_path)
}

/// Spawns the child process with a sanitized environment and piped I/O
pub fn spawn_child(program: &str, args: &[String]) -> anyhow::Result<Child> {
    let filtered_path = get_sanitized_path()?;

    let child = Command::new(program)
        .args(args)
        .env("PATH", filtered_path) // Inject sanitized PATH
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
        
    Ok(child)
}
