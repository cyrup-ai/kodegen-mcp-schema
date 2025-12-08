//! Schema types for update_pull_request tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use kodegen_config::{CATEGORY_GITHUB, GITHUB_UPDATE_PULL_REQUEST};

use crate::{ToolArgs, tool_metadata};
use super::prompts::UpdatePullRequestPrompts;

// ============================================================================
// ARGS STRUCT
// ============================================================================

/// Arguments for updating a pull request
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct UpdatePullRequestArgs {
    /// Repository owner (user or organization)
    pub owner: String,
    /// Repository name
    pub repo: String,
    /// Pull request number
    pub pr_number: u64,
    /// New title (optional)
    #[serde(default)]
    pub title: Option<String>,
    /// New body/description (optional)
    #[serde(default)]
    pub body: Option<String>,
    /// New state: "open" or "closed" (optional)
    #[serde(default)]
    pub state: Option<String>,
    /// New base branch (optional)
    #[serde(default)]
    pub base: Option<String>,
    /// Whether maintainers can modify the pull request (optional)
    #[serde(default)]
    pub maintainer_can_modify: Option<bool>,
}

// ============================================================================
// OUTPUT STRUCT
// ============================================================================

/// Output from `github_update_pull_request` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitHubUpdatePrOutput {
    pub success: bool,
    pub owner: String,
    pub repo: String,
    pub pr_number: u64,
    pub message: String,
}

// ============================================================================
// TOOLARGS IMPLEMENTATION
// ============================================================================

#[tool_metadata(
    description = "Update an existing pull request"
)]
impl ToolArgs for UpdatePullRequestArgs {
    type Output = GitHubUpdatePrOutput;
    type Prompts = UpdatePullRequestPrompts;

    const NAME: &'static str = GITHUB_UPDATE_PULL_REQUEST;
    const CATEGORY: &'static str = CATEGORY_GITHUB;
    const DESCRIPTION: &'static str = "Update an existing pull request";
}
