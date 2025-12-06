//! Prompt argument types for git_stash_save tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for git_stash_save tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitStashSavePromptArgs {
    /// Scenario to show examples for
    /// - "basic": Simple stash save
    /// - "messages": Descriptive stash messages
    /// - "options": Stash save options
    /// - "workflows": Common stash workflows
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
