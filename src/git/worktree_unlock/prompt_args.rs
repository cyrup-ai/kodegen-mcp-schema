//! Prompt argument types for git_worktree_unlock tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for `git_worktree_unlock` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitWorktreeUnlockPromptArgs {
    /// Optional: Use case for customized examples
    /// - "basic": Unlocking a worktree
    /// - "cleanup": Enabling automatic cleanup
    ///
    /// Default if omitted: comprehensive overview covering all aspects
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
