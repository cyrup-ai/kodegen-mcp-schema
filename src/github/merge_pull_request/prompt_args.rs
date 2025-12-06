//! Prompt argument types for github_merge_pr tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for github_merge_pr tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct MergePullRequestPromptArgs {
    /// Scenario to show examples for
    /// - "basic": Simple PR merging
    /// - "strategies": Merge strategies
    /// - "messages": Commit messages
    /// - "workflows": Complete merge workflows
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
