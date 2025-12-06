//! Schema types for prompt_add tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// ============================================================================
// CANONICAL TOOL NAME CONSTANTS
// ============================================================================

/// Canonical tool name for prompt_add
pub const PROMPT_ADD: &str = "prompt_add";

// ============================================================================
// PROMPT ADD TOOL
// ============================================================================

/// Arguments for `prompt_add` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AddPromptArgs {
    /// Filename for the prompt (without .j2.md extension)
    /// Must contain only alphanumeric characters, hyphens, and underscores
    pub name: String,

    /// Full prompt content including YAML frontmatter
    pub content: String,
}

// ============================================================================
// OUTPUT TYPES
// ============================================================================

/// Output from `prompt_add` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PromptAddOutput {
    /// Whether the operation succeeded
    pub success: bool,
    /// Name of the prompt created
    pub name: String,
    /// Human-readable message
    pub message: String,
    /// Path to the created file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Template content length in characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_length: Option<usize>,
    /// Number of parameters defined
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_count: Option<usize>,
}

// ============================================================================
// TOOL ARGS TRAIT IMPLEMENTATION
// ============================================================================

use crate::{ToolArgs, tool_metadata};
use super::prompts::PromptAddPrompts;

#[tool_metadata(
    name = "prompt_add",
    category = "prompt",
    description = "Create new reusable Jinja2 prompt template with variables, metadata, and categorization"
)]
impl ToolArgs for AddPromptArgs {
    type Output = PromptAddOutput;
    type Prompts = PromptAddPrompts;

    const NAME: &'static str = PROMPT_ADD;
    const CATEGORY: &'static str = "prompt";
    const DESCRIPTION: &'static str = "Create new reusable Jinja2 prompt template with variables, metadata, and categorization";
}
