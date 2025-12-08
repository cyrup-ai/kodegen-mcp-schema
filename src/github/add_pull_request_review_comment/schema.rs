//! Schema types for add_pull_request_review_comment tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use kodegen_config::{CATEGORY_GITHUB, GITHUB_ADD_PULL_REQUEST_REVIEW_COMMENT};

use crate::{ToolArgs, tool_metadata};
use super::prompts::AddPullRequestReviewCommentPrompts;

// ============================================================================
// ARGS STRUCT
// ============================================================================

/// Arguments for `add_pull_request_review_comment` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AddPullRequestReviewCommentArgs {
    /// Repository owner (user or organization)
    pub owner: String,
    /// Repository name
    pub repo: String,
    /// Pull request number
    pub pull_number: u64,
    /// Comment body text
    pub body: String,
    /// Commit SHA to comment on (required for new comments)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_id: Option<String>,
    /// File path to comment on (required for new comments)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Line number in the diff to comment on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<u32>,
    /// Side of diff: "LEFT" or "RIGHT" (default: RIGHT)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    /// Start line for multi-line comment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_line: Option<u32>,
    /// Side of start line: "LEFT" or "RIGHT"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_side: Option<String>,
    /// Subject type (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_type: Option<String>,
    /// Comment ID to reply to (for threaded replies)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_reply_to: Option<u64>,
}

// ============================================================================
// OUTPUT STRUCT
// ============================================================================

/// Output from `github_add_pull_request_review_comment` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitHubAddPrReviewCommentOutput {
    pub success: bool,
    pub owner: String,
    pub repo: String,
    pub pr_number: u64,
    pub comment_id: u64,
    pub message: String,
}

// ============================================================================
// TOOLARGS IMPLEMENTATION
// ============================================================================

#[tool_metadata(
    description = "Add a comment to a pull request review"
)]
impl ToolArgs for AddPullRequestReviewCommentArgs {
    type Output = GitHubAddPrReviewCommentOutput;
    type Prompts = AddPullRequestReviewCommentPrompts;

    const NAME: &'static str = GITHUB_ADD_PULL_REQUEST_REVIEW_COMMENT;
    const CATEGORY: &'static str = CATEGORY_GITHUB;
    const DESCRIPTION: &'static str = "Add a comment to a pull request review";
}
