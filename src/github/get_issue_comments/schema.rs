//! Schema types for get_issue_comments tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{ToolArgs, tool_metadata};
use super::prompts::GetIssueCommentsPrompts;

// ============================================================================
// CANONICAL TOOL NAME CONSTANT
// ============================================================================

/// Canonical tool name for getting issue comments
pub const GITHUB_GET_ISSUE_COMMENTS: &str = "github_get_issue_comments";

// ============================================================================
// ARGS STRUCT
// ============================================================================

/// Arguments for `get_issue_comments` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GetIssueCommentsArgs {
    /// Repository owner (user or organization)
    pub owner: String,
    /// Repository name
    pub repo: String,
    /// Issue number to fetch comments for
    pub issue_number: u64,
}

// ============================================================================
// OUTPUT STRUCT
// ============================================================================

/// Output from `github_get_issue_comments` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitHubGetIssueCommentsOutput {
    pub success: bool,
    pub owner: String,
    pub repo: String,
    pub issue_number: u64,
    pub count: usize,
    pub comments: Vec<GitHubComment>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitHubComment {
    pub id: u64,
    pub author: String,
    pub body: String,
    pub created_at: String,
    pub updated_at: String,
}

// ============================================================================
// TOOLARGS IMPLEMENTATION
// ============================================================================

#[tool_metadata(
    name = "github_get_issue_comments",
    category = "github",
    description = "Get all comments on a GitHub issue"
)]
impl ToolArgs for GetIssueCommentsArgs {
    type Output = GitHubGetIssueCommentsOutput;
    type Prompts = GetIssueCommentsPrompts;

    const NAME: &'static str = GITHUB_GET_ISSUE_COMMENTS;
    const CATEGORY: &'static str = "github";
    const DESCRIPTION: &'static str = "Get all comments on a GitHub issue";
}
