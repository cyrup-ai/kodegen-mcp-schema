//! Prompt argument types for github_list_pull_requests tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for github_list_pull_requests tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ListPullRequestsPromptArgs {
    /// Scenario to show examples for
    /// - "basic": Simple PR listing
    /// - "filtering": Filter by state, branch
    /// - "review": Review-focused listing
    /// - "workflows": PR management workflows
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
