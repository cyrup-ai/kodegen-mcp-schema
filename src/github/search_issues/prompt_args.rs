//! Prompt argument types for github_search_issues tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for github_search_issues tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SearchIssuesPromptArgs {
    /// Scenario to show examples for
    /// - "basic": Simple issue search
    /// - "syntax": Search syntax reference
    /// - "patterns": Common search patterns
    /// - "workflows": Search workflows
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
