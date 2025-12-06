//! Prompt argument types for github_create_pull_request tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for github_create_pull_request tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CreatePullRequestPromptArgs {
    /// Scenario to show examples for
    /// - "basic": Simple PR creation
    /// - "description": Writing good PR descriptions
    /// - "options": Draft PRs and options
    /// - "workflows": Complete PR workflows
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
