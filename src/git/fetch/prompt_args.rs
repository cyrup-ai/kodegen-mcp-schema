//! Prompt argument types for git_fetch tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for git_fetch tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitFetchPromptArgs {
    /// Scenario to show examples for
    /// - "basic": Simple fetching
    /// - "remotes": Fetching from specific remotes
    /// - "prune": Cleaning stale references
    /// - "workflows": Fetch workflows
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
