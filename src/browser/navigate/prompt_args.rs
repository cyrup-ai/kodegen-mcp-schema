//! Prompt argument types for browser_navigate tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for browser_navigate tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BrowserNavigatePromptArgs {
    /// Scenario to show examples for
    /// - "basic": Simple URL navigation
    /// - "query_params": Building URLs with parameters
    /// - "authentication": Handling login redirects
    /// - "spa_pages": Single-page app navigation
    /// - "wait_strategies": Different page load scenarios
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
