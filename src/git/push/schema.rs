//! Schema types for git_push tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use kodegen_config::{CATEGORY_GIT, GIT_PUSH};
use crate::{ToolArgs, tool_metadata};
use super::prompts::PushPrompts;


// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

pub fn default_remote() -> String {
    "origin".to_string()
}

// ============================================================================
// GIT_PUSH TOOL
// ============================================================================

/// Arguments for `git_push` tool
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct GitPushArgs {
    /// Path to repository
    pub path: String,

    /// Remote name (defaults to "origin")
    #[serde(default = "default_remote")]
    pub remote: String,

    /// Refspecs to push (e.g., ["refs/heads/main", "refs/tags/v1.0"]).
    /// Empty list pushes the current branch to the remote.
    /// 
    /// Accepts both single string and array: `refspecs: "main"` or `refspecs: ["main", "develop"]`
    #[serde(default, deserialize_with = "crate::serde_helpers::string_or_vec")]
    pub refspecs: Vec<String>,

    /// Force push (use with caution in shared repositories)
    #[serde(default)]
    pub force: bool,

    /// Push all tags to the remote
    #[serde(default)]
    pub tags: bool,

    /// Push operation timeout in seconds (default: 300 for 5 minutes)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_secs: Option<u64>,
}

// ============================================================================
// OUTPUT TYPE
// ============================================================================

/// Output from `git_push` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitPushOutput {
    pub success: bool,
    pub remote: String,
    pub refs_pushed: u32,
    pub tags_pushed: u32,
    pub force: bool,
    pub warnings: Vec<String>,
}

#[tool_metadata(
    description = "Push commits to a remote repository"
)]
impl ToolArgs for GitPushArgs {
    type Output = GitPushOutput;
    type Prompts = PushPrompts;

    const NAME: &'static str = GIT_PUSH;
    const CATEGORY: &'static str = CATEGORY_GIT;
    const DESCRIPTION: &'static str = "Push commits to a remote repository";
}
