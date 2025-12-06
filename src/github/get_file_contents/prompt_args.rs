//! Prompt argument types for github_get_file_contents tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for github_get_file_contents tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GetFileContentsPromptArgs {
    /// Scenario to show examples for
    /// - "files": Reading file contents
    /// - "directories": Listing directories
    /// - "branches": Reading from branches
    /// - "workflows": Content workflows
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
