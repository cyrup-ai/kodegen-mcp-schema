//! Schema types for git_worktree_prune tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use kodegen_config::{CATEGORY_GIT, GIT_WORKTREE_PRUNE};
use crate::{ToolArgs, tool_metadata};
use super::prompts::WorktreePrunePrompts;

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
    description = "Remove stale administrative files for deleted working trees"
)]
impl ToolArgs for GitWorktreePruneArgs {
    type Output = GitWorktreePruneOutput;
    type Prompts = WorktreePrunePrompts;

    const NAME: &'static str = GIT_WORKTREE_PRUNE;
    const CATEGORY: &'static kodegen_config::Category = CATEGORY_GIT;
    const DESCRIPTION: &'static str = "Remove stale administrative files for deleted working trees";
}
