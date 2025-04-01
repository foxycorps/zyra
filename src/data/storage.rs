use super::SolMetadata;
use anyhow::{anyhow, Result};
use std::{fs, process::Command, path::PathBuf};

impl SolMetadata {
    /// save the database file to disk.
    pub fn save(&self) -> Result<()> {
        let data = serde_json::to_string_pretty(self)?;
        let path = Self::get_storage_path()?;
        fs::write(path, data)?;
        Ok(())
    }

    /// load the database file from disk.
    pub fn load() -> Result<Self> {
        let path = Self::get_storage_path()?;
        if !path.exists() {
            return Ok(SolMetadata {
                stacks: vec![],
                version: "0.1.0".to_string(),
                detached_head_context: None,
            });
        }
        let data = fs::read_to_string(path)?;
        let metadata: SolMetadata = serde_json::from_str(&data)?;
        Ok(metadata)
    }

    /// get_storage_path returns the path to the database file.
    fn get_storage_path() -> Result<PathBuf> {
        // Get the git directory using git rev-parse
        let output = Command::new("git")
            .arg("rev-parse")
            .arg("--git-dir")
            .output()?;

        if !output.status.success() {
            return Err(anyhow!("Not in a git repository. Please run this command from within a git repository."));
        }

        let git_dir = String::from_utf8(output.stdout)?.trim().to_string();
        let mut path = PathBuf::from(git_dir);
        path.push("zyra-metadata.json");
        Ok(path)
    }
}
