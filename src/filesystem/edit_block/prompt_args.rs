//! Prompt argument types for fs_edit_block tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for fs_edit_block tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FsEditBlockPromptArgs {
    /// Scenario to show examples for
    /// - "basic": Basic string replacement
    /// - "precision": Precise editing patterns
    /// - "multiline": Multi-line edits
    /// - "safety": Safe editing practices
    /// - "workflows": Common editing workflows
    /// - "comprehensive": All scenarios combined
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
