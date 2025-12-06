//! Prompt argument types for github_push_file tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for github_push_file tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GithubPushFilePromptArgs {
    /// Scenario to show examples for
    /// - "basics": Basic file creation and updates
    /// - "create": Creating new files
    /// - "branches": Pushing to branches
    /// - "workflows": File creation workflows
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
