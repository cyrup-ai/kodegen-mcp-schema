//! Prompt argument types for github_get_me tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for github_get_me tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GetMePromptArgs {
    /// Scenario to show examples for
    /// - "basic": Getting user info
    /// - "verification": Verifying auth setup
    /// - "workflows": Identity workflows
    /// - "integration": Integration patterns
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
