//! Prompt argument types for sequential_thinking tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for sequential_thinking tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SequentialThinkingPromptArgs {
    /// Scenario to show examples for
    /// - "basic": Basic sequential thoughts
    /// - "revision": Revising previous thoughts
    /// - "branching": Creating thought branches
    /// - "sessions": Managing thinking sessions
    /// - "patterns": Common thinking patterns
    /// - "comprehensive": All scenarios combined
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
