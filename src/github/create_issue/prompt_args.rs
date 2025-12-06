//! Prompt argument types for github_create_issue tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for github_create_issue tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CreateIssuePromptArgs {
    /// Scenario to show examples for
    /// - "bug_report": Creating bug reports
    /// - "feature_request": Feature request issues
    /// - "metadata": Labels, assignees, milestones
    /// - "templates": Using issue templates
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
