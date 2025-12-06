//! Prompt argument types for browser_web_search tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for browser_web_search tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BrowserWebSearchPromptArgs {
    /// Scenario to show examples for
    /// - "basic": Simple search queries
    /// - "advanced": Search operators and filters
    /// - "programming": Searching for code and docs
    /// - "comparison": When to use this vs other search tools
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
