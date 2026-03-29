use serde::Deserialize;
use std::env;
use std::fs;
use std::io::Read;

#[derive(Deserialize, Debug)]
struct Release {
    tag_name: String,
    assets: Vec<Asset>,
}

#[derive(Deserialize, Debug)]
struct Asset {
    name: String,
    browser_download_url: String,
}

pub struct AxiomUpdater;

const GITHUB_API_URL: &str = "https://api.github.com/repos/mpineda/axiom/releases/latest";
const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");

impl AxiomUpdater {
    /// Checks for a new version on GitHub
    pub fn check_latest() -> anyhow::Result<Option<(String, String)>> {
        let client = ureq::AgentBuilder::new()
            .user_agent("axiom-cli-updater")
            .build();

        let response: Release = client.get(GITHUB_API_URL).call()?.into_json()?;
        
        // Compare tag_name (e.g., "v0.2.0") with CURRENT_VERSION (e.g., "0.1.0")
        let latest_tag = response.tag_name.trim_start_matches('v');
        if latest_tag != CURRENT_VERSION {
            // Find the correct asset for the current OS/Arch
            let os = env::consts::OS;
            let arch = env::consts::ARCH;
            
            // Format example: axiom-x86_64-apple-darwin
            let target_os = match os {
                "linux" => "unknown-linux-gnu",
                "macos" => "apple-darwin",
                "windows" => "pc-windows-msvc",
                _ => return Ok(None),
            };
            
            let target_arch = match arch {
                "x86_64" => "x86_64",
                "aarch64" => "aarch64",
                _ => return Ok(None),
            };

            let asset_pattern = format!("{}-{}", target_arch, target_os);
            if let Some(asset) = response.assets.iter().find(|a| a.name.contains(&asset_pattern)) {
                return Ok(Some((response.tag_name, asset.browser_download_url.clone())));
            }
        }

        Ok(None)
    }

    /// Downloads and replaces the current binary
    pub fn run_self_update(url: &str) -> anyhow::Result<()> {
        let current_exe = env::current_exe()?;
        let tmp_path = current_exe.with_extension("tmp_update");
        let backup_path = current_exe.with_extension("old");

        println!("Downloading update from {}...", url);
        
        let client = ureq::AgentBuilder::new()
            .user_agent("axiom-cli-updater")
            .build();

        let mut response = client.get(url).call()?.into_reader();
        let mut buffer = Vec::new();
        response.read_to_end(&mut buffer)?;

        // Write the new binary to a temporary file
        fs::write(&tmp_path, buffer)?;
        
        // Give execution permissions (Unix only)
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&tmp_path)?.permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&tmp_path, perms)?;
        }

        // Atomic swap (sort of): move current to old, move tmp to current
        if current_exe.exists() {
            fs::rename(&current_exe, &backup_path)?;
        }
        
        if let Err(e) = fs::rename(&tmp_path, &current_exe) {
            // If renaming the new one failed, try to restore the old one
            let _ = fs::rename(&backup_path, &current_exe);
            return Err(anyhow::anyhow!("Failed to replace binary: {}", e));
        }

        // Clean up backup if possible (might fail if exe is busy, which is fine)
        let _ = fs::remove_file(backup_path);
        
        println!("✅ Update successful! Please run 'axiom --version' to verify.");
        Ok(())
    }
}
