use anyhow::{anyhow, Result};
use std::{path::Path, process::Command};

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
