use anyhow::Result;
use colored::Colorize;

use crate::{data, errors, git};

pub fn prev() -> Result<()> {
    if !git::repo::is_repo()? {
        return Err(errors::GitError::NotGitRepository.into());
    }

    let mut state = data::SolMetadata::load()?;

    // If we're in a detached HEAD state, handle that case
    if state.is_in_detached_head() {
        let context = state.get_detached_head_context().unwrap();
        let stack = state.get_stack(&context.stack_name)?;
        let current_idx = stack.branches.iter().position(|b| b.name == context.branch_name)
            .ok_or_else(|| anyhow::anyhow!("Could not find branch '{}' in stack", context.branch_name))?;

        if current_idx == 0 {
            return Err(anyhow::anyhow!("Already at the first branch in the stack"));
        }

        let prev_branch_name = stack.branches[current_idx - 1].name.clone();
        
        // Switch to the branch first
        git::branch::switch(&prev_branch_name, false)?;
        
        // Update the commit hash after switching
        let current_commit = git::commit::get_hash()?;
        state.update_branch_commit_hash(&prev_branch_name, &current_commit)?;
        
        // Then update state
        state.clear_detached_head_context();
        state.save()?;
        
        println!("Switched to branch '{}'", prev_branch_name.blue());
        return Ok(());
    }

    // Normal branch navigation
    let current_stack = state.get_current_stack()?;
    let current_branch = git::branch::get_current_branch()?;
    
    let current_idx = current_stack.branches.iter().position(|b| b.name == current_branch)
        .ok_or_else(|| anyhow::anyhow!("Could not find branch '{}' in stack", current_branch))?;

    if current_idx == 0 {
        return Err(anyhow::anyhow!("Already at the first branch in the stack"));
    }

    let prev_branch = &current_stack.branches[current_idx - 1];
    let prev_branch_name = prev_branch.name.clone();
    
    git::branch::switch(&prev_branch_name, false)?;
    
    // Update the commit hash after switching
    let current_commit = git::commit::get_hash()?;
    state.update_branch_commit_hash(&prev_branch_name, &current_commit)?;
    state.save()?;
    
    println!("Switched to branch '{}'", prev_branch_name.blue());
    
    Ok(())
}