//! Prompt argument types for git_remote_remove tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for `git_remote_remove` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitRemoteRemovePromptArgs {
    /// Optional: Use case for customized examples
    /// - "basic": Simple remote removal
    /// - "cleanup": Cleaning up old remotes
    /// - "replacement": Remove and replace workflow
    ///
    /// Default if omitted: comprehensive overview covering all aspects
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
