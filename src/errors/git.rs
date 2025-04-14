use thiserror::Error;

/// Error type for git operations.
#[derive(Error, Debug)]
pub enum GitError {
    #[error("Git command failed: {0}")]
    CommandFailed(String),

    #[error("Git command not found")]
    CommandNotFound,

    #[error("Not a git repository")]
    NotGitRepository,

    #[error("Invalid git output: {0}")]
    InvalidOutput(String),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("UTF-8 error: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),
}

/// Error type for GitHub API operations
#[derive(Debug, Error)]
pub enum GitHubError {
    #[error("GitHub authentication failed: Please set GITHUB_TOKEN or SAGE_GITHUB_TOKEN environment variable")]
    AuthenticationError,

    #[error("GitHub API request failed: {0}")]
    RequestError(String),

    #[error("GitHub resource not found: {0}")]
    NotFound(String),

    #[error("GitHub rate limit exceeded. Please wait or use an authenticated token")]
    RateLimitExceeded,
}
