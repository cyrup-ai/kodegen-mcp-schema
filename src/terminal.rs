//! Terminal tool argument schemas

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::ToolArgs;

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
///
/// Public API has exactly 4 actions:
/// - EXEC: Execute command (default)
/// - READ: Get current buffer snapshot
/// - LIST: Show all active terminals
/// - KILL: Gracefully shutdown terminal
///
/// Note: Cancellation of running commands is handled internally via
/// CancellationToken - not exposed as a public action.
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

/// Snapshot of a terminal instance (for LIST action)
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TerminalSnapshot {
    /// Terminal number
    pub terminal: u32,
    /// Current working directory
    pub cwd: String,
    /// Exit code of last command (None if still running)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i32>,
    /// Whether last command completed
    pub completed: bool,
}

/// Response from unified `terminal` tool
///
/// Note: Terminal output is returned in the display field (Vec[Content]0) only.
/// This struct contains only metadata about the command execution.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TerminalOutput {
    /// Terminal number (None for LIST action which returns multiple terminals)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminal: Option<u32>,

    /// Exit code (Some(0) = success, Some(non-zero) = error, None = still running)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i32>,

    /// Current working directory after command completes
    pub cwd: String,

    /// Execution duration in milliseconds
    pub duration_ms: u64,

    /// Whether command completed (true) or is still running/timed out (false)
    pub completed: bool,

    /// List of all terminal snapshots (present for LIST action)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub terminals: Vec<TerminalSnapshot>,
}

/// Prompt arguments for unified `terminal` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TerminalPromptArgs {}

// ============================================================================
// TOOL ARGS IMPLEMENTATION (Args→Output Binding)
// ============================================================================

impl ToolArgs for TerminalInput {
    type Output = TerminalOutput;
}
