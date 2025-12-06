//! Schema types for process_kill tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{ToolArgs, tool_metadata};
use super::prompts::ProcessKillPrompts;

// ============================================================================
// CANONICAL TOOL NAME CONSTANTS
// ============================================================================

/// Canonical tool name for process_kill
pub const PROCESS_KILL: &str = "process_kill";

// ============================================================================
// PROCESS KILL TOOL
// ============================================================================

/// Arguments for `process_kill` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ProcessKillArgs {
    /// Process ID to terminate
    pub pid: u32,
}

/// Output from `process_kill` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ProcessKillOutput {
    /// Whether the kill operation succeeded
    pub success: bool,
    /// Process ID that was targeted
    pub pid: u32,
    /// Human-readable result message
    pub message: String,
}

// ============================================================================
// TOOL ARGS IMPLEMENTATION (Args→Output Binding)
// ============================================================================

#[tool_metadata(
    name = "process_kill",
    category = "process",
    description = "Terminate a process by PID. Sends SIGKILL signal - the process cannot catch or ignore it. Use with caution"
)]
impl ToolArgs for ProcessKillArgs {
    type Output = ProcessKillOutput;
    type Prompts = ProcessKillPrompts;

    const NAME: &'static str = PROCESS_KILL;
    const CATEGORY: &'static str = "process";
    const DESCRIPTION: &'static str = "Terminate a process by PID. Sends SIGKILL signal - the process cannot catch or ignore it. Use with caution";
}
