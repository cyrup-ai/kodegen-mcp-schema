//! Process tool argument schemas

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// ============================================================================
// CANONICAL TOOL NAME CONSTANTS
// ============================================================================

/// Canonical tool names for all process tools
/// These constants are the single source of truth for process tool names.
/// All tool implementations, metadata, and clients MUST reference these constants.
pub const PROCESS_KILL: &str = "process_kill";
pub const PROCESS_LIST: &str = "process_list";

// ============================================================================
// PROCESS LIST
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

/// Prompt arguments for `process_list` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ProcessListPromptArgs {}

// ============================================================================
// PROCESS KILL
// ============================================================================

/// Arguments for `process_kill` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ProcessKillArgs {
    /// Process ID to terminate
    pub pid: u32,
}

/// Prompt arguments for `process_kill` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ProcessKillPromptArgs {
    /// Optional scenario to focus teaching on (e.g., 'hung_process', 'daemon', 'cleanup', 'error_handling')
    #[serde(default)]
    pub scenario: Option<String>,
    
    /// Whether to include extensive safety warnings and gotchas
    #[serde(default)]
    pub show_safety_warnings: Option<bool>,
}
