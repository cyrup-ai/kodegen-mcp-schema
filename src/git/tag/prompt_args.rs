//! Prompt argument types for git_tag tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for `git_tag` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitTagPromptArgs {
    /// Optional: Use case for customized examples
    /// - "create": Creating tags
    /// - "annotated": Annotated vs lightweight tags
    /// - "semver": Semantic versioning with tags
    /// - "list": Listing and managing tags
    ///
    /// Default if omitted: comprehensive overview covering all aspects
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
