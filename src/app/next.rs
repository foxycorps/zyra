use anyhow::Result;
use colored::Colorize;

use crate::{data, errors, git};

pub fn next() -> Result<()> {
    if !git::repo::is_repo()? {
        return Err(errors::GitError::NotGitRepository.into());
    }

    // Getting the current state
    let state = data::SolMetadata::load()?;

    // Getting the current stack
    let current_stack = state.get_current_stack()?;

    // Get the current branch
    let current_branch = git::branch::get_branch_name()?;

    // Check to make sure that the branch is in the stack
    if !current_stack.has_branch(&current_branch) {
        return Err(errors::AppError::BranchNotPartOfStack.into());
    }

    let children = current_stack.get_children(&current_branch)?;

    match children.len() {
        0 => Err(errors::AppError::NoNextBranch.into()),
        1 => {
            // We will just switch to that branch
            git::branch::switch(&children[0].name, false)?;
            println!("Switched to branch '{}'", children[0].name.blue());
            Ok(())
        },
        _ => {
            // We assume there is more than one child branch,
            // so we will ask which branch to switch to.
            let branch_names: Vec<_> = children.iter().map(|branch| branch.name.clone()).collect();
            let selection = inquire::Select::new("Select branch to switch to", branch_names).prompt()?;
            git::branch::switch(&selection, false)?;
            println!("Switched to branch '{}'", selection.blue());
            Ok(())
        }
    }
}