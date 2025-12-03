//! Browser tool argument schemas

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// ============================================================================
// CANONICAL TOOL NAME CONSTANTS
// ============================================================================
// These constants provide a single source of truth for browser tool names,
// eliminating string duplication across tool implementations and metadata.

// Core browser automation tools
pub const BROWSER_NAVIGATE: &str = "browser_navigate";
pub const BROWSER_CLICK: &str = "browser_click";
pub const BROWSER_TYPE_TEXT: &str = "browser_type_text";
pub const BROWSER_SCREENSHOT: &str = "browser_screenshot";
pub const BROWSER_EXTRACT_TEXT: &str = "browser_extract_text";
pub const BROWSER_SCROLL: &str = "browser_scroll";

// Advanced browser tools
pub const BROWSER_AGENT: &str = "browser_agent";

// Research (long-running with progress streaming)
pub const BROWSER_RESEARCH: &str = "browser_research";

// Research session management (DEPRECATED - will be removed)
pub const BROWSER_START_RESEARCH: &str = "browser_start_research";
pub const BROWSER_GET_RESEARCH_STATUS: &str = "browser_get_research_status";
pub const BROWSER_GET_RESEARCH_RESULT: &str = "browser_get_research_result";
pub const BROWSER_STOP_RESEARCH: &str = "browser_stop_research";
pub const BROWSER_LIST_RESEARCH_SESSIONS: &str = "browser_list_research_sessions";

// Web search
pub const BROWSER_WEB_SEARCH: &str = "browser_web_search";

// ============================================================================
// ACTION ENUMS
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

/// Actions for browser_agent tool
#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BrowserAgentAction {
    /// Prompt the agent with a new task (spawn background work)
    Prompt,
    /// Read current progress from an active agent
    Read,
    /// Kill a running agent (destroys slot permanently)
    Kill,
}

