//! Prompt argument types for git_push tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for git_push tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitPushPromptArgs {
    /// Scenario to show examples for
    /// - "basic": Simple pushing
    /// - "upstream": Setting up tracking
    /// - "tags": Pushing tags
    /// - "force": Force push scenarios
    /// - "workflows": Complete push workflows
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
