//! Schema types for get_issue tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use kodegen_config::{CATEGORY_GITHUB, GITHUB_GET_ISSUE};

use crate::{ToolArgs, tool_metadata};
use super::prompts::GetIssuePrompts;

// ============================================================================
// ARGS STRUCT
// ============================================================================

/// Arguments for `get_issue` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GetIssueArgs {
    /// Repository owner (user or organization)
    pub owner: String,
    /// Repository name
    pub repo: String,
    /// Issue number
    pub issue_number: u64,
}

// ============================================================================
// OUTPUT STRUCT
// ============================================================================

/// Output from `github_get_issue` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitHubGetIssueOutput {
    pub success: bool,
    pub owner: String,
    pub repo: String,
    pub issue: GitHubIssue,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitHubIssue {
    pub number: u64,
    pub title: String,
    pub body: Option<String>,
    pub state: String,
    pub author: String,
    pub created_at: String,
    pub updated_at: String,
    pub labels: Vec<String>,
    pub assignees: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closed_at: Option<String>,
    pub comments_count: u32,
    pub html_url: String,
}

// ============================================================================
// TOOLARGS IMPLEMENTATION
// ============================================================================

#[tool_metadata(
    description = "Get details about a specific GitHub issue"
)]
impl ToolArgs for GetIssueArgs {
    type Output = GitHubGetIssueOutput;
    type Prompts = GetIssuePrompts;

    const NAME: &'static str = GITHUB_GET_ISSUE;
    const CATEGORY: &'static str = CATEGORY_GITHUB;
    const DESCRIPTION: &'static str = "Get details about a specific GitHub issue";
}
