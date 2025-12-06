//! Schema types for browser_click tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::{ToolArgs, tool_metadata};
use super::prompts::ClickPrompts;

// ============================================================================
// CANONICAL TOOL NAME CONSTANTS
// ============================================================================

pub const BROWSER_CLICK: &str = "browser_click";

// ============================================================================
// INPUT ARGS
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BrowserClickArgs {
    /// CSS selector for element to click
    pub selector: String,

    /// Optional: timeout in milliseconds (default: 5000)
    #[serde(default)]
    pub timeout_ms: Option<u64>,

    /// Optional: wait for navigation after click (default: false)
    /// Set to true when clicking links, submit buttons, or elements that trigger page navigation
    #[serde(default)]
    pub wait_for_navigation: Option<bool>,
}

// ============================================================================
// OUTPUT
// ============================================================================

/// Output from `browser_click` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BrowserClickOutput {
    pub success: bool,
    pub selector: String,
    pub message: String,
}

// ============================================================================
// TOOL ARGS TRAIT IMPL
// ============================================================================

#[tool_metadata(
    name = "browser_click",
    category = "browser",
    description = "Click an element on the page using CSS selectors with automatic wait for clickability"
)]
impl ToolArgs for BrowserClickArgs {
    type Output = BrowserClickOutput;
    type Prompts = ClickPrompts;

    const NAME: &'static str = BROWSER_CLICK;
    const CATEGORY: &'static str = "browser";
    const DESCRIPTION: &'static str = "Click an element on the page using CSS selectors with automatic wait for clickability";
}
