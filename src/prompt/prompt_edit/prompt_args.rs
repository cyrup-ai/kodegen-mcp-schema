//! Prompt argument types for edit_prompt tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for edit_prompt tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct EditPromptPromptArgs {
    /// Scenario to show examples for
    /// - "basic": Basic prompt editing (default)
    /// - "refinement": Iterative prompt refinement
    /// - "versioning": Managing prompt versions
    /// - "workflows": Edit workflows and patterns
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
