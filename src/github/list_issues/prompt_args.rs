//! Prompt argument types for github_list_issues tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for github_list_issues tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GithubListIssuesPromptArgs {
    /// Scenario to show examples for
    /// - "basic": Simple issue listing
    /// - "filtering": Filter by state, labels
    /// - "sorting": Sort options
    /// - "workflows": Issue management workflows
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
