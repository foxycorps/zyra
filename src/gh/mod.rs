use anyhow::{anyhow, Result};
use octocrab::Octocrab;
use std::env;
use std::process::Command;
use std::sync::OnceLock;

pub mod pulls;

// Global instance of authenticated Octocrab client
static OCTOCRAB_INSTANCE: OnceLock<Octocrab> = OnceLock::new();

/// Try to get GitHub token from environment variables
fn get_token_from_env() -> Option<String> {
    // Check for ZYRA_GITHUB_TOKEN environment variable
    if let Ok(token) = env::var("ZYRA_GITHUB_TOKEN") {
        return Some(token);
    }

    // Then check for standard GH_TOKEN environment variable
    if let Ok(token) = env::var("GH_TOKEN") {
        return Some(token);
    }

    // Final check for GITHUB_TOKEN environment variable
    if let Ok(token) = env::var("GITHUB_TOKEN") {
        return Some(token);
    }

    None
}

/// Try and get GitHub token from the gh CLI
fn get_token_from_gh_cli() -> Option<String> {
    // Check if gh CLI is installed and authenticated
    let result = Command::new("gh").arg("auth").arg("token").output();

    match result {
        Ok(output) => {
            if output.status.success() {
                // Convert the output to a string and trim whitespace
                let token = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !token.is_empty() {
                    return Some(token);
                }
            }
        }
        Err(_) => {} // gh CLI not installed or other error
    }

    None
}

/// Build Octocrab instance with available authentication
fn build_octocrab() -> Result<Octocrab> {
    // First try to get token from environment
    if let Some(token) = get_token_from_env() {
        return Octocrab::builder()
            .personal_token(token)
            .build()
            .map_err(|e| anyhow!("Failed to authenticate with GitHub token: {}", e));
    }

    // Then try and get token from gh CLI
    if let Some(token) = get_token_from_gh_cli() {
        return Octocrab::builder()
            .personal_token(token)
            .build()
            .map_err(|e| anyhow!("Failed to authenticate with GitHub token: {}", e));
    }

    // Finally try and use git config credentials
    let builder = Octocrab::builder();
    builder
        .build()
        .map_err(|e| anyhow!("Failed to authenticate with GitHub token: {}", e))
}

/// Get a properly authenticated instance of the GitHub API client
pub fn get_instance() -> &'static Octocrab {
    OCTOCRAB_INSTANCE.get_or_init(|| match build_octocrab() {
        Ok(client) => client,
        Err(e) => {
            eprintln!("Warning: GitHub authentication failed - {}", e);
            eprintln!("Set GITHUB_TOKEN or ZYRA_GITHUB_TOKEN environment variables");
            Octocrab::default()
        }
    })
}
