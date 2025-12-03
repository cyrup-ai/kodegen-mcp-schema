//! Citescrape tool argument schemas

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// ============================================================================
// CANONICAL TOOL NAME CONSTANTS
// ============================================================================

/// Canonical tool names for citescrape tools
pub const SCRAPE_URL: &str = "scrape_url";
pub const WEB_SEARCH: &str = "web_search";

// ============================================================================
// ACTION ENUM
// ============================================================================

/// Scrape action types - Elite Terminal Pattern
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ScrapeAction {
    /// Start new web crawl (default action)
    #[default]
    Crawl,
    /// Read current progress without blocking
    Read,
    /// List all crawls for connection
    List,
    /// Cancel crawl and cleanup resources
    Kill,
    /// Search indexed content (replaces scrape_search_results tool)
    Search,
}

// ============================================================================
// DEFAULT HELPERS
// ============================================================================

const fn zero() -> u32 { 
    0 
}

const fn default_await_completion_ms() -> u64 { 
    600_000  // 10 minutes
}

// Note: Field is `crawl_id` not `crawl` to avoid verb/noun ambiguity
// Unlike terminal where "terminal:0" is clearly a noun, "crawl:0" could be 
// misread as a command. "crawl_id:0" is unambiguous.

// ============================================================================
// SCRAPE URL
// ============================================================================

fn default_max_depth() -> u8 {
    3
}

fn default_true() -> bool {
    true
}

fn default_crawl_rate() -> f64 {
    2.0
}

fn default_search_limit() -> usize {
    10
}

fn default_true_search() -> bool {
    true
}

/// Arguments for unified `scrape_url` tool - Elite Terminal Pattern
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ScrapeUrlArgs {
    // ===== ACTION CONTROL =====
    
    /// Action to perform (CRAWL/READ/LIST/KILL/SEARCH)
    #[serde(default)]
    pub action: ScrapeAction,

    /// Crawl instance ID (0, 1, 2...) for connection isolation
    /// Named `crawl_id` (not `crawl`) to avoid verb/noun ambiguity
    #[serde(default = "zero")]
    pub crawl_id: u32,

    /// Maximum time to wait for completion (ms)
    /// - On timeout: returns current progress, crawl continues in background
    /// - Special value 0: fire-and-forget background crawl
    /// - Use action=READ to check progress after timeout
    #[serde(default = "default_await_completion_ms")]
    pub await_completion_ms: u64,

    // ===== CRAWL-SPECIFIC FIELDS =====
    
    /// Target URL (required for CRAWL action)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    /// Output directory for crawled content
    #[serde(default)]
    pub output_dir: Option<String>,

    /// Maximum crawl depth (default: 3)
    #[serde(default = "default_max_depth")]
    pub max_depth: u8,

    /// Maximum number of pages to crawl (default: unbounded)
    #[serde(default)]
    pub limit: Option<usize>,

    /// Save markdown format (default: true)
    #[serde(default = "default_true")]
    pub save_markdown: bool,

    /// Save screenshots (default: false for speed)
    #[serde(default)]
    pub save_screenshots: bool,

    /// Enable search indexing (default: true)
    #[serde(default = "default_true")]
    pub enable_search: bool,

    /// Crawl rate in requests per second (default: 2.0)
    #[serde(default = "default_crawl_rate")]
    pub crawl_rate_rps: f64,

    /// Allow subdomain crawling (default: false)
    #[serde(default)]
    pub allow_subdomains: bool,

    /// Content types to generate
    #[serde(default)]
    pub content_types: Option<Vec<String>>,

    // ===== SEARCH-SPECIFIC FIELDS (replaces scrape_search_results) =====
    
    /// Search query (required for SEARCH action)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,

    /// Maximum search results (default: 10)
    #[serde(default = "default_search_limit")]
    pub search_limit: usize,

    /// Search result offset for pagination (default: 0)
    #[serde(default)]
    pub search_offset: usize,

    /// Enable search result highlighting (default: true)
    #[serde(default = "default_true_search")]
    pub search_highlight: bool,
}

/// Prompt arguments for `scrape_url` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ScrapeUrlPromptArgs {}

// ============================================================================
// WEB SEARCH
// ============================================================================

/// Arguments for `web_search` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct WebSearchArgs {
    /// Search query string (required)
    pub query: String,
}

/// Prompt arguments for `web_search` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct WebSearchPromptArgs {
    /// Type of search queries to focus examples on (e.g., 'technical', 'documentation', 'code')
    #[serde(default)]
    pub query_type: Option<String>,

    /// Domain or technology focus area (e.g., 'Rust', 'Python', 'security', 'web development')
    #[serde(default)]
    pub focus_area: Option<String>,

    /// Complexity level (e.g., 'basic', 'intermediate', 'advanced')
    #[serde(default)]
    pub depth: Option<String>,
}

// ============================================================================
// OUTPUT TYPES
// ============================================================================

/// Output from `scrape_url` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ScrapeUrlOutput {
    pub crawl_id: u32,
    pub status: String,
    pub url: Option<String>,
    pub pages_crawled: usize,
    pub pages_queued: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_dir: Option<String>,
    pub elapsed_ms: u64,
    pub completed: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// For LIST action
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crawls: Option<Vec<CrawlSnapshot>>,
    /// For SEARCH action
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_results: Option<Vec<ScrapeSearchResult>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CrawlSnapshot {
    pub crawl_id: u32,
    pub status: String,
    pub url: Option<String>,
    pub pages_crawled: usize,
    pub elapsed_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ScrapeSearchResult {
    pub url: String,
    pub title: Option<String>,
    pub snippet: String,
    pub score: f32,
    pub path: Option<String>,
}

/// Output from `web_search` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct WebSearchOutput {
    pub success: bool,
    pub query: String,
    pub results_count: usize,
    pub results: Vec<WebSearchResultItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct WebSearchResultItem {
    pub rank: u32,
    pub title: String,
    pub url: String,
    pub snippet: Option<String>,
}

// ============================================================================
// TOOL ARGS TRAIT IMPLEMENTATIONS
// ============================================================================

use crate::ToolArgs;

impl ToolArgs for ScrapeUrlArgs {
    type Output = ScrapeUrlOutput;
}

impl ToolArgs for WebSearchArgs {
    type Output = WebSearchOutput;
}
