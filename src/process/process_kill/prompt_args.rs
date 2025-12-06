//! Prompt argument types for process_kill tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for process_kill tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ProcessKillPromptArgs {
    /// Scenario to show examples for
    /// - "basic": Basic process termination
    /// - "safety": Safe killing practices
    /// - "workflows": Process management workflows
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
