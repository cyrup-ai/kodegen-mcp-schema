//! Prompt argument types for fs_delete_file tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for fs_delete_file tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FsDeleteFilePromptArgs {
    /// Scenario to show examples for
    /// - "basic": Simple file deletion
    /// - "verification": Check before delete
    /// - "cleanup": Batch cleanup patterns
    /// - "safety": When NOT to delete
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
