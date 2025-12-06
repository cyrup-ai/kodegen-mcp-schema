//! Prompt argument types for github_repo_search tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for github_repo_search tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SearchRepositoriesPromptArgs {
    /// Scenario to show examples for
    /// - "basic": Simple repo search
    /// - "syntax": Search syntax reference
    /// - "patterns": Common search patterns
    /// - "workflows": Discovery workflows
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
