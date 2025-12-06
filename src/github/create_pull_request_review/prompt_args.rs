//! Prompt argument types for github_create_pull_request_review tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CreatePullRequestReviewPromptArgs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focus_area: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skill_level: Option<String>,
}
