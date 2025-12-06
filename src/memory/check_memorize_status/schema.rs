//! Schema types for memory_check_memorize_status tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// ============================================================================
// CANONICAL TOOL NAME CONSTANTS
// ============================================================================

/// Canonical tool name for memory_check_memorize_status
pub const MEMORY_CHECK_MEMORIZE_STATUS: &str = "memory_check_memorize_status";

// ============================================================================
// MEMORY CHECK MEMORIZE STATUS TOOL
// ============================================================================

/// Arguments for `memory_check_memorize_status` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CheckMemorizeStatusArgs {
    /// Session ID from memorize() call
    pub session_id: String,
}

// ============================================================================
// OUTPUT TYPES
// ============================================================================

/// Output from `memory_check_memorize_status` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CheckMemorizeStatusOutput {
    /// Session ID
    pub session_id: String,
    /// Current status: IN_PROGRESS, COMPLETED, FAILED
    pub status: String,
    /// Memory ID (present when completed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_id: Option<String>,
    /// Library name
    pub library: String,
    /// Progress information
    pub progress: MemorizeProgress,
    /// Runtime in milliseconds
    pub runtime_ms: u64,
    /// Error message (present when failed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Progress information for memorize operation
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct MemorizeProgress {
    /// Current stage
    pub stage: String,
    /// Number of files loaded
    pub files_loaded: usize,
    /// Total bytes processed
    pub total_size_bytes: usize,
}

// ============================================================================
// TOOL ARGS TRAIT IMPLEMENTATION
// ============================================================================

use crate::ToolArgs;
use super::prompts::CheckMemorizeStatusPrompts;

impl ToolArgs for CheckMemorizeStatusArgs {
    type Output = CheckMemorizeStatusOutput;
    type Prompts = CheckMemorizeStatusPrompts;

    const NAME: &'static str = MEMORY_CHECK_MEMORIZE_STATUS;
    const CATEGORY: &'static str = "memory";
    const DESCRIPTION: &'static str = "Check the status of a memorize operation. Use this to verify if an asynchronous memorization has completed, is still in progress, or has failed.";
}
