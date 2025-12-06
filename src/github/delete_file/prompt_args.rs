//! Prompt argument types for github_delete_file tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for github_delete_file tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GithubDeleteFilePromptArgs {
    /// Scenario to show examples for
    /// - "basic": Simple file deletion
    /// - "safety": Safe deletion practices
    /// - "workflows": Deletion workflows
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
