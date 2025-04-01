use thiserror::Error;

pub mod git;
pub use git::GitError;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Git error: {0}")]
    Git(#[from] GitError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("No previous branch")]
    NoPreviousBranch,

    #[error("No next branch")]
    NoNextBranch,

    #[error("Branch not part of stack")]
    BranchNotPartOfStack,

    #[error("{0}")]
    Other(String),
}

impl From<String> for AppError {
    fn from(msg: String) -> Self {
        Self::Other(msg)
    }
}

impl From<&str> for AppError {
    fn from(msg: &str) -> Self {
        Self::Other(msg.to_string())
    }
}
