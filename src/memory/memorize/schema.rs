//! Schema types for memory_memorize tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use kodegen_config::{CATEGORY_MEMORY, MEMORY_MEMORIZE};

// ============================================================================
// MEMORY MEMORIZE TOOL
// ============================================================================

/// Arguments for `memory_memorize` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct MemorizeArgs {
    /// Library name to store the memory in
    pub library: String,
    /// Content to memorize
    pub content: String,
}

// ============================================================================
// OUTPUT TYPES
// ============================================================================

/// Output from `memory_memorize` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct MemorizeOutput {
    /// Session ID for tracking async progress
    pub session_id: String,
    /// Current status: IN_PROGRESS, COMPLETED, FAILED
    pub status: String,
    /// Library name
    pub library: String,
    /// Human-readable message
    pub message: String,
}

// ============================================================================
// TOOL ARGS TRAIT IMPLEMENTATION
// ============================================================================

use crate::{ToolArgs, tool_metadata};
use super::prompts::MemorizePrompts;

#[tool_metadata(
    description = "Store content in a named memory library with automatic embedding generation. The memory will be tagged with the library name and can be retrieved later using recall(). Each library is a separate namespace for organizing memories."
)]
impl ToolArgs for MemorizeArgs {
    type Output = MemorizeOutput;
    type Prompts = MemorizePrompts;

    const NAME: &'static str = MEMORY_MEMORIZE;
    const CATEGORY: &'static str = CATEGORY_MEMORY;
    const DESCRIPTION: &'static str = "Store content in a named memory library with automatic embedding generation. The memory will be tagged with the library name and can be retrieved later using recall(). Each library is a separate namespace for organizing memories.";
}
