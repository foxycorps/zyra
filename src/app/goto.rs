use anyhow::Result;
use colored::Colorize;

use crate::{data, errors, git};

pub fn goto(name: &str) -> Result<()> {

    if !git::repo::is_repo()? {
        return Err(errors::GitError::NotGitRepository.into());
    }

    let state = data::SolMetadata::load()?;

    // Stack check
    if state.has_stack(name) {
        // We will get its root branch
        let stack = state.get_stack(name)?;
        git::branch::switch(&stack.head_branch.name, false)?;
        println!("Switched to stack '{}'", name.blue());
        return Ok(());
    }

    // We are here... so everything from this point on must be within the current stack
    let current_stack = state.get_current_stack()?;

    // Branch check
    if state.has_branch(name) && current_stack.has_branch(name){
        // We will switch to this branch
        git::branch::switch(name, false)?;
        println!("Switched to branch '{}'", name.blue());
        return Ok(());
    }

    // Commit check -- Must be within this current branch
    if git::commit::is_commit(name) {
        // We will switch to this commit
        git::branch::switch_to_commit(name)?;
        println!("Switched to commit '{}'", name.blue());
        return Ok(());
    }

    Ok(())
}