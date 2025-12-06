//! Prompt argument types for git_init tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for git_init tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitInitPromptArgs {
    /// Scenario to show examples for
    /// - "basic": Simple repository creation
    /// - "options": Init with options
    /// - "bare": Bare repository creation
    /// - "workflows": Complete init workflows
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
