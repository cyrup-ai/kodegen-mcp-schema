//! Prompt argument types for github_list_branches tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for github_list_branches tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ListBranchesPromptArgs {
    /// Scenario to show examples for
    /// - "basic": Simple branch listing
    /// - "protection": Protected branches
    /// - "workflows": Branch management workflows
    /// - "pagination": Handling large branch lists
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
