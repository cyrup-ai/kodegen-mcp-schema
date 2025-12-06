//! Prompt argument types for browser_click tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for browser_click tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BrowserClickPromptArgs {
    /// Scenario to show examples for
    /// - "selectors": CSS selector patterns for different elements
    /// - "coordinates": Using x,y coordinates
    /// - "waiting": Handling dynamic content and timing
    /// - "troubleshooting": Debugging click failures
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
