//! Prompt argument types for github_create_pull_request_review tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for github_create_pull_request_review tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CreatePullRequestReviewPromptArgs {
    /// Scenario to show examples for
    /// - "basic": Review event types (APPROVE, REQUEST_CHANGES, COMMENT)
    /// - "inline_comments": Adding comments to specific code lines
    /// - "workflows": Automated review patterns
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
