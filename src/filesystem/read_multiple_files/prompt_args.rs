//! Prompt argument types for fs_read_multiple_files tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for fs_read_multiple_files tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FsReadMultipleFilesPromptArgs {
    /// Scenario to show examples for
    /// - "basic": Reading multiple files
    /// - "related_files": Reading related code files
    /// - "config_set": Reading configuration files
    /// - "error_handling": Handling partial failures
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
