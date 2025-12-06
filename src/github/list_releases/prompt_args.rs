//! Prompt argument types for github_list_releases tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for github_list_releases tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ListReleasesPromptArgs {
    /// Scenario to show examples for
    /// - "basic": Simple release listing
    /// - "details": Release details
    /// - "workflows": Release management workflows
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
