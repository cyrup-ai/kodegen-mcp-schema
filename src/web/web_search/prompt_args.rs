//! Prompt argument types for web_search tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for web_search tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct WebSearchPromptArgs {
    /// Scenario to show examples for
    /// - "basic": Basic web searches
    /// - "queries": Effective query patterns
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
