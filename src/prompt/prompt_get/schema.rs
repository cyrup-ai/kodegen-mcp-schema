//! Schema types for prompt_get tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use kodegen_config::{CATEGORY_PROMPT, PROMPT_GET};
use std::collections::HashMap;

// ============================================================================
// SHARED TYPES (used by multiple prompt tools)
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
// PROMPT GET TOOL
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

// ============================================================================
// OUTPUT TYPES
// ============================================================================

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

// ============================================================================
// TOOL ARGS TRAIT IMPLEMENTATION
// ============================================================================

use crate::{ToolArgs, tool_metadata};
use super::prompts::PromptGetPrompts;

#[tool_metadata(
    description = "Retrieve prompt template by name, render with variables, or list available templates by category"
)]
impl ToolArgs for GetPromptArgs {
    type Output = PromptGetOutput;
    type Prompts = PromptGetPrompts;

    const NAME: &'static str = PROMPT_GET;
    const CATEGORY: &'static str = CATEGORY_PROMPT;
    const DESCRIPTION: &'static str = "Retrieve prompt template by name, render with variables, or list available templates by category";
}
