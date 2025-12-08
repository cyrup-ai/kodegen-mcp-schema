//! Schema types for get_me tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use kodegen_config::{CATEGORY_GITHUB, GITHUB_GET_ME};

use crate::{ToolArgs, tool_metadata};
use super::prompts::GetMePrompts;

// ============================================================================
// ARGS STRUCT
// ============================================================================

/// Arguments for get_me tool (no arguments needed)
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GetMeArgs {
    // No fields - uses GITHUB_TOKEN for authentication
}

// ============================================================================
// OUTPUT STRUCT
// ============================================================================

/// Output from `github_get_me` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitHubGetMeOutput {
    pub success: bool,
    pub login: String,
    pub id: u64,
    pub name: Option<String>,
    pub email: Option<String>,
    pub avatar_url: String,
    pub html_url: String,
    pub bio: Option<String>,
    pub location: Option<String>,
    pub company: Option<String>,
    pub followers: u32,
    pub following: u32,
    pub public_repos: u32,
    pub created_at: String,
}

// ============================================================================
// TOOLARGS IMPLEMENTATION
// ============================================================================

#[tool_metadata(
    description = "Get authenticated user information"
)]
impl ToolArgs for GetMeArgs {
    type Output = GitHubGetMeOutput;
    type Prompts = GetMePrompts;

    const NAME: &'static str = GITHUB_GET_ME;
    const CATEGORY: &'static str = CATEGORY_GITHUB;
    const DESCRIPTION: &'static str = "Get authenticated user information";
}
