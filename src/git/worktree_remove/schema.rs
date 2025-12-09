//! Schema types for git_worktree_remove tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use kodegen_config::{CATEGORY_GIT, GIT_WORKTREE_REMOVE};
use crate::{ToolArgs, tool_metadata};
use super::prompts::WorktreeRemovePrompts;

// ============================================================================
// GIT_WORKTREE_REMOVE TOOL
// ============================================================================

/// Arguments for `git_worktree_remove` tool
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct GitWorktreeRemoveArgs {
    /// Path to repository
    pub path: String,

    /// Path to the worktree to remove (both working directory and admin files)
    pub worktree_path: String,

    /// Force removal even if worktree is locked (default: false)
    #[serde(default)]
    pub force: bool,
}

// ============================================================================
// OUTPUT TYPE
// ============================================================================

/// Output from `git_worktree_remove` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitWorktreeRemoveOutput {
    pub success: bool,
    pub worktree_path: String,
    pub message: String,
}

#[tool_metadata(
    description = "Remove a working tree and its administrative files"
)]
impl ToolArgs for GitWorktreeRemoveArgs {
    type Output = GitWorktreeRemoveOutput;
    type Prompts = WorktreeRemovePrompts;

    const NAME: &'static str = GIT_WORKTREE_REMOVE;
    const CATEGORY: &'static kodegen_config::Category = CATEGORY_GIT;
    const DESCRIPTION: &'static str = "Remove a working tree and its administrative files";
}
