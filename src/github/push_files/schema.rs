//! Schema types for push_files tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{ToolArgs, tool_metadata};
use super::prompts::PushFilesPrompts;

// ============================================================================
// CANONICAL TOOL NAME CONSTANT
// ============================================================================

/// Canonical tool name for pushing files
pub const GITHUB_PUSH_FILES: &str = "github_push_files";

// ============================================================================
// ARGS STRUCT
// ============================================================================

/// Arguments for push_files tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PushFilesArgs {
    /// Repository owner (user or organization)
    pub owner: String,
    /// Repository name
    pub repo: String,
    /// Branch name
    pub branch: String,
    /// Map of file paths to base64-encoded content
    pub files: HashMap<String, String>,
    /// Commit message
    pub message: String,
}

// ============================================================================
// OUTPUT STRUCT
// ============================================================================

/// Output from `github_push_files` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitHubPushFilesOutput {
    pub success: bool,
    pub owner: String,
    pub repo: String,
    pub branch: String,
    pub files_pushed: u32,
    pub commit_sha: String,
    pub commit_message: String,
    pub html_url: String,
}

// ============================================================================
// TOOLARGS IMPLEMENTATION
// ============================================================================

#[tool_metadata(
    name = "github_push_files",
    category = "github",
    description = "Push files to a GitHub repository"
)]
impl ToolArgs for PushFilesArgs {
    type Output = GitHubPushFilesOutput;
    type Prompts = PushFilesPrompts;

    const NAME: &'static str = GITHUB_PUSH_FILES;
    const CATEGORY: &'static str = "github";
    const DESCRIPTION: &'static str = "Push files to a GitHub repository";
}
