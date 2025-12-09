//! Schema types for memory_list_libraries tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use kodegen_config::{CATEGORY_MEMORY, MEMORY_LIST_LIBRARIES};

// ============================================================================
// MEMORY LIST LIBRARIES TOOL
// ============================================================================

/// Arguments for `memory_list_libraries` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ListMemoryLibrariesArgs {}

// ============================================================================
// OUTPUT TYPES
// ============================================================================

/// Output from `memory_list_libraries` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ListMemoryLibrariesOutput {
    /// List of library names
    pub libraries: Vec<String>,
    /// Number of libraries
    pub count: usize,
}

// ============================================================================
// TOOL ARGS TRAIT IMPLEMENTATION
// ============================================================================

use crate::{ToolArgs, tool_metadata};
use super::prompts::MemoryListLibrariesPrompts;

#[tool_metadata(
    description = "List all unique memory library names that have been created. Returns a list of all libraries that contain at least one memory. Use this to discover what libraries are available for recall."
)]
impl ToolArgs for ListMemoryLibrariesArgs {
    type Output = ListMemoryLibrariesOutput;
    type Prompts = MemoryListLibrariesPrompts;

    const NAME: &'static str = MEMORY_LIST_LIBRARIES;
    const CATEGORY: &'static kodegen_config::Category = CATEGORY_MEMORY;
    const DESCRIPTION: &'static str = "List all unique memory library names that have been created. Returns a list of all libraries that contain at least one memory. Use this to discover what libraries are available for recall.";
}
