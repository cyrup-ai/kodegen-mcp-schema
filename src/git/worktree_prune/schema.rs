//! Schema types for git_worktree_prune tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::{ToolArgs, tool_metadata};
use super::prompts::WorktreePrunePrompts;

// ============================================================================
// CANONICAL TOOL NAME CONSTANT
// ============================================================================

/// Canonical tool name for git_worktree_prune
pub const GIT_WORKTREE_PRUNE: &str = "git_worktree_prune";

// ============================================================================
// GIT_WORKTREE_PRUNE TOOL
// ============================================================================

/// Arguments for `git_worktree_prune` tool
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct GitWorktreePruneArgs {
    /// Path to repository
    pub path: String,
}

// ============================================================================
// OUTPUT TYPE
// ============================================================================

/// Output from `git_worktree_prune` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitWorktreePruneOutput {
    pub success: bool,
    pub pruned_count: usize,
    pub message: String,
}

#[tool_metadata(
    name = "git_worktree_prune",
    category = "git",
    description = "Remove stale administrative files for deleted working trees"
)]
impl ToolArgs for GitWorktreePruneArgs {
    type Output = GitWorktreePruneOutput;
    type Prompts = WorktreePrunePrompts;

    const NAME: &'static str = GIT_WORKTREE_PRUNE;
    const CATEGORY: &'static str = "git";
    const DESCRIPTION: &'static str = "Remove stale administrative files for deleted working trees";
}
