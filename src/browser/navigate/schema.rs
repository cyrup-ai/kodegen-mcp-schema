//! Schema types for browser_navigate tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use kodegen_config::{CATEGORY_BROWSER, BROWSER_NAVIGATE};
use crate::{ToolArgs, tool_metadata};
use super::prompts::NavigatePrompts;

// ============================================================================
// INPUT ARGS
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BrowserNavigateArgs {
    /// URL to navigate to (must start with http:// or https://)
    pub url: String,

    /// Optional: wait for specific CSS selector before returning
    #[serde(default)]
    pub wait_for_selector: Option<String>,

    /// Optional: timeout in milliseconds (default: 30000)
    #[serde(default)]
    pub timeout_ms: Option<u64>,
}

// ============================================================================
// OUTPUT
// ============================================================================

/// Output from `browser_navigate` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BrowserNavigateOutput {
    pub success: bool,
    pub url: String,
    pub title: Option<String>,
    pub status_code: Option<u16>,
}

// ============================================================================
// TOOL ARGS TRAIT IMPL
// ============================================================================

#[tool_metadata(
    description = "Navigate browser to specified URL with configurable load timeout and error handling"
)]
impl ToolArgs for BrowserNavigateArgs {
    type Output = BrowserNavigateOutput;
    type Prompts = NavigatePrompts;

    const NAME: &'static str = BROWSER_NAVIGATE;
    const CATEGORY: &'static str = CATEGORY_BROWSER;
    const DESCRIPTION: &'static str = "Navigate browser to specified URL with configurable load timeout and error handling";
}
