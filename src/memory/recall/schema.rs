//! Schema types for memory_recall tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use kodegen_config::{CATEGORY_MEMORY, MEMORY_RECALL};

// ============================================================================
// DEFAULT HELPERS
// ============================================================================

fn default_recall_limit() -> usize {
    10
}

// ============================================================================
// MEMORY RECALL TOOL
// ============================================================================

/// Arguments for `memory_recall` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RecallArgs {
    /// Library name to search in
    pub library: String,
    /// Context/query to search for
    pub context: String,
    /// Maximum number of results (default: 10)
    #[serde(default = "default_recall_limit")]
    pub limit: usize,
}

// ============================================================================
// OUTPUT TYPES
// ============================================================================

/// Output from `memory_recall` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RecallOutput {
    /// Retrieved memories
    pub memories: Vec<RecalledMemory>,
    /// Library that was searched
    pub library: String,
    /// Number of results
    pub count: usize,
    /// Search time in milliseconds
    pub elapsed_ms: f64,
}

/// A single recalled memory
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RecalledMemory {
    /// Memory ID
    pub id: String,
    /// Memory content
    pub content: String,
    /// Creation timestamp
    pub created_at: String,
    /// Cosine similarity score (0-1)
    pub similarity: f32,
    /// Importance score
    pub importance: f32,
    /// Combined score (similarity × importance)
    pub score: f32,
    /// Rank in results (1-indexed)
    pub rank: usize,
}

// ============================================================================
// TOOL ARGS TRAIT IMPLEMENTATION
// ============================================================================

use crate::{ToolArgs, tool_metadata};
use super::prompts::MemoryRecallPrompts;

#[tool_metadata(
    description = "Retrieve relevant memories from a library using semantic search. Searches for content similar to the provided context and returns the most relevant results. Uses vector similarity (cosine) to find semantically related memories."
)]
impl ToolArgs for RecallArgs {
    type Output = RecallOutput;
    type Prompts = MemoryRecallPrompts;

    const NAME: &'static str = MEMORY_RECALL;
    const CATEGORY: &'static str = CATEGORY_MEMORY;
    const DESCRIPTION: &'static str = "Retrieve relevant memories from a library using semantic search. Searches for content similar to the provided context and returns the most relevant results. Uses vector similarity (cosine) to find semantically related memories.";
}
