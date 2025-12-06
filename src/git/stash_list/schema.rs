//! Schema types for git_stash_list tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::{ToolArgs, tool_metadata};
use super::prompts::StashListPrompts;

// ============================================================================
// CANONICAL TOOL NAME CONSTANT
// ============================================================================

/// Canonical tool name for git_stash_list
pub const GIT_STASH_LIST: &str = "git_stash_list";

// ============================================================================
// GIT_STASH_LIST TOOL
// ============================================================================

/// Arguments for `git_stash_list` tool
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct GitStashListArgs {
    /// Path to repository
    pub path: String,
}

// ============================================================================
// OUTPUT TYPE
// ============================================================================

/// A single stash entry
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct StashEntry {
    /// Stash index (0 is most recent)
    pub index: usize,
    /// Stash reference (e.g., "stash@{0}")
    #[serde(rename = "ref")]
    pub ref_name: String,
    /// Branch where stash was created
    pub branch: String,
    /// Stash message/description
    pub message: String,
}

/// Output from `git_stash_list` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitStashListOutput {
    /// Whether the operation succeeded
    pub success: bool,
    /// List of stash entries (newest first)
    pub stashes: Vec<StashEntry>,
}

#[tool_metadata(
    name = "git_stash_list",
    category = "git",
    description = "List all stashed changes in the repository"
)]
impl ToolArgs for GitStashListArgs {
    type Output = GitStashListOutput;
    type Prompts = StashListPrompts;

    const NAME: &'static str = GIT_STASH_LIST;
    const CATEGORY: &'static str = "git";
    const DESCRIPTION: &'static str = "List all stashed changes in the repository";
}
