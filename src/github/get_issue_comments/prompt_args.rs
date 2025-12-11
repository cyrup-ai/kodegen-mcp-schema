//! Prompt argument types for github_get_issue_comments tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for github_get_issue_comments tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GetIssueCommentsPromptArgs {
    /// Scenario to show examples for
    /// - "basic": Simple comment retrieval
    /// - "pagination": Handling pagination and large comment threads
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
