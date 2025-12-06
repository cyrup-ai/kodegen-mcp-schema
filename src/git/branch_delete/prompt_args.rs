//! Prompt argument types for git_branch_delete tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for git_branch_delete tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitBranchDeletePromptArgs {
    /// Scenario to show examples for
    /// - "local": Deleting local branches
    /// - "remote": Deleting remote branches
    /// - "cleanup": Branch cleanup strategies
    /// - "safety": Safe deletion practices
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