// ============================================================================
// WAIT FOR
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum WaitCondition {
    /// Element exists in DOM (uses page.find_element)
    Present,

    /// Element is visible (not display:none, visibility:hidden, etc)
    Visible,

    /// Element is visible AND not disabled (ready for interaction)
    Clickable,

    /// Element text contains specific string
    TextContains,

    /// Element attribute has specific value
    AttributeIs,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BrowserWaitForArgs {
    /// CSS selector for element to wait for
    pub selector: String,

    /// Condition to wait for
    pub condition: WaitCondition,

    /// Optional: timeout in milliseconds (default: 10000)
    #[serde(default)]
    pub timeout_ms: Option<u64>,

    /// Optional: expected text (required for TextContains condition)
    #[serde(default)]
    pub text: Option<String>,

    /// Optional: attribute name (required for AttributeIs condition)
    #[serde(default)]
    pub attribute_name: Option<String>,

    /// Optional: attribute value (required for AttributeIs condition)
    #[serde(default)]
    pub attribute_value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BrowserWaitForPromptArgs {}

// ============================================================================
// SCREENSHOT
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

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BrowserScreenshotPromptArgs {}

// ============================================================================
// BROWSER RESEARCH - REMOVED (replaced by async session-based tools)
// ============================================================================

// ============================================================================
// BROWSER RESEARCH (Long-Running with Progress Streaming)
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

/// Prompt arguments for `browser_research` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BrowserResearchPromptArgs {
    /// Optional: Research depth preference (shallow/moderate/deep)
    #[serde(default)]
    pub research_depth: Option<String>,
    
    /// Optional: Use case context (technical/news/documentation/general)
    #[serde(default)]
    pub use_case: Option<String>,
}

// ============================================================================
// ASYNC RESEARCH SESSION MANAGEMENT (DEPRECATED - kept for compatibility)
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct StartBrowserResearchArgs {
    /// Research query or topic to investigate
    pub query: String,

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
// ACTION-BASED HELPERS
// ============================================================================

fn zero() -> u32 {
    0
}

fn default_research_timeout_ms() -> u64 {
    300000 // 5 minutes
}

fn default_agent_timeout_ms() -> u64 {
    600000 // 10 minutes
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct StartBrowserResearchPromptArgs {}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GetResearchStatusArgs {
    /// Unique session identifier returned from start_browser_research
    pub session_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GetResearchStatusPromptArgs {}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GetResearchResultArgs {
    /// Unique session identifier returned from start_browser_research
    pub session_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GetResearchResultPromptArgs {}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct StopBrowserResearchArgs {
    /// Unique session identifier returned from start_browser_research
    pub session_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct StopBrowserResearchPromptArgs {}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ListResearchSessionsArgs {}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ListResearchSessionsPromptArgs {}

// ============================================================================
// EXTRACT TEXT
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BrowserExtractTextArgs {
    /// Optional: CSS selector for specific element (default: entire page)
    #[serde(default)]
    pub selector: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BrowserExtractTextPromptArgs {}

// ============================================================================
// WAIT
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BrowserWaitArgs {
    /// Duration to wait in milliseconds
    pub duration_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BrowserWaitPromptArgs {}

// ============================================================================
// NAVIGATE
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

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BrowserNavigatePromptArgs {
    /// Optional: Level of detail in examples ("basic" or "advanced")
    /// - "basic": URL validation, timeout essentials, simple examples
    /// - "advanced": Edge cases, selector nuances, troubleshooting, integration patterns
    #[serde(default)]
    pub complexity_level: Option<String>,

    /// Optional: Topic focus for examples ("general", "selectors", "timeouts", or "redirects")
    /// - "general": Balanced coverage of all features
    /// - "selectors": Focus on wait_for_selector CSS selector patterns and timing
    /// - "timeouts": Focus on timeout configuration and timeout-related errors
    /// - "redirects": Focus on redirect handling and URL change detection
    #[serde(default)]
    pub example_focus: Option<String>,
}

// ============================================================================
// CLICK
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

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BrowserClickPromptArgs {}

// ============================================================================
// SCROLL
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

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BrowserScrollPromptArgs {}

// ============================================================================
// TYPE TEXT
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BrowserTypeTextArgs {
    /// CSS selector for input element
    pub selector: String,

    /// Text to type into the element
    pub text: String,

    /// Optional: clear existing text first (default: true)
    #[serde(default = "default_clear")]
    pub clear: bool,

    /// Optional: timeout in milliseconds (default: 5000)
    #[serde(default)]
    pub timeout_ms: Option<u64>,
}

fn default_clear() -> bool {
    true
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BrowserTypeTextPromptArgs {}

// ============================================================================
// BROWSER AGENT
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BrowserAgentArgs {
    /// Action to perform on the agent session
    pub action: BrowserAgentAction,

    /// Agent number (0-based, default: 0) - unique per connection_id
    #[serde(default = "zero")]
    pub agent: u32,

    /// Maximum time in milliseconds to wait for completion (default: 600000ms = 10 minutes)
    /// - On timeout: returns current progress, agent continues in background
    /// - Special value 0: fire-and-forget (returns immediately)
    #[serde(default = "default_agent_timeout_ms")]
    pub await_completion_ms: u64,

    /// Task description for the agent to accomplish (required for EXEC, ignored for READ/KILL)
    #[serde(default)]
    pub task: Option<String>,

    /// Optional additional context or hints
    #[serde(default)]
    pub additional_info: Option<String>,

    /// Optional initial URL to navigate to before starting
    #[serde(default)]
    pub start_url: Option<String>,

    /// Maximum steps agent can take (default: 10)
    #[serde(default = "default_max_steps")]
    pub max_steps: u32,

    /// Maximum actions per step (default: 3)
    #[serde(default = "default_max_actions")]
    pub max_actions_per_step: u32,

    /// LLM temperature for action generation (default: 0.7)
    #[serde(default = "default_agent_temperature")]
    pub temperature: f64,

    /// Max tokens per LLM call (default: 2048)
    #[serde(default = "default_agent_max_tokens")]
    pub max_tokens: u64,

    /// Vision model timeout in seconds (default: 60s)
    /// Vision analysis is typically fast, but allow time for model loading
    #[serde(default = "default_vision_timeout_secs")]
    pub vision_timeout_secs: u64,

    /// LLM generation timeout in seconds (default: 120s)
    /// Allow time for complex reasoning and high token generation
    #[serde(default = "default_llm_timeout_secs")]
    pub llm_timeout_secs: u64,
}

fn default_max_steps() -> u32 {
    10
}
fn default_max_actions() -> u32 {
    3
}
fn default_agent_temperature() -> f64 {
    0.7
}
fn default_agent_max_tokens() -> u64 {
    2048
}
fn default_vision_timeout_secs() -> u64 {
    60
}
fn default_llm_timeout_secs() -> u64 {
    120
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BrowserAgentPromptArgs {}

// ============================================================================
// OUTPUT TYPES
// ============================================================================

/// Output from `browser_agent` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BrowserAgentOutput {
    /// Agent number
    pub agent: u32,
    /// Task being executed
    pub task: String,
    /// Current step count
    pub steps_taken: usize,
    /// Whether agent is complete
    pub completed: bool,
    /// Error message if any
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Progress summary
    pub summary: String,
    /// Detailed history
    pub history: Vec<BrowserAgentStepInfo>,
}

/// Step information from browser agent execution
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BrowserAgentStepInfo {
    pub step: usize,
    pub timestamp: String,
    pub actions: Vec<String>,
    pub summary: String,
    pub complete: bool,
}

/// Output from agent kill action
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BrowserAgentKillOutput {
    /// Agent number that was killed
    pub agent: u32,
    /// Success message
    pub message: String,
}

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

/// Output from `browser_navigate` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BrowserNavigateOutput {
    pub success: bool,
    pub url: String,
    pub title: Option<String>,
    pub status_code: Option<u16>,
}

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

/// Output from `browser_click` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BrowserClickOutput {
    pub success: bool,
    pub selector: String,
    pub message: String,
}

/// Output from `browser_type` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BrowserTypeOutput {
    pub success: bool,
    pub selector: String,
    pub text_length: usize,
    pub message: String,
}

/// Output from `browser_scroll` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BrowserScrollOutput {
    pub success: bool,
    pub direction: String,
    pub amount: i32,
    pub message: String,
}

/// Output from `browser_eval` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BrowserEvalOutput {
    pub success: bool,
    pub result: serde_json::Value,
}

/// Output from `browser_extract_text` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BrowserExtractTextOutput {
    pub success: bool,
    pub text: String,
    pub length: usize,
}

// ============================================================================
// TOOL ARGS TRAIT IMPLEMENTATIONS
// ============================================================================

use crate::ToolArgs;

impl ToolArgs for BrowserAgentArgs {
    type Output = BrowserAgentOutput;
}

impl ToolArgs for BrowserResearchArgs {
    type Output = BrowserResearchOutput;
}

impl ToolArgs for BrowserNavigateArgs {
    type Output = BrowserNavigateOutput;
}

impl ToolArgs for BrowserScreenshotArgs {
    type Output = BrowserScreenshotOutput;
}

impl ToolArgs for BrowserClickArgs {
    type Output = BrowserClickOutput;
}

impl ToolArgs for BrowserTypeTextArgs {
    type Output = BrowserTypeOutput;
}

impl ToolArgs for BrowserScrollArgs {
    type Output = BrowserScrollOutput;
}

impl ToolArgs for BrowserExtractTextArgs {
    type Output = BrowserExtractTextOutput;
}
