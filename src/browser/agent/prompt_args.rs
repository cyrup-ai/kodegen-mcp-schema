//! Prompt argument types for browser_agent tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for browser_agent tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BrowserAgentPromptArgs {
    /// Scenario to show examples for
    /// - "basic": Basic browser automation
    /// - "navigation": Web navigation patterns
    /// - "extraction": Data extraction tasks
    /// - "management": Agent management
    /// - "comprehensive": All scenarios combined
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
