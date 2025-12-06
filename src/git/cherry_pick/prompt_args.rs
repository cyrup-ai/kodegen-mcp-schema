//! Prompt argument types for git_cherry_pick tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for git_cherry_pick tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitCherryPickPromptArgs {
    /// Scenario to show examples for
    /// - "single": Cherry-pick one commit
    /// - "multiple": Cherry-pick several commits
    /// - "conflicts": Handling conflicts
    /// - "options": Advanced cherry-pick options
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
