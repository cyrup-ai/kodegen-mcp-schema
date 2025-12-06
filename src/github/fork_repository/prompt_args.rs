//! Prompt argument types for github_fork_repo tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for github_fork_repo tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitHubForkRepositoryPromptArgs {
    /// Scenario to show examples for
    /// - "basic": Simple forking
    /// - "organization": Fork to org
    /// - "workflows": Fork contribution workflows
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
