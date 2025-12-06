//! Schema types for process_list tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{ToolArgs, tool_metadata};
use super::prompts::ProcessListPrompts;

// ============================================================================
// CANONICAL TOOL NAME CONSTANTS
// ============================================================================

/// Canonical tool name for process_list
pub const PROCESS_LIST: &str = "process_list";

// ============================================================================
// PROCESS LIST TOOL
// ============================================================================

/// Arguments for `process_list` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ProcessListArgs {
    /// Optional: filter by name (case-insensitive substring match)
    #[serde(default)]
    pub filter: Option<String>,

    /// Maximum number of processes to return (0 = unlimited)
    #[serde(default)]
    pub limit: usize,
}

/// Output from `process_list` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ProcessListOutput {
    /// Whether the operation succeeded
    pub success: bool,
    /// Number of processes returned
    pub count: usize,
    /// List of process information
    pub processes: Vec<ProcessInfo>,
}

/// Information about a single process
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ProcessInfo {
    /// Process ID
    pub pid: u32,
    /// Process name/command
    pub name: String,
    /// CPU usage percentage
    pub cpu_percent: f32,
    /// Memory usage in megabytes
    pub memory_mb: f64,
}

// ============================================================================
// TOOL ARGS IMPLEMENTATION (Args→Output Binding)
// ============================================================================

#[tool_metadata(
    name = "process_list",
    category = "process",
    description = "List all running processes with PID, command name, CPU usage, and memory usage. Supports filtering by process name"
)]
impl ToolArgs for ProcessListArgs {
    type Output = ProcessListOutput;
    type Prompts = ProcessListPrompts;

    const NAME: &'static str = PROCESS_LIST;
    const CATEGORY: &'static str = "process";
    const DESCRIPTION: &'static str = "List all running processes with PID, command name, CPU usage, and memory usage. Supports filtering by process name";
}
