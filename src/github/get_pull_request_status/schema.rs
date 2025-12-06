//! Schema types for get_pull_request_status tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{ToolArgs, tool_metadata};
use super::prompts::GetPullRequestStatusPrompts;

// ============================================================================
// CANONICAL TOOL NAME CONSTANT
// ============================================================================

/// Canonical tool name for getting pull request status
pub const GITHUB_GET_PULL_REQUEST_STATUS: &str = "github_get_pull_request_status";

// ============================================================================
// ARGS STRUCT
// ============================================================================

/// Arguments for getting pull request status
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GetPullRequestStatusArgs {
    /// Repository owner (user or organization)
    pub owner: String,
    /// Repository name
    pub repo: String,
    /// Pull request number
    pub pr_number: u64,
}

// ============================================================================
// OUTPUT STRUCT
// ============================================================================

/// Output from `github_get_pull_request_status` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitHubGetPrStatusOutput {
    pub success: bool,
    pub owner: String,
    pub repo: String,
    pub pr_number: u64,
    pub state: String,
    pub mergeable: Option<bool>,
    pub checks_status: String,
    pub checks_count: u32,
    pub checks_passed: u32,
    pub checks_failed: u32,
}

// ============================================================================
// TOOLARGS IMPLEMENTATION
// ============================================================================

#[tool_metadata(
    name = "github_get_pull_request_status",
    category = "github",
    description = "Get pull request status"
)]
impl ToolArgs for GetPullRequestStatusArgs {
    type Output = GitHubGetPrStatusOutput;
    type Prompts = GetPullRequestStatusPrompts;

    const NAME: &'static str = GITHUB_GET_PULL_REQUEST_STATUS;
    const CATEGORY: &'static str = "github";
    const DESCRIPTION: &'static str = "Get pull request status";
}
