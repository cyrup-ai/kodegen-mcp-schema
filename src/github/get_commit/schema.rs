//! Schema types for get_commit tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{ToolArgs, tool_metadata};
use super::prompts::GetCommitPrompts;

// ============================================================================
// CANONICAL TOOL NAME CONSTANT
// ============================================================================

/// Canonical tool name for getting a commit
pub const GITHUB_GET_COMMIT: &str = "github_get_commit";

// ============================================================================
// ARGS STRUCT
// ============================================================================

/// Arguments for getting a commit
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GetCommitArgs {
    /// Repository owner
    pub owner: String,
    /// Repository name
    pub repo: String,
    /// Commit SHA
    pub commit_sha: String,
    /// Page number for files (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    /// Results per page (optional, max 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<u8>,
}

// ============================================================================
// OUTPUT STRUCT
// ============================================================================

/// Output from `github_get_commit` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitHubGetCommitOutput {
    pub success: bool,
    pub owner: String,
    pub repo: String,
    pub commit: GitHubCommitDetail,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitHubCommitDetail {
    pub sha: String,
    pub message: String,
    pub author_name: String,
    pub author_email: String,
    pub committer_name: String,
    pub committer_email: String,
    pub author_date: String,
    pub commit_date: String,
    pub parents: Vec<String>,
    pub html_url: String,
    pub stats: Option<GitHubCommitStats>,
    pub files: Vec<GitHubCommitFile>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitHubCommitStats {
    pub additions: u32,
    pub deletions: u32,
    pub total: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitHubCommitFile {
    pub filename: String,
    pub status: String, // "added", "removed", "modified", "renamed"
    pub additions: u32,
    pub deletions: u32,
    pub changes: u32,
    pub patch: Option<String>,
}

// ============================================================================
// TOOLARGS IMPLEMENTATION
// ============================================================================

#[tool_metadata(
    name = "github_get_commit",
    category = "github",
    description = "Get details about a specific commit"
)]
impl ToolArgs for GetCommitArgs {
    type Output = GitHubGetCommitOutput;
    type Prompts = GetCommitPrompts;

    const NAME: &'static str = GITHUB_GET_COMMIT;
    const CATEGORY: &'static str = "github";
    const DESCRIPTION: &'static str = "Get details about a specific commit";
}
