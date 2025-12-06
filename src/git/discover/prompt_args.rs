//! Prompt argument types for git_discover tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for `git_discover` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitDiscoverPromptArgs {
    /// Optional: Use case for customized examples
    /// - "basic": Finding repository from subdirectory
    /// - "nested": Discovering in deeply nested paths
    /// - "monorepo": Working with monorepo structures
    ///
    /// Default if omitted: comprehensive overview covering all aspects
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
