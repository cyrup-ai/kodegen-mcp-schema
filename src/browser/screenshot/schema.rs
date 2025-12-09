//! Schema types for browser_screenshot tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use kodegen_config::{CATEGORY_BROWSER, BROWSER_SCREENSHOT};
use crate::{ToolArgs, tool_metadata};
use super::prompts::ScreenshotPrompts;

// ============================================================================
// INPUT ARGS
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BrowserScreenshotArgs {
    /// Optional: CSS selector to screenshot specific element (default: full page)
    #[serde(default)]
    pub selector: Option<String>,

    /// Optional: format (png or jpeg, default: png)
    #[serde(default)]
    pub format: Option<String>,
}

// ============================================================================
// OUTPUT
// ============================================================================

/// Output from `browser_screenshot` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BrowserScreenshotOutput {
    pub success: bool,
    pub path: Option<String>,
    pub width: u32,
    pub height: u32,
    pub format: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base64: Option<String>,
}

// ============================================================================
// TOOL ARGS TRAIT IMPL
// ============================================================================

#[tool_metadata(
    description = "Capture a screenshot of the page or specific element in PNG format with base64 encoding"
)]
impl ToolArgs for BrowserScreenshotArgs {
    type Output = BrowserScreenshotOutput;
    type Prompts = ScreenshotPrompts;

    const NAME: &'static str = BROWSER_SCREENSHOT;
    const CATEGORY: &'static kodegen_config::Category = CATEGORY_BROWSER;
    const DESCRIPTION: &'static str = "Capture a screenshot of the page or specific element in PNG format with base64 encoding";
}
