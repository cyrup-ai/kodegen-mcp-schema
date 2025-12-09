//! Schema types for get_pull_request_reviews tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use kodegen_config::{CATEGORY_GITHUB, GITHUB_GET_PULL_REQUEST_REVIEWS};

use crate::{ToolArgs, tool_metadata};
use super::prompts::GetPullRequestReviewsPrompts;

// ============================================================================
// ARGS STRUCT
// ============================================================================

/// Arguments for `get_pull_request_reviews` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GetPullRequestReviewsArgs {
    /// Repository owner (user or organization)
    pub owner: String,
    /// Repository name
    pub repo: String,
    /// Pull request number
    pub pull_number: u64,
}

// ============================================================================
// OUTPUT STRUCT
// ============================================================================

/// Output from `github_get_pr_reviews` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitHubPrReviewsOutput {
    pub success: bool,
    pub owner: String,
    pub repo: String,
    pub pr_number: u64,
    pub reviews: Vec<GitHubReview>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitHubReview {
    pub id: u64,
    pub author: String,
    pub state: String,
    pub body: Option<String>,
    pub submitted_at: String,
}

// ============================================================================
// TOOLARGS IMPLEMENTATION
// ============================================================================

#[tool_metadata(
    description = "Get pull request reviews"
)]
impl ToolArgs for GetPullRequestReviewsArgs {
    type Output = GitHubPrReviewsOutput;
    type Prompts = GetPullRequestReviewsPrompts;

    const NAME: &'static str = GITHUB_GET_PULL_REQUEST_REVIEWS;
    const CATEGORY: &'static kodegen_config::Category = CATEGORY_GITHUB;
    const DESCRIPTION: &'static str = "Get pull request reviews";
}
