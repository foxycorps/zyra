use anyhow::{anyhow, Result};
use colored::Colorize;

use crate::{data, errors, git};

pub fn restack(all: bool) -> Result<()> {
    if !git::repo::is_repo()? {
        return Err(errors::git::GitError::NotGitRepository.into());
    }

    let mut state = data::SolMetadata::load()?;
    let current_branch = git::branch::get_current_branch()?;
    
    // Get the current stack and find the starting index
    let start_idx = {
        let current_stack = state.get_current_stack()?;
        if all {
            1 // Skip the root branch (index 0)
        } else {
            // Find the current branch index
            current_stack.branches.iter()
                .position(|b| b.name == current_branch)
                .ok_or_else(|| anyhow!("Current branch not found in stack"))?
        }
    };

    // Get a mutable reference to update the stack
    let stack = state.get_current_stack_mut()?;
    let total_branches = stack.branches.len();
    
    // Iterate through the branches starting from start_idx
    for i in start_idx..total_branches {
        let branch = &stack.branches[i];
        let branch_name = branch.name.clone();
        let parent = branch.parent.clone();
        
        // Skip branches with empty parent (root branches)
        if parent.is_empty() {
            continue;
        }
        
        // Switch to the branch we want to rebase
        git::branch::switch(&branch_name, false)?;
        
        println!("Rebasing '{}' onto '{}'...", branch_name.yellow(), parent.blue());
        
        // Attempt to rebase
        if let Err(e) = git::rebase::rebase_onto(&parent) {
            // If rebase fails, abort it and return error
            let _ = git::rebase::abort_rebase();
            return Err(anyhow!(
                "Failed to rebase '{}' onto '{}': {}",
                branch_name,
                parent,
                e
            ));
        }
        
        // Update the commit hash in our metadata
        let new_commit = git::commit::get_hash()?;
        stack.branches[i].set_commit_hash(&new_commit);
    }
    
    // Save the updated metadata
    state.save()?;
    
    // Switch back to the original branch
    git::branch::switch(&current_branch, false)?;
    
    println!("Successfully restacked {} branch(es)", 
        (total_branches - start_idx).to_string().green());
    
    Ok(())
}