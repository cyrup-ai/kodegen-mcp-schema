//! Prompt argument types for fs_list_directory tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for fs_list_directory tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FsListDirectoryPromptArgs {
    /// Scenario to show examples for
    /// - "basic": Simple directory listing
    /// - "hidden": Including hidden files
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
