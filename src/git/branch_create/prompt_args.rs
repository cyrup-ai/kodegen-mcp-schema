//! Prompt argument types for git_branch_create tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for git_branch_create tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitBranchCreatePromptArgs {
    /// Scenario to show examples for
    /// - "feature": Creating feature branches
    /// - "from_commit": Branches from specific points
    /// - "tracking": Remote tracking branches
    /// - "naming": Branch naming conventions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
