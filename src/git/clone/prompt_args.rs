//! Prompt argument types for git_clone tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for git_clone tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitClonePromptArgs {
    /// Scenario to show examples for
    /// - "basic": Simple repository cloning
    /// - "shallow": Shallow clones for speed
    /// - "branch": Cloning specific branches
    /// - "options": Advanced clone options
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
