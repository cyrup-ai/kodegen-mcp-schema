//! Prompt tool argument schemas

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// TYPED PARAMETER VALUES (replaces serde_json::Value for parameters)
// ============================================================================

/// Typed template parameter value
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum TemplateParamValue {
    /// String parameter value
    String(String),
    /// Numeric parameter value (integer or float)
    Number(f64),
    /// Boolean parameter value
    Bool(bool),
    /// Array of strings
    StringArray(Vec<String>),
}

// ============================================================================
// TYPED OUTPUT RESULT STRUCTURES
// ============================================================================

/// Category information for list_categories action
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CategoryInfo {
    /// Category name
    pub name: String,
    /// Number of prompts in this category
    pub count: usize,
}

/// Result from list_categories action
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PromptCategoriesResult {
    /// List of categories with counts
    pub categories: Vec<CategoryInfo>,
    /// Total number of categories
    pub total: usize,
    /// Elapsed time in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elapsed_ms: Option<f64>,
}

/// Parameter definition for prompt metadata
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PromptParameterDef {
    /// Parameter name
    pub name: String,
    /// Parameter description
    pub description: String,
    /// Parameter type (string, number, boolean, array)
    #[serde(default)]
    pub param_type: PromptParameterType,
    /// Whether this parameter is required
    #[serde(default)]
    pub required: bool,
    /// Default value if not provided
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<TemplateParamValue>,
}

/// Parameter type enum
#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum PromptParameterType {
    #[default]
    String,
    Number,
    Boolean,
    Array,
}

/// Prompt summary for list_prompts action
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PromptSummary {
    /// Prompt filename (without extension)
    pub name: String,
    /// Human-readable title
    pub title: String,
    /// Description of what the prompt does
    pub description: String,
    /// Categories this prompt belongs to
    pub categories: Vec<String>,
    /// Author of the prompt
    pub author: String,
    /// Whether this prompt is verified
    pub verified: bool,
    /// Parameter definitions
    pub parameters: Vec<PromptParameterDef>,
}

/// Result from list_prompts action
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PromptListResult {
    /// List of prompt summaries
    pub prompts: Vec<PromptSummary>,
    /// Number of prompts returned
    pub count: usize,
    /// Category filter applied (if any)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// Elapsed time in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elapsed_ms: Option<f64>,
}

/// Full prompt metadata
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PromptMetadataOutput {
    /// Human-readable title
    pub title: String,
    /// Description of what the prompt does
    pub description: String,
    /// Categories this prompt belongs to
    pub categories: Vec<String>,
    /// Secondary tag (if any)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_tag: Option<String>,
    /// Author of the prompt
    pub author: String,
    /// Whether this prompt is verified
    pub verified: bool,
    /// Vote count
    pub votes: u32,
    /// Parameter definitions
    pub parameters: Vec<PromptParameterDef>,
}

/// Result from get action (raw template)
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PromptContentResult {
    /// Prompt name
    pub name: String,
    /// Prompt metadata
    pub metadata: PromptMetadataOutput,
    /// Raw template content (Jinja2)
    pub content: String,
    /// Always false for get action
    pub rendered: bool,
    /// Elapsed time in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elapsed_ms: Option<f64>,
}

/// Result from render action
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PromptRenderedResult {
    /// Prompt name
    pub name: String,
    /// Rendered content (parameters applied)
    pub content: String,
    /// Always true for render action
    pub rendered: bool,
    /// Elapsed time in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elapsed_ms: Option<f64>,
}

/// Typed result enum for prompt_get output
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "action", rename_all = "snake_case")]
pub enum PromptResult {
    /// Result from list_categories action
    ListCategories(PromptCategoriesResult),
    /// Result from list_prompts action
    ListPrompts(PromptListResult),
    /// Result from get action
    Get(PromptContentResult),
    /// Result from render action
    Render(PromptRenderedResult),
}

// ============================================================================
// TOOL NAME CONSTANTS - Canonical Source of Truth
// ============================================================================

/// Canonical tool name for prompt_add
pub const PROMPT_ADD: &str = "prompt_add";

/// Canonical tool name for prompt_delete
pub const PROMPT_DELETE: &str = "prompt_delete";

/// Canonical tool name for prompt_edit
pub const PROMPT_EDIT: &str = "prompt_edit";

/// Canonical tool name for prompt_get
pub const PROMPT_GET: &str = "prompt_get";

// ============================================================================
// GET PROMPT
// ============================================================================

/// Action enum for get_prompt tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum GetPromptAction {
    /// List all categories with counts
    ListCategories,
    /// List all prompts (optionally filtered by category)
    ListPrompts,
    /// Get prompt metadata and raw template
    Get,
    /// Render prompt with parameters
    Render,
}

/// Arguments for `prompt_get` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GetPromptArgs {
    /// Action to perform
    pub action: GetPromptAction,

    /// Prompt name (for get/render actions)
    #[serde(default)]
    pub name: Option<String>,

    /// Category filter (for `list_prompts` action)
    #[serde(default)]
    pub category: Option<String>,

    /// Parameters for rendering (for render action)
    #[serde(default)]
    pub parameters: Option<HashMap<String, TemplateParamValue>>,
}

/// Prompt arguments for `get_prompt` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GetPromptPromptArgs {}

// ============================================================================
// ADD PROMPT
// ============================================================================

/// Arguments for `add_prompt` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AddPromptArgs {
    /// Filename for the prompt (without .j2.md extension)
    /// Must contain only alphanumeric characters, hyphens, and underscores
    pub name: String,

    /// Full prompt content including YAML frontmatter
    pub content: String,
}

/// Prompt arguments for `add_prompt` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AddPromptPromptArgs {
    /// Type of template to focus examples on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_scope: Option<String>,
    
    /// How detailed the teaching should be
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail_level: Option<String>,
}

// ============================================================================
// EDIT PROMPT
// ============================================================================

/// Arguments for `edit_prompt` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct EditPromptArgs {
    /// Name of the prompt to edit
    pub name: String,

    /// New content (including frontmatter)
    pub content: String,
}

/// Prompt arguments for `edit_prompt` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct EditPromptPromptArgs {}

// ============================================================================
// DELETE PROMPT
// ============================================================================

/// Arguments for `delete_prompt` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DeletePromptArgs {
    /// Name of the prompt to delete
    pub name: String,

    /// Confirmation flag (must be true)
    #[serde(default)]
    pub confirm: bool,
}

/// Prompt arguments for `delete_prompt` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DeletePromptPromptArgs {}

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

/// Output from `prompt_get` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PromptGetOutput {
    /// Whether the operation succeeded
    pub success: bool,
    /// Action that was performed
    pub action: GetPromptAction,
    /// Typed result based on action performed
    pub result: PromptResult,
}

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
// TOOL ARGS TRAIT IMPLEMENTATIONS
// ============================================================================

impl crate::ToolArgs for AddPromptArgs {
    type Output = PromptAddOutput;
}

impl crate::ToolArgs for GetPromptArgs {
    type Output = PromptGetOutput;
}

impl crate::ToolArgs for DeletePromptArgs {
    type Output = PromptDeleteOutput;
}

impl crate::ToolArgs for EditPromptArgs {
    type Output = PromptEditOutput;
}
