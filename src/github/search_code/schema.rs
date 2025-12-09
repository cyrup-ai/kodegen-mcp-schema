//! Schema types for search_code tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use kodegen_config::{CATEGORY_GITHUB, GITHUB_SEARCH_CODE};

use crate::{ToolArgs, tool_metadata};
use super::prompts::SearchCodePrompts;

// ============================================================================
// ARGS STRUCT
// ============================================================================

/// Arguments for searching code
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SearchCodeArgs {
    /// Search query using GitHub code search syntax
    pub query: String,
    /// Sort by: "indexed" (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    /// Order: "asc" or "desc" (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    /// Page number (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    /// Results per page (optional, max 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<u8>,
    /// Enrich results with star counts (default: false)
    #[serde(default)]
    pub enrich_stars: bool,
}

// ============================================================================
// OUTPUT STRUCT
// ============================================================================

/// Output from `github_search_code` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitHubSearchCodeOutput {
    pub success: bool,
    pub query: String,
    pub total_count: u32,
    pub items: Vec<GitHubCodeSearchResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitHubCodeSearchResult {
    pub name: String,
    pub path: String,
    pub sha: String,
    pub repository_full_name: String,
    pub repository_owner: String,
    pub repository_name: String,
    pub html_url: String,
    pub git_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub star_count: Option<u32>,
}

// ============================================================================
// TOOLARGS IMPLEMENTATION
// ============================================================================

#[tool_metadata(
    description = "Search code across GitHub repositories using GitHub's code search syntax"
)]
impl ToolArgs for SearchCodeArgs {
    type Output = GitHubSearchCodeOutput;
    type Prompts = SearchCodePrompts;

    const NAME: &'static str = GITHUB_SEARCH_CODE;
    const CATEGORY: &'static kodegen_config::Category = CATEGORY_GITHUB;
    const DESCRIPTION: &'static str = "Search code across GitHub repositories using GitHub's code search syntax";
}
