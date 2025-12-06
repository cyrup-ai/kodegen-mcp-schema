//! Schema types for list_issues tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{ToolArgs, tool_metadata};
use super::prompts::ListIssuesPrompts;

// ============================================================================
// CANONICAL TOOL NAME CONSTANT
// ============================================================================

/// Canonical tool name for listing issues
pub const GITHUB_LIST_ISSUES: &str = "github_list_issues";

// ============================================================================
// ARGS STRUCT
// ============================================================================

/// Arguments for `list_issues` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ListIssuesArgs {
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
    /// Filter by assignee username (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee: Option<String>,
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

/// Output from `github_list_issues` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitHubListIssuesOutput {
    pub success: bool,
    pub owner: String,
    pub repo: String,
    pub count: usize,
    pub issues: Vec<GitHubIssueSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitHubIssueSummary {
    pub number: u64,
    pub title: String,
    pub state: String,
    pub author: String,
    pub created_at: String,
    pub labels: Vec<String>,
}

// ============================================================================
// TOOLARGS IMPLEMENTATION
// ============================================================================

#[tool_metadata(
    name = "github_list_issues",
    category = "github",
    description = "List issues in a GitHub repository"
)]
impl ToolArgs for ListIssuesArgs {
    type Output = GitHubListIssuesOutput;
    type Prompts = ListIssuesPrompts;

    const NAME: &'static str = GITHUB_LIST_ISSUES;
    const CATEGORY: &'static str = "github";
    const DESCRIPTION: &'static str = "List issues in a GitHub repository";
}
