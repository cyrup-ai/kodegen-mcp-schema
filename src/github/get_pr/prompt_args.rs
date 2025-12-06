//! Prompt argument types for github_get_pr tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for github_get_pr tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GithubGetPrPromptArgs {
    /// Scenario to show examples for
    /// - "basic": Getting PR details
    /// - "status": Merge and review status
    /// - "changes": PR changes info
    /// - "workflows": PR review workflows
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
