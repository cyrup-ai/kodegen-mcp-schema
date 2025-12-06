//! Prompt argument types for github_delete_branch tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DeleteBranchPromptArgs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_permissions: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_recovery: Option<bool>,
}
