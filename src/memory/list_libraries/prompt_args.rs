//! Prompt argument types for memory_list_libraries tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for memory_list_libraries tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct MemoryListLibrariesPromptArgs {
    /// Scenario to show examples for
    /// - "basic": Listing libraries
    /// - "workflows": Memory organization workflows
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
