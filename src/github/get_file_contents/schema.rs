//! Schema types for get_file_contents tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{ToolArgs, tool_metadata};
use super::prompts::GetFileContentsPrompts;

// ============================================================================
// CANONICAL TOOL NAME CONSTANT
// ============================================================================

/// Canonical tool name for getting file contents
pub const GITHUB_GET_FILE_CONTENTS: &str = "github_get_file_contents";

// ============================================================================
// ARGS STRUCT
// ============================================================================

/// Arguments for getting file or directory contents
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GetFileContentsArgs {
    /// Repository owner (user or organization)
    pub owner: String,
    /// Repository name
    pub repo: String,
    /// File or directory path
    pub path: String,
    /// Branch, tag, or commit (optional, defaults to default branch)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_name: Option<String>,
}

// ============================================================================
// OUTPUT STRUCT
// ============================================================================

/// Output from `github_get_file_contents` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitHubGetFileContentsOutput {
    pub success: bool,
    pub owner: String,
    pub repo: String,
    pub path: String,
    pub ref_name: Option<String>,
    pub content_type: String, // "file" or "directory"
    pub file_content: Option<GitHubFileContent>,
    pub directory_contents: Option<Vec<GitHubDirectoryEntry>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitHubFileContent {
    pub name: String,
    pub path: String,
    pub sha: String,
    pub size: u64,
    pub content: String, // decoded base64 content
    pub encoding: String,
    pub html_url: String,
    pub git_url: String,
    pub download_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitHubDirectoryEntry {
    pub name: String,
    pub path: String,
    pub sha: String,
    pub size: u64,
    pub entry_type: String, // "file", "dir", "symlink"
    pub html_url: String,
}

// ============================================================================
// TOOLARGS IMPLEMENTATION
// ============================================================================

#[tool_metadata(
    name = "github_get_file_contents",
    category = "github",
    description = "Get file or directory contents from a GitHub repository"
)]
impl ToolArgs for GetFileContentsArgs {
    type Output = GitHubGetFileContentsOutput;
    type Prompts = GetFileContentsPrompts;

    const NAME: &'static str = GITHUB_GET_FILE_CONTENTS;
    const CATEGORY: &'static str = "github";
    const DESCRIPTION: &'static str = "Get file or directory contents from a GitHub repository";
}
