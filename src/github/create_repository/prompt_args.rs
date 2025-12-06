//! Prompt argument types for github_create_repository tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for github_create_repository tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CreateRepositoryPromptArgs {
    /// Scenario to show examples for
    /// - "basic": Simple repo creation
    /// - "options": Visibility, features
    /// - "organization": Org repos
    /// - "workflows": Complete repo setup
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
