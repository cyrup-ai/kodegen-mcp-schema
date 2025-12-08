//! Schema types for add_issue_comment tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use kodegen_config::{CATEGORY_GITHUB, GITHUB_ADD_ISSUE_COMMENT};

use crate::{ToolArgs, tool_metadata};
use super::prompts::AddIssueCommentPrompts;

// ============================================================================
// ARGS STRUCT
// ============================================================================

/// Arguments for `add_issue_comment` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AddIssueCommentArgs {
    /// Repository owner (user or organization)
    pub owner: String,
    /// Repository name
    pub repo: String,
    /// Issue number to comment on
    pub issue_number: u64,
    /// Comment text (Markdown supported)
    pub body: String,
}

// ============================================================================
// OUTPUT STRUCT
// ============================================================================

/// Output from `github_add_issue_comment` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitHubAddIssueCommentOutput {
    pub success: bool,
    pub owner: String,
    pub repo: String,
    pub issue_number: u64,
    pub comment_id: u64,
    pub message: String,
}

// ============================================================================
// TOOLARGS IMPLEMENTATION
// ============================================================================

#[tool_metadata(
    description = "Add a comment to an existing GitHub issue"
)]
impl ToolArgs for AddIssueCommentArgs {
    type Output = GitHubAddIssueCommentOutput;
    type Prompts = AddIssueCommentPrompts;

    const NAME: &'static str = GITHUB_ADD_ISSUE_COMMENT;
    const CATEGORY: &'static str = CATEGORY_GITHUB;
    const DESCRIPTION: &'static str = "Add a comment to an existing GitHub issue";
}
