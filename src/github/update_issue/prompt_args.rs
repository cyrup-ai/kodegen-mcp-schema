//! Prompt argument types for github_update_issue tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for github_update_issue tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct UpdateIssuePromptArgs {
    /// Scenario to show examples for
    /// - "state": Opening/closing issues
    /// - "metadata": Labels, assignees, milestone
    /// - "content": Title and body updates
    /// - "workflows": Issue management workflows
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
