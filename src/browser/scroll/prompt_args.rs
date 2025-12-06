//! Prompt argument types for browser_scroll tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for browser_scroll tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BrowserScrollPromptArgs {
    /// Scenario to show examples for
    /// - "basic": Simple page scrolling with x/y deltas
    /// - "element_into_view": Scrolling to reveal elements with selectors
    /// - "infinite_scroll": Loading dynamic content patterns
    /// - "element_containers": Scrolling within scrollable divs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
