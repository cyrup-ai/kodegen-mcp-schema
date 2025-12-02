//! Terminal tool argument schemas

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// ============================================================================
// CANONICAL TOOL NAME CONSTANTS
// ============================================================================

/// Canonical tool name for unified terminal tool
pub const TERMINAL: &str = "terminal";

// ============================================================================
// UNIFIED TERMINAL TOOL
// ============================================================================

const fn zero() -> u32 {
    0
}

const fn default_await_completion_ms() -> u64 {
    300_000 // 5 minutes
}

const fn default_clear() -> bool {
    true
}

const fn default_tail() -> u32 {
    2000
}

/// Terminal action types
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TerminalAction {
    /// Execute a command (default) - requires `command` field
    #[default]
    Exec,
    /// Read current 80x24 VTE buffer snapshot without executing
    Read,
    /// List all active terminals with their current states
    List,
    /// Gracefully shutdown terminal and cleanup all resources
    Kill,
    /// Send Ctrl+C to interrupt any running command
    Interrupt,
}

/// Arguments for unified `terminal` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TerminalInput {
    /// Action to perform - defaults to EXEC for backward compatibility
    #[serde(default)]
    pub action: TerminalAction,

    /// Command to execute (required for EXEC, ignored for READ/LIST/KILL)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,

    /// Terminal number (0, 1, 2...) - defaults to 0
    /// Terminals are reusable and stateful - use different numbers for parallel work
    #[serde(default = "zero")]
    pub terminal: u32,

    /// Maximum time in milliseconds to wait for command completion (default 300000ms = 5 minutes)
    ///
    /// - On timeout: returns current 80x24 VTE buffer snapshot, command continues in background
    /// - Special value 0: fire-and-forget background task (returns immediately)
    /// - Command continues running after timeout - use action=READ to check progress
    #[serde(default = "default_await_completion_ms")]
    pub await_completion_ms: u64,

    /// Run `clear` command before executing (EXEC only)
    /// Clears VTE buffer so output contains only new command's output
    /// Default: true
    #[serde(default = "default_clear")]
    pub clear: bool,

    /// Limit lines returned from buffer
    /// Default: 2000 (scrollback buffer size)
    /// Max: 2000
    #[serde(default = "default_tail")]
    pub tail: u32,
}

/// Response from unified `terminal` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TerminalOutput {
    /// Terminal number (None for LIST action which returns multiple terminals)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminal: Option<u32>,

    /// Command output (80x24 VTE buffer - rendered terminal output, not raw bytes)
    /// For LIST: JSON array of terminal snapshots
    pub output: String,

    /// Exit code (Some(0) = success, Some(non-zero) = error, None = still running)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i32>,

    /// Current working directory after command completes
    pub cwd: String,

    /// Execution duration in milliseconds
    pub duration_ms: u64,

    /// Whether command completed (true) or is still running/timed out (false)
    pub completed: bool,
}

/// Prompt arguments for unified `terminal` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TerminalPromptArgs {}
