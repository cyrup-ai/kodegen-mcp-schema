//! Prompt argument types for fs_search tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for fs_search tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FsSearchPromptArgs {
    /// Scenario to show examples for
    /// - "basic": Basic content and filename search
    /// - "patterns": Regex patterns and filtering
    /// - "options": Search options and modes
    /// - "background": Background search management
    /// - "workflows": Search workflows
    /// - "comprehensive": All scenarios combined
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
