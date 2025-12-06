//! Schema types for list_pull_requests tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{ToolArgs, tool_metadata};
use super::prompts::ListPullRequestsPrompts;

// ============================================================================
// CANONICAL TOOL NAME CONSTANT
// ============================================================================

/// Canonical tool name for listing pull requests
pub const GITHUB_LIST_PULL_REQUESTS: &str = "github_list_pull_requests";

// ============================================================================
// ARGS STRUCT
// ============================================================================

/// Arguments for `list_pull_requests` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ListPullRequestsArgs {
    /// Repository owner (user or organization)
    pub owner: String,
    /// Repository name
    pub repo: String,
    /// Filter by state: "open", "closed", or "all" (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Filter by labels (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    /// Page number for pagination (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    /// Results per page, max 100 (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<u32>,
}

// ============================================================================
// OUTPUT STRUCT
// ============================================================================

/// Output from `github_list_pull_requests` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitHubListPrsOutput {
    pub success: bool,
    pub owner: String,
    pub repo: String,
    pub count: usize,
    pub pull_requests: Vec<GitHubPrSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitHubPrSummary {
    pub number: u64,
    pub title: String,
    pub state: String,
    pub author: String,
    pub head_ref: String,
    pub base_ref: String,
    pub created_at: String,
    pub draft: bool,
}

// ============================================================================
// TOOLARGS IMPLEMENTATION
// ============================================================================

#[tool_metadata(
    name = "github_list_pull_requests",
    category = "github",
    description = "List pull requests in a GitHub repository"
)]
impl ToolArgs for ListPullRequestsArgs {
    type Output = GitHubListPrsOutput;
    type Prompts = ListPullRequestsPrompts;

    const NAME: &'static str = GITHUB_LIST_PULL_REQUESTS;
    const CATEGORY: &'static str = "github";
    const DESCRIPTION: &'static str = "List pull requests in a GitHub repository";
}
