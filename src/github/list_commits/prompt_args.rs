//! Prompt argument types for github_list_commits tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for github_list_commits tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ListCommitsPromptArgs {
    /// Scenario to show examples for
    /// - "basic": Simple commit listing
    /// - "filtering": Filter by author, path, date
    /// - "branches": Commits on branches
    /// - "workflows": Commit analysis workflows
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
