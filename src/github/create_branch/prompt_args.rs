//! Prompt argument types for github_create_branch tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for github_create_branch tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CreateBranchPromptArgs {
    /// Scenario to show examples for
    /// - "basic": Simple branch creation
    /// - "from_ref": Branch from specific commit/tag
    /// - "workflows": Branch creation workflows
    /// - "integration": Integration with git operations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
