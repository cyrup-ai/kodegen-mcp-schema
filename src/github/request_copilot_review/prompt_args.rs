//! Prompt argument types for github_request_copilot_review tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RequestCopilotReviewPromptArgs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focus_area: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth: Option<String>,
}
