//! Schema types for git_worktree_list tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use kodegen_config::{CATEGORY_GIT, GIT_WORKTREE_LIST};
use crate::{ToolArgs, tool_metadata};
use super::super::GitWorktreeInfo;
use super::prompts::WorktreeListPrompts;

// ============================================================================
// GIT_WORKTREE_LIST TOOL
// ============================================================================

/// Arguments for `git_worktree_list` tool
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct GitWorktreeListArgs {
    /// Path to repository
    pub path: String,
}

// ============================================================================
// OUTPUT TYPE
// ============================================================================

/// Output from `git_worktree_list` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitWorktreeListOutput {
    pub success: bool,
    pub worktrees: Vec<GitWorktreeInfo>,
    pub count: usize,
}

#[tool_metadata(
    description = "List all working trees in a repository"
)]
impl ToolArgs for GitWorktreeListArgs {
    type Output = GitWorktreeListOutput;
    type Prompts = WorktreeListPrompts;

    const NAME: &'static str = GIT_WORKTREE_LIST;
    const CATEGORY: &'static str = CATEGORY_GIT;
    const DESCRIPTION: &'static str = "List all working trees in a repository";
}
