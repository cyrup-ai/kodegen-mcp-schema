//! Prompt argument types for git_status tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for git_status tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitStatusPromptArgs {
    /// Scenario to show examples for
    /// - "basic": Simple status check
    /// - "interpreting": Understanding status output
    /// - "workflows": Status in workflows
    /// - "options": Status options
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
