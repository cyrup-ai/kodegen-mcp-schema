//! Prompt argument types for github_get_issue tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for github_get_issue tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GetIssuePromptArgs {
    /// Scenario to show examples for
    /// - "basic": Getting issue details
    /// - "metadata": Labels, assignees, milestone
    /// - "comments": Getting issue comments
    /// - "workflows": Issue investigation workflows
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
