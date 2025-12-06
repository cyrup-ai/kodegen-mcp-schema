//! Prompt argument types for git_remote_add tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for git_remote_add tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitRemoteAddPromptArgs {
    /// Scenario to show examples for
    /// - "basic": Adding remotes
    /// - "fork": Fork workflow setup
    /// - "multiple": Multiple remotes
    /// - "configure": Remote configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
