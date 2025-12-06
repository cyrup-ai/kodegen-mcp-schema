//! Prompt argument types for delete_prompt tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for delete_prompt tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DeletePromptPromptArgs {
    /// Scenario to show examples for
    /// - "basic": Basic deletion
    /// - "cleanup": Cleanup workflows
    /// - "safety": Safe deletion practices
    /// - "comprehensive": All scenarios combined
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
