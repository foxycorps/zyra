use crate::errors::git::GitHubError;
use crate::gh;
use anyhow::Result;
use octocrab::models::pulls::PullRequest;

/// Maps octocrab errors to our custom GitHubError types
fn map_github_error(err: octocrab::Error) -> anyhow::Error {
    // Convert the error to a string to check for specific error conditions
    let err_string = err.to_string();

    if err_string.contains("401") || err_string.contains("Unauthorized") {
        GitHubError::AuthenticationError.into()
    } else if err_string.contains("404") || err_string.contains("Not Found") {
        GitHubError::NotFound("Pull request or repository not found".to_string()).into()
    } else if err_string.contains("403") || err_string.contains("rate limit") {
        GitHubError::RateLimitExceeded.into()
    } else {
        GitHubError::RequestError(format!("GitHub API error: {}", err)).into()
    }
}

/// Creates a new pull request for a given repository
pub async fn create_pull_request(
    owner: &str,
    repo: &str,
    title: &str,
    head: &str,
    base: &str,
    body: &str,
    draft: bool,
) -> Result<PullRequest> {
    gh::get_instance()
        .pulls(owner, repo)
        .create(title, head, base)
        .body(body)
        .draft(Some(draft))
        .send()
        .await
        .map_err(map_github_error)
}

/// Find an existing pull request for a branch
pub async fn find_pull_request(
    owner: &str,
    repo: &str,
    head: &str,
) -> Result<Option<PullRequest>> {
    // List open pull requests for the repository
    let pulls = gh::get_instance()
        .pulls(owner, repo)
        .list()
        .state(octocrab::params::State::Open)
        .per_page(100) // Increase to handle repositories with many PRs
        .send()
        .await
        .map_err(map_github_error)?;
    
    // Find the PR that matches our head branch
    let pr = pulls.items.into_iter().find(|pr| {
        // The head ref might be in the format "username:branch-name"
        // or just "branch-name" if it's in the same repository
        pr.head.ref_field == head || pr.head.ref_field.ends_with(&format!(":{}", head))
    });
    
    Ok(pr)
}

/// Update an existing pull request
pub async fn update_pull_request(
    owner: &str,
    repo: &str,
    pr_number: u64,
    title: Option<&str>,
    body: Option<&str>,
    base: Option<&str>,
) -> Result<PullRequest> {
    // Store the instance in a variable to extend its lifetime
    let instance = gh::get_instance();
    let pulls = instance.pulls(owner, repo);
    let mut update = pulls.update(pr_number);
    
    if let Some(title_text) = title {
        update = update.title(title_text);
    }
    
    if let Some(body_text) = body {
        update = update.body(body_text);
    }
    
    if let Some(base_branch) = base {
        update = update.base(base_branch);
    }
    
    update.send().await.map_err(map_github_error)
}
