//! Schema types for fetch tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Canonical tool name for fetch
pub const FETCH: &str = "fetch";

/// Arguments for `fetch` tool - simplified single-page fetcher
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FetchArgs {
    /// URL to fetch (required)
    pub url: String,
}

/// Output from `fetch` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FetchOutput {
    /// ANSI syntax-highlighted markdown content for terminal display
    pub display: String,

    /// Absolute file path to the saved markdown file
    pub path: String,

    /// TypeScript snippet for searching this content with scrape_url
    pub search_helper: String,

    /// URL that was fetched
    pub url: String,

    /// Page title if available
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// Content length in bytes
    pub content_length: usize,
}

// ============================================================================
// TOOL ARGS TRAIT IMPLEMENTATION
// ============================================================================

use crate::{ToolArgs, tool_metadata};
use super::prompts::FetchPrompts;

#[tool_metadata(
    name = "fetch",
    category = "citescrape",
    description = "Fetch a single web page and display as ANSI-highlighted markdown. Simplified wrapper around scrape_url for quick page retrieval."
)]
impl ToolArgs for FetchArgs {
    type Output = FetchOutput;
    type Prompts = FetchPrompts;

    const NAME: &'static str = FETCH;
    const CATEGORY: &'static str = "citescrape";
    const DESCRIPTION: &'static str = "Fetch a single web page and display as ANSI-highlighted markdown. Simplified wrapper around scrape_url for quick page retrieval.";
}
