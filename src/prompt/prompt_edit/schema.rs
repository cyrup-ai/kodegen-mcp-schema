//! Schema types for prompt_edit tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use kodegen_config::{CATEGORY_PROMPT, PROMPT_EDIT};

// ============================================================================
// PROMPT EDIT TOOL
// ============================================================================

/// Arguments for `prompt_edit` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct EditPromptArgs {
    /// Name of the prompt to edit
    pub name: String,

    /// New content (including frontmatter)
    pub content: String,
}

// ============================================================================
// OUTPUT TYPES
// ============================================================================

/// Output from `prompt_edit` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PromptEditOutput {
    /// Whether the operation succeeded
    pub success: bool,
    /// Name of the prompt edited
    pub name: String,
    /// Human-readable message
    pub message: String,
    /// Path to the edited file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

// ============================================================================
// TOOL ARGS TRAIT IMPLEMENTATION
// ============================================================================

use crate::{ToolArgs, tool_metadata};
use super::prompts::PromptEditPrompts;

#[tool_metadata(
    description = "Update existing prompt template content, metadata, or variables with version iteration support"
)]
impl ToolArgs for EditPromptArgs {
    type Output = PromptEditOutput;
    type Prompts = PromptEditPrompts;

    const NAME: &'static str = PROMPT_EDIT;
    const CATEGORY: &'static str = CATEGORY_PROMPT;
    const DESCRIPTION: &'static str = "Update existing prompt template content, metadata, or variables with version iteration support";
}
