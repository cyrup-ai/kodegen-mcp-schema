//! Prompt argument types for git_config_get tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for git_config_get tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitConfigGetPromptArgs {
    /// Scenario to show examples for
    /// - "user": User identity settings
    /// - "repo": Repository-specific settings
    /// - "scopes": Global vs local config
    /// - "list": Listing all config
    ///
    /// Default if omitted: comprehensive overview covering all aspects
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
