//! Schema types for browser_scroll tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use kodegen_config::{CATEGORY_BROWSER, BROWSER_SCROLL};
use crate::{ToolArgs, tool_metadata};
use super::prompts::ScrollPrompts;

// ============================================================================
// INPUT ARGS
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BrowserScrollArgs {
    /// Optional: CSS selector to scroll to element (takes priority over x/y)
    #[serde(default)]
    pub selector: Option<String>,

    /// Optional: horizontal scroll amount in pixels (default: 0)
    #[serde(default)]
    pub x: Option<i32>,

    /// Optional: vertical scroll amount in pixels (default: 0)
    #[serde(default)]
    pub y: Option<i32>,
}

// ============================================================================
// OUTPUT
// ============================================================================

/// Output from `browser_scroll` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BrowserScrollOutput {
    pub success: bool,
    pub direction: String,
    pub amount: i32,
    pub message: String,
}

// ============================================================================
// TOOL ARGS TRAIT IMPL
// ============================================================================

#[tool_metadata(
    description = "Scroll the page in specified direction or to a specific element, useful for lazy-loaded content"
)]
impl ToolArgs for BrowserScrollArgs {
    type Output = BrowserScrollOutput;
    type Prompts = ScrollPrompts;

    const NAME: &'static str = BROWSER_SCROLL;
    const CATEGORY: &'static kodegen_config::Category = CATEGORY_BROWSER;
    const DESCRIPTION: &'static str = "Scroll the page in specified direction or to a specific element, useful for lazy-loaded content";
}
