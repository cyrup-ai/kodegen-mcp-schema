//! Schema types for create_or_update_file tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use kodegen_config::{CATEGORY_GITHUB, GITHUB_CREATE_OR_UPDATE_FILE};

use crate::{ToolArgs, tool_metadata};
use super::prompts::CreateOrUpdateFilePrompts;

// ============================================================================
// ARGS STRUCT
// ============================================================================

/// Arguments for creating or updating a file
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CreateOrUpdateFileArgs {
    /// Repository owner (user or organization)
    pub owner: String,
    /// Repository name
    pub repo: String,
    /// File path
    pub path: String,
    /// Commit message
    pub message: String,
    /// File content (plain text, will be base64 encoded automatically)
    pub content: String,
    /// Branch name (optional, defaults to default branch)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    /// File SHA for updates (optional, omit for creation)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha: Option<String>,
}

// ============================================================================
// OUTPUT STRUCT
// ============================================================================

/// Output from `github_create_or_update_file` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitHubCreateOrUpdateFileOutput {
    pub success: bool,
    pub owner: String,
    pub repo: String,
    pub path: String,
    pub sha: String,
    pub commit_sha: String,
    pub commit_message: String,
    pub html_url: String,
    pub operation: String, // "created" or "updated"
}

// ============================================================================
// TOOLARGS IMPLEMENTATION
// ============================================================================

#[tool_metadata(
    description = "Create or update a file in a repository"
)]
impl ToolArgs for CreateOrUpdateFileArgs {
    type Output = GitHubCreateOrUpdateFileOutput;
    type Prompts = CreateOrUpdateFilePrompts;

    const NAME: &'static str = GITHUB_CREATE_OR_UPDATE_FILE;
    const CATEGORY: &'static str = CATEGORY_GITHUB;
    const DESCRIPTION: &'static str = "Create or update a file in a repository";
}
