//! Prompt argument types for git_history tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for `git_history` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitHistoryPromptArgs {
    /// Optional: Use case for customized examples
    /// - "basic": Viewing commit history
    /// - "filtering": Filtering history by author, date, etc.
    /// - "searching": Searching commit messages
    ///
    /// Default if omitted: comprehensive overview covering all aspects
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
