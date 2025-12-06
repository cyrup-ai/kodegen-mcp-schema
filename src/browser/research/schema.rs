//! Schema types for browser_research tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::{ToolArgs, tool_metadata};
use super::prompts::ResearchPrompts;

// ============================================================================
// CANONICAL TOOL NAME CONSTANTS
// ============================================================================

pub const BROWSER_RESEARCH: &str = "browser_research";

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

const fn zero() -> u32 {
    0
}

fn default_research_timeout_ms() -> u64 {
    300000 // 5 minutes
}

fn default_max_pages() -> usize {
    5
}

fn default_max_depth() -> usize {
    2
}

fn default_search_engine() -> String {
    "google".into()
}

fn default_true() -> bool {
    true
}

fn default_false() -> bool {
    false
}

fn default_timeout() -> u64 {
    60
}

fn default_temperature() -> f64 {
    0.5
}

fn default_max_tokens() -> u64 {
    2048
}

// ============================================================================
// ACTION ENUM
// ============================================================================

/// Actions for browser_research tool
#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BrowserResearchAction {
    /// Start a new research query (spawn background work)
    Research,
    /// Read current progress from an active research session
    Read,
    /// List all active research sessions
    List,
    /// Kill a running research session (destroys slot permanently)
    Kill,
}

// ============================================================================
// INPUT ARGS
// ============================================================================

/// Arguments for `browser_research` tool (long-running with progress streaming)
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BrowserResearchArgs {
    /// Action to perform on the research session
    pub action: BrowserResearchAction,

    /// Session number (0-based, default: 0) - unique per connection_id
    #[serde(default = "zero")]
    pub session: u32,

    /// Maximum time in milliseconds to wait for completion (default: 300000ms = 5 minutes)
    /// - On timeout: returns current progress, research continues in background
    /// - Special value 0: fire-and-forget (returns immediately)
    #[serde(default = "default_research_timeout_ms")]
    pub await_completion_ms: u64,

    /// Research query or topic to investigate (required for EXEC, ignored for READ/LIST/KILL)
    #[serde(default)]
    pub query: Option<String>,

    /// Maximum number of pages to visit (default: 5)
    #[serde(default = "default_max_pages")]
    pub max_pages: usize,

    /// Maximum link-following depth (default: 2)
    #[serde(default = "default_max_depth")]
    pub max_depth: usize,

    /// Search engine to use: "google", "bing", "duckduckgo" (default: "google")
    #[serde(default = "default_search_engine")]
    pub search_engine: String,

    /// Include hyperlinks in content extraction (default: true)
    #[serde(default = "default_true")]
    pub include_links: bool,

    /// Extract and parse HTML tables (default: true)
    #[serde(default = "default_true")]
    pub extract_tables: bool,

    /// Extract image URLs and alt text (default: false)
    #[serde(default = "default_false")]
    pub extract_images: bool,

    /// Timeout per page navigation in seconds (default: 60)
    #[serde(default = "default_timeout")]
    pub timeout_seconds: u64,

    /// LLM temperature for summarization (0.0=deterministic, 2.0=creative, default: 0.5)
    #[serde(default = "default_temperature")]
    pub temperature: f64,

    /// Maximum tokens for LLM summary generation (default: 2048)
    #[serde(default = "default_max_tokens")]
    pub max_tokens: u64,
}

// ============================================================================
// OUTPUT TYPES
// ============================================================================

/// Output from `browser_research` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BrowserResearchOutput {
    pub session: u32,
    pub status: String,
    pub query: String,
    pub pages_analyzed: usize,
    pub max_pages: usize,
    pub completed: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_findings: Option<Vec<String>>,
    pub sources: Vec<ResearchSource>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ResearchSource {
    pub url: String,
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
}

// ============================================================================
// TOOL ARGS TRAIT IMPL
// ============================================================================

#[tool_metadata(
    name = "browser_research",
    category = "browser",
    description = "Conduct comprehensive web research with multi-page crawling, content analysis, and AI-powered synthesis"
)]
impl ToolArgs for BrowserResearchArgs {
    type Output = BrowserResearchOutput;
    type Prompts = ResearchPrompts;

    const NAME: &'static str = BROWSER_RESEARCH;
    const CATEGORY: &'static str = "browser";
    const DESCRIPTION: &'static str = "Conduct comprehensive web research with multi-page crawling, content analysis, and AI-powered synthesis";
}
