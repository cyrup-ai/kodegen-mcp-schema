//! Prompt argument types for git_commit tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for git_commit tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitCommitPromptArgs {
    /// Scenario to show examples for
    /// - "basic": Simple commits with messages
    /// - "messages": Writing good commit messages
    /// - "amend": Modifying previous commits
    /// - "workflows": Complete commit workflows
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
