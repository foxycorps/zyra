use super::SolMetadata;
use anyhow::{anyhow, Result};
use std::fs;

impl SolMetadata {
    //// save the database file to disk.
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
            });
        }
        let data = fs::read_to_string(path)?;
        let metadata: SolMetadata = serde_json::from_str(&data)?;
        Ok(metadata)
    }

    /// get_storage_path returns the path to the database file.
    fn get_storage_path() -> Result<std::path::PathBuf> {
        // Attempt to find the git repository from the current directory
        match git2::Repository::discover(".") {
            Ok(repo) => {
                // We found a git repository
                let git_dir = repo.path().to_path_buf();
                let mut path = git_dir.clone();
                path.push("sol-metadata.json");
                Ok(path)
            }
            Err(_) => {
                // No git repository found
                Err(anyhow!("Not in a git repository. Please run this command from within a git repository."))
            }
        }
    }
}
