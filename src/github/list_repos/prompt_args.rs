//! Prompt argument types for github_list_repos tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for github_list_repos tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GithubListReposPromptArgs {
    /// Scenario to show examples for
    /// - "user": User repositories
    /// - "organization": Org repositories
    /// - "sorting": Sort options
    /// - "workflows": Repo discovery workflows
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
