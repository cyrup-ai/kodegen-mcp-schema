//! Prompt argument types for memory_recall tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for memory_recall tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct MemoryRecallPromptArgs {
    /// Scenario to show examples for
    /// - "basic": Basic recall
    /// - "semantic": Semantic search examples
    /// - "workflows": Recall in workflows
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
