//! Schema types for git_worktree_lock tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::{ToolArgs, tool_metadata};
use super::prompts::WorktreeLockPrompts;

// ============================================================================
// CANONICAL TOOL NAME CONSTANT
// ============================================================================

/// Canonical tool name for git_worktree_lock
pub const GIT_WORKTREE_LOCK: &str = "git_worktree_lock";

// ============================================================================
// GIT_WORKTREE_LOCK TOOL
// ============================================================================

/// Arguments for `git_worktree_lock` tool
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct GitWorktreeLockArgs {
    /// Path to repository
    pub path: String,

    /// Path to the worktree to lock (prevents deletion)
    pub worktree_path: String,

    /// Optional reason for locking (e.g., "On removable drive").
    /// Stored in the lock file for documentation purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

// ============================================================================
// OUTPUT TYPE
// ============================================================================

/// Output from `git_worktree_lock` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitWorktreeLockOutput {
    pub success: bool,
    pub worktree_path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    pub message: String,
}

#[tool_metadata(
    name = "git_worktree_lock",
    category = "git",
    description = "Lock a working tree to prevent automatic deletion"
)]
impl ToolArgs for GitWorktreeLockArgs {
    type Output = GitWorktreeLockOutput;
    type Prompts = WorktreeLockPrompts;

    const NAME: &'static str = GIT_WORKTREE_LOCK;
    const CATEGORY: &'static str = "git";
    const DESCRIPTION: &'static str = "Lock a working tree to prevent automatic deletion";
}
