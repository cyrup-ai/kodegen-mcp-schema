//! Prompt argument types for git_revert tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for git_revert tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitRevertPromptArgs {
    /// Scenario to show examples for
    /// - "single": Revert single commit
    /// - "multiple": Revert multiple commits
    /// - "merge": Revert merge commits
    /// - "conflicts": Handling revert conflicts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
