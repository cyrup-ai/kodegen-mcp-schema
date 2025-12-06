//! Prompt argument types for git_stash_apply tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for git_stash_apply tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitStashApplyPromptArgs {
    /// Scenario to show examples for
    /// - "basic": Simple stash apply
    /// - "specific": Apply specific stash
    /// - "conflicts": Handling apply conflicts
    /// - "workflows": Common apply workflows
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
