use anyhow::{anyhow, Result};
use hashbrown::HashMap;

use crate::{
    data::{self, BranchStatus, StackBranch},
    errors, git,
};

/// Submit a branch or all branches in the stack to create/update PRs
pub async fn submit(all: bool, force: bool, no_push: bool) -> Result<()> {
    // Validate repository state
    validate_repository()?;
    
    // Load and update state
    let mut state = data::SolMetadata::load()?;
    update_current_branch_state(&mut state)?;
    
    if !all {
        // Submit only the current branch
        submit_current_branch(&state, force, no_push).await?;
    } else {
        // Submit all branches in the stack
        submit_all_branches(&mut state, force, no_push).await?;
    }
    
    Ok(())
}

/// Validate that we're in a git repository and fetch latest changes
fn validate_repository() -> Result<()> {
    if !git::repo::is_repo()? {
        return Err(errors::GitError::NotGitRepository.into());
    }
    
    // Fetch latest changes from remote
    git::repo::fetch()?;
    
    Ok(())
}

/// Update the current branch's commit hash in the state
fn update_current_branch_state(state: &mut data::SolMetadata) -> Result<()> {
    let current_branch = git::branch::get_branch_name()?;
    let current_commit = git::commit::get_hash()?;
    
    state.update_branch_commit_hash(&current_branch, &current_commit)?;
    state.save()?;
    
    Ok(())
}

/// Submit only the current branch
async fn submit_current_branch(state: &data::SolMetadata, force: bool, no_push: bool) -> Result<()> {
    let current_branch = git::branch::get_branch_name()?;
    let current_stack = state.get_current_stack()?;
    
    // Find the current branch in the stack
    let branch = current_stack.branches.iter()
        .find(|b| b.name == current_branch)
        .ok_or_else(|| anyhow!("Current branch '{}' not found in stack", current_branch))?;
    
    // Validate branch can be submitted
    validate_branch_for_submission(branch, &current_stack.branches)?;
    
    // Push branch if needed
    if !no_push {
        push_branch(branch, force)?;
    }
    
    // Create or update PR
    create_or_update_pr(branch, &current_stack.branches).await?;
    
    // No need to reload state here as create_or_update_pr already updates it
    
    Ok(())
}

/// Validate that a branch can be submitted (has commits, parent not merged)
fn validate_branch_for_submission(branch: &StackBranch, all_branches: &[StackBranch]) -> Result<()> {
    // Verify there are commits between base and current branch
    if !git::commit::has_commits_between(&branch.parent, &branch.name)? {
        return Err(anyhow!("No commits found between '{}' and '{}'. Nothing to submit.", 
            branch.parent, branch.name));
    }
    
    // Check if parent branch has been merged
    if let Some(parent_branch) = all_branches.iter().find(|b| b.name == branch.parent) {
        if matches!(parent_branch.status, BranchStatus::Merged) {
            return Err(anyhow!(
                "Failed to create a pull request. The parent branch '{}' has already been merged. \
                Please run 'zyra sync' to rebase the branch first.",
                branch.parent
            ));
        }
    }
    
    Ok(())
}

/// Push a branch to remote with appropriate force option
fn push_branch(branch: &StackBranch, force: bool) -> Result<()> {
    let remote = git::repo::get_remote()?;
    println!("Pushing branch '{}' to remote '{}'...", branch.name, remote);
    
    if force {
        git::branch::force_push(&branch.name)?;
    } else {
        git::branch::push_with_lease(&branch.name)?;
    }
    
    Ok(())
}

/// Submit all branches in the stack
async fn submit_all_branches(state: &mut data::SolMetadata, force: bool, no_push: bool) -> Result<()> {
    let current_stack = state.get_current_stack()?;
    
    // Create a map of parent -> children branches
    let children_map = build_branch_hierarchy(&current_stack.branches);
    
    // Find the root branch (the one with empty parent)
    let root_branch = current_stack.branches.iter()
        .find(|b| b.parent.is_empty())
        .ok_or_else(|| anyhow!("Could not find the root branch in the stack"))?;
    
    // Process the root branch and all its children
    process_branch_and_children(root_branch, &children_map, force, no_push).await
}

/// Build a map of parent -> children branches
fn build_branch_hierarchy<'a>(branches: &'a [StackBranch]) -> HashMap<String, Vec<&'a StackBranch>> {
    let mut children_map: HashMap<String, Vec<&'a StackBranch>> = HashMap::new();
    
    for branch in branches {
        children_map
            .entry(branch.parent.clone())
            .or_insert(Vec::new())
            .push(branch);
    }
    
    children_map
}

/// Process a branch and all its children recursively
async fn process_branch_and_children<'a>(
    branch: &'a StackBranch,
    children_map: &'a HashMap<String, Vec<&'a StackBranch>>,
    force: bool,
    no_push: bool,
) -> Result<()> {
    // Skip branches that don't need submission
    if !should_submit_branch(branch)? {
        println!("Skipping branch '{}' as it doesn't need submission", branch.name);
        return Ok(());
    }
    
    println!("Processing branch '{}'...", branch.name);
    
    // Switch to the branch
    git::branch::switch(&branch.name, false)?;
    
    // Update branch state with latest commit
    update_branch_state(&branch.name)?;
    
    // Push branch if needed
    if !no_push {
        push_branch(branch, force)?;
    }
    
    // Get all branches for PR creation
    let all_branches = {
        let state = data::SolMetadata::load()?;
        let stack = state.get_current_stack()?;
        stack.branches.clone()
    };
    
    // Create or update PR
    create_or_update_pr(branch, &all_branches).await?;
    
    // Process all children branches
    if let Some(children) = children_map.get(&branch.name) {
        for child in children {
            // Use Box::pin to handle recursive async calls
            let future = Box::pin(process_branch_and_children(child, children_map, force, no_push));
            future.await?;
        }
    }
    
    Ok(())
}

