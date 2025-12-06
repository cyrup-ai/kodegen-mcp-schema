//! Prompt argument types for fs_create_directory tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for fs_create_directory tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FsCreateDirectoryPromptArgs {
    /// Scenario to show examples for
    /// - "basic": Simple directory creation
    /// - "nested": Creating nested paths
    /// - "project_setup": Creating project structures
    /// - "idempotent": Safe repeated calls
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
