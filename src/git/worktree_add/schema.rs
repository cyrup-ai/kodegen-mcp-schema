//! Schema types for git_worktree_add tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::{ToolArgs, tool_metadata};
use super::prompts::WorktreeAddPrompts;

// ============================================================================
// CANONICAL TOOL NAME CONSTANT
// ============================================================================

/// Canonical tool name for git_worktree_add
pub const GIT_WORKTREE_ADD: &str = "git_worktree_add";

// ============================================================================
// GIT_WORKTREE_ADD TOOL
// ============================================================================

/// Arguments for `git_worktree_add` tool
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct GitWorktreeAddArgs {
    /// Path to repository
    pub path: String,

    /// Path where the new worktree will be created
    pub worktree_path: String,

    /// Branch or commit to checkout in the worktree (optional, defaults to HEAD).
    /// Can be a branch name, tag, or commit SHA.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,

    /// Force creation even if worktree path already exists (default: false)
    #[serde(default)]
    pub force: bool,
}

// ============================================================================
// OUTPUT TYPE
// ============================================================================

/// Output from `git_worktree_add` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitWorktreeAddOutput {
    pub success: bool,
    pub worktree_path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    pub message: String,
}

#[tool_metadata(
    name = "git_worktree_add",
    category = "git",
    description = "Create a new linked working tree for parallel development"
)]
impl ToolArgs for GitWorktreeAddArgs {
    type Output = GitWorktreeAddOutput;
    type Prompts = WorktreeAddPrompts;

    const NAME: &'static str = GIT_WORKTREE_ADD;
    const CATEGORY: &'static str = "git";
    const DESCRIPTION: &'static str = "Create a new linked working tree for parallel development";
}
