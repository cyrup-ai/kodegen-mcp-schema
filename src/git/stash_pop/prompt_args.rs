//! Prompt argument types for git_stash_pop tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for git_stash_pop tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitStashPopPromptArgs {
    /// Scenario to show examples for
    /// - "basic": Simple stash pop
    /// - "specific": Pop specific stash
    /// - "conflicts": Handling pop conflicts
    /// - "workflows": Common pop workflows
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
