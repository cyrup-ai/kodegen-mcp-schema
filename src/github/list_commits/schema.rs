//! Schema types for list_commits tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{ToolArgs, tool_metadata};
use super::prompts::ListCommitsPrompts;

// ============================================================================
// CANONICAL TOOL NAME CONSTANT
// ============================================================================

/// Canonical tool name for listing commits
pub const GITHUB_LIST_COMMITS: &str = "github_list_commits";

// ============================================================================
// ARGS STRUCT
// ============================================================================

/// Arguments for listing commits
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ListCommitsArgs {
    /// Repository owner
    pub owner: String,
    /// Repository name
    pub repo: String,
    /// SHA or branch to start listing from (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha: Option<String>,
    /// Filter by file path (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Filter by author (GitHub login or email) (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    /// Only commits after this date (ISO 8601) (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<String>,
    /// Only commits before this date (ISO 8601) (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until: Option<String>,
    /// Page number (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    /// Results per page (optional, max 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<u8>,
}

// ============================================================================
// OUTPUT STRUCT
// ============================================================================

/// Output from `github_list_commits` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitHubListCommitsOutput {
    pub success: bool,
    pub owner: String,
    pub repo: String,
    pub count: usize,
    pub commits: Vec<GitHubCommitSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitHubCommitSummary {
    pub sha: String,
    pub message: String,
    pub author_name: String,
    pub author_email: String,
    pub date: String,
    pub html_url: String,
}

// ============================================================================
// TOOLARGS IMPLEMENTATION
// ============================================================================

#[tool_metadata(
    name = "github_list_commits",
    category = "github",
    description = "List commits in a GitHub repository"
)]
impl ToolArgs for ListCommitsArgs {
    type Output = GitHubListCommitsOutput;
    type Prompts = ListCommitsPrompts;

    const NAME: &'static str = GITHUB_LIST_COMMITS;
    const CATEGORY: &'static str = "github";
    const DESCRIPTION: &'static str = "List commits in a GitHub repository";
}
