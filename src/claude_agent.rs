//! Claude Agent tool argument schemas

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// PROMPT INPUT TYPES (shared across tools)
// ============================================================================

/// Input for agent prompts - can be plain string or template
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type", content = "value")]
pub enum PromptInput {
    /// Plain text prompt
    #[serde(rename = "string")]
    String(String),

    /// Template-based prompt with parameters
    #[serde(rename = "template")]
    Template(PromptTemplateInput),
}

/// Template reference with parameters
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PromptTemplateInput {
    /// Template name (e.g., "code_review", "bug_fix")
    pub name: String,

    /// Parameters to pass to template rendering
    #[serde(default)]
    pub parameters: HashMap<String, serde_json::Value>,
}

// ============================================================================
// TOOL NAME CONSTANTS (Memory Tools)
// ============================================================================

/// Tool name constant for memory_memorize
pub const MEMORY_MEMORIZE: &str = "memory_memorize";

/// Tool name constant for memory_check_memorize_status
pub const MEMORY_CHECK_MEMORIZE_STATUS: &str = "memory_check_memorize_status";

/// Tool name constant for memory_recall
pub const MEMORY_RECALL: &str = "memory_recall";

/// Tool name constant for memory_list_libraries
pub const MEMORY_LIST_LIBRARIES: &str = "memory_list_libraries";

// Agent Management Tools
/// Tool name constant for unified claude_agent tool
pub const CLAUDE_AGENT: &str = "claude_agent";



// ============================================================================
// UNIFIED CLAUDE AGENT (Elite Registry Pattern)
// ============================================================================

/// Action enumeration for unified claude_agent tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ClaudeAgentAction {
    /// Spawn new agent session with initial prompt
    #[default]
    Spawn,
    /// Send additional prompt to existing agent session
    Send,
    /// Read current agent output
    Read,
    /// List all agent sessions for this connection
    List,
    /// Terminate agent session and cleanup
    Kill,
}

fn zero() -> u32 {
    0
}

fn default_timeout_ms() -> u64 {
    300_000
}

/// Arguments for unified claude_agent tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ClaudeAgentArgs {
    /// Action to perform
    #[serde(default)]
    pub action: ClaudeAgentAction,

    /// Agent instance number (0, 1, 2...)
    #[serde(default = "zero")]
    pub agent: u32,

    /// Maximum time to wait for completion (ms)
    /// - On timeout: returns current output, agent continues in background
    /// - Special value 0: fire-and-forget background agent
    #[serde(default = "default_timeout_ms")]
    pub await_completion_ms: u64,

    // SPAWN/SEND-specific fields
    /// Prompt for agent (required for SPAWN/SEND)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,

    /// System prompt to define agent behavior (SPAWN only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_prompt: Option<String>,

    /// Maximum conversation turns (SPAWN only, default: 10)
    #[serde(default)]
    pub max_turns: Option<u32>,

    /// AI model to use (SPAWN only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,

    /// Working directory for agent operations (SPAWN only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cwd: Option<String>,

    /// Tools the agent CAN use (allowlist, SPAWN only)
    /// 
    /// Accepts both single string and array: `allowed_tools: "fs_search"` or `allowed_tools: ["fs_search", "fs_read"]`
    #[serde(default, deserialize_with = "crate::serde_helpers::string_or_vec")]
    pub allowed_tools: Vec<String>,

    /// Tools the agent CANNOT use (blocklist, SPAWN only)
    /// 
    /// Accepts both single string and array: `disallowed_tools: "terminal"` or `disallowed_tools: ["terminal", "bash"]`
    #[serde(default, deserialize_with = "crate::serde_helpers::string_or_vec")]
    pub disallowed_tools: Vec<String>,

    /// Additional context directories (SPAWN only)
    /// 
    /// Accepts both single string and array: `add_dirs: "./src"` or `add_dirs: ["./src", "./tests"]`
    #[serde(default, deserialize_with = "crate::serde_helpers::string_or_vec")]
    pub add_dirs: Vec<String>,
}

/// Prompt arguments for claude_agent tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ClaudeAgentPromptArgs {
    /// Which action(s) to focus on: "spawn", "send", "terminate", or "all" (default: "all")
    #[serde(default = "default_focus_area")]
    pub focus_area: String,

    /// Detail level: "basic" for core usage, "advanced" for edge cases and best practices (default: "basic")
    #[serde(default = "default_detail_level")]
    pub detail_level: String,
}

fn default_focus_area() -> String {
    "all".to_string()
}

fn default_detail_level() -> String {
    "basic".to_string()
}

// ============================================================================
// MEMORY TOOLS (for candle-agent)
// ============================================================================

// ========== Memorize Tool ==========

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct MemorizeArgs {
    /// Library name to store the memory in
    pub library: String,
    /// Content to memorize
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct MemorizePromptArgs {}

// ========== Recall Tool ==========

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RecallArgs {
    /// Library name to search in
    pub library: String,
    /// Context/query to search for
    pub context: String,
    /// Maximum number of results (default: 10)
    #[serde(default = "default_recall_limit")]
    pub limit: usize,
}

fn default_recall_limit() -> usize {
    10
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RecallPromptArgs {}

// ========== List Memory Libraries Tool ==========

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ListMemoryLibrariesArgs {}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ListMemoryLibrariesPromptArgs {}

// ========== Check Memorize Status Tool ==========

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CheckMemorizeStatusArgs {
    /// Session ID from memorize() call
    pub session_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CheckMemorizeStatusPromptArgs {}
