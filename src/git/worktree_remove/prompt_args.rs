//! Prompt argument types for git_worktree_remove tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for `git_worktree_remove` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitWorktreeRemovePromptArgs {
    /// Optional: Use case for customized examples
    /// - "basic": Removing a worktree
    /// - "force": Forced removal with uncommitted changes
    ///
    /// Default if omitted: comprehensive overview covering all aspects
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
