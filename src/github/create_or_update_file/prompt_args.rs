//! Prompt argument types for github_create_or_update_file tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CreateOrUpdateFilePromptArgs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
