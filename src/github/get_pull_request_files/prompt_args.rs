//! Prompt argument types for github_get_pull_request_files tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for github_get_pull_request_files tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GetPullRequestFilesPromptArgs {
    /// Scenario to show examples for
    /// - "basic": Getting changed files in a PR
    /// - "analysis": Analyzing PR changes for complexity and impact
    /// - "workflows": Complete code review workflows
    /// - "collaboration": Team review and commenting patterns
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
