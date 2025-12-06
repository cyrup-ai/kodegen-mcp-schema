//! Prompt argument types for git_branch_list tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for git_branch_list tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitBranchListPromptArgs {
    /// Scenario to show examples for
    /// - "basic": Simple branch listing
    /// - "remote": Remote branches
    /// - "filtering": Filtered branch lists
    /// - "analysis": Branch analysis workflows
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
