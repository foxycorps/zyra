use anyhow::{anyhow, Result};
use std::process::Command;

/// get_branch_name returns the current branch name.
pub fn get_branch_name() -> Result<String> {
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("--abbrev-ref")
        .arg("HEAD")
        .output()?;

    if !output.status.success() {
        return Err(anyhow!("{}", String::from_utf8(output.stderr)?));
    }

    Ok(String::from_utf8(output.stdout)?.trim().to_string())
}

/// switches to the branch specified by the name, will create the branch if it does not exist.
pub fn switch(name: &str, create: bool) -> Result<()> {
    let mut cmd = Command::new("git");
    cmd.arg("switch");

    if create {
        cmd.arg("-c");
    }

    let output = cmd.arg(name).output()?;

    if !output.status.success() {
        return Err(anyhow!("{}", String::from_utf8(output.stderr)?));
    }

    Ok(())
}

/// Sets the upstream branch for the current branch.
pub fn set_upstream(name: &str) -> Result<()> {
    let output = Command::new("git")
        .arg("branch")
        .arg("--set-upstream-to")
        .arg(name)
        .output()?;

    if !output.status.success() {
        return Err(anyhow!("{}", String::from_utf8(output.stderr)?));
    }
    Ok(())
}

/// Gets the list of branches known to git.
pub fn get_branches(local_only: bool) -> Result<Vec<String>> {
    let mut cmd = Command::new("git");
    cmd.arg("branch");

    if !local_only {
        cmd.arg("-a"); // Show all branches including remotes
    }

    cmd.arg("--format=%(refname:short)"); // Get clean branch names

    let output = cmd.output()?;

    if !output.status.success() {
        return Err(anyhow!("{}", String::from_utf8(output.stderr)?));
    }

    let branches = String::from_utf8(output.stdout)?
        .lines()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    Ok(branches)
}

/// Switches to the commit specified by the hash.
pub fn switch_to_commit(commit_hash: &str) -> Result<()> {
    let output = Command::new("git")
        .arg("checkout")
        .arg(commit_hash)
        .output()?;

    if !output.status.success() {
        return Err(anyhow!("{}", String::from_utf8(output.stderr)?));
    }

    Ok(())
}

/// Get the current branch name
pub fn get_current_branch() -> Result<String> {
    get_branch_name()
}

/// Check if a commit hash exists
pub fn commit_exists(commit_hash: &str) -> Result<bool> {
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("--verify")
        .arg(format!("{}^{{commit}}", commit_hash))
        .output()?;

    Ok(output.status.success())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_branches() {
        let result = get_branches(true);
        match result {
            Ok(branches) => {
                println!("Local branches: {:?}", branches);
                assert!(!branches.is_empty(), "Should have at least one local branch");
            }
            Err(e) => {
                println!("Error getting branches: {}", e);
            }
        }
    }
}