//! Prompt argument types for web_search tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct WebSearchPromptArgs {
    /// Scenario: basic, research
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
