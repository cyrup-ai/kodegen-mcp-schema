//! Prompt arguments for terminal tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Arguments for customizing terminal tool prompt examples
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TerminalPromptArgs {
    /// Scenario to show examples for
    /// - "basic": Simple command execution
    /// - "parallel": Multiple terminal usage
    /// - "background": Fire-and-forget patterns
    /// - "monitoring": Progress checking with READ
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
