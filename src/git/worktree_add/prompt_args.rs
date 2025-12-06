//! Prompt argument types for git_worktree_add tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for `git_worktree_add` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitWorktreeAddPromptArgs {
    /// Optional: Use case for customized examples
    /// - "basic": Creating a linked worktree
    /// - "parallel": Working on multiple features simultaneously
    /// - "branch": Creating new branch in worktree
    ///
    /// Default if omitted: comprehensive overview covering all aspects
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
