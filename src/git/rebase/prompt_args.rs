//! Prompt argument types for git_rebase tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for git_rebase tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitRebasePromptArgs {
    /// Scenario to show examples for
    /// - "basic": Simple rebasing
    /// - "onto": Rebasing onto specific commits
    /// - "conflicts": Handling rebase conflicts
    /// - "workflows": Complete rebase workflows
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
