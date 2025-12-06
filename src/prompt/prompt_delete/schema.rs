//! Schema types for prompt_delete tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// ============================================================================
// CANONICAL TOOL NAME CONSTANTS
// ============================================================================

/// Canonical tool name for prompt_delete
pub const PROMPT_DELETE: &str = "prompt_delete";

// ============================================================================
// PROMPT DELETE TOOL
// ============================================================================

/// Arguments for `prompt_delete` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DeletePromptArgs {
    /// Name of the prompt to delete
    pub name: String,

    /// Confirmation flag (must be true)
    #[serde(default)]
    pub confirm: bool,
}

// ============================================================================
// OUTPUT TYPES
// ============================================================================

/// Output from `prompt_delete` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PromptDeleteOutput {
    /// Whether the operation succeeded
    pub success: bool,
    /// Name of the prompt deleted
    pub name: String,
    /// Human-readable message
    pub message: String,
}

// ============================================================================
// TOOL ARGS TRAIT IMPLEMENTATION
// ============================================================================

use crate::{ToolArgs, tool_metadata};
use super::prompts::PromptDeletePrompts;

#[tool_metadata(
    name = "prompt_delete",
    category = "prompt",
    description = "Delete prompt template with required confirmation to prevent accidental removal"
)]
impl ToolArgs for DeletePromptArgs {
    type Output = PromptDeleteOutput;
    type Prompts = PromptDeletePrompts;

    const NAME: &'static str = PROMPT_DELETE;
    const CATEGORY: &'static str = "prompt";
    const DESCRIPTION: &'static str = "Delete prompt template with required confirmation to prevent accidental removal";
}
