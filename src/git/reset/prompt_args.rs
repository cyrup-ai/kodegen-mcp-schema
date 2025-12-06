//! Prompt argument types for git_reset tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for git_reset tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitResetPromptArgs {
    /// Scenario to show examples for
    /// - "unstage": Unstaging files
    /// - "soft": Soft reset (keep changes staged)
    /// - "mixed": Mixed reset (keep changes unstaged)
    /// - "hard": Hard reset (discard changes)
    /// - "safety": Safe reset practices
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
