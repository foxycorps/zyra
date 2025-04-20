use anyhow::{anyhow, Result};
use std::process::Command;

/// Rebase the current branch onto the target branch
pub fn rebase_onto(target_branch: &str) -> Result<()> {
    let output = Command::new("git")
        .arg("rebase")
        .arg(target_branch)
        .output()?;

    if !output.status.success() {
        return Err(anyhow!("Failed to rebase: {}", String::from_utf8(output.stderr)?));
    }

    Ok(())
}

/// Abort an ongoing rebase operation
pub fn abort_rebase() -> Result<()> {
    let output = Command::new("git")
        .arg("rebase")
        .arg("--abort")
        .output()?;

    if !output.status.success() {
        return Err(anyhow!("Failed to abort rebase: {}", String::from_utf8(output.stderr)?));
    }

    Ok(())
}

/// Continue a rebase operation after resolving conflicts
pub fn continue_rebase() -> Result<()> {
    let output = Command::new("git")
        .arg("rebase")
        .arg("--continue")
        .output()?;

    if !output.status.success() {
        return Err(anyhow!("Failed to continue rebase: {}", String::from_utf8(output.stderr)?));
    }

    Ok(())
}

/// Check if there's an ongoing rebase operation
pub fn is_rebasing() -> Result<bool> {
    let git_dir = Command::new("git")
        .arg("rev-parse")
        .arg("--git-dir")
        .output()?;

    if !git_dir.status.success() {
        return Err(anyhow!("Failed to get git directory"));
    }

    let git_dir = String::from_utf8(git_dir.stdout)?.trim().to_string();
    let rebase_dir = std::path::Path::new(&git_dir).join("rebase-merge");
    
    Ok(rebase_dir.exists())
} 