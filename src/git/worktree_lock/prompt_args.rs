//! Prompt argument types for git_worktree_lock tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for `git_worktree_lock` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitWorktreeLockPromptArgs {
    /// Optional: Use case for customized examples
    /// - "basic": Locking a worktree
    /// - "prevent": Preventing automatic cleanup
    ///
    /// Default if omitted: comprehensive overview covering all aspects
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
