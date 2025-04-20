use crate::git;
use anyhow::{anyhow, Result};

use super::*;

impl Stack {
    pub fn new(name: String, base_branch: String) -> Self {
        // We will first create a new branch for the stack.
        let branch = StackBranch::new(name.clone(), base_branch.clone());
        Stack {
            name,
            base_branch,
            head_branch: branch.clone(),
            branches: vec![branch],
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    /// Add a new branch to the stack
    pub fn add_branch(&mut self, branch: StackBranch) -> Result<()> {
        self.branches.push(branch);
        Ok(())
    }

    /// Get a branch by name
    pub fn get_branch(&self, name: &str) -> Result<&StackBranch> {
        self.branches
            .iter()
            .find(|branch| branch.name == name)
            .ok_or_else(|| anyhow!("Branch not found"))
    }

    /// Remove a branch from the stack
    pub fn remove_branch(&mut self, branch_name: &str) -> Result<()> {
        let index = self
            .branches
            .iter()
            .position(|branch| branch.name == branch_name)
            .ok_or_else(|| anyhow!("Branch not found"))?;
        self.branches.remove(index);
        Ok(())
    }

    /// Get children branches of a branch
    pub fn get_children(&self, branch_name: &str) -> Result<Vec<&StackBranch>> {
        let mut children = self
            .branches
            .iter()
            .filter(|branch| branch.parent == branch_name)
            .collect::<Vec<&StackBranch>>();

        // Sort children by creation date
        children.sort_by(|a, b| a.created_at.cmp(&b.created_at));

        Ok(children)
    }

    /// Checking if a branch exists.
    pub fn has_branch(&self, name: &str) -> bool {
        self.branches.iter().any(|branch| branch.name == name)
    }

    /// Update the commit hash for a branch in the stack
    pub fn update_branch_commit_hash(
        &mut self,
        branch_name: &str,
        commit_hash: &str,
    ) -> Result<()> {
        let branch = self
            .branches
            .iter_mut()
            .find(|b| b.name == branch_name)
            .ok_or_else(|| anyhow!("Branch '{}' not found in stack", branch_name))?;

        branch.set_commit_hash(commit_hash);
        self.updated_at = Utc::now();

        // If this is also the head branch, update it too
        if self.head_branch.name == branch_name {
            self.head_branch.set_commit_hash(commit_hash);
        }

        Ok(())
    }
}

impl StackBranch {
    pub fn new(name: String, commit_hash: String) -> Self {
        StackBranch {
            name,
            commit_hash,
            pr_id: -1,
            status: BranchStatus::Pending,
            parent: String::new(), // Initialize with empty string
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    pub fn set_status(&mut self, status: BranchStatus) {
        self.status = status;
        self.updated_at = Utc::now();
    }

    /// Set the commit hash for this branch
    pub fn set_commit_hash(&mut self, commit_hash: &str) {
        self.commit_hash = commit_hash.to_string();
        self.updated_at = Utc::now();
    }

    pub fn set_pr_id(&mut self, pr_id: u64) {
        self.pr_id = pr_id as i64;
    }

    pub fn set_parent(&mut self, parent: &str) {
        self.parent = parent.to_string();
    }
}

impl SolMetadata {
    /// Add a new stack to the metadata
    pub fn add_stack(&mut self, stack: &Stack) -> Result<()> {
        self.stacks.push(stack.clone());
        Ok(())
    }

    /// Checking if a stack exists.
    pub fn has_stack(&self, name: &str) -> bool {
        self.stacks.iter().any(|stack| stack.name == name)
    }

    /// Checking if a branch exists.
    pub fn has_branch(&self, name: &str) -> bool {
        self.stacks.iter().any(|stack| stack.has_branch(name))
    }

    /// Gets the current stack based on the current branch.
    pub fn get_current_stack(&self) -> Result<&Stack> {
        let branch_name = git::branch::get_branch_name()?;
        let stack = self
            .stacks
            .iter()
            .find(|stack| stack.has_branch(&branch_name))
            .ok_or_else(|| anyhow!("No stack found for current branch."))?;
        Ok(stack)
    }

    /// Gets a mutable reference to the current stack based on the current branch.
    pub fn get_current_stack_mut(&mut self) -> Result<&mut Stack> {
        let branch_name = git::branch::get_branch_name()?;
        let stack = self
            .stacks
            .iter_mut()
            .find(|stack| stack.has_branch(&branch_name))
            .ok_or_else(|| anyhow!("No stack found for current branch."))?;
        Ok(stack)
    }

    /// Get stack by name
    pub fn get_stack(&self, name: &str) -> Result<&Stack> {
        self.stacks
            .iter()
            .find(|stack| stack.name == name)
            .ok_or_else(|| anyhow!("Stack not found"))
    }

    /// Update the commit hash for a branch in the current stack
    pub fn update_branch_commit_hash(
        &mut self,
        branch_name: &str,
        commit_hash: &str,
    ) -> Result<()> {
        let current_stack = self.get_current_stack_mut()?;
        current_stack.update_branch_commit_hash(branch_name, commit_hash)?;
        Ok(())
    }

    /// Update the PR ID for a branch in the current stack
    pub fn update_branch_pr_id(
        &mut self,
        branch_name: &str,
        pr_id: u64,
    ) -> Result<()> {
        let current_stack = self.get_current_stack_mut()?;
        
        let branch = current_stack.branches.iter_mut()
            .find(|b| b.name == branch_name)
            .ok_or_else(|| anyhow!("Branch '{}' not found in stack", branch_name))?;
        
        branch.set_pr_id(pr_id);
        current_stack.updated_at = Utc::now();
        
        // If this is also the head branch, update it too
        if current_stack.head_branch.name == branch_name {
            current_stack.head_branch.set_pr_id(pr_id);
        }
        
        Ok(())
    }
    
    /// Update the status for a branch in the current stack
    pub fn update_branch_status(
        &mut self,
        branch_name: &str,
        status: BranchStatus,
    ) -> Result<()> {
        let current_stack = self.get_current_stack_mut()?;
        
        let branch = current_stack.branches.iter_mut()
            .find(|b| b.name == branch_name)
            .ok_or_else(|| anyhow!("Branch '{}' not found in stack", branch_name))?;
        
        branch.set_status(status);
        current_stack.updated_at = Utc::now();
        
        // If this is also the head branch, update it too
        if current_stack.head_branch.name == branch_name {
            current_stack.head_branch.set_status(status);
        }
        
        Ok(())
    }
}