/// Update a branch's state in the metadata
fn update_branch_state(branch_name: &str) -> Result<()> {
    let current_commit = git::commit::get_hash()?;
    let mut state = data::SolMetadata::load()?;
    state.update_branch_commit_hash(branch_name, &current_commit)?;
    state.save()?;
    
    Ok(())
}

/// Check if a branch should be submitted (has commits and is not the root branch)
fn should_submit_branch(branch: &StackBranch) -> Result<bool> {
    if branch.parent.is_empty() {
        return Ok(false); // Don't submit the root branch
    }
    
    let has_commits = git::commit::has_commits_between(&branch.parent, &branch.name)?;
    if !has_commits {
        println!("No commits found between '{}' and '{}'. Skipping.", branch.parent, branch.name);
        return Ok(false);
    }
    
    Ok(true)
}

/// Create or update a pull request for a branch
async fn create_or_update_pr(branch: &StackBranch, all_branches: &[StackBranch]) -> Result<()> {
    let (owner, repo) = git::repo::owner_repo()?;
    
    // Check if PR already exists in GitHub
    let existing_pr = crate::gh::pulls::find_pull_request(&owner, &repo, &branch.name).await?;
    
    if let Some(pr) = existing_pr {
        // We found an existing PR
        println!("Found existing PR #{} for '{}'", pr.number, branch.name);
        
        // Check if our metadata needs updating
        if branch.pr_id != pr.number as i64 {
            println!("Updating PR number in metadata from {} to {}", branch.pr_id, pr.number);
            update_branch_pr_info(&branch.name, pr.number as u64)?;
        }
        
        // Check if PR is pointing to the correct parent branch
        if pr.base.ref_field != branch.parent {
            println!("PR is pointing to '{}' but should point to '{}'. Updating...", 
                     pr.base.ref_field, branch.parent);
            
            // Get PR details to update
            let details = crate::tui::pulls::create_pull_request()?;
            let pr_body = build_pr_body(details.body, all_branches);
            
            // Update the PR
            crate::gh::pulls::update_pull_request(
                &owner,
                &repo,
                pr.number as u64,
                Some(&details.title),
                Some(&pr_body),
                Some(&branch.parent)
            ).await?;
            
            println!("PR #{} updated for '{}'", pr.number, branch.name);
        } else {
            println!("PR #{} is correctly configured for '{}'", pr.number, branch.name);
        }
    } else {
        // No existing PR found, create a new one
        create_new_pr(branch, all_branches, &owner, &repo).await?;
    }
    
    Ok(())
}

/// Create a new pull request
async fn create_new_pr(branch: &StackBranch, all_branches: &[StackBranch], owner: &str, repo: &str) -> Result<()> {
    // Getting the user to fill out the PR data
    let details = crate::tui::pulls::create_pull_request()?;
    
    println!("Creating PR for '{}'", branch.name);
    
    // Add stack information to PR body
    let pr_body = build_pr_body(details.body, all_branches);
    
    // Create the PR
    let pr = crate::gh::pulls::create_pull_request(
        owner,
        repo,
        &details.title,
        &branch.name,
        &branch.parent,
        &pr_body,
        details.draft,
    ).await?;
    
    println!("PR #{} created for '{}'", pr.number, branch.name);
    
    // Update branch with PR information
    update_branch_pr_info(&branch.name, pr.number as u64)?;
    
    Ok(())
}

/// Build the PR body with stack information
fn build_pr_body(base_body: String, all_branches: &[StackBranch]) -> String {
    let mut pr_body = base_body;
    
    if !all_branches.is_empty() {
        pr_body.push_str("\n\n### Stack Information\n");
        pr_body.push_str("This PR is part of a stack of branches:\n");
        
        // Add information about the stack
        for b in all_branches {
            let status_marker = match b.status {
                BranchStatus::Pending => "⏳",
                BranchStatus::Merged => "✅",
                BranchStatus::Conflict => "❌",
                BranchStatus::Testing => "🧪",
            };
            
            let pr_info = if b.pr_id > 0 {
                format!(" (PR #{})", b.pr_id)
            } else {
                "".to_string()
            };
            
            pr_body.push_str(&format!("\n- {} {}{}", status_marker, b.name, pr_info));
        }
    }
    
    pr_body
}

/// Update a branch's PR ID and status in the metadata
fn update_branch_pr_info(branch_name: &str, pr_id: u64) -> Result<()> {
    let mut state = data::SolMetadata::load()?;
    state.update_branch_pr_id(branch_name, pr_id)?;
    state.update_branch_status(branch_name, data::BranchStatus::Pending)?;
    state.save()?;
    
    Ok(())
}
