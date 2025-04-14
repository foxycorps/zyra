use super::{BranchStatus, Stack, StackBranch};
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
    /// Get the path from the root to the head branch
    pub fn get_path_to_head(&self) -> Vec<&StackBranch> {
        let mut path = Vec::new();
        let mut current = &self.head_branch;
        path.push(current);

        while let Some(branch) = self.branches.iter().find(|b| b.name == current.name) {
            current = if !branch.parent.is_empty() {
                if let Some(parent) = self.branches.iter().find(|b| b.name == branch.parent) {
                    path.push(parent);
                    parent
                } else {
                    break;
                }
            } else {
                break;
            };
        }

        path.reverse();
        path
    }

    /// Displays a full stack report.
    pub fn display(&self, show_graph: bool) -> String {
        let current_branch = crate::git::branch::get_branch_name().unwrap_or("main".to_string());

        let mut display = String::new();
        display.push_str(&format!(
            "{}  {}\n",
            "[zyra]".bright_purple(),
            format!("Stack: {}", self.name).bold()
        ));

        if !show_graph {
            // Find the current branch in the stack
            let current_branch_obj = self.branches.iter().find(|b| b.name == current_branch);

            // If the current branch is not found, just show the head branch
            if current_branch_obj.is_none() {
                let branch = &self.head_branch;
                let active = if branch.name == current_branch {
                    "●".bright_green().bold()
                } else {
                    "○".dimmed()
                };

                // Format commit hash with brackets
                let commit_hash = format!(
                    "[{}]",
                    branch
                        .commit_hash
                        .to_string()
                        .get(..7)
                        .unwrap_or(&branch.commit_hash)
                )
                .blue();

                // Format status if not pending
                let status = if !matches!(branch.status, BranchStatus::Pending) {
                    format!(" {}", branch.status.to_string().green())
                } else {
                    "".to_string()
                };

                // Add PR indicator if PR exists
                let pr_indicator = if branch.pr_id > 0 {
                    format!(" {}", format!("PR #{}", branch.pr_id).bright_cyan())
                } else {
                    "".to_string()
                };

                display.push_str(&format!(
                    "   {} {} {}{}{}\n",
                    active,
                    branch.name.yellow().bold(),
                    commit_hash,
                    status,
                    pr_indicator
                ));

                return display;
            }

            // Build the path from current branch to root
            let mut path_to_root = Vec::new();
            let mut current = current_branch_obj.unwrap();
            path_to_root.push(current);

            while !current.parent.is_empty() {
                if let Some(parent) = self.branches.iter().find(|b| b.name == current.parent) {
                    path_to_root.push(parent);
                    current = parent;
                } else {
                    break;
                }
            }

            // Display the path in reverse order (from root to current)
            for (i, branch) in path_to_root.iter().rev().enumerate() {
                let active = if branch.name == current_branch {
                    "●".bright_green().bold()
                } else {
                    "○".dimmed()
                };

                // Format commit hash with brackets
                let commit_hash = format!(
                    "[{}]",
                    branch
                        .commit_hash
                        .to_string()
                        .get(..7)
                        .unwrap_or(&branch.commit_hash)
                )
                .blue();

                // Format status if not pending
                let status = if !matches!(branch.status, BranchStatus::Pending) {
                    format!(" {}", branch.status.to_string().green())
                } else {
                    "".to_string()
                };

                // Add PR indicator if PR exists
                let pr_indicator = if branch.pr_id > 0 {
                    format!(" {}", format!("PR #{}", branch.pr_id).bright_cyan())
                } else {
                    "".to_string()
                };

                display.push_str(&format!(
                    "   {} {} {}{}{}\n",
                    active,
                    branch.name.yellow().bold(),
                    commit_hash,
                    status,
                    pr_indicator
                ));

                // Add a separator line between branches
                if i < path_to_root.len() - 1 {
                    display.push_str("   │\n");
                }
            }

            return display;
        }

        // Graph display when --graph is used
        let mut children_map: std::collections::HashMap<String, Vec<&StackBranch>> =
            std::collections::HashMap::new();

        // First, collect all branches by their parent
        for branch in &self.branches {
            children_map
                .entry(branch.parent.clone())
                .or_insert_with(Vec::new)
                .push(branch);
        }

        // Find the root branches - these are branches whose parent is not in the stack
        // or branches with empty parent
        let root_branches: Vec<&StackBranch> = self
            .branches
            .iter()
            .filter(|branch| {
                branch.parent.is_empty() || !self.branches.iter().any(|b| b.name == branch.parent)
            })
            .collect();

        fn display_branch(
            branch: &StackBranch,
            children_map: &std::collections::HashMap<String, Vec<&StackBranch>>,
            current_branch: &str,
            prefix: &str,
            is_last: bool,
            display: &mut String,
        ) {
            let active = if branch.name == current_branch {
                "●".bright_green().bold()
            } else {
                "○".dimmed()
            };
            let branch_symbol = if is_last { "└──" } else { "├──" };

            // Format commit hash with brackets
            let commit_hash = format!(
                "[{}]",
                branch
                    .commit_hash
                    .to_string()
                    .get(..7)
                    .unwrap_or(&branch.commit_hash)
            )
            .blue();

            // Format status if not pending
            let status = if !matches!(branch.status, BranchStatus::Pending) {
                format!(" {}", branch.status.to_string().green())
            } else {
                "".to_string()
            };

            // Add PR indicator if PR exists
            let pr_indicator = if branch.pr_id > 0 {
                format!(" {}", format!("PR #{}", branch.pr_id).bright_cyan())
            } else {
                "".to_string()
            };

            // Root indicator
            let root_indicator = if branch.parent.is_empty() {
                " (root)".dimmed()
            } else {
                "".into()
            };

            display.push_str(&format!(
                "{} {}{}{} {}{}{}{}\n",
                active,
                prefix,
                branch_symbol,
                branch.name.yellow().bold(),
                commit_hash,
                status,
                pr_indicator,
                root_indicator
            ));

            if let Some(children) = children_map.get(&branch.name) {
                let child_prefix = if is_last {
                    format!("{}    ", prefix)
                } else {
                    format!("{}│   ", prefix)
                };

                for (i, child) in children.iter().enumerate() {
                    display_branch(
                        child,
                        children_map,
                        current_branch,
                        &child_prefix,
                        i == children.len() - 1,
                        display,
                    );
                }
            }
        }

        // Display all root branches
        for (i, branch) in root_branches.iter().enumerate() {
            display_branch(
                branch,
                &children_map,
                &current_branch,
                "",
                i == root_branches.len() - 1,
                &mut display,
            );
        }

        display
    }

    /// json representation of the stack.
    pub fn json(&self, pretty: bool) -> String {
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

        if pretty {
            serde_json::to_string_pretty(&stack).unwrap()
        } else {
            serde_json::to_string(&stack).unwrap()
        }
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
