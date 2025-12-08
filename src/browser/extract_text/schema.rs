//! Schema types for browser_extract_text tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use kodegen_config::{CATEGORY_BROWSER, BROWSER_EXTRACT_TEXT};
use crate::{ToolArgs, tool_metadata};
use super::prompts::ExtractTextPrompts;

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
    description = "Extract visible text content from the page or specific elements using CSS selectors"
)]
impl ToolArgs for BrowserExtractTextArgs {
    type Output = BrowserExtractTextOutput;
    type Prompts = ExtractTextPrompts;

    const NAME: &'static str = BROWSER_EXTRACT_TEXT;
    const CATEGORY: &'static str = CATEGORY_BROWSER;
    const DESCRIPTION: &'static str = "Extract visible text content from the page or specific elements using CSS selectors";
}
