//! Schema types for git_log tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use kodegen_config::{CATEGORY_GIT, GIT_LOG};
use crate::{ToolArgs, tool_metadata};
use super::super::GitCommitInfo;
use super::prompts::LogPrompts;

// ============================================================================
// GIT_LOG TOOL
// ============================================================================

/// Arguments for `git_log` tool
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct GitLogArgs {
    /// Path to repository
    pub path: String,

    /// Maximum number of commits to return
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_count: Option<usize>,

    /// Number of commits to skip
    #[serde(default)]
    pub skip: usize,

    /// Filter commits by file path
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_filter: Option<String>,
}

// ============================================================================
// OUTPUT TYPE
// ============================================================================

/// Output from `git_log` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitLogOutput {
    pub success: bool,
    pub commits: Vec<GitCommitInfo>,
    pub count: usize,
}

#[tool_metadata(
    description = "View commit history with optional filtering"
)]
impl ToolArgs for GitLogArgs {
    type Output = GitLogOutput;
    type Prompts = LogPrompts;

    const NAME: &'static str = GIT_LOG;
    const CATEGORY: &'static str = CATEGORY_GIT;
    const DESCRIPTION: &'static str = "View commit history with optional filtering";
}
