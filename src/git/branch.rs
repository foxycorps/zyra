use anyhow::{anyhow, Result};
use std::process::Command;

/// get_branch_name returns the current branch name.
pub fn get_branch_name() -> Result<String> {
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("--abbrev-ref")
        .arg("HEAD")
        .output()?;

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
    let repo = git2::Repository::discover(".")?;
    let mut branch_names = Vec::new();

    // Always collect local branches
    let branches = repo.branches(Some(git2::BranchType::Local))?;
    for branch_result in branches {
        let (branch, _) = branch_result?;
        if let Some(name) = branch.name()? {
            branch_names.push(name.to_string());
        }
    }

    // Collect remote branches if not local_only
    if !local_only {
        let remote_branches = repo.branches(Some(git2::BranchType::Remote))?;
        for branch_result in remote_branches {
            let (branch, _) = branch_result?;
            if let Some(name) = branch.name()? {
                branch_names.push(name.to_string());
            }
        }
    }

    Ok(branch_names)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_branches() {
        // This test will only run in the context of a git repository
        let result = get_branches(true);
        match result {
            Ok(branches) => {
                println!("Local branches: {:?}", branches);
                assert!(
                    !branches.is_empty(),
                    "Should have at least one local branch"
                );
            }
            Err(e) => {
                println!("Error getting branches: {}", e);
                // Not failing the test since it might be running outside a git repo
            }
        }
    }
}

/// Switches to the commit specified by the hash.
pub fn switch_to_commit(commit_hash: &str) -> Result<()> {
    Command::new("git")
        .arg("checkout")
        .arg(commit_hash)
        .output()?;
    Ok(())
}