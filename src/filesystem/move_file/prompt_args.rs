//! Prompt argument types for fs_move_file tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for fs_move_file tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FsMoveFilePromptArgs {
    /// Scenario to show examples for
    /// - "rename": Renaming files/directories
    /// - "relocate": Moving to different location
    /// - "organize": Organizing file structures
    /// - "backup": Creating backups via move
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
