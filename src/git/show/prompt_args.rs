//! Prompt argument types for git_show tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for git_show tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitShowPromptArgs {
    /// Scenario to show examples for
    /// - "commits": Show commit details
    /// - "tags": Show tag information
    /// - "files": Show file at commit
    /// - "formatting": Output format options
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
