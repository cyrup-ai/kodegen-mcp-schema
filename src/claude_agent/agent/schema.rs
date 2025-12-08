//! Schema types for claude_agent tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use kodegen_config::{CATEGORY_CLAUDE_AGENT, CLAUDE_AGENT};
use crate::ToolArgs;

// ============================================================================
// ACTION ENUM
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

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

const fn zero() -> u32 {
    0
}

const fn default_timeout_ms() -> u64 {
    300_000
}

// ============================================================================
// ARGS STRUCT
// ============================================================================

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

// ============================================================================
// OUTPUT STRUCTS
// ============================================================================

/// Output from `claude_agent` tool
/// Covers all actions: SPAWN, SEND, READ, LIST, KILL
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ClaudeAgentOutput {
    /// Agent instance number
    pub agent: u32,

    /// Action that was performed
    pub action: String,

    /// Session ID (present for SPAWN, READ, SEND)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,

    /// Output text from agent
    pub output: String,

    /// Number of messages in conversation (READ)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_count: Option<usize>,

    /// Whether agent is actively working (READ)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working: Option<bool>,

    /// Whether the operation/agent completed
    pub completed: bool,

    /// Exit code (Some for completed, None for still running)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i32>,

    /// For LIST action - all agents
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agents: Option<Vec<ClaudeAgentSummary>>,
}

/// Summary of a single agent for LIST action
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ClaudeAgentSummary {
    /// Agent instance number
    pub agent: u32,

    /// Session UUID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,

    /// Total messages in conversation
    pub message_count: usize,

    /// Whether agent is actively working
    pub working: bool,

    /// Whether agent has completed
    pub completed: bool,
}

// ============================================================================
// TOOL ARGS IMPLEMENTATION (Args→Output Binding)
// ============================================================================

use crate::tool_metadata;
use super::prompts::ClaudeAgentPrompts;

#[tool_metadata(
    description = "Unified Claude agent interface with action-based dispatch (SPAWN/SEND/READ/LIST/KILL). Spawn autonomous sub-agents for task delegation and parallel processing"
)]
impl ToolArgs for ClaudeAgentArgs {
    type Output = ClaudeAgentOutput;
    type Prompts = ClaudeAgentPrompts;

    const NAME: &'static str = CLAUDE_AGENT;
    const CATEGORY: &'static str = CATEGORY_CLAUDE_AGENT;
    const DESCRIPTION: &'static str = "Unified Claude agent interface with action-based dispatch (SPAWN/SEND/READ/LIST/KILL). Spawn autonomous sub-agents for task delegation and parallel processing";
}
