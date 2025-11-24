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

const fn one() -> u32 {
    1
}

/// Arguments for unified `terminal` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TerminalInput {
    /// Command to execute
    pub command: String,

    /// Terminal number (1, 2, 3...) - defaults to 1
    /// Terminals are reusable and stateful - use different numbers for parallel work
    #[serde(default = "one")]
    pub terminal: u32,
}

/// Response from unified `terminal` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TerminalOutput {
    /// Terminal number
    pub terminal: u32,

    /// Command output (stdout + stderr interleaved)
    pub output: String,

    /// Exit code (0 = success, non-zero = error)
    pub exit_code: i32,

    /// Current working directory after command completes
    pub cwd: String,

    /// Execution duration in milliseconds
    pub duration_ms: u64,
}

/// Prompt arguments for unified `terminal` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TerminalPromptArgs {}
