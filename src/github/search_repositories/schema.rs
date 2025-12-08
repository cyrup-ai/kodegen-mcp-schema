//! Schema types for search_repositories tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use kodegen_config::{CATEGORY_GITHUB, GITHUB_SEARCH_REPOSITORIES};

use crate::{ToolArgs, tool_metadata};
use super::prompts::SearchRepositoriesPrompts;

// ============================================================================
// ARGS STRUCT
// ============================================================================

/// Arguments for searching repositories
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SearchRepositoriesArgs {
    /// Search query using GitHub repository search syntax
    pub query: String,
    /// Sort by: "stars", "forks", or "updated" (optional)
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
}

// ============================================================================
// OUTPUT STRUCT
// ============================================================================

/// Output from `github_search_repositories` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitHubSearchReposOutput {
    pub success: bool,
    pub query: String,
    pub total_count: u32,
    pub items: Vec<GitHubRepoSearchResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitHubRepoSearchResult {
    pub full_name: String,
    pub name: String,
    pub owner: String,
    pub description: Option<String>,
    pub html_url: String,
    pub language: Option<String>,
    pub stars: u32,
    pub forks: u32,
    pub watchers: u32,
    pub open_issues: u32,
    pub created_at: String,
    pub updated_at: String,
    pub pushed_at: Option<String>,
    pub topics: Vec<String>,
    pub archived: bool,
    pub fork: bool,
}

// ============================================================================
// TOOLARGS IMPLEMENTATION
// ============================================================================

#[tool_metadata(
    description = "Search repositories across GitHub using query syntax"
)]
impl ToolArgs for SearchRepositoriesArgs {
    type Output = GitHubSearchReposOutput;
    type Prompts = SearchRepositoriesPrompts;

    const NAME: &'static str = GITHUB_SEARCH_REPOSITORIES;
    const CATEGORY: &'static str = CATEGORY_GITHUB;
    const DESCRIPTION: &'static str = "Search repositories across GitHub using query syntax";
}
