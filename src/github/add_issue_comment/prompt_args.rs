//! Prompt argument types for github_add_comment tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for github_add_comment tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AddIssueCommentPromptArgs {
    /// Scenario to show examples for
    /// - "issues": Commenting on issues
    /// - "prs": Commenting on PRs
    /// - "formatting": Markdown and mentions
    /// - "workflows": Comment workflows
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
