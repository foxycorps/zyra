use super::{BranchStatus, Stack};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize)]
pub struct StackDisplay {
    pub stack: String,
    pub branches: Vec<BranchDisplay>,
}

#[derive(Serialize, Deserialize)]
pub struct BranchDisplay {
    pub name: String,
    pub status: String,
    pub commit: String,
}

impl Stack {
    /// Displays a full stack report.
    pub fn display(&self) -> String {
        let current_branch = crate::git::branch::get_branch_name().unwrap_or("main".to_string());

        // TODO: Need to actually loop through them and order them to actually look at their
        // parent... as currently it just shows the order they were added to the list.
        let mut display = String::new();
        display.push_str(&format!("[sol] Current Stack: {}\n", self.name));
        for (idx, branch) in self.branches.iter().enumerate() {
            let icon = if idx == self.branches.len() - 1 {
                "└─"
            } else {
                "├─"
            };
            let active = if branch.name == current_branch {
                "*"
            } else {
                " "
            };
            display.push_str(&format!(
                "{}  {} {} (commit: {}) [{}]\n",
                active,
                icon,
                branch.name.yellow(),
                branch
                    .commit_hash
                    .to_string()
                    .get(..7)
                    .unwrap_or(&branch.commit_hash)
                    .blue(),
                branch.status.to_string().green()
            ));
        }

        display
    }

    /// json representation of the stack.
    pub fn json(&self) -> String {
        // We will actually create a dedicated struct for this... and use serde to serialize it.
        let mut branches = Vec::new();
        for branch in self.branches.iter() {
            branches.push(BranchDisplay {
                name: branch.name.clone(),
                status: branch.status.to_string(),
                commit: branch
                    .commit_hash
                    .clone()
                    .get(..7)
                    .unwrap_or(&branch.commit_hash)
                    .to_string(),
            });
        }

        let stack = StackDisplay {
            stack: self.name.clone(),
            branches,
        };

        serde_json::to_string_pretty(&stack).unwrap()
    }

    /// displays a simple representation of the stack.
    pub fn simple_display(&self) -> String {
        // We will loop through each branch, and add it to the display, with a ➜ between each.
        let mut display = String::new();
        for (i, branch) in self.branches.iter().enumerate() {
            if i == 0 {
                display.push_str(&branch.name);
            } else {
                display.push_str(&format!(" ➜ {}", branch.name));
            }
        }
        display
    }
}

impl fmt::Display for BranchStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
