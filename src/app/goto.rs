use anyhow::Result;
use colored::Colorize;

use crate::{data, errors, git};

pub fn goto(name: &str) -> Result<()> {
    if !git::repo::is_repo()? {
        return Err(errors::GitError::NotGitRepository.into());
    }

    let mut state = data::SolMetadata::load()?;

    // Stack check
    if state.has_stack(name) {
        // We will get its root branch
        let head_branch_name = {
            let stack = state.get_stack(name)?;
            stack.head_branch.name.clone()
        };
        
        git::branch::switch(&head_branch_name, false)?;
        
        // Update the commit hash after switching
        let current_commit = git::commit::get_hash()?;
        state.update_branch_commit_hash(&head_branch_name, &current_commit)?;
        state.save()?;
        
        println!("Switched to stack '{}'", name.blue());
        return Ok(());
    }

    // We are here... so everything from this point on must be within the current stack
    let current_stack = state.get_current_stack()?;

    // Branch check
    if state.has_branch(name) && current_stack.has_branch(name) {
        // We will switch to this branch
        git::branch::switch(name, false)?;
        
        // Update the commit hash after switching
        let current_commit = git::commit::get_hash()?;
        state.update_branch_commit_hash(name, &current_commit)?;
        state.save()?;
        
        println!("Switched to branch '{}'", name.blue());
        return Ok(());
    }

    // Commit check -- Must be within this current branch
    if git::commit::is_commit(name) {
        // Store current branch and stack context
        let current_branch = git::branch::get_current_branch()?;
        
        // Switch to the commit
        git::branch::switch_to_commit(name)?;
        
        // Update state to track that we're in a commit within the current stack/branch
        state.set_detached_head_context(current_stack.name.clone(), current_branch)?;
        state.save()?;
        
        println!("Switched to commit '{}' (stack context preserved)", name.blue());
        return Ok(());
    }

    Err(anyhow::anyhow!("Could not find stack, branch, or commit '{}'", name))
}