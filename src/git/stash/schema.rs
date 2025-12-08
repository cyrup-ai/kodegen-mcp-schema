//! Schema types for git_stash tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use kodegen_config::{CATEGORY_GIT, GIT_STASH};
use crate::{ToolArgs, tool_metadata};
use super::prompts::StashPrompts;


// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

pub fn default_true() -> bool {
    true
}

pub fn default_stash_operation() -> String {
    "save".to_string()
}

// ============================================================================
// GIT_STASH TOOL
// ============================================================================

/// Arguments for `git_stash` tool
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct GitStashArgs {
    /// Path to repository
    pub path: String,

    /// Operation: "save" or "pop" (default: "save")
    #[serde(default = "default_stash_operation")]
    pub operation: String,

    /// Optional stash message (for save operation)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    /// Include untracked files (default: true)
    #[serde(default = "default_true")]
    pub include_untracked: bool,
}

// ============================================================================
// OUTPUT TYPE
// ============================================================================

/// Output from `git_stash` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitStashOutput {
    pub success: bool,
    pub operation: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_hash: Option<String>,
}

#[tool_metadata(
    description = "Temporarily store uncommitted changes and restore them later"
)]
impl ToolArgs for GitStashArgs {
    type Output = GitStashOutput;
    type Prompts = StashPrompts;

    const NAME: &'static str = GIT_STASH;
    const CATEGORY: &'static str = CATEGORY_GIT;
    const DESCRIPTION: &'static str = "Temporarily store uncommitted changes and restore them later";
}
