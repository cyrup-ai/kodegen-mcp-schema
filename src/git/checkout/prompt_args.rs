//! Prompt argument types for git_checkout tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for git_checkout tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitCheckoutPromptArgs {
    /// Scenario to show examples for
    /// - "switch_branch": Switching between branches
    /// - "create_branch": Create and checkout new branch
    /// - "restore_files": Restore specific files
    /// - "detached": Detached HEAD operations
    /// - "workflows": Complete checkout workflows
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
