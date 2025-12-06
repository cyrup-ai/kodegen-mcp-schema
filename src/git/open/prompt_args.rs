//! Prompt argument types for git_open tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for `git_open` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitOpenPromptArgs {
    /// Optional: Use case for customized examples
    /// - "basic": Opening an existing repository
    /// - "status": Understanding repository state after opening
    /// - "vs_discover": When to use open vs discover
    ///
    /// Default if omitted: comprehensive overview covering all aspects
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
