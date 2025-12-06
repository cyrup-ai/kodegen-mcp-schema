//! Prompt argument types for memory_memorize tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for memory_memorize tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct MemorizePromptArgs {
    /// Scenario to show examples for
    /// - "basic": Storing memories
    /// - "organization": Library organization
    /// - "patterns": Common memorization patterns
    /// - "workflows": Memory workflows
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
