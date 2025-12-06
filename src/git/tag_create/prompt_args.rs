//! Prompt argument types for git_tag_create tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for git_tag_create tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitTagCreatePromptArgs {
    /// Scenario to show examples for
    /// - "annotated": Annotated tags with messages
    /// - "lightweight": Simple pointer tags
    /// - "versioning": Semantic version tags
    /// - "workflows": Release tagging workflows
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
