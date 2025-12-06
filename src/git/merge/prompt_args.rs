//! Prompt argument types for git_merge tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for git_merge tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitMergePromptArgs {
    /// Scenario to show examples for
    /// - "basic": Simple branch merging
    /// - "strategies": Merge strategies and options
    /// - "conflicts": Handling merge conflicts
    /// - "workflows": Complete merge workflows
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
