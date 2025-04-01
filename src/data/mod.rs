use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// Exporting the other parts of the data system.
pub mod display;
pub mod operations;
pub mod storage;

/// Represents a complete stack
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Stack {
    name: String,                 // e.g. "feature-stack"
    base_branch: String,          // e.g. "main"
    pub head_branch: StackBranch, // The first branch in the stack
    branches: Vec<StackBranch>,   // Ordered list of branches in th stack
    created_at: DateTime<Utc>,    // Timestamp for creation
    updated_at: DateTime<Utc>,    // Last update timestamp
}

/// Reprensents an individual branch in a stack.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct StackBranch {
    pub name: String,         // e.g. "feature-1"
    commit_hash: String,      // Last commit on this branch
    pr_id: Option<u32>,       // Optional PR ID from remote
    pub status: BranchStatus, // Enum: { Pending, Merged, Conflict, Testing }
    pub parent: Option<String>,   // Name of the parent branch, if any
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    pub depth: u8,
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
    stacks: Vec<Stack>,
    version: String, // For future migration compatibility
}
