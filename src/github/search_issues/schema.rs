//! Schema types for search_issues tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{ToolArgs, tool_metadata};
use super::prompts::SearchIssuesPrompts;

// ============================================================================
// CANONICAL TOOL NAME CONSTANT
// ============================================================================

/// Canonical tool name for GitHub issue search
pub const GITHUB_SEARCH_ISSUES: &str = "github_search_issues";

// ============================================================================
// ARGS STRUCT
// ============================================================================

/// Arguments for `search_issues` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SearchIssuesArgs {
    /// GitHub search query (supports complex syntax)
    pub query: String,
    /// Sort results by: "comments", "reactions", "created", "updated" (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    /// Sort order: "asc" or "desc" (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
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

/// Output from `github_search_issues` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitHubSearchIssuesOutput {
    pub success: bool,
    pub query: String,
    pub total_count: u32,
    pub items: Vec<GitHubIssueSummary>,
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
    name = "github_search_issues",
    category = "github",
    description = "Search issues and pull requests across GitHub using query syntax"
)]
impl ToolArgs for SearchIssuesArgs {
    type Output = GitHubSearchIssuesOutput;
    type Prompts = SearchIssuesPrompts;

    const NAME: &'static str = GITHUB_SEARCH_ISSUES;
    const CATEGORY: &'static str = "github";
    const DESCRIPTION: &'static str = "Search issues and pull requests across GitHub using query syntax";
}
