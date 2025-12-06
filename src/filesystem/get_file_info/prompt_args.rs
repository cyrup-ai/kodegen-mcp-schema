//! Prompt argument types for fs_get_file_info tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for fs_get_file_info tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FsGetFileInfoPromptArgs {
    /// Scenario to show examples for
    /// - "basic": Getting file metadata
    /// - "size_check": Checking file sizes
    /// - "timestamps": Working with modification times
    /// - "verification": Confirming file exists/properties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
