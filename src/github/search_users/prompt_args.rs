//! Prompt argument types for github_user_search tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for github_user_search tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SearchUsersPromptArgs {
    /// Scenario to show examples for
    /// - "basic": Simple user search
    /// - "syntax": Search syntax reference
    /// - "workflows": User discovery workflows
    /// - "advanced": Advanced patterns (pagination, sorting, filtering)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
