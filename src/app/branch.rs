use crate::{data, errors, git};
use anyhow::{anyhow, Result};

pub fn branch(name: String, from: Option<String>, verbose: bool) -> Result<()> {
    if !git::repo::is_repo()? {
        return Err(errors::git::GitError::NotGitRepository.into());
    }

    let mut state = data::SolMetadata::load()?;

    // Validation checks
    {
        let current_stack = state.get_current_stack()?;

        // Lets make sure this branch name is unique.
        if current_stack.has_branch(&name) || state.has_branch(&name) {
            return Err(anyhow!("Branch already exists."));
        }

        // Check to ensure the `from` branch exists.
        if let Some(from_branch) = &from {
            if !state.has_branch(from_branch) {
                return Err(anyhow!("Branch does not exist."));
            }

            // We will also check that it is in the same stack.
            if !current_stack.has_branch(from_branch) {
                return Err(anyhow!(
                    "Branch does not exist in current stack. Run `sol ls` to see available branches."
                ));
            }
        }
    } // The immutable borrow of state ends here

    // Switch to the from branch if specified
    if let Some(from_branch) = from {
        git::branch::switch(&from_branch, false)?;
    }

    // Getting the current commit hash.
    let commit_hash = git::commit::get_hash()?;
    let current_branch = git::branch::get_branch_name()?;

    // Creating the branch.
    let mut branch = data::StackBranch::new(name.clone(), commit_hash);
    branch.set_parent(current_branch.clone());

    // Add the branch to the current stack
    let current_stack_mut = state.get_current_stack_mut()?;
    current_stack_mut.add_branch(branch)?;

    // Store the stack display before saving
    let stack_display = current_stack_mut.simple_display();

    state.save()?;

    // Create the new git branch
    git::branch::switch(&name, true)?;
    git::branch::set_upstream(&name)?;

    if verbose {
        println!(
            "[sol] Created new branch '{}' from parent branch '{}'.",
            name, current_branch
        );
    } else {
        println!("Updated stack: {}", stack_display);
    }

    Ok(())
}
