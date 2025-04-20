use anyhow::{anyhow, Result};
use std::process::Command;

/// is_repo returns if user is in an active repo.
pub fn is_repo() -> Result<bool> {
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("--is-inside-work-tree")
        .output()?;
    if output.status.success() {
        Ok(true)
    } else {
        Ok(false)
    }
}

/// fetch fetches the latest changes from the remote.
pub fn fetch() -> Result<()> {
    let output = Command::new("git")
        .arg("fetch")
        .arg("--all")
        .arg("--prune")
        .output()?;

    if !output.status.success() {
        return Err(anyhow!("{}", String::from_utf8(output.stderr)?));
    }

    Ok(())
}

/// get the owner and repo name from the remote URL
pub fn owner_repo() -> Result<(String, String)> {
    let result = Command::new("git")
        .arg("remote")
        .arg("get-url")
        .arg("origin")
        .output()?;

    // The repo url could be SSH or it could be HTTPS
    // We are going to handle both cases here.

    let remote_url = String::from_utf8(result.stdout)?.trim().to_string();
    if remote_url.starts_with("git@github.com:") {
        let parts = remote_url
            .trim_start_matches("git@github.com:")
            .trim_end_matches(".git")
            .split('/')
            .collect::<Vec<_>>();

        if parts.len() >= 2 {
            return Ok((parts[0].to_string(), parts[1].to_string()));
        }
    }

    // If we are here... we have an HTTPS URL
    let parts = remote_url
        .trim_start_matches("https://github.com/")
        .trim_end_matches(".git")
        .split("/")
        .collect::<Vec<_>>();

    if parts.len() >= 2 {
        return Ok((parts[0].to_string(), parts[1].to_string()));
    }

    unreachable!("Invalid remote URL");
}

/// get_remote returns the name of the remote (usually "origin")
pub fn get_remote() -> Result<String> {
    let output = Command::new("git").arg("remote").output()?;

    let remotes = String::from_utf8(output.stdout)?.trim().to_string();
    let remote_list: Vec<&str> = remotes.split('\n').collect();

    // If there's only one remote, use it
    if remote_list.len() == 1 {
        return Ok(remote_list[0].to_string());
    }

    // If "origin" is in the list, use it
    if remote_list.contains(&"origin") {
        return Ok("origin".to_string());
    }

    // Otherwise, use the first remote
    if !remote_list.is_empty() {
        return Ok(remote_list[0].to_string());
    }

    Err(anyhow!("No git remote found"))
}
