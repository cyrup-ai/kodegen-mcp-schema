//! Prompt argument types for github_add_pull_request_review_comment tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for github_add_pull_request_review_comment tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AddPullRequestReviewCommentPromptArgs {
    /// Scenario to show examples for
    /// - "multiline": Multi-line comments spanning multiple diff lines
    /// - "reply": Replying to existing comments for threaded discussions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
