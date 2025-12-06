//! Prompt argument types for fs_delete_directory tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for fs_delete_directory tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FsDeleteDirectoryPromptArgs {
    /// Scenario to show examples for
    /// - "basic": Simple deletion with safety
    /// - "safety": Understanding recursive confirmation
    /// - "verification": Verify before delete workflow
    /// - "alternatives": When NOT to delete
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
