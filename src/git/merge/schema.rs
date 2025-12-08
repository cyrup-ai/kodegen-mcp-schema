//! Schema types for git_merge tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use kodegen_config::{CATEGORY_GIT, GIT_MERGE};
use crate::{ToolArgs, tool_metadata};
use super::prompts::MergePrompts;


// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

pub fn default_true() -> bool {
    true
}

// ============================================================================
// GIT_MERGE TOOL
// ============================================================================

/// Arguments for `git_merge` tool
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct GitMergeArgs {
    /// Path to repository
    pub path: String,

    /// Branch or commit to merge into current branch
    pub branch: String,

    /// Allow fast-forward merges when possible (default: true).
    /// When false, always creates a merge commit even if fast-forward is possible.
    #[serde(default = "default_true")]
    pub fast_forward: bool,

    /// Automatically create merge commit (default: true).
    /// When false, performs merge but leaves changes staged for manual commit.
    #[serde(default = "default_true")]
    pub auto_commit: bool,
}

// ============================================================================
// OUTPUT TYPE
// ============================================================================

/// Output from `git_merge` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitMergeOutput {
    pub success: bool,
    pub merge_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_id: Option<String>,
    pub message: String,
}

#[tool_metadata(
    description = "Merge changes from one branch into another"
)]
impl ToolArgs for GitMergeArgs {
    type Output = GitMergeOutput;
    type Prompts = MergePrompts;

    const NAME: &'static str = GIT_MERGE;
    const CATEGORY: &'static str = CATEGORY_GIT;
    const DESCRIPTION: &'static str = "Merge changes from one branch into another";
}
