//! Process tool argument schemas

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

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
pub struct ProcessKillPromptArgs {}
