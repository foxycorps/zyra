use crate::{data, errors, git};
use anyhow::Result;

pub fn log(graph: bool, verbose: bool, json: bool) -> Result<()> {
    if !git::repo::is_repo()? {
        return Err(errors::git::GitError::NotGitRepository.into());
    }

    let state = data::SolMetadata::load()?;
    let current_stack = state.get_current_stack()?;

    if json {
        println!("{}", current_stack.json());
        return Ok(());
    }

    println!("{}", current_stack.display());
    Ok(())
}
