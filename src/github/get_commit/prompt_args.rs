//! Prompt argument types for github_get_commit tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GetCommitPromptArgs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explain_response: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explain_pagination: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explain_diffs: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explain_use_cases: Option<bool>,
}
