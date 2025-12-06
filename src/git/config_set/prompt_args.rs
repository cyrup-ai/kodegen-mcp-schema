//! Prompt argument types for git_config_set tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for `git_config_set` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitConfigSetPromptArgs {
    /// Optional: Scenario to show examples for
    /// - "identity": Setting user identity
    /// - "behavior": Configuring git behavior
    /// - "aliases": Creating command aliases
    /// - "repo_specific": Per-repository settings
    ///
    /// Default if omitted: comprehensive overview covering all scenarios
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
