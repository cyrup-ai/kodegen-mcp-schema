//! Schema types for create_pull_request_review tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{ToolArgs, tool_metadata};
use super::prompts::CreatePullRequestReviewPrompts;

// ============================================================================
// CANONICAL TOOL NAME CONSTANT
// ============================================================================

/// Canonical tool name for creating a pull request review
pub const GITHUB_CREATE_PULL_REQUEST_REVIEW: &str = "github_create_pull_request_review";

// ============================================================================
// ARGS STRUCT
// ============================================================================

/// Arguments for `create_pull_request_review` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CreatePullRequestReviewArgs {
    /// Repository owner (user or organization)
    pub owner: String,
    /// Repository name
    pub repo: String,
    /// Pull request number
    pub pull_number: u64,
    /// Review action: "APPROVE", "REQUEST_CHANGES", or "COMMENT"
    pub event: String,
    /// Review comment/body text (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// Specific commit SHA to review (optional, defaults to latest)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_id: Option<String>,
}

// ============================================================================
// OUTPUT STRUCT
// ============================================================================

/// Output from `github_create_pull_request_review` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitHubCreatePrReviewOutput {
    pub success: bool,
    pub owner: String,
    pub repo: String,
    pub pr_number: u64,
    pub review_id: u64,
    pub event: String,
    pub message: String,
}

// ============================================================================
// TOOLARGS IMPLEMENTATION
// ============================================================================

#[tool_metadata(
    name = "github_create_pull_request_review",
    category = "github",
    description = "Create a review on a pull request"
)]
impl ToolArgs for CreatePullRequestReviewArgs {
    type Output = GitHubCreatePrReviewOutput;
    type Prompts = CreatePullRequestReviewPrompts;

    const NAME: &'static str = GITHUB_CREATE_PULL_REQUEST_REVIEW;
    const CATEGORY: &'static str = "github";
    const DESCRIPTION: &'static str = "Create a review on a pull request";
}
