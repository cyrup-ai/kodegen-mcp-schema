//! Prompt argument types for git_stash_list tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for git_stash_list tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitStashListPromptArgs {
    /// Scenario to show examples for
    /// - "basic": Simple stash listing
    /// - "details": Detailed stash info
    /// - "management": Managing stash entries
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
