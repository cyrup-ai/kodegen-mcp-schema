//! Schema types for search_users tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use kodegen_config::{CATEGORY_GITHUB, GITHUB_SEARCH_USERS};

use crate::{ToolArgs, tool_metadata};
use super::prompts::SearchUsersPrompts;

// ============================================================================
// ARGS STRUCT
// ============================================================================

/// Arguments for searching users
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SearchUsersArgs {
    /// Search query using GitHub user search syntax
    pub query: String,
    /// Sort by: "followers", "repositories", or "joined" (optional)
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

/// Output from `github_search_users` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitHubSearchUsersOutput {
    pub success: bool,
    pub query: String,
    pub total_count: u32,
    pub items: Vec<GitHubUserSearchResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitHubUserSearchResult {
    pub login: String,
    pub id: u64,
    pub avatar_url: String,
    pub html_url: String,
    pub user_type: String, // "User" or "Organization"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub followers: Option<u32>,
}

// ============================================================================
// TOOLARGS IMPLEMENTATION
// ============================================================================

#[tool_metadata(
    description = "Search users across GitHub using query syntax"
)]
impl ToolArgs for SearchUsersArgs {
    type Output = GitHubSearchUsersOutput;
    type Prompts = SearchUsersPrompts;

    const NAME: &'static str = GITHUB_SEARCH_USERS;
    const CATEGORY: &'static kodegen_config::Category = CATEGORY_GITHUB;
    const DESCRIPTION: &'static str = "Search users across GitHub using query syntax";
}
