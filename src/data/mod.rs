use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// Exporting the other parts of the data system.
pub mod display;
pub mod operations;
pub mod storage;

/// Represents a complete stack
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Stack {
    pub name: String,               // e.g. "feature-stack"
    pub base_branch: String,        // e.g. "main"
    pub head_branch: StackBranch,   // The first branch in the stack
    pub branches: Vec<StackBranch>, // Ordered list of branches in th stack
    created_at: DateTime<Utc>,      // Timestamp for creation
    updated_at: DateTime<Utc>,      // Last update timestamp
}

/// Reprensents an individual branch in a stack.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct StackBranch {
    pub name: String,         // e.g. "feature-1"
    commit_hash: String,      // Last commit on this branch
    pub pr_id: i64,           // PR ID from remote, -1 if not set
    pub status: BranchStatus, // Enum: { Pending, Merged, Conflict, Testing }
    pub parent: String,       // Name of the parent branch
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

/// Status of a branch within the stack.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default)]
pub enum BranchStatus {
    #[default]
    Pending,
    Merged,
    Conflict,
    Testing,
}

/// Global storage structure to handle multiple stacks.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SolMetadata {
    pub stacks: Vec<Stack>,
    pub version: String, // For future migration compatibility
    #[serde(default)]
    pub detached_head_context: Option<DetachedHeadContext>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DetachedHeadContext {
    pub stack_name: String,
    pub branch_name: String,
}

impl SolMetadata {
    pub fn set_detached_head_context(
        &mut self,
        stack_name: String,
        branch_name: String,
    ) -> Result<()> {
        self.detached_head_context = Some(DetachedHeadContext {
            stack_name,
            branch_name,
        });
        Ok(())
    }

    pub fn clear_detached_head_context(&mut self) {
        self.detached_head_context = None;
    }

    pub fn get_detached_head_context(&self) -> Option<&DetachedHeadContext> {
        self.detached_head_context.as_ref()
    }

    pub fn is_in_detached_head(&self) -> bool {
        self.detached_head_context.is_some()
    }
}
