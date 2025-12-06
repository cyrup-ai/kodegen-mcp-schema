//! Prompt argument types for git_tag_list tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for git_tag_list tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitTagListPromptArgs {
    /// Scenario to show examples for
    /// - "basic": Simple tag listing
    /// - "filtering": Filter and search tags
    /// - "sorting": Sort tags by version
    /// - "workflows": Tag listing workflows
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
