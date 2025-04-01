use crate::{data::*, git};
use anyhow::{anyhow, Result};

pub fn init(name: String, base: Option<String>, verbose: bool) -> Result<()> {
    let mut metadata = SolMetadata::load()?;

    // Checking if the stack already exists.
    if metadata.has_stack(&name) {
        return Err(anyhow!("Stack already exists."));
    }

    // Check if the potential branch exists.
    if metadata.has_branch(&name) {
        return Err(anyhow!("Branch already exists."));
    }

    // Need to check if the base branch actually exists in git.
    let list = git::branch::get_branches(true)?;
    if !list.contains(&base.clone().unwrap_or("main".to_string())) {
        return Err(anyhow!("Base branch does not exist."));
    }

    let stack = Stack::new(name.clone(), base.unwrap_or("main".to_string()));
    metadata.add_stack(&stack)?;
    metadata.save()?;

    git::branch::switch(&name, true)?;
    git::branch::set_upstream(&name)?;

    if verbose {
        println!(
            "[sol] Initialized new '{}' from base branch '{}'.",
            name.clone(),
            "main"
        );
    }
    println!("Created branch: {}", stack.head_branch.name);
    Ok(())
}
