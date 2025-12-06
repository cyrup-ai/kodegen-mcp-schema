//! Prompt argument types for get_prompt tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for get_prompt tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GetPromptPromptArgs {
    /// Scenario to show examples for
    /// - "basic": Retrieving prompts
    /// - "variables": Variable substitution
    /// - "rendering": Template rendering
    /// - "workflows": Common usage patterns
    /// - "comprehensive": All scenarios combined
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
