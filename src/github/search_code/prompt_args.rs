//! Prompt argument types for github_search_code tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for github_search_code tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SearchCodePromptArgs {
    /// Scenario to show examples for
    /// - "basic": Simple code search
    /// - "syntax": Search syntax reference
    /// - "patterns": Common search patterns
    /// - "workflows": Research workflows
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
