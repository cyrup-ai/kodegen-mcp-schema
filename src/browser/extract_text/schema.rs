//! Schema types for browser_extract_text tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::{ToolArgs, tool_metadata};
use super::prompts::ExtractTextPrompts;

// ============================================================================
// CANONICAL TOOL NAME CONSTANTS
// ============================================================================

pub const BROWSER_EXTRACT_TEXT: &str = "browser_extract_text";

// ============================================================================
// INPUT ARGS
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BrowserExtractTextArgs {
    /// Optional: CSS selector for specific element (default: entire page)
    #[serde(default)]
    pub selector: Option<String>,
}

// ============================================================================
// OUTPUT
// ============================================================================

/// Output from `browser_extract_text` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BrowserExtractTextOutput {
    pub success: bool,
    pub text: String,
    pub length: usize,
}

// ============================================================================
// TOOL ARGS TRAIT IMPL
// ============================================================================

#[tool_metadata(
    name = "browser_extract_text",
    category = "browser",
    description = "Extract visible text content from the page or specific elements using CSS selectors"
)]
impl ToolArgs for BrowserExtractTextArgs {
    type Output = BrowserExtractTextOutput;
    type Prompts = ExtractTextPrompts;

    const NAME: &'static str = BROWSER_EXTRACT_TEXT;
    const CATEGORY: &'static str = "browser";
    const DESCRIPTION: &'static str = "Extract visible text content from the page or specific elements using CSS selectors";
}
