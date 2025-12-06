//! Prompt argument types for browser_screenshot tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for browser_screenshot tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BrowserScreenshotPromptArgs {
    /// Scenario to show examples for
    /// - "debugging": Using screenshots to understand page state
    /// - "element_capture": Capturing specific elements
    /// - "full_page": Full page vs viewport screenshots
    /// - "verification": Visual verification workflows
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
