use anyhow::Result;
use std::process::Command;

/// get_hash returns the current commit hash.
pub fn get_hash() -> Result<String> {
    let output = Command::new("git").arg("rev-parse").arg("HEAD").output()?;

    Ok(String::from_utf8(output.stdout)?.trim().to_string())
}

/// is_commit checks if the given commit hash is valid.
pub fn is_commit(commit_hash: &str) -> bool {
    let output = Command::new("git")
        .arg("cat-file")
        .arg("-e")
        .arg(commit_hash)
        .output();

    match output {
        Ok(_) => true,
        Err(_) => false,
    }
}

/// has_commits_between checks if there are commits between two branches.
pub fn has_commits_between(base: &str, head: &str) -> Result<bool> {
    let output = Command::new("git")
        .arg("rev-list")
        .arg("--count")
        .arg(format!("{}..{}", base, head))
        .output()?;

    let count = String::from_utf8(output.stdout)?.trim().parse::<i32>()?;
    Ok(count > 0)
}