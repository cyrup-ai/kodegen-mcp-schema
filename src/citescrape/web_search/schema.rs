//! Schema types for web_search tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use kodegen_config::{CATEGORY_CITESCRAPE, WEB_SEARCH};

// ============================================================================
// WEB SEARCH TOOL
// ============================================================================

/// Arguments for `web_search` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct WebSearchArgs {
    /// Search query string (required)
    pub query: String,
}

// ============================================================================
// OUTPUT TYPES
// ============================================================================

/// Output from `web_search` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct WebSearchOutput {
    pub success: bool,
    pub query: String,
    pub results_count: usize,
    pub results: Vec<WebSearchResultItem>,
}

/// Individual search result item
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct WebSearchResultItem {
    pub rank: u32,
    pub title: String,
    pub url: String,
    pub snippet: Option<String>,
}

// ============================================================================
// TOOL ARGS TRAIT IMPLEMENTATION
// ============================================================================

use crate::{ToolArgs, tool_metadata};
use super::prompts::WebSearchPrompts;

#[tool_metadata(
    description = "Perform web search using DuckDuckGo and return structured results with titles, URLs, and snippets"
)]
impl ToolArgs for WebSearchArgs {
    type Output = WebSearchOutput;
    type Prompts = WebSearchPrompts;

    const NAME: &'static str = WEB_SEARCH;
    const CATEGORY: &'static kodegen_config::Category = CATEGORY_CITESCRAPE;
    const DESCRIPTION: &'static str = "Perform web search using DuckDuckGo and return structured results with titles, URLs, and snippets";

    fn icon() -> char {
        '⚶'  // VESTA - tool-specific icon (overrides citescrape default)
    }
}
