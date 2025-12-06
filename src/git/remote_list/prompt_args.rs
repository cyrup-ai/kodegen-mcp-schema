//! Prompt argument types for git_remote_list tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for `git_remote_list` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitRemoteListPromptArgs {
    /// Scenario to show examples for
    /// - "basic": Simple remote listing
    /// - "verbose": Detailed remote info
    /// - "verification": Checking remote setup
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
