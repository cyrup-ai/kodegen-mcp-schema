//! Prompt argument types for fetch tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for fetch tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FetchPromptArgs {
    /// Example scenario (optional, defaults to comprehensive)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
