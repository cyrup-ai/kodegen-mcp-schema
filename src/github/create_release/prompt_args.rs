//! Prompt argument types for github_create_release tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for github_create_release tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GithubCreateReleasePromptArgs {
    /// Scenario to show examples for
    /// - "basic": Simple release creation
    /// - "notes": Writing release notes
    /// - "options": Draft, pre-release, assets
    /// - "workflows": Release workflows
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
