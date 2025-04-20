use super::SolMetadata;
use anyhow::{anyhow, Result};
use std::{fs, process::Command, path::PathBuf};
use serde_json::{Value, json};

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
        
        // Read the file content
        let data = fs::read_to_string(&path)?;
        
        // First parse as generic JSON Value to check for old format
        let mut json_value: Value = serde_json::from_str(&data)?;
        
        // Migrate the data if needed
        Self::migrate_metadata(&mut json_value);
        
        // Convert the migrated JSON back to string
        let migrated_data = serde_json::to_string(&json_value)?;
        
        // Parse the migrated data into SolMetadata
        let metadata: SolMetadata = serde_json::from_str(&migrated_data)?;
        
        Ok(metadata)
    }
    
    /// Migrate metadata from old format to new format
    fn migrate_metadata(json_value: &mut Value) {
        if let Value::Object(obj) = json_value {
            if let Some(Value::Array(stacks)) = obj.get_mut("stacks") {
                for stack in stacks {
                    if let Value::Object(stack_obj) = stack {
                        // Migrate head_branch
                        if let Some(head_branch) = stack_obj.get_mut("head_branch") {
                            Self::migrate_branch(head_branch);
                        }
                        
                        // Migrate all branches
                        if let Some(Value::Array(branches)) = stack_obj.get_mut("branches") {
                            for branch in branches {
                                Self::migrate_branch(branch);
                            }
                        }
                    }
                }
            }
        }
    }
    
    /// Migrate a single branch from old format to new format
    fn migrate_branch(branch: &mut Value) {
        if let Value::Object(branch_obj) = branch {
            // Convert null pr_id to -1
            if let Some(pr_id) = branch_obj.get("pr_id") {
                if pr_id.is_null() {
                    branch_obj.insert("pr_id".to_string(), json!(-1));
                }
            }
            
            // Convert null parent to empty string
            if let Some(parent) = branch_obj.get("parent") {
                if parent.is_null() {
                    branch_obj.insert("parent".to_string(), json!(""));
                }
            }
            
            // Remove depth field if it exists
            branch_obj.remove("depth");
        }
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
