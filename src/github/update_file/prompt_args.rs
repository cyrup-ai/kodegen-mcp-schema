//! Prompt argument types for github_update_file tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for github_update_file tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GithubUpdateFilePromptArgs {
    /// Scenario to show examples for
    /// - "basic": Simple file update
    /// - "workflow": Read-modify-update workflow
    /// - "branches": Updates on branches
    /// - "advanced": Batch updates, error handling, committer info
    /// - "comprehensive": Complete guide to all features
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
