//! Prompt argument types for git_worktree_prune tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for `git_worktree_prune` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitWorktreePrunePromptArgs {
    /// Optional: Use case for customized examples
    /// - "basic": Pruning stale worktrees
    /// - "cleanup": Removing deleted worktrees
    ///
    /// Default if omitted: comprehensive overview covering all aspects
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
