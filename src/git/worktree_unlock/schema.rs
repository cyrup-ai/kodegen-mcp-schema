//! Schema types for git_worktree_unlock tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::{ToolArgs, tool_metadata};
use super::prompts::WorktreeUnlockPrompts;

// ============================================================================
// CANONICAL TOOL NAME CONSTANT
// ============================================================================

/// Canonical tool name for git_worktree_unlock
pub const GIT_WORKTREE_UNLOCK: &str = "git_worktree_unlock";

// ============================================================================
// GIT_WORKTREE_UNLOCK TOOL
// ============================================================================

/// Arguments for `git_worktree_unlock` tool
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct GitWorktreeUnlockArgs {
    /// Path to repository
    pub path: String,

    /// Path to worktree to unlock
    pub worktree_path: String,
}

// ============================================================================
// OUTPUT TYPE
// ============================================================================

/// Output from `git_worktree_unlock` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitWorktreeUnlockOutput {
    pub success: bool,
    pub worktree_path: String,
    pub message: String,
}

#[tool_metadata(
    name = "git_worktree_unlock",
    category = "git",
    description = "Unlock a working tree to allow automatic cleanup"
)]
impl ToolArgs for GitWorktreeUnlockArgs {
    type Output = GitWorktreeUnlockOutput;
    type Prompts = WorktreeUnlockPrompts;

    const NAME: &'static str = GIT_WORKTREE_UNLOCK;
    const CATEGORY: &'static str = "git";
    const DESCRIPTION: &'static str = "Unlock a working tree to allow automatic cleanup";
}
