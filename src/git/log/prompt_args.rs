//! Prompt argument types for git_log tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for git_log tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitLogPromptArgs {
    /// Scenario to show examples for
    /// - "basic": Simple log viewing
    /// - "filtering": Filter by file and directory
    /// - "pagination": Paginate through history
    /// - "file_history": Track file changes over time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
