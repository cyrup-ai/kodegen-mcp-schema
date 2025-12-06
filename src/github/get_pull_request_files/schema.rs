//! Schema types for get_pull_request_files tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{ToolArgs, tool_metadata};
use super::prompts::GetPullRequestFilesPrompts;

// ============================================================================
// CANONICAL TOOL NAME CONSTANT
// ============================================================================

/// Canonical tool name for getting pull request files
pub const GITHUB_GET_PULL_REQUEST_FILES: &str = "github_get_pull_request_files";

// ============================================================================
// ARGS STRUCT
// ============================================================================

/// Arguments for getting pull request files
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GetPullRequestFilesArgs {
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

/// Output from `github_get_pull_request_files` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitHubGetPrFilesOutput {
    pub success: bool,
    pub owner: String,
    pub repo: String,
    pub pr_number: u64,
    pub count: usize,
    pub files: Vec<GitHubPrFile>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitHubPrFile {
    pub filename: String,
    pub status: String,
    pub additions: u32,
    pub deletions: u32,
    pub changes: u32,
    pub patch: Option<String>,
}

// ============================================================================
// TOOLARGS IMPLEMENTATION
// ============================================================================

#[tool_metadata(
    name = "github_get_pull_request_files",
    category = "github",
    description = "Get all files changed in a pull request"
)]
impl ToolArgs for GetPullRequestFilesArgs {
    type Output = GitHubGetPrFilesOutput;
    type Prompts = GetPullRequestFilesPrompts;

    const NAME: &'static str = GITHUB_GET_PULL_REQUEST_FILES;
    const CATEGORY: &'static str = "github";
    const DESCRIPTION: &'static str = "Get all files changed in a pull request";
}
