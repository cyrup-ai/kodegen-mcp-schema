//! Prompt argument types for git_pull tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for git_pull tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitPullPromptArgs {
    /// Scenario to show examples for
    /// - "basic": Simple pulling
    /// - "rebase": Pull with rebase
    /// - "remotes": Pulling from different remotes
    /// - "conflicts": Handling pull conflicts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
