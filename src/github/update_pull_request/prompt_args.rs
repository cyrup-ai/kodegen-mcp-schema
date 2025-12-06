//! Prompt argument types for github_update_pull_request tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for github_update_pull_request tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct UpdatePullRequestPromptArgs {
    /// Scenario to show examples for
    /// - "state": Opening/closing PRs
    /// - "content": Title and body updates
    /// - "draft": Draft status changes
    /// - "workflows": PR management workflows
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
