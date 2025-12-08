//! Schema types for create_pull_request tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use kodegen_config::{CATEGORY_GITHUB, GITHUB_CREATE_PULL_REQUEST};

use crate::{ToolArgs, tool_metadata};
use super::prompts::CreatePullRequestPrompts;

// ============================================================================
// ARGS STRUCT
// ============================================================================

/// Arguments for creating a pull request
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CreatePullRequestArgs {
    /// Repository owner (user or organization)
    pub owner: String,
    /// Repository name
    pub repo: String,
    /// Title of the pull request
    pub title: String,
    /// Body/description of the pull request (optional)
    #[serde(default)]
    pub body: Option<String>,
    /// The name of the branch where your changes are implemented (head branch)
    pub head: String,
    /// The name of the branch you want the changes pulled into (base branch)
    pub base: String,
    /// Whether to create the pull request as a draft (optional, defaults to false)
    #[serde(default)]
    pub draft: Option<bool>,
    /// Whether maintainers can modify the pull request (optional, defaults to true)
    #[serde(default)]
    pub maintainer_can_modify: Option<bool>,
}

// ============================================================================
// OUTPUT STRUCT
// ============================================================================

/// Output from `github_create_pull_request` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitHubCreatePrOutput {
    pub success: bool,
    pub owner: String,
    pub repo: String,
    pub pr_number: u64,
    pub html_url: String,
    pub message: String,
}

// ============================================================================
// TOOLARGS IMPLEMENTATION
// ============================================================================

#[tool_metadata(
    description = "Create a new pull request"
)]
impl ToolArgs for CreatePullRequestArgs {
    type Output = GitHubCreatePrOutput;
    type Prompts = CreatePullRequestPrompts;

    const NAME: &'static str = GITHUB_CREATE_PULL_REQUEST;
    const CATEGORY: &'static str = CATEGORY_GITHUB;
    const DESCRIPTION: &'static str = "Create a new pull request";
}
