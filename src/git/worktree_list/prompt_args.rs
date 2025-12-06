//! Prompt argument types for git_worktree_list tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for `git_worktree_list` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitWorktreeListPromptArgs {
    /// Optional: Use case for customized examples
    /// - "basic": Listing all worktrees
    /// - "status": Checking worktree status
    ///
    /// Default if omitted: comprehensive overview covering all aspects
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
