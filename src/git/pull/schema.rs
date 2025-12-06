//! Schema types for git_pull tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::{ToolArgs, tool_metadata};
use super::prompts::PullPrompts;

// ============================================================================
// CANONICAL TOOL NAME CONSTANT
// ============================================================================

/// Canonical tool name for git_pull
pub const GIT_PULL: &str = "git_pull";


// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

pub fn default_remote() -> String {
    "origin".to_string()
}

pub fn default_true() -> bool {
    true
}

// ============================================================================
// GIT_PULL TOOL
// ============================================================================

/// Arguments for `git_pull` tool
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct GitPullArgs {
    /// Path to repository
    pub path: String,

    /// Remote name (defaults to "origin")
    #[serde(default = "default_remote")]
    pub remote: String,

    /// Allow fast-forward merges (default: true)
    #[serde(default = "default_true")]
    pub fast_forward: bool,

    /// Automatically create merge commit (default: true)
    #[serde(default = "default_true")]
    pub auto_commit: bool,
}

// ============================================================================
// OUTPUT TYPE
// ============================================================================

/// Output from `git_pull` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitPullOutput {
    pub success: bool,
    pub remote: String,
    pub merge_outcome: String,
}

#[tool_metadata(
    name = "git_pull",
    category = "git",
    description = "Fetch and integrate changes from a remote repository"
)]
impl ToolArgs for GitPullArgs {
    type Output = GitPullOutput;
    type Prompts = PullPrompts;

    const NAME: &'static str = GIT_PULL;
    const CATEGORY: &'static str = "git";
    const DESCRIPTION: &'static str = "Fetch and integrate changes from a remote repository";
}
