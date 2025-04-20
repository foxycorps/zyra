use crate::{data, errors, git};
use anyhow::Result;

pub fn log(graph: bool, verbose: bool, json: bool, pretty: bool) -> Result<()> {
    if !git::repo::is_repo()? {
        return Err(errors::git::GitError::NotGitRepository.into());
    }

    let state = data::SolMetadata::load()?;
    
    // Try to get the current stack, but handle the case where there's no stack for the current branch
    let current_stack = match state.get_current_stack() {
        Ok(stack) => stack,
        Err(_) => {
            println!("No stack found for the current branch. Run 'zyra init' to create a new stack.");
            return Ok(());
        }
    };

    if json {
        println!("{}", current_stack.json(pretty));
        return Ok(());
    }

    println!("{}", current_stack.display(graph));
    Ok(())
}
