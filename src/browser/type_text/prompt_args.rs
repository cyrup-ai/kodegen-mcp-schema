//! Prompt argument types for browser_type_text tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for browser_type_text tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BrowserTypeTextPromptArgs {
    /// Scenario to show examples for
    /// - "basic": Simple text input
    /// - "forms": Complete form filling workflows
    /// - "special_keys": Enter, Tab, special characters
    /// - "clearing": Clearing existing content before typing
    /// - "sensitive": Handling passwords and sensitive data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
