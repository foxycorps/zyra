use anyhow::Result;
use colored::Colorize;

use crate::{data::{self, StackBranch}, errors, git};

pub fn prev(steps: usize) -> Result<()> {

    if !git::repo::is_repo()? {
        return Err(errors::GitError::NotGitRepository.into());
    }

    // Getting the current state
    let state = data::SolMetadata::load()?;

    // Getting the current stack
    let current_stack = state.get_current_stack()?;

    // Get the current branch
    let current_branch = git::branch::get_branch_name()?;

    // Determine if we are at the root of the stack
    if current_stack.head_branch.name == current_branch {
        // We are on the head branch of the stack.
        // There is nowhere to go back to
        return Err(errors::AppError::NoPreviousBranch.into());
    }

    // Get the related metadata for the current branch
    let current_branch_metadata = current_stack.get_branch(&current_branch)?;
    let parent_branch = current_branch_metadata.parent.as_ref().unwrap();
    let parent_branch_metadata: &StackBranch;
    match current_stack.get_branch(&parent_branch) {
        Ok(branch) => parent_branch_metadata = branch,
        Err(_) => {
            // The parent branch does not exist.
            // We are at the root of the stack.
            return Err(errors::AppError::NoPreviousBranch.into());
        }
    }

    // We will now try and switch to the parent branch
    git::branch::switch(&parent_branch_metadata.name, false)?;

    println!("Switched to branch '{}'", parent_branch_metadata.name.blue());
    
    Ok(())
}