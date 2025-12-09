//! Schema types for browser_web_search tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use kodegen_config::{CATEGORY_BROWSER, BROWSER_WEB_SEARCH};
use crate::{ToolArgs, tool_metadata};
use super::prompts::WebSearchPrompts;

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

fn default_max_results() -> usize {
    10
}

// ============================================================================
// INPUT ARGS
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BrowserWebSearchArgs {
    /// Search query string (required)
    pub query: String,
    
    /// Maximum number of results to return (default: 10)
    #[serde(default = "default_max_results")]
    pub max_results: usize,
}

// ============================================================================
// OUTPUT TYPES
// ============================================================================

/// Output from `browser_web_search` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BrowserWebSearchOutput {
    pub success: bool,
    pub query: String,
    pub results_count: usize,
    pub results: Vec<WebSearchResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct WebSearchResult {
    pub rank: u32,
    pub title: String,
    pub url: String,
    pub snippet: Option<String>,
}

// ============================================================================
// TOOL ARGS TRAIT IMPL
// ============================================================================

#[tool_metadata(
    description = "Perform web searches using various search engines and retrieve ranked results with titles, URLs, and snippets"
)]
impl ToolArgs for BrowserWebSearchArgs {
    type Output = BrowserWebSearchOutput;
    type Prompts = WebSearchPrompts;

    const NAME: &'static str = BROWSER_WEB_SEARCH;
    const CATEGORY: &'static kodegen_config::Category = CATEGORY_BROWSER;
    const DESCRIPTION: &'static str = "Perform web searches using various search engines and retrieve ranked results with titles, URLs, and snippets";
}
