//! Schema types for git_stash_apply tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use kodegen_config::{CATEGORY_GIT, GIT_STASH_APPLY};
use crate::{ToolArgs, tool_metadata};
use super::prompts::StashApplyPrompts;

// ============================================================================
// GIT_STASH_APPLY TOOL
// ============================================================================

/// Arguments for `git_stash_apply` tool
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct GitStashApplyArgs {
    /// Path to repository
    pub path: String,

    /// Optional stash reference to apply (e.g., "stash@{0}", "stash@{1}", or just "0", "1")
    /// If not specified, applies the most recent stash (stash@{0})
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stash: Option<String>,
}

// ============================================================================
// OUTPUT TYPE
// ============================================================================

/// Output from `git_stash_apply` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitStashApplyOutput {
    /// Whether the operation succeeded
    pub success: bool,

    /// The stash that was applied
    pub stash: String,

    /// Optional stash message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    /// Files that were restored
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files_restored: Option<Vec<String>>,

    /// Number of lines added
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additions: Option<u32>,

    /// Number of lines deleted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletions: Option<u32>,

    /// Files with conflicts (if any)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflicts: Option<Vec<String>>,
}

#[tool_metadata(
    description = "Apply stashed changes without removing from stash"
)]
impl ToolArgs for GitStashApplyArgs {
    type Output = GitStashApplyOutput;
    type Prompts = StashApplyPrompts;

    const NAME: &'static str = GIT_STASH_APPLY;
    const CATEGORY: &'static kodegen_config::Category = CATEGORY_GIT;
    const DESCRIPTION: &'static str = "Apply stashed changes without removing from stash";
}
